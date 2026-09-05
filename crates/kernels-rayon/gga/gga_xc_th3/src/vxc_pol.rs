//! GGA_XC_TH3 vxc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_xc_th3.c`
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
pub fn gga_xc_th3_vxc_pol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    param_omega_0: f64,
    param_omega_1: f64,
    param_omega_2: f64,
    param_omega_3: f64,
    param_omega_4: f64,
    param_omega_5: f64,
    param_omega_6: f64,
    param_omega_7: f64,
    param_omega_8: f64,
    param_omega_9: f64,
    param_omega_10: f64,
    param_omega_11: f64,
    param_omega_12: f64,
    param_omega_13: f64,
    param_omega_18: f64,
    param_omega_14: f64,
    param_omega_15: f64,
    param_omega_16: f64,
    param_omega_17: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_omega_0 = f64x8::splat(param_omega_0);
    let param_omega_1 = f64x8::splat(param_omega_1);
    let param_omega_2 = f64x8::splat(param_omega_2);
    let param_omega_3 = f64x8::splat(param_omega_3);
    let param_omega_4 = f64x8::splat(param_omega_4);
    let param_omega_5 = f64x8::splat(param_omega_5);
    let param_omega_6 = f64x8::splat(param_omega_6);
    let param_omega_7 = f64x8::splat(param_omega_7);
    let param_omega_8 = f64x8::splat(param_omega_8);
    let param_omega_9 = f64x8::splat(param_omega_9);
    let param_omega_10 = f64x8::splat(param_omega_10);
    let param_omega_11 = f64x8::splat(param_omega_11);
    let param_omega_12 = f64x8::splat(param_omega_12);
    let param_omega_13 = f64x8::splat(param_omega_13);
    let param_omega_18 = f64x8::splat(param_omega_18);
    let param_omega_14 = f64x8::splat(param_omega_14);
    let param_omega_15 = f64x8::splat(param_omega_15);
    let param_omega_16 = f64x8::splat(param_omega_16);
    let param_omega_17 = f64x8::splat(param_omega_17);
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
        let mut acc_vrho_0 = V_ZERO;
        let mut acc_vrho_1 = V_ZERO;
        let mut acc_vsigma_0 = V_ZERO;
        let mut acc_vsigma_1 = V_ZERO;
        let mut acc_vsigma_2 = V_ZERO;
        {
            let t1 = param_omega_0;
            let t2 = (simd::pow(v_rho0, f64x8::splat(1.0) / f64x8::splat(6.0)));
            let t3 = t2 * v_rho0;
            let t4 = (simd::pow(v_rho1, f64x8::splat(1.0) / f64x8::splat(6.0)));
            let t5 = t4 * v_rho1;
            let t6 = t3 + t5;
            let t8 = param_omega_1;
            let t9 = (simd::cbrt(v_rho0));
            let t10 = t9 * v_rho0;
            let t11 = (simd::cbrt(v_rho1));
            let t12 = t11 * v_rho1;
            let t13 = t10 + t12;
            let t15 = param_omega_2;
            let t16 = ((v_rho0).sqrt());
            let t17 = t16 * v_rho0;
            let t18 = ((v_rho1).sqrt());
            let t19 = t18 * v_rho1;
            let t20 = t17 + t19;
            let t22 = param_omega_3;
            let t23 = t9 * t9;
            let t24 = t23 * v_rho0;
            let t25 = t11 * t11;
            let t26 = t25 * v_rho1;
            let t27 = t24 + t26;
            let t29 = param_omega_4;
            let t30 = (simd::pow(v_rho0, f64x8::splat(1.0) / f64x8::splat(12.0)));
            let t31 = t30 * t30;
            let t32 = t31 * t31;
            let t33 = t32 * t30;
            let t35 = (simd::pow(v_rho1, f64x8::splat(1.0) / f64x8::splat(12.0)));
            let t36 = t35 * t35;
            let t37 = t36 * t36;
            let t38 = t37 * t35;
            let t41 = t29 * (t33 * v_rho0 + t38 * v_rho1);
            let t42 = ((v_sigma0).sqrt());
            let t43 = f64x8::splat(1.0) / t10;
            let t44 = t42 * t43;
            let t45 = v_rho0 - v_rho1;
            let t46 = v_rho0 + v_rho1;
            let t47 = f64x8::splat(1.0) / t46;
            let t48 = t45 * t47;
            let t49 = f64x8::splat(1.0) + t48;
            let t50 = (t49).simd_le(zeta_threshold);
            let t51 = (simd::cbrt(zeta_threshold));
            let t52 = t51 * zeta_threshold;
            let t53 = (simd::cbrt(t49));
            let t55 = ((t50).select(t52, t53 * t49));
            let t56 = f64x8::splat(M_CBRT2);
            let t57 = t56 * t56;
            let t58 = t55 * t57;
            let t60 = ((v_sigma2).sqrt());
            let t61 = f64x8::splat(1.0) / t12;
            let t62 = t60 * t61;
            let t63 = f64x8::splat(1.0) - t48;
            let t64 = (t63).simd_le(zeta_threshold);
            let t65 = (simd::cbrt(t63));
            let t67 = ((t64).select(t52, t65 * t63));
            let t68 = t67 * t57;
            let t71 = t44 * t58 / f64x8::splat(4.0) + t62 * t68 / f64x8::splat(4.0);
            let t74 = param_omega_5;
            let t75 = t74 * t20;
            let t78 = param_omega_6;
            let t79 = t78 * t27;
            let t82 = param_omega_7;
            let t83 = t2 * t2;
            let t84 = t83 * t83;
            let t85 = t84 * t2;
            let t86 = t85 * v_rho0;
            let t87 = t4 * t4;
            let t88 = t87 * t87;
            let t89 = t88 * t4;
            let t90 = t89 * v_rho1;
            let t91 = t86 + t90;
            let t92 = t82 * t91;
            let t95 = param_omega_8;
            let t96 = t95 * t27;
            let t97 = v_rho0 * v_rho0;
            let t99 = f64x8::splat(1.0) / t23 / t97;
            let t100 = v_sigma0 * t99;
            let t101 = t55 * t55;
            let t102 = t101 * t56;
            let t103 = t100 * t102;
            let t104 = v_rho1 * v_rho1;
            let t106 = f64x8::splat(1.0) / t25 / t104;
            let t107 = v_sigma2 * t106;
            let t108 = t67 * t67;
            let t109 = t108 * t56;
            let t110 = t107 * t109;
            let t112 = t103 / f64x8::splat(8.0) + t110 / f64x8::splat(8.0);
            let t115 = param_omega_9;
            let t116 = t115 * t91;
            let t119 = param_omega_10;
            let t120 = t97 + t104;
            let t121 = t119 * t120;
            let t124 = param_omega_11;
            let t125 = t124 * t27;
            let t129 = v_sigma0 + f64x8::splat(2.0) * v_sigma1 + v_sigma2;
            let t130 = t46 * t46;
            let t131 = (simd::cbrt(t46));
            let t132 = t131 * t131;
            let t134 = f64x8::splat(1.0) / t132 / t130;
            let t136 = t103 / f64x8::splat(4.0) + t110 / f64x8::splat(4.0) - t129 * t134;
            let t138 = param_omega_12;
            let t139 = t138 * t91;
            let t141 = param_omega_13;
            let t142 = t141 * t120;
            let t144 = param_omega_14;
            let t145 = t144 * t6;
            let t146 = t45 * t45;
            let t147 = f64x8::splat(1.0) / t130;
            let t148 = t146 * t147;
            let t150 = param_omega_15;
            let t151 = t150 * t13;
            let t153 = param_omega_16;
            let t154 = t153 * t20;
            let t156 = param_omega_17;
            let t157 = t156 * t27;
            let t159 = param_omega_18;
            let t160 = (simd::pow(v_rho0, f64x8::splat(1.0833333333333333)));
            let t161 = (simd::pow(v_rho1, f64x8::splat(1.0833333333333333)));
            let t164 = t1 * t6 + t8 * t13 + t15 * t20 + t22 * t27 + t41 * t71 / f64x8::splat(2.0) + t75 * t71 / f64x8::splat(2.0) + t79 * t71 / f64x8::splat(2.0) + t92 * t71 / f64x8::splat(2.0) + t96 * t112 / f64x8::splat(2.0) + t116 * t112 / f64x8::splat(2.0) + t121 * t112 / f64x8::splat(2.0) + t125 * t136 + t139 * t136 + t142 * t136 + t145 * t148 + t151 * t148 + t154 * t148 + t157 * t148 + t159 * (t160 + t161);
            let tzk0 = t164 * t47;
            acc_zk = tzk0;
            let t173 = (simd::pow(v_rho0, f64x8::splat(0.08333333333333333)));
            let t177 = f64x8::splat(1.0) / t9 / t97;
            let t178 = t42 * t177;
            let t181 = t45 * t147;
            let t182 = t47 - t181;
            let t185 = ((t50).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t53 * t182));
            let t186 = t185 * t57;
            let t189 = -t182;
            let t192 = ((t64).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t65 * t189));
            let t193 = t192 * t57;
            let t196 = -t178 * t58 / f64x8::splat(3.0) + t44 * t186 / f64x8::splat(4.0) + t62 * t193 / f64x8::splat(4.0);
            let t199 = t29 * t33;
            let t204 = t74 * t16;
            let t209 = t78 * t23;
            let t214 = t82 * t85;
            let t217 = t97 * v_rho0;
            let t219 = f64x8::splat(1.0) / t23 / t217;
            let t220 = v_sigma0 * t219;
            let t221 = t220 * t102;
            let t223 = t55 * t56;
            let t224 = t223 * t185;
            let t225 = t100 * t224;
            let t227 = t67 * t56;
            let t228 = t227 * t192;
            let t229 = t107 * t228;
            let t231 = -t221 / f64x8::splat(3.0) + t225 / f64x8::splat(4.0) + t229 / f64x8::splat(4.0);
            let t234 = t95 * t23;
            let t239 = t115 * t85;
            let t244 = f64x8::splat(7.0) / f64x8::splat(6.0) * t1 * t2 + f64x8::splat(4.0) / f64x8::splat(3.0) * t8 * t9 + f64x8::splat(3.0) / f64x8::splat(2.0) * t15 * t16 + f64x8::splat(5.0) / f64x8::splat(3.0) * t22 * t23 + f64x8::splat(1.0833333333333333) * t159 * t173 + t41 * t196 / f64x8::splat(2.0) + f64x8::splat(17.0) / f64x8::splat(24.0) * t199 * t71 + t75 * t196 / f64x8::splat(2.0) + f64x8::splat(3.0) / f64x8::splat(4.0) * t204 * t71 + t79 * t196 / f64x8::splat(2.0) + f64x8::splat(5.0) / f64x8::splat(6.0) * t209 * t71 + t92 * t196 / f64x8::splat(2.0) + f64x8::splat(11.0) / f64x8::splat(12.0) * t214 * t71 + t96 * t231 / f64x8::splat(2.0) + f64x8::splat(5.0) / f64x8::splat(6.0) * t234 * t112 + t116 * t231 / f64x8::splat(2.0) + f64x8::splat(11.0) / f64x8::splat(12.0) * t239 * t112 + t121 * t231 / f64x8::splat(2.0);
            let t245 = t119 * v_rho0;
            let t250 = t130 * t46;
            let t252 = f64x8::splat(1.0) / t132 / t250;
            let t254 = f64x8::splat(8.0) / f64x8::splat(3.0) * t129 * t252;
            let t255 = -f64x8::splat(2.0) / f64x8::splat(3.0) * t221 + t225 / f64x8::splat(2.0) + t229 / f64x8::splat(2.0) + t254;
            let t257 = t124 * t23;
            let t261 = t138 * t85;
            let t265 = t141 * v_rho0;
            let t268 = f64x8::splat(1.0) / t250;
            let t269 = t146 * t268;
            let t271 = f64x8::splat(2.0) * t145 * t269;
            let t273 = f64x8::splat(2.0) * t151 * t181;
            let t275 = f64x8::splat(2.0) * t151 * t269;
            let t277 = f64x8::splat(2.0) * t154 * t181;
            let t279 = f64x8::splat(2.0) * t154 * t269;
            let t281 = f64x8::splat(2.0) * t157 * t181;
            let t283 = f64x8::splat(2.0) * t157 * t269;
            let t285 = f64x8::splat(2.0) * t145 * t181;
            let t286 = t144 * t2;
            let t289 = t150 * t9;
            let t292 = t153 * t16;
            let t295 = t156 * t23;
            let t298 = t245 * t112 + t125 * t255 + f64x8::splat(5.0) / f64x8::splat(3.0) * t257 * t136 + t139 * t255 + f64x8::splat(11.0) / f64x8::splat(6.0) * t261 * t136 + t142 * t255 + f64x8::splat(2.0) * t265 * t136 - t271 + t273 - t275 + t277 - t279 + t281 - t283 + t285 + f64x8::splat(7.0) / f64x8::splat(6.0) * t286 * t148 + f64x8::splat(4.0) / f64x8::splat(3.0) * t289 * t148 + f64x8::splat(3.0) / f64x8::splat(2.0) * t292 * t148 + f64x8::splat(5.0) / f64x8::splat(3.0) * t295 * t148;
            let tvrho0 = t244 + t298;
            acc_vrho_0 = tvrho0;
            let t299 = -t47 - t181;
            let t302 = ((t50).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t53 * t299));
            let t303 = t302 * t57;
            let t307 = f64x8::splat(1.0) / t11 / t104;
            let t308 = t60 * t307;
            let t311 = -t299;
            let t314 = ((t64).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t65 * t311));
            let t315 = t314 * t57;
            let t318 = t44 * t303 / f64x8::splat(4.0) - t308 * t68 / f64x8::splat(3.0) + t62 * t315 / f64x8::splat(4.0);
            let t321 = t29 * t38;
            let t326 = t74 * t18;
            let t331 = t78 * t25;
            let t336 = t82 * t89;
            let t339 = t223 * t302;
            let t340 = t100 * t339;
            let t342 = t104 * v_rho1;
            let t344 = f64x8::splat(1.0) / t25 / t342;
            let t345 = v_sigma2 * t344;
            let t346 = t345 * t109;
            let t348 = t227 * t314;
            let t349 = t107 * t348;
            let t351 = t340 / f64x8::splat(4.0) - t346 / f64x8::splat(3.0) + t349 / f64x8::splat(4.0);
            let t354 = t95 * t25;
            let t359 = t115 * t89;
            let t364 = t119 * v_rho1;
            let t369 = t340 / f64x8::splat(2.0) - f64x8::splat(2.0) / f64x8::splat(3.0) * t346 + t349 / f64x8::splat(2.0) + t254;
            let t371 = t124 * t25;
            let t375 = t138 * t89;
            let t378 = t41 * t318 / f64x8::splat(2.0) + f64x8::splat(17.0) / f64x8::splat(24.0) * t321 * t71 + t75 * t318 / f64x8::splat(2.0) + f64x8::splat(3.0) / f64x8::splat(4.0) * t326 * t71 + t79 * t318 / f64x8::splat(2.0) + f64x8::splat(5.0) / f64x8::splat(6.0) * t331 * t71 + t92 * t318 / f64x8::splat(2.0) + f64x8::splat(11.0) / f64x8::splat(12.0) * t336 * t71 + t96 * t351 / f64x8::splat(2.0) + f64x8::splat(5.0) / f64x8::splat(6.0) * t354 * t112 + t116 * t351 / f64x8::splat(2.0) + f64x8::splat(11.0) / f64x8::splat(12.0) * t359 * t112 + t121 * t351 / f64x8::splat(2.0) + t364 * t112 + t125 * t369 + f64x8::splat(5.0) / f64x8::splat(3.0) * t371 * t136 + t139 * t369 + f64x8::splat(11.0) / f64x8::splat(6.0) * t375 * t136;
            let t380 = t141 * v_rho1;
            let t383 = t144 * t4;
            let t386 = t150 * t11;
            let t389 = t153 * t18;
            let t392 = t156 * t25;
            let t403 = (simd::pow(v_rho1, f64x8::splat(0.08333333333333333)));
            let t406 = t142 * t369 + f64x8::splat(2.0) * t380 * t136 - t271 - t273 - t275 - t277 - t279 - t281 - t283 + f64x8::splat(7.0) / f64x8::splat(6.0) * t383 * t148 + f64x8::splat(4.0) / f64x8::splat(3.0) * t386 * t148 + f64x8::splat(3.0) / f64x8::splat(2.0) * t389 * t148 + f64x8::splat(5.0) / f64x8::splat(3.0) * t392 * t148 - t285 + f64x8::splat(7.0) / f64x8::splat(6.0) * t1 * t4 + f64x8::splat(4.0) / f64x8::splat(3.0) * t8 * t11 + f64x8::splat(3.0) / f64x8::splat(2.0) * t15 * t18 + f64x8::splat(5.0) / f64x8::splat(3.0) * t22 * t25 + f64x8::splat(1.0833333333333333) * t159 * t403;
            let tvrho1 = t378 + t406;
            acc_vrho_1 = tvrho1;
            let t407 = f64x8::splat(1.0) / t42;
            let t408 = t41 * t407;
            let t410 = t43 * t55 * t57;
            let t413 = t75 * t407;
            let t416 = t79 * t407;
            let t419 = t92 * t407;
            let t423 = t99 * t101 * t56;
            let t431 = t423 / f64x8::splat(4.0) - t134;
            let tvsigma0 = t408 * t410 / f64x8::splat(16.0) + t413 * t410 / f64x8::splat(16.0) + t416 * t410 / f64x8::splat(16.0) + t419 * t410 / f64x8::splat(16.0) + t96 * t423 / f64x8::splat(16.0) + t116 * t423 / f64x8::splat(16.0) + t121 * t423 / f64x8::splat(16.0) + t125 * t431 + t139 * t431 + t142 * t431;
            acc_vsigma_0 = tvsigma0;
            let tvsigma1 = -f64x8::splat(2.0) * t125 * t134 - f64x8::splat(2.0) * t139 * t134 - f64x8::splat(2.0) * t142 * t134;
            acc_vsigma_1 = tvsigma1;
            let t439 = f64x8::splat(1.0) / t60;
            let t440 = t41 * t439;
            let t442 = t61 * t67 * t57;
            let t445 = t75 * t439;
            let t448 = t79 * t439;
            let t451 = t92 * t439;
            let t455 = t106 * t108 * t56;
            let t463 = t455 / f64x8::splat(4.0) - t134;
            let tvsigma2 = t440 * t442 / f64x8::splat(16.0) + t445 * t442 / f64x8::splat(16.0) + t448 * t442 / f64x8::splat(16.0) + t451 * t442 / f64x8::splat(16.0) + t96 * t455 / f64x8::splat(16.0) + t116 * t455 / f64x8::splat(16.0) + t121 * t455 / f64x8::splat(16.0) + t125 * t463 + t139 * t463 + t142 * t463;
            acc_vsigma_2 = tvsigma2;
        }
        store_add(zk, ip, m, acc_zk);
        store_strided(vrho, ip, m, 2, 0, acc_vrho_0);
        store_strided(vrho, ip, m, 2, 1, acc_vrho_1);
        store_strided(vsigma, ip, m, 3, 0, acc_vsigma_0);
        store_strided(vsigma, ip, m, 3, 1, acc_vsigma_1);
        store_strided(vsigma, ip, m, 3, 2, acc_vsigma_2);
        ip += 8;
    }
}
