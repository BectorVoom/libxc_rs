//! LDA_C_CHACHIYO_MOD lxc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_c_chachiyo_mod.c`
//! by tools/translate_rayon/from_maple.py, then rewritten to
//! `wide::f64x8` by simd.py. Eight grid points per step; every lane runs maple2c's expression
//! sequence in its original order.
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]
use libxc_rkernel_math::constants::*;
use libxc_rkernel_math::simd;
use libxc_rkernel_math::wide::{f64x8, CmpEq, CmpGe, CmpGt, CmpLe, CmpLt, CmpNe};

const V_ZERO: f64x8 = f64x8::new([0.0; 8]);
const V_ONE: f64x8 = f64x8::new([1.0; 8]);

// Transcendentals in exact mode come from `libxc_rkernel_math::simd`,
// which is bit-identical / correctly-rounded per lane to the scalar calls
// the scalar kernel makes. In exact mode, the SIMD kernel produces output
// bit-identical to its scalar form.

/// Load 8 consecutive grid points.
///
/// The tail is padded by repeating the last element, not by zero-filling:
/// these formulas divide by rho, so a zero lane would raise inf/NaN in lanes
/// whose results are then discarded -- harmless to the answer, but it makes
/// any real NaN impossible to spot while debugging.
#[inline(always)]
fn load(s: &[f64], ip: usize, np: usize) -> f64x8 {
    if ip + 8 <= np {
        let mut b = [0.0f64; 8];
        b.copy_from_slice(&s[ip..ip + 8]);
        f64x8::new(b)
    } else {
        let mut b = [s[np - 1]; 8];
        b[..np - ip].copy_from_slice(&s[ip..np]);
        f64x8::new(b)
    }
}

/// Accumulate 8 consecutive grid points into an output array.
///
/// `+=`, not `=`. The scalar kernel writes `out[ip] += v`; a plain store is a
/// different operation in two ways. It keeps the sign of a negative zero where
/// `0.0 + -0.0` gives `+0.0` -- a bit difference the fingerprint gate reports
/// as a rejection even though no value changed (`gga_x_pbepow fxc` was
/// rejected on exactly this, 273 of 200,000 `v2sigma2` elements) -- and it
/// would discard whatever a caller had already put in the buffer.
#[inline(always)]
fn store_add(s: &mut [f64], ip: usize, m: usize, acc: f64x8) {
    let a: [f64; 8] = acc.into();
    if m == 8 {
        let mut b = [0.0f64; 8];
        b.copy_from_slice(&s[ip..ip + 8]);
        let r: [f64; 8] = (f64x8::new(b) + acc).into();
        s[ip..ip + 8].copy_from_slice(&r);
    } else {
        for k in 0..m {
            s[ip + k] += a[k];
        }
    }
}

/// Load 8 elements with a given stride and offset.
#[inline(always)]
fn load_strided(s: &[f64], ip: usize, np: usize, stride: usize, offset: usize) -> f64x8 {
    let mut b = [0.0f64; 8];
    if ip + 8 <= np {
        let base = ip * stride + offset;
        b[0] = s[base];
        b[1] = s[base + stride];
        b[2] = s[base + 2 * stride];
        b[3] = s[base + 3 * stride];
        b[4] = s[base + 4 * stride];
        b[5] = s[base + 5 * stride];
        b[6] = s[base + 6 * stride];
        b[7] = s[base + 7 * stride];
    } else {
        for k in 0..8 {
            let p = (ip + k).min(np - 1);
            b[k] = s[p * stride + offset];
        }
    }
    f64x8::new(b)
}

/// Accumulate 8 elements with a given stride and offset.
///
/// `+=`, not `=`: the scalar kernel this was translated from writes
/// `out[ip * stride + offset] += v`, and a plain store is not the same
/// operation. It differs on the sign of zero -- `0.0 + -0.0` is `+0.0`
/// while a store of `-0.0` keeps the sign -- which is a bit difference
/// the fingerprint gate sees, and it would silently drop a caller's
/// existing contribution if one were ever there.
///
/// The read is not free on this path: a polarized `kxc`/`lxc` kernel
/// writes many strided outputs per point, and `lda_c_pw_erf kxc pol`
/// measured 84 -> 114 ns/pt (1.36x). It is charged anyway, because the
/// scalar kernel this is compared against does the same read. Gathering
/// into a vector, adding once and scattering back was tried and is no
/// faster (117 ns/pt), so the cost is the load itself, not scheduling.
#[inline(always)]
fn store_strided(s: &mut [f64], ip: usize, m: usize, stride: usize, offset: usize, acc: f64x8) {
    let a: [f64; 8] = acc.into();
    if m == 8 {
        let base = ip * stride + offset;
        s[base] += a[0];
        s[base + stride] += a[1];
        s[base + 2 * stride] += a[2];
        s[base + 3 * stride] += a[3];
        s[base + 4 * stride] += a[4];
        s[base + 5 * stride] += a[5];
        s[base + 6 * stride] += a[6];
        s[base + 7 * stride] += a[7];
    } else {
        for k in 0..m {
            s[(ip + k) * stride + offset] += a[k];
        }
    }
}

#[allow(unused_variables, non_snake_case)]
pub fn lda_c_chachiyo_mod_lxc_pol(
    rho: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    v2rho2: &mut [f64],
    v3rho3: &mut [f64],
    v4rho4: &mut [f64],
    param_af: f64,
    param_ap: f64,
    param_bf: f64,
    param_bp: f64,
    param_cf: f64,
    param_cp: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_af = f64x8::splat(param_af);
    let param_ap = f64x8::splat(param_ap);
    let param_bf = f64x8::splat(param_bf);
    let param_bp = f64x8::splat(param_bp);
    let param_cf = f64x8::splat(param_cf);
    let param_cp = f64x8::splat(param_cp);
    let dens_threshold = f64x8::splat(dens_threshold);
    let zeta_threshold = f64x8::splat(zeta_threshold);
    let mut ip = 0usize;
    while ip < np {
        let m = (np - ip).min(8);
        let v_rho0 = load_strided(rho, ip, np, 2, 0);
        let v_rho1 = load_strided(rho, ip, np, 2, 1);
        let mut acc_zk = V_ZERO;
        let mut acc_vrho_0 = V_ZERO;
        let mut acc_vrho_1 = V_ZERO;
        let mut acc_v2rho2_0 = V_ZERO;
        let mut acc_v2rho2_1 = V_ZERO;
        let mut acc_v2rho2_2 = V_ZERO;
        let mut acc_v3rho3_0 = V_ZERO;
        let mut acc_v3rho3_1 = V_ZERO;
        let mut acc_v3rho3_2 = V_ZERO;
        let mut acc_v3rho3_3 = V_ZERO;
        let mut acc_v4rho4_0 = V_ZERO;
        let mut acc_v4rho4_1 = V_ZERO;
        let mut acc_v4rho4_2 = V_ZERO;
        let mut acc_v4rho4_3 = V_ZERO;
        let mut acc_v4rho4_4 = V_ZERO;
        {
            let t1 = f64x8::splat(M_CBRT3);
            let t2 = t1 * t1;
            let t3 = param_bp * t2;
            let t5 = (simd::cbrt(f64x8::splat(1.0) / f64x8::splat(M_PI)));
            let t7 = f64x8::splat(M_CBRT4);
            let t8 = f64x8::splat(1.0) / t5 * t7;
            let t9 = v_rho0 + v_rho1;
            let t10 = (simd::cbrt(t9));
            let t11 = t8 * t10;
            let t14 = param_cp * t1;
            let t15 = t5 * t5;
            let t17 = t7 * t7;
            let t18 = f64x8::splat(1.0) / t15 * t17;
            let t19 = t10 * t10;
            let t20 = t18 * t19;
            let t23 = f64x8::splat(1.0) + t3 * t11 / f64x8::splat(3.0) + t14 * t20 / f64x8::splat(3.0);
            let t24 = (simd::ln(t23));
            let t25 = param_ap * t24;
            let t26 = param_bf * t2;
            let t29 = param_cf * t1;
            let t32 = f64x8::splat(1.0) + t26 * t11 / f64x8::splat(3.0) + t29 * t20 / f64x8::splat(3.0);
            let t33 = (simd::ln(t32));
            let t35 = param_af * t33 - t25;
            let t36 = v_rho0 - v_rho1;
            let t37 = f64x8::splat(1.0) / t9;
            let t38 = t36 * t37;
            let t39 = f64x8::splat(1.0) + t38;
            let t40 = (t39).simd_le(zeta_threshold);
            let t41 = (simd::cbrt(zeta_threshold));
            let t42 = t41 * t41;
            let t43 = (simd::cbrt(t39));
            let t44 = t43 * t43;
            let t45 = ((t40).select(t42, t44));
            let t46 = f64x8::splat(1.0) - t38;
            let t47 = (t46).simd_le(zeta_threshold);
            let t48 = (simd::cbrt(t46));
            let t49 = t48 * t48;
            let t50 = ((t47).select(t42, t49));
            let t52 = t45 / f64x8::splat(2.0) + t50 / f64x8::splat(2.0);
            let t53 = t52 * t52;
            let t56 = -f64x8::splat(2.0) * t53 * t52 + f64x8::splat(2.0);
            let t57 = t35 * t56;
            let tzk0 = t25 + t57;
            acc_zk = tzk0;
            let t59 = t8 / t19;
            let t63 = t18 / t10;
            let t66 = t3 * t59 / f64x8::splat(9.0) + f64x8::splat(2.0) / f64x8::splat(9.0) * t14 * t63;
            let t68 = f64x8::splat(1.0) / t23;
            let t69 = param_ap * t66 * t68;
            let t74 = t26 * t59 / f64x8::splat(9.0) + f64x8::splat(2.0) / f64x8::splat(9.0) * t29 * t63;
            let t76 = f64x8::splat(1.0) / t32;
            let t78 = param_af * t74 * t76 - t69;
            let t79 = t78 * t56;
            let t80 = t35 * t53;
            let t81 = f64x8::splat(1.0) / t43;
            let t82 = t9 * t9;
            let t83 = f64x8::splat(1.0) / t82;
            let t84 = t36 * t83;
            let t85 = t37 - t84;
            let t88 = ((t40).select(f64x8::splat(0.0), f64x8::splat(2.0) / f64x8::splat(3.0) * t81 * t85));
            let t89 = f64x8::splat(1.0) / t48;
            let t90 = -t85;
            let t93 = ((t47).select(f64x8::splat(0.0), f64x8::splat(2.0) / f64x8::splat(3.0) * t89 * t90));
            let t95 = t88 / f64x8::splat(2.0) + t93 / f64x8::splat(2.0);
            let t96 = t80 * t95;
            let t97 = f64x8::splat(6.0) * t96;
            let tvrho0 = t25 + t57 + t9 * (t69 + t79 - t97);
            acc_vrho_0 = tvrho0;
            let t100 = -t37 - t84;
            let t103 = ((t40).select(f64x8::splat(0.0), f64x8::splat(2.0) / f64x8::splat(3.0) * t81 * t100));
            let t104 = -t100;
            let t107 = ((t47).select(f64x8::splat(0.0), f64x8::splat(2.0) / f64x8::splat(3.0) * t89 * t104));
            let t109 = t103 / f64x8::splat(2.0) + t107 / f64x8::splat(2.0);
            let t110 = t80 * t109;
            let t111 = f64x8::splat(6.0) * t110;
            let tvrho1 = t25 + t57 + t9 * (t69 + t79 - t111);
            acc_vrho_1 = tvrho1;
            let t114 = f64x8::splat(2.0) * t69;
            let t115 = f64x8::splat(2.0) * t79;
            let t119 = t8 / t19 / t9;
            let t123 = t18 / t10 / t9;
            let t126 = -f64x8::splat(2.0) / f64x8::splat(27.0) * t3 * t119 - f64x8::splat(2.0) / f64x8::splat(27.0) * t14 * t123;
            let t127 = param_ap * t126;
            let t128 = t127 * t68;
            let t129 = t66 * t66;
            let t131 = t23 * t23;
            let t132 = f64x8::splat(1.0) / t131;
            let t133 = param_ap * t129 * t132;
            let t137 = -f64x8::splat(2.0) / f64x8::splat(27.0) * t26 * t119 - f64x8::splat(2.0) / f64x8::splat(27.0) * t29 * t123;
            let t138 = param_af * t137;
            let t140 = t74 * t74;
            let t142 = t32 * t32;
            let t143 = f64x8::splat(1.0) / t142;
            let t145 = -param_af * t140 * t143 + t138 * t76 - t128 + t133;
            let t146 = t145 * t56;
            let t147 = t78 * t53;
            let t148 = t147 * t95;
            let t149 = f64x8::splat(12.0) * t148;
            let t150 = t35 * t52;
            let t151 = t95 * t95;
            let t152 = t150 * t151;
            let t153 = f64x8::splat(12.0) * t152;
            let t155 = f64x8::splat(1.0) / t43 / t39;
            let t156 = t85 * t85;
            let t159 = t82 * t9;
            let t160 = f64x8::splat(1.0) / t159;
            let t161 = t36 * t160;
            let t163 = -f64x8::splat(2.0) * t83 + f64x8::splat(2.0) * t161;
            let t167 = ((t40).select(f64x8::splat(0.0), -f64x8::splat(2.0) / f64x8::splat(9.0) * t155 * t156 + f64x8::splat(2.0) / f64x8::splat(3.0) * t81 * t163));
            let t169 = f64x8::splat(1.0) / t48 / t46;
            let t170 = t90 * t90;
            let t173 = -t163;
            let t177 = ((t47).select(f64x8::splat(0.0), -f64x8::splat(2.0) / f64x8::splat(9.0) * t169 * t170 + f64x8::splat(2.0) / f64x8::splat(3.0) * t89 * t173));
            let t179 = t167 / f64x8::splat(2.0) + t177 / f64x8::splat(2.0);
            let t180 = t80 * t179;
            let t181 = f64x8::splat(6.0) * t180;
            let tv2rho20 = t114 + t115 - f64x8::splat(12.0) * t96 + t9 * (t128 - t133 + t146 - t149 - t153 - t181);
            acc_v2rho2_0 = tv2rho20;
            let t185 = t147 * t109;
            let t187 = t109 * t95;
            let t188 = t150 * t187;
            let t190 = t155 * t100;
            let t193 = t81 * t36;
            let t197 = ((t40).select(f64x8::splat(0.0), -f64x8::splat(2.0) / f64x8::splat(9.0) * t190 * t85 + f64x8::splat(4.0) / f64x8::splat(3.0) * t193 * t160));
            let t198 = t169 * t104;
            let t201 = t89 * t36;
            let t205 = ((t47).select(f64x8::splat(0.0), -f64x8::splat(2.0) / f64x8::splat(9.0) * t198 * t90 - f64x8::splat(4.0) / f64x8::splat(3.0) * t201 * t160));
            let t207 = t197 / f64x8::splat(2.0) + t205 / f64x8::splat(2.0);
            let t208 = t80 * t207;
            let tv2rho21 = t114 + t115 - t97 - t111 + t9 * (t128 - t133 + t146 - f64x8::splat(6.0) * t148 - f64x8::splat(6.0) * t185 - f64x8::splat(12.0) * t188 - f64x8::splat(6.0) * t208);
            acc_v2rho2_1 = tv2rho21;
            let t213 = f64x8::splat(12.0) * t185;
            let t214 = t109 * t109;
            let t215 = t150 * t214;
            let t216 = f64x8::splat(12.0) * t215;
            let t217 = t100 * t100;
            let t221 = f64x8::splat(2.0) * t83 + f64x8::splat(2.0) * t161;
            let t225 = ((t40).select(f64x8::splat(0.0), -f64x8::splat(2.0) / f64x8::splat(9.0) * t155 * t217 + f64x8::splat(2.0) / f64x8::splat(3.0) * t81 * t221));
            let t226 = t104 * t104;
            let t229 = -t221;
            let t233 = ((t47).select(f64x8::splat(0.0), -f64x8::splat(2.0) / f64x8::splat(9.0) * t169 * t226 + f64x8::splat(2.0) / f64x8::splat(3.0) * t89 * t229));
            let t235 = t225 / f64x8::splat(2.0) + t233 / f64x8::splat(2.0);
            let t236 = t80 * t235;
            let t237 = f64x8::splat(6.0) * t236;
            let tv2rho22 = t114 + t115 - f64x8::splat(12.0) * t110 + t9 * (t128 - t133 + t146 - t213 - t216 - t237);
            acc_v2rho2_2 = tv2rho22;
            let t240 = f64x8::splat(3.0) * t128;
            let t241 = f64x8::splat(3.0) * t133;
            let t242 = f64x8::splat(3.0) * t146;
            let t248 = t8 / t19 / t82;
            let t253 = t18 / t10 / t82;
            let t257 = param_ap * (f64x8::splat(10.0) / f64x8::splat(81.0) * t3 * t248 + f64x8::splat(8.0) / f64x8::splat(81.0) * t14 * t253);
            let t258 = t257 * t68;
            let t259 = t132 * t66;
            let t260 = t127 * t259;
            let t261 = f64x8::splat(3.0) * t260;
            let t265 = f64x8::splat(1.0) / t131 / t23;
            let t266 = param_ap * t129 * t66 * t265;
            let t267 = f64x8::splat(2.0) * t266;
            let t273 = param_af * (f64x8::splat(10.0) / f64x8::splat(81.0) * t26 * t248 + f64x8::splat(8.0) / f64x8::splat(81.0) * t29 * t253);
            let t275 = t143 * t74;
            let t281 = f64x8::splat(1.0) / t142 / t32;
            let t284 = f64x8::splat(2.0) * param_af * t140 * t74 * t281 - f64x8::splat(3.0) * t138 * t275 + t273 * t76 - t258 + t261 - t267;
            let t285 = t284 * t56;
            let t286 = t145 * t53;
            let t287 = t286 * t95;
            let t288 = f64x8::splat(18.0) * t287;
            let t289 = t78 * t52;
            let t290 = t289 * t151;
            let t292 = t147 * t179;
            let t294 = t151 * t95;
            let t295 = t35 * t294;
            let t296 = f64x8::splat(12.0) * t295;
            let t297 = t95 * t179;
            let t298 = t150 * t297;
            let t299 = f64x8::splat(36.0) * t298;
            let t300 = t39 * t39;
            let t302 = f64x8::splat(1.0) / t43 / t300;
            let t303 = t156 * t85;
            let t306 = t155 * t85;
            let t309 = t82 * t82;
            let t310 = f64x8::splat(1.0) / t309;
            let t311 = t36 * t310;
            let t313 = f64x8::splat(6.0) * t160 - f64x8::splat(6.0) * t311;
            let t317 = ((t40).select(f64x8::splat(0.0), f64x8::splat(8.0) / f64x8::splat(27.0) * t302 * t303 - f64x8::splat(2.0) / f64x8::splat(3.0) * t306 * t163 + f64x8::splat(2.0) / f64x8::splat(3.0) * t81 * t313));
            let t318 = t46 * t46;
            let t320 = f64x8::splat(1.0) / t48 / t318;
            let t321 = t170 * t90;
            let t324 = t169 * t90;
            let t327 = -t313;
            let t331 = ((t47).select(f64x8::splat(0.0), f64x8::splat(8.0) / f64x8::splat(27.0) * t320 * t321 - f64x8::splat(2.0) / f64x8::splat(3.0) * t324 * t173 + f64x8::splat(2.0) / f64x8::splat(3.0) * t89 * t327));
            let t333 = t317 / f64x8::splat(2.0) + t331 / f64x8::splat(2.0);
            let t334 = t80 * t333;
            let t335 = f64x8::splat(6.0) * t334;
            let tv3rho30 = t240 - t241 + t242 - f64x8::splat(36.0) * t148 - f64x8::splat(36.0) * t152 - f64x8::splat(18.0) * t180 + t9 * (t258 - t261 + t267 + t285 - t288 - f64x8::splat(36.0) * t290 - f64x8::splat(18.0) * t292 - t296 - t299 - t335);
            acc_v3rho3_0 = tv3rho30;
            let t339 = f64x8::splat(24.0) * t188;
            let t340 = f64x8::splat(12.0) * t208;
            let t344 = t286 * t109;
            let t346 = t289 * t187;
            let t347 = f64x8::splat(24.0) * t346;
            let t348 = t147 * t207;
            let t349 = f64x8::splat(12.0) * t348;
            let t350 = t35 * t151;
            let t351 = t350 * t109;
            let t353 = t207 * t95;
            let t354 = t150 * t353;
            let t356 = t109 * t179;
            let t357 = t150 * t356;
            let t359 = t302 * t100;
            let t362 = t155 * t36;
            let t373 = ((t40).select(f64x8::splat(0.0), f64x8::splat(8.0) / f64x8::splat(27.0) * t359 * t156 - f64x8::splat(8.0) / f64x8::splat(9.0) * t362 * t160 * t85 - f64x8::splat(2.0) / f64x8::splat(9.0) * t190 * t163 + f64x8::splat(4.0) / f64x8::splat(3.0) * t81 * t160 - f64x8::splat(4.0) * t193 * t310));
            let t374 = t320 * t104;
            let t377 = t169 * t36;
            let t388 = ((t47).select(f64x8::splat(0.0), f64x8::splat(8.0) / f64x8::splat(27.0) * t374 * t170 + f64x8::splat(8.0) / f64x8::splat(9.0) * t377 * t160 * t90 - f64x8::splat(2.0) / f64x8::splat(9.0) * t198 * t173 - f64x8::splat(4.0) / f64x8::splat(3.0) * t89 * t160 + f64x8::splat(4.0) * t201 * t310));
            let t390 = t373 / f64x8::splat(2.0) + t388 / f64x8::splat(2.0);
            let t391 = t80 * t390;
            let t393 = t258 - t261 + t267 + t285 - f64x8::splat(12.0) * t287 - f64x8::splat(12.0) * t290 - f64x8::splat(6.0) * t292 - f64x8::splat(6.0) * t344 - t347 - t349 - f64x8::splat(12.0) * t351 - f64x8::splat(24.0) * t354 - f64x8::splat(12.0) * t357 - f64x8::splat(6.0) * t391;
            let tv3rho31 = t9 * t393 - f64x8::splat(24.0) * t148 - t153 - t181 - t213 + t240 - t241 + t242 - t339 - t340;
            acc_v3rho3_1 = tv3rho31;
            let t398 = t289 * t214;
            let t400 = t35 * t95;
            let t401 = t400 * t214;
            let t403 = t109 * t207;
            let t404 = t150 * t403;
            let t406 = t147 * t235;
            let t408 = t235 * t95;
            let t409 = t150 * t408;
            let t411 = t302 * t217;
            let t416 = t155 * t221;
            let t421 = -f64x8::splat(2.0) * t160 - f64x8::splat(6.0) * t311;
            let t425 = ((t40).select(f64x8::splat(0.0), f64x8::splat(8.0) / f64x8::splat(27.0) * t411 * t85 - f64x8::splat(8.0) / f64x8::splat(9.0) * t190 * t161 - f64x8::splat(2.0) / f64x8::splat(9.0) * t416 * t85 + f64x8::splat(2.0) / f64x8::splat(3.0) * t81 * t421));
            let t426 = t320 * t226;
            let t431 = t169 * t229;
            let t434 = -t421;
            let t438 = ((t47).select(f64x8::splat(0.0), f64x8::splat(8.0) / f64x8::splat(27.0) * t426 * t90 + f64x8::splat(8.0) / f64x8::splat(9.0) * t198 * t161 - f64x8::splat(2.0) / f64x8::splat(9.0) * t431 * t90 + f64x8::splat(2.0) / f64x8::splat(3.0) * t89 * t434));
            let t440 = t425 / f64x8::splat(2.0) + t438 / f64x8::splat(2.0);
            let t441 = t80 * t440;
            let t443 = t258 - t261 + t267 + t285 - f64x8::splat(6.0) * t287 - f64x8::splat(12.0) * t344 - t347 - t349 - f64x8::splat(12.0) * t398 - f64x8::splat(12.0) * t401 - f64x8::splat(24.0) * t404 - f64x8::splat(6.0) * t406 - f64x8::splat(12.0) * t409 - f64x8::splat(6.0) * t441;
            let tv3rho32 = t9 * t443 - t149 - f64x8::splat(24.0) * t185 - t216 - t237 + t240 - t241 + t242 - t339 - t340;
            acc_v3rho3_2 = tv3rho32;
            let t448 = f64x8::splat(18.0) * t344;
            let t451 = t214 * t109;
            let t452 = t35 * t451;
            let t453 = f64x8::splat(12.0) * t452;
            let t454 = t109 * t235;
            let t455 = t150 * t454;
            let t456 = f64x8::splat(36.0) * t455;
            let t457 = t217 * t100;
            let t463 = -f64x8::splat(6.0) * t160 - f64x8::splat(6.0) * t311;
            let t467 = ((t40).select(f64x8::splat(0.0), f64x8::splat(8.0) / f64x8::splat(27.0) * t302 * t457 - f64x8::splat(2.0) / f64x8::splat(3.0) * t190 * t221 + f64x8::splat(2.0) / f64x8::splat(3.0) * t81 * t463));
            let t468 = t226 * t104;
            let t473 = -t463;
            let t477 = ((t47).select(f64x8::splat(0.0), f64x8::splat(8.0) / f64x8::splat(27.0) * t320 * t468 - f64x8::splat(2.0) / f64x8::splat(3.0) * t198 * t229 + f64x8::splat(2.0) / f64x8::splat(3.0) * t89 * t473));
            let t479 = t467 / f64x8::splat(2.0) + t477 / f64x8::splat(2.0);
            let t480 = t80 * t479;
            let t481 = f64x8::splat(6.0) * t480;
            let tv3rho33 = t240 - t241 + t242 - f64x8::splat(36.0) * t185 - f64x8::splat(36.0) * t215 - f64x8::splat(18.0) * t236 + t9 * (t258 - t261 + t267 + t285 - t448 - f64x8::splat(36.0) * t398 - f64x8::splat(18.0) * t406 - t453 - t456 - t481);
            acc_v3rho3_3 = tv3rho33;
            let t484 = f64x8::splat(12.0) * t260;
            let t485 = f64x8::splat(8.0) * t266;
            let t488 = f64x8::splat(4.0) * t258;
            let t493 = f64x8::splat(4.0) * t285;
            let t496 = f64x8::splat(12.0) * t127 * t265 * t129;
            let t498 = f64x8::splat(4.0) * t257 * t259;
            let t499 = t126 * t126;
            let t502 = f64x8::splat(3.0) * param_ap * t499 * t132;
            let t503 = t129 * t129;
            let t505 = t131 * t131;
            let t508 = f64x8::splat(6.0) * param_ap * t503 / t505;
            let t509 = t145 * t52;
            let t510 = t509 * t151;
            let t512 = t289 * t297;
            let t514 = t179 * t179;
            let t522 = t8 / t19 / t159;
            let t527 = t18 / t10 / t159;
            let t532 = param_ap * (-f64x8::splat(80.0) / f64x8::splat(243.0) * t3 * t522 - f64x8::splat(56.0) / f64x8::splat(243.0) * t14 * t527) * t68;
            let t533 = t284 * t53;
            let t534 = t533 * t95;
            let t536 = t286 * t179;
            let t538 = t78 * t294;
            let t540 = t147 * t333;
            let t546 = f64x8::splat(1.0) / t43 / t300 / t39;
            let t547 = t156 * t156;
            let t553 = t163 * t163;
            let t559 = f64x8::splat(1.0) / t309 / t9;
            let t560 = t36 * t559;
            let t562 = -f64x8::splat(24.0) * t310 + f64x8::splat(24.0) * t560;
            let t566 = ((t40).select(f64x8::splat(0.0), -f64x8::splat(56.0) / f64x8::splat(81.0) * t546 * t547 + f64x8::splat(16.0) / f64x8::splat(9.0) * t302 * t156 * t163 - f64x8::splat(2.0) / f64x8::splat(3.0) * t155 * t553 - f64x8::splat(8.0) / f64x8::splat(9.0) * t306 * t313 + f64x8::splat(2.0) / f64x8::splat(3.0) * t81 * t562));
            let t569 = f64x8::splat(1.0) / t48 / t318 / t46;
            let t570 = t170 * t170;
            let t576 = t173 * t173;
            let t585 = ((t47).select(f64x8::splat(0.0), -f64x8::splat(56.0) / f64x8::splat(81.0) * t569 * t570 + f64x8::splat(16.0) / f64x8::splat(9.0) * t320 * t170 * t173 - f64x8::splat(2.0) / f64x8::splat(3.0) * t169 * t576 - f64x8::splat(8.0) / f64x8::splat(9.0) * t324 * t327 - f64x8::splat(2.0) / f64x8::splat(3.0) * t89 * t562));
            let t602 = t137 * t137;
            let t606 = t140 * t140;
            let t608 = t142 * t142;
            let t613 = (param_af * (-f64x8::splat(80.0) / f64x8::splat(243.0) * t26 * t522 - f64x8::splat(56.0) / f64x8::splat(243.0) * t29 * t527) * t76 - f64x8::splat(4.0) * t273 * t275 + f64x8::splat(12.0) * t138 * t281 * t140 - f64x8::splat(3.0) * param_af * t602 * t143 - f64x8::splat(6.0) * param_af * t606 / t608 - t532 + t498 - t496 + t502 + t508) * t56;
            let t614 = t496 - t498 - t502 - t508 - f64x8::splat(72.0) * t510 - f64x8::splat(144.0) * t512 - f64x8::splat(36.0) * t150 * t514 - f64x8::splat(48.0) * t150 * t95 * t333 + t532 - f64x8::splat(24.0) * t534 - f64x8::splat(36.0) * t536 - f64x8::splat(48.0) * t538 - f64x8::splat(24.0) * t540 - f64x8::splat(72.0) * t350 * t179 - f64x8::splat(6.0) * t80 * (t566 / f64x8::splat(2.0) + t585 / f64x8::splat(2.0)) + t613;
            let tv4rho40 = t9 * t614 - f64x8::splat(72.0) * t287 - f64x8::splat(144.0) * t290 - f64x8::splat(72.0) * t292 - f64x8::splat(48.0) * t295 - f64x8::splat(144.0) * t298 - f64x8::splat(24.0) * t334 - t484 + t485 + t488 + t493;
            acc_v4rho4_0 = tv4rho40;
            let t617 = f64x8::splat(72.0) * t346;
            let t623 = f64x8::splat(36.0) * t348;
            let t626 = t147 * t390;
            let t654 = f64x8::splat(16.0) * t193 * t559;
            let t656 = ((t40).select(f64x8::splat(0.0), -f64x8::splat(56.0) / f64x8::splat(81.0) * t546 * t100 * t303 + f64x8::splat(16.0) / f64x8::splat(9.0) * t302 * t36 * t160 * t156 + f64x8::splat(8.0) / f64x8::splat(9.0) * t359 * t85 * t163 - f64x8::splat(4.0) / f64x8::splat(3.0) * t155 * t160 * t85 + f64x8::splat(4.0) * t362 * t310 * t85 - f64x8::splat(4.0) / f64x8::splat(3.0) * t362 * t160 * t163 - f64x8::splat(2.0) / f64x8::splat(9.0) * t190 * t313 - f64x8::splat(8.0) * t81 * t310 + t654));
            let t681 = f64x8::splat(16.0) * t201 * t559;
            let t683 = ((t47).select(f64x8::splat(0.0), -f64x8::splat(56.0) / f64x8::splat(81.0) * t569 * t104 * t321 - f64x8::splat(16.0) / f64x8::splat(9.0) * t320 * t36 * t160 * t170 + f64x8::splat(8.0) / f64x8::splat(9.0) * t374 * t90 * t173 + f64x8::splat(4.0) / f64x8::splat(3.0) * t169 * t160 * t90 - f64x8::splat(4.0) * t377 * t310 * t90 + f64x8::splat(4.0) / f64x8::splat(3.0) * t377 * t160 * t173 - f64x8::splat(2.0) / f64x8::splat(9.0) * t198 * t327 + f64x8::splat(8.0) * t89 * t310 - t681));
            let t689 = t78 * t151 * t109;
            let t691 = t533 * t109;
            let t693 = t286 * t207;
            let t694 = f64x8::splat(18.0) * t693;
            let t697 = t613 - f64x8::splat(12.0) * t538 - f64x8::splat(18.0) * t626 - f64x8::splat(36.0) * t350 * t207 - f64x8::splat(6.0) * t80 * (t656 / f64x8::splat(2.0) + t683 / f64x8::splat(2.0)) - f64x8::splat(36.0) * t689 - f64x8::splat(6.0) * t691 - t694 - f64x8::splat(6.0) * t540 - t502 - t508 - f64x8::splat(36.0) * t510;
            let t701 = t509 * t187;
            let t702 = f64x8::splat(36.0) * t701;
            let t703 = t289 * t353;
            let t705 = t289 * t356;
            let t718 = -f64x8::splat(12.0) * t150 * t109 * t333 - f64x8::splat(36.0) * t150 * t207 * t179 - f64x8::splat(36.0) * t150 * t390 * t95 - f64x8::splat(36.0) * t400 * t356 + t496 - t498 - f64x8::splat(36.0) * t512 + t532 - f64x8::splat(18.0) * t534 - f64x8::splat(18.0) * t536 - t702 - f64x8::splat(72.0) * t703 - f64x8::splat(36.0) * t705;
            let tv4rho41 = -t484 + t485 - f64x8::splat(72.0) * t290 - t299 - t617 - f64x8::splat(36.0) * t351 - f64x8::splat(72.0) * t354 - f64x8::splat(36.0) * t357 + t488 - f64x8::splat(54.0) * t287 - f64x8::splat(36.0) * t292 - t296 - t335 - t448 - t623 - f64x8::splat(18.0) * t391 + t493 + t9 * (t697 + t718);
            acc_v4rho4_1 = tv4rho41;
            let t747 = t36 * t36;
            let t750 = f64x8::splat(1.0) / t309 / t82;
            let t766 = ((t40).select(f64x8::splat(0.0), -f64x8::splat(56.0) / f64x8::splat(81.0) * t546 * t217 * t156 + f64x8::splat(64.0) / f64x8::splat(27.0) * t359 * t85 * t36 * t160 + f64x8::splat(8.0) / f64x8::splat(27.0) * t411 * t163 - f64x8::splat(16.0) / f64x8::splat(9.0) * t155 * t747 * t750 - f64x8::splat(8.0) / f64x8::splat(9.0) * t190 * t160 + f64x8::splat(8.0) / f64x8::splat(3.0) * t190 * t311 + f64x8::splat(8.0) / f64x8::splat(27.0) * t302 * t221 * t156 - f64x8::splat(4.0) / f64x8::splat(9.0) * t155 * t421 * t85 - f64x8::splat(2.0) / f64x8::splat(9.0) * t416 * t163 + t654));
            let t792 = ((t47).select(f64x8::splat(0.0), -f64x8::splat(56.0) / f64x8::splat(81.0) * t569 * t226 * t170 - f64x8::splat(64.0) / f64x8::splat(27.0) * t374 * t90 * t36 * t160 + f64x8::splat(8.0) / f64x8::splat(27.0) * t426 * t173 - f64x8::splat(16.0) / f64x8::splat(9.0) * t169 * t747 * t750 + f64x8::splat(8.0) / f64x8::splat(9.0) * t198 * t160 - f64x8::splat(8.0) / f64x8::splat(3.0) * t198 * t311 + f64x8::splat(8.0) / f64x8::splat(27.0) * t320 * t229 * t170 - f64x8::splat(4.0) / f64x8::splat(9.0) * t169 * t434 * t90 - f64x8::splat(2.0) / f64x8::splat(9.0) * t431 * t173 - t681));
            let t797 = t289 * t403;
            let t804 = t289 * t408;
            let t812 = t509 * t214;
            let t815 = t78 * t95 * t214;
            let t820 = t286 * t235;
            let t822 = t147 * t440;
            let t824 = t207 * t207;
            let t829 = -f64x8::splat(6.0) * t80 * (t766 / f64x8::splat(2.0) + t792 / f64x8::splat(2.0)) - f64x8::splat(48.0) * t797 - f64x8::splat(48.0) * t400 * t403 - f64x8::splat(24.0) * t150 * t109 * t390 - f64x8::splat(24.0) * t804 - f64x8::splat(24.0) * t150 * t440 * t95 - f64x8::splat(12.0) * t150 * t235 * t179 + t613 - f64x8::splat(12.0) * t812 - f64x8::splat(24.0) * t815 - f64x8::splat(12.0) * t35 * t179 * t214 - f64x8::splat(6.0) * t820 - f64x8::splat(12.0) * t822 - f64x8::splat(24.0) * t150 * t824 - f64x8::splat(12.0) * t350 * t235;
            let t840 = -f64x8::splat(12.0) * t626 - f64x8::splat(24.0) * t689 - f64x8::splat(12.0) * t691 - f64x8::splat(24.0) * t693 - t502 - t508 - f64x8::splat(12.0) * t510 + t532 - f64x8::splat(12.0) * t534 - f64x8::splat(6.0) * t536 + t496 - t498 - f64x8::splat(48.0) * t701 - f64x8::splat(48.0) * t703 - f64x8::splat(24.0) * t705;
            let t843 = -f64x8::splat(36.0) * t287 - f64x8::splat(12.0) * t292 - f64x8::splat(36.0) * t344 - f64x8::splat(48.0) * t348 - f64x8::splat(12.0) * t391 - f64x8::splat(24.0) * t398 - f64x8::splat(24.0) * t401 - f64x8::splat(12.0) * t406 - f64x8::splat(12.0) * t441 + t493 + t9 * (t829 + t840);
            let tv4rho42 = -t484 + t485 - f64x8::splat(24.0) * t290 - f64x8::splat(96.0) * t346 - f64x8::splat(24.0) * t351 - f64x8::splat(48.0) * t354 - f64x8::splat(24.0) * t357 - f64x8::splat(48.0) * t404 - f64x8::splat(24.0) * t409 + t488 + t843;
            acc_v4rho4_2 = tv4rho42;
            let t869 = f64x8::splat(12.0) * t310 + f64x8::splat(24.0) * t560;
            let t873 = ((t40).select(f64x8::splat(0.0), -f64x8::splat(56.0) / f64x8::splat(81.0) * t546 * t457 * t85 + f64x8::splat(16.0) / f64x8::splat(9.0) * t411 * t161 + f64x8::splat(8.0) / f64x8::splat(9.0) * t359 * t221 * t85 - f64x8::splat(4.0) / f64x8::splat(3.0) * t362 * t160 * t221 - f64x8::splat(2.0) / f64x8::splat(3.0) * t190 * t421 - f64x8::splat(2.0) / f64x8::splat(9.0) * t155 * t463 * t85 + f64x8::splat(2.0) / f64x8::splat(3.0) * t81 * t869));
            let t894 = ((t47).select(f64x8::splat(0.0), -f64x8::splat(56.0) / f64x8::splat(81.0) * t569 * t468 * t90 - f64x8::splat(16.0) / f64x8::splat(9.0) * t426 * t161 + f64x8::splat(8.0) / f64x8::splat(9.0) * t374 * t229 * t90 + f64x8::splat(4.0) / f64x8::splat(3.0) * t377 * t160 * t229 - f64x8::splat(2.0) / f64x8::splat(3.0) * t198 * t434 - f64x8::splat(2.0) / f64x8::splat(9.0) * t169 * t473 * t90 - f64x8::splat(2.0) / f64x8::splat(3.0) * t89 * t869));
            let t899 = t35 * t214;
            let t902 = t147 * t479;
            let t904 = t78 * t451;
            let t906 = t289 * t454;
            let t921 = -f64x8::splat(6.0) * t80 * (t873 / f64x8::splat(2.0) + t894 / f64x8::splat(2.0)) - f64x8::splat(36.0) * t899 * t207 - f64x8::splat(6.0) * t902 - f64x8::splat(12.0) * t904 - f64x8::splat(36.0) * t906 - f64x8::splat(36.0) * t400 * t454 - f64x8::splat(36.0) * t150 * t207 * t235 - f64x8::splat(36.0) * t150 * t109 * t440 - f64x8::splat(12.0) * t150 * t479 * t95 - f64x8::splat(72.0) * t797 - f64x8::splat(36.0) * t804 + t613;
            let t928 = -f64x8::splat(36.0) * t812 - f64x8::splat(36.0) * t815 - f64x8::splat(18.0) * t820 - f64x8::splat(18.0) * t822 - f64x8::splat(18.0) * t691 - t694 - t502 - t508 + t532 - f64x8::splat(6.0) * t534 + t496 - t498 - t702;
            let tv4rho43 = -t484 + t485 - t617 - f64x8::splat(72.0) * t404 - f64x8::splat(36.0) * t409 - t456 + t488 - t288 - f64x8::splat(54.0) * t344 - t623 - f64x8::splat(72.0) * t398 - f64x8::splat(36.0) * t401 - f64x8::splat(36.0) * t406 - f64x8::splat(18.0) * t441 - t453 - t481 + t493 + t9 * (t921 + t928);
            acc_v4rho4_3 = tv4rho43;
            let t938 = t235 * t235;
            let t950 = t217 * t217;
            let t955 = t221 * t221;
            let t961 = f64x8::splat(24.0) * t310 + f64x8::splat(24.0) * t560;
            let t965 = ((t40).select(f64x8::splat(0.0), -f64x8::splat(56.0) / f64x8::splat(81.0) * t546 * t950 + f64x8::splat(16.0) / f64x8::splat(9.0) * t411 * t221 - f64x8::splat(2.0) / f64x8::splat(3.0) * t155 * t955 - f64x8::splat(8.0) / f64x8::splat(9.0) * t190 * t463 + f64x8::splat(2.0) / f64x8::splat(3.0) * t81 * t961));
            let t966 = t226 * t226;
            let t971 = t229 * t229;
            let t980 = ((t47).select(f64x8::splat(0.0), -f64x8::splat(56.0) / f64x8::splat(81.0) * t569 * t966 + f64x8::splat(16.0) / f64x8::splat(9.0) * t426 * t229 - f64x8::splat(2.0) / f64x8::splat(3.0) * t169 * t971 - f64x8::splat(8.0) / f64x8::splat(9.0) * t198 * t473 - f64x8::splat(2.0) / f64x8::splat(3.0) * t89 * t961));
            let t986 = t496 - t498 - t502 - t508 - f64x8::splat(144.0) * t906 - f64x8::splat(36.0) * t150 * t938 - f64x8::splat(48.0) * t150 * t109 * t479 + t532 - f64x8::splat(24.0) * t691 - f64x8::splat(72.0) * t812 - f64x8::splat(36.0) * t820 - f64x8::splat(24.0) * t902 - f64x8::splat(72.0) * t899 * t235 - f64x8::splat(6.0) * t80 * (t965 / f64x8::splat(2.0) + t980 / f64x8::splat(2.0)) + t613 - f64x8::splat(48.0) * t904;
            let tv4rho44 = t9 * t986 - f64x8::splat(72.0) * t344 - f64x8::splat(144.0) * t398 - f64x8::splat(72.0) * t406 - f64x8::splat(48.0) * t452 - f64x8::splat(144.0) * t455 - f64x8::splat(24.0) * t480 - t484 + t485 + t488 + t493;
            acc_v4rho4_4 = tv4rho44;
        }
        store_add(zk, ip, m, acc_zk);
        store_strided(vrho, ip, m, 2, 0, acc_vrho_0);
        store_strided(vrho, ip, m, 2, 1, acc_vrho_1);
        store_strided(v2rho2, ip, m, 3, 0, acc_v2rho2_0);
        store_strided(v2rho2, ip, m, 3, 1, acc_v2rho2_1);
        store_strided(v2rho2, ip, m, 3, 2, acc_v2rho2_2);
        store_strided(v3rho3, ip, m, 4, 0, acc_v3rho3_0);
        store_strided(v3rho3, ip, m, 4, 1, acc_v3rho3_1);
        store_strided(v3rho3, ip, m, 4, 2, acc_v3rho3_2);
        store_strided(v3rho3, ip, m, 4, 3, acc_v3rho3_3);
        store_strided(v4rho4, ip, m, 5, 0, acc_v4rho4_0);
        store_strided(v4rho4, ip, m, 5, 1, acc_v4rho4_1);
        store_strided(v4rho4, ip, m, 5, 2, acc_v4rho4_2);
        store_strided(v4rho4, ip, m, 5, 3, acc_v4rho4_3);
        store_strided(v4rho4, ip, m, 5, 4, acc_v4rho4_4);
        ip += 8;
    }
}
