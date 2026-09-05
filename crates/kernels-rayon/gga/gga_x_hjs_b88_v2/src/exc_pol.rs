//! GGA_X_HJS_B88_V2 exc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_hjs_b88_v2.c`
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
pub fn gga_x_hjs_b88_v2_exc_pol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    param_a_0: f64,
    param_a_1: f64,
    param_a_2: f64,
    param_a_3: f64,
    param_a_4: f64,
    param_a_5: f64,
    param_b_0: f64,
    param_b_1: f64,
    param_b_2: f64,
    param_b_3: f64,
    param_b_4: f64,
    param_b_5: f64,
    param_b_6: f64,
    param_b_7: f64,
    param_b_8: f64,
    param_hyb_omega_0: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_a_0 = f64x8::splat(param_a_0);
    let param_a_1 = f64x8::splat(param_a_1);
    let param_a_2 = f64x8::splat(param_a_2);
    let param_a_3 = f64x8::splat(param_a_3);
    let param_a_4 = f64x8::splat(param_a_4);
    let param_a_5 = f64x8::splat(param_a_5);
    let param_b_0 = f64x8::splat(param_b_0);
    let param_b_1 = f64x8::splat(param_b_1);
    let param_b_2 = f64x8::splat(param_b_2);
    let param_b_3 = f64x8::splat(param_b_3);
    let param_b_4 = f64x8::splat(param_b_4);
    let param_b_5 = f64x8::splat(param_b_5);
    let param_b_6 = f64x8::splat(param_b_6);
    let param_b_7 = f64x8::splat(param_b_7);
    let param_b_8 = f64x8::splat(param_b_8);
    let param_hyb_omega_0 = f64x8::splat(param_hyb_omega_0);
    let dens_threshold = f64x8::splat(dens_threshold);
    let zeta_threshold = f64x8::splat(zeta_threshold);
    let mut ip = 0usize;
    while ip < np {
        let m = (np - ip).min(8);
        let v_rho0 = load_strided(rho, ip, np, 2, 0);
        let v_rho1 = load_strided(rho, ip, np, 2, 1);
        let v_sigma0 = load_strided(sigma, ip, np, 3, 0);
        let v_sigma1 = load_strided(sigma, ip, np, 3, 1);
        let v_sigma2 = load_strided(sigma, ip, np, 3, 2);
        let mut acc_zk = V_ZERO;
        {
            let t1 = (v_rho0).simd_le(dens_threshold);
            let t2 = f64x8::splat(M_CBRT3);
            let t3 = f64x8::splat(M_CBRTPI);
            let t5 = t2 / t3;
            let t6 = v_rho0 + v_rho1;
            let t7 = f64x8::splat(1.0) / t6;
            let t10 = (f64x8::splat(2.0) * v_rho0 * t7).simd_le(zeta_threshold);
            let t11 = zeta_threshold - f64x8::splat(1.0);
            let t14 = (f64x8::splat(2.0) * v_rho1 * t7).simd_le(zeta_threshold);
            let t15 = -t11;
            let t16 = v_rho0 - v_rho1;
            let t17 = t16 * t7;
            let t18 = ((t10).select(t11, (t14).select(t15, t17)));
            let t19 = f64x8::splat(1.0) + t18;
            let t20 = (t19).simd_le(zeta_threshold);
            let t21 = (simd::cbrt(zeta_threshold));
            let t22 = t21 * zeta_threshold;
            let t23 = (simd::cbrt(t19));
            let t25 = ((t20).select(t22, t23 * t19));
            let t26 = (simd::cbrt(t6));
            let t27 = t25 * t26;
            let t28 = t2 * t2;
            let t29 = param_hyb_omega_0 * t28;
            let t30 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t31 = (simd::cbrt(t30));
            let t32 = f64x8::splat(1.0) / t31;
            let t33 = t29 * t32;
            let t35 = (f64x8::splat(1.0) + t17).simd_le(zeta_threshold);
            let t37 = (f64x8::splat(1.0) - t17).simd_le(zeta_threshold);
            let t38 = ((t35).select(t11, (t37).select(t15, t17)));
            let t39 = f64x8::splat(1.0) + t38;
            let t40 = (t39).simd_le(zeta_threshold);
            let t41 = (simd::cbrt(t39));
            let t42 = ((t40).select(t21, t41));
            let t43 = f64x8::splat(1.0) / t42;
            let t44 = f64x8::splat(1.0) / t26;
            let t45 = t43 * t44;
            let t46 = f64x8::splat(M_CBRT6);
            let t47 = t46 * t46;
            let t48 = t47 * t32;
            let t49 = ((v_sigma0).sqrt());
            let t50 = (simd::cbrt(v_rho0));
            let t52 = f64x8::splat(1.0) / t50 / v_rho0;
            let t56 = (simd::exp(-t48 * t49 * t52 / f64x8::splat(12.0)));
            let t57 = (simd::exp(f64x8::splat(20.0)));
            let t59 = f64x8::splat(1.0) / (t57 - f64x8::splat(1.0));
            let t60 = t56 + t59;
            let t62 = f64x8::splat(1.0) / (f64x8::splat(1.0) + t59);
            let t64 = (simd::ln(t60 * t62));
            let t65 = t64 * t64;
            let t66 = param_a_0;
            let t68 = param_a_1;
            let t69 = t65 * t64;
            let t71 = param_a_2;
            let t72 = t65 * t65;
            let t74 = param_a_3;
            let t75 = t72 * t64;
            let t77 = param_a_4;
            let t78 = t72 * t65;
            let t80 = param_a_5;
            let t81 = t72 * t69;
            let t83 = t66 * t65 - t68 * t69 + t71 * t72 - t74 * t75 + t77 * t78 - t80 * t81;
            let t84 = t65 * t83;
            let t85 = param_b_0;
            let t87 = param_b_1;
            let t89 = param_b_2;
            let t91 = param_b_3;
            let t93 = param_b_4;
            let t95 = param_b_5;
            let t97 = param_b_6;
            let t99 = param_b_7;
            let t100 = t72 * t72;
            let t102 = param_b_8;
            let t105 = -t102 * t100 * t64 + t99 * t100 - t85 * t64 + t87 * t65 - t89 * t69 + t91 * t72 - t93 * t75 + t95 * t78 - t97 * t81 + f64x8::splat(1.0);
            let t106 = f64x8::splat(1.0) / t105;
            let t107 = t84 * t106;
            let t108 = (f64x8::splat(1e-10)).simd_lt(t107);
            let t109 = ((t108).select(t107, f64x8::splat(1e-10)));
            let t110 = param_hyb_omega_0 * param_hyb_omega_0;
            let t111 = t110 * t2;
            let t112 = t31 * t31;
            let t113 = f64x8::splat(1.0) / t112;
            let t114 = t42 * t42;
            let t115 = f64x8::splat(1.0) / t114;
            let t116 = t113 * t115;
            let t117 = t26 * t26;
            let t118 = f64x8::splat(1.0) / t117;
            let t120 = t111 * t116 * t118;
            let t122 = f64x8::splat(0.60965) + t109 + t120 / f64x8::splat(3.0);
            let t123 = ((t122).sqrt());
            let t124 = f64x8::splat(1.0) / t123;
            let t126 = t33 * t45 * t124;
            let t128 = f64x8::splat(1.0) - t126 / f64x8::splat(3.0);
            let t129 = f64x8::splat(0.60965) + t109;
            let t130 = f64x8::splat(1.0) / t129;
            let t134 = f64x8::splat(1.0) + t65 / f64x8::splat(4.0);
            let t135 = f64x8::splat(1.0) / t134;
            let t139 = f64x8::splat(1.0) + f64x8::splat(0.3121563353845126) * t65 * t135 + f64x8::splat(4.21411052769092) * t109;
            let t141 = f64x8::splat(1.0) / t30;
            let t142 = t110 * param_hyb_omega_0 * t141;
            let t143 = t114 * t42;
            let t144 = f64x8::splat(1.0) / t143;
            let t145 = t144 * t7;
            let t147 = f64x8::splat(1.0) / t123 / t122;
            let t149 = t142 * t145 * t147;
            let t151 = f64x8::splat(2.0) - t126 + t149 / f64x8::splat(3.0);
            let t152 = t139 * t151;
            let t153 = t129 * t129;
            let t154 = f64x8::splat(1.0) / t153;
            let t160 = t153 * t129;
            let t162 = ((t129).sqrt());
            let t163 = t162 * t160;
            let t164 = ((f64x8::splat(M_PI)).sqrt());
            let t165 = f64x8::splat(4.0) / f64x8::splat(5.0) * t164;
            let t166 = ((t109).sqrt());
            let t169 = (f64x8::splat(0.0)).simd_lt(f64x8::splat(0.7572109999) + t109);
            let t171 = ((t169).select(f64x8::splat(0.757211) + t109, f64x8::splat(1e-10)));
            let t172 = ((t171).sqrt());
            let t174 = t165 + f64x8::splat(12.0) / f64x8::splat(5.0) * t166 - f64x8::splat(12.0) / f64x8::splat(5.0) * t172;
            let t176 = f64x8::splat(0.0474596) * t139 * t129 + f64x8::splat(0.028363733333333332) * t153 - f64x8::splat(0.9086532) * t160 - t163 * t174;
            let t179 = t110 * t110;
            let t181 = t179 * param_hyb_omega_0 * t2;
            let t183 = f64x8::splat(1.0) / t112 / t30;
            let t184 = t181 * t183;
            let t185 = t114 * t114;
            let t187 = f64x8::splat(1.0) / t185 / t42;
            let t189 = f64x8::splat(1.0) / t117 / t6;
            let t190 = t187 * t189;
            let t191 = t122 * t122;
            let t193 = f64x8::splat(1.0) / t123 / t191;
            let t197 = f64x8::splat(8.0) - f64x8::splat(5.0) * t126 + f64x8::splat(10.0) / f64x8::splat(3.0) * t149 - t184 * t190 * t193 / f64x8::splat(3.0);
            let t198 = t176 * t197;
            let t199 = f64x8::splat(1.0) / t160;
            let t203 = f64x8::splat(3.0) * t120;
            let t204 = f64x8::splat(9.0) * t109 + t203;
            let t205 = ((t204).sqrt());
            let t207 = f64x8::splat(9.0) * t171 + t203;
            let t208 = ((t207).sqrt());
            let t210 = t205 / f64x8::splat(3.0) - t208 / f64x8::splat(3.0);
            let t214 = t32 * t43;
            let t216 = t29 * t214 * t44;
            let t218 = t216 / f64x8::splat(3.0) + t205 / f64x8::splat(3.0);
            let t220 = t216 / f64x8::splat(3.0) + t123;
            let t221 = f64x8::splat(1.0) / t220;
            let t223 = (simd::ln(t218 * t221));
            let t227 = t216 / f64x8::splat(3.0) + t208 / f64x8::splat(3.0);
            let t229 = (simd::ln(t227 * t221));
            let t232 = f64x8::splat(0.757211) + f64x8::splat(0.04727288888888889) * t128 * t130 + f64x8::splat(0.026366444444444446) * t152 * t154 - t198 * t199 / f64x8::splat(9.0) + f64x8::splat(2.0) / f64x8::splat(3.0) * t33 * t45 * t210 + f64x8::splat(2.0) * t109 * t223 - f64x8::splat(2.0) * t171 * t229;
            let t236 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t27 * t232));
            let t237 = (v_rho1).simd_le(dens_threshold);
            let t238 = -t16;
            let t240 = ((t14).select(t11, (t10).select(t15, t238 * t7)));
            let t241 = f64x8::splat(1.0) + t240;
            let t242 = (t241).simd_le(zeta_threshold);
            let t243 = (simd::cbrt(t241));
            let t245 = ((t242).select(t22, t243 * t241));
            let t246 = t245 * t26;
            let t247 = ((t37).select(t11, (t35).select(t15, -t17)));
            let t248 = f64x8::splat(1.0) + t247;
            let t249 = (t248).simd_le(zeta_threshold);
            let t250 = (simd::cbrt(t248));
            let t251 = ((t249).select(t21, t250));
            let t252 = f64x8::splat(1.0) / t251;
            let t253 = t252 * t44;
            let t254 = ((v_sigma2).sqrt());
            let t255 = (simd::cbrt(v_rho1));
            let t257 = f64x8::splat(1.0) / t255 / v_rho1;
            let t261 = (simd::exp(-t48 * t254 * t257 / f64x8::splat(12.0)));
            let t262 = t261 + t59;
            let t264 = (simd::ln(t262 * t62));
            let t265 = t264 * t264;
            let t267 = t265 * t264;
            let t269 = t265 * t265;
            let t271 = t269 * t264;
            let t273 = t269 * t265;
            let t275 = t269 * t267;
            let t277 = t66 * t265 - t68 * t267 + t71 * t269 - t74 * t271 + t77 * t273 - t80 * t275;
            let t278 = t265 * t277;
            let t286 = t269 * t269;
            let t290 = -t102 * t286 * t264 - t85 * t264 + t87 * t265 - t89 * t267 + t91 * t269 - t93 * t271 + t95 * t273 - t97 * t275 + t99 * t286 + f64x8::splat(1.0);
            let t291 = f64x8::splat(1.0) / t290;
            let t292 = t278 * t291;
            let t293 = (f64x8::splat(1e-10)).simd_lt(t292);
            let t294 = ((t293).select(t292, f64x8::splat(1e-10)));
            let t295 = t251 * t251;
            let t296 = f64x8::splat(1.0) / t295;
            let t297 = t113 * t296;
            let t299 = t111 * t297 * t118;
            let t301 = f64x8::splat(0.60965) + t294 + t299 / f64x8::splat(3.0);
            let t302 = ((t301).sqrt());
            let t303 = f64x8::splat(1.0) / t302;
            let t305 = t33 * t253 * t303;
            let t307 = f64x8::splat(1.0) - t305 / f64x8::splat(3.0);
            let t308 = f64x8::splat(0.60965) + t294;
            let t309 = f64x8::splat(1.0) / t308;
            let t313 = f64x8::splat(1.0) + t265 / f64x8::splat(4.0);
            let t314 = f64x8::splat(1.0) / t313;
            let t318 = f64x8::splat(1.0) + f64x8::splat(0.3121563353845126) * t265 * t314 + f64x8::splat(4.21411052769092) * t294;
            let t319 = t295 * t251;
            let t320 = f64x8::splat(1.0) / t319;
            let t321 = t320 * t7;
            let t323 = f64x8::splat(1.0) / t302 / t301;
            let t325 = t142 * t321 * t323;
            let t327 = f64x8::splat(2.0) - t305 + t325 / f64x8::splat(3.0);
            let t328 = t318 * t327;
            let t329 = t308 * t308;
            let t330 = f64x8::splat(1.0) / t329;
            let t336 = t329 * t308;
            let t338 = ((t308).sqrt());
            let t339 = t338 * t336;
            let t340 = ((t294).sqrt());
            let t343 = (f64x8::splat(0.0)).simd_lt(f64x8::splat(0.7572109999) + t294);
            let t345 = ((t343).select(f64x8::splat(0.757211) + t294, f64x8::splat(1e-10)));
            let t346 = ((t345).sqrt());
            let t348 = t165 + f64x8::splat(12.0) / f64x8::splat(5.0) * t340 - f64x8::splat(12.0) / f64x8::splat(5.0) * t346;
            let t350 = f64x8::splat(0.0474596) * t318 * t308 + f64x8::splat(0.028363733333333332) * t329 - f64x8::splat(0.9086532) * t336 - t339 * t348;
            let t353 = t295 * t295;
            let t355 = f64x8::splat(1.0) / t353 / t251;
            let t356 = t355 * t189;
            let t357 = t301 * t301;
            let t359 = f64x8::splat(1.0) / t302 / t357;
            let t363 = f64x8::splat(8.0) - f64x8::splat(5.0) * t305 + f64x8::splat(10.0) / f64x8::splat(3.0) * t325 - t184 * t356 * t359 / f64x8::splat(3.0);
            let t364 = t350 * t363;
            let t365 = f64x8::splat(1.0) / t336;
            let t369 = f64x8::splat(3.0) * t299;
            let t370 = f64x8::splat(9.0) * t294 + t369;
            let t371 = ((t370).sqrt());
            let t373 = f64x8::splat(9.0) * t345 + t369;
            let t374 = ((t373).sqrt());
            let t376 = t371 / f64x8::splat(3.0) - t374 / f64x8::splat(3.0);
            let t380 = t32 * t252;
            let t382 = t29 * t380 * t44;
            let t384 = t382 / f64x8::splat(3.0) + t371 / f64x8::splat(3.0);
            let t386 = t382 / f64x8::splat(3.0) + t302;
            let t387 = f64x8::splat(1.0) / t386;
            let t389 = (simd::ln(t384 * t387));
            let t393 = t382 / f64x8::splat(3.0) + t374 / f64x8::splat(3.0);
            let t395 = (simd::ln(t393 * t387));
            let t398 = f64x8::splat(0.757211) + f64x8::splat(0.04727288888888889) * t307 * t309 + f64x8::splat(0.026366444444444446) * t328 * t330 - t364 * t365 / f64x8::splat(9.0) + f64x8::splat(2.0) / f64x8::splat(3.0) * t33 * t253 * t376 + f64x8::splat(2.0) * t294 * t389 - f64x8::splat(2.0) * t345 * t395;
            let t402 = ((t237).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t246 * t398));
            let tzk0 = t236 + t402;
            acc_zk = tzk0;
        }
        store_add(zk, ip, m, acc_zk);
        ip += 8;
    }
}
