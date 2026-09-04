//! MGGA_X_MVS fxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_mvs.c`
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

#[allow(unused_variables, non_snake_case)]
pub fn mgga_x_mvs_fxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    vlapl: &mut [f64],
    vtau: &mut [f64],
    v2rho2: &mut [f64],
    v2rhosigma: &mut [f64],
    v2rholapl: &mut [f64],
    v2rhotau: &mut [f64],
    v2sigma2: &mut [f64],
    v2sigmalapl: &mut [f64],
    v2sigmatau: &mut [f64],
    v2lapl2: &mut [f64],
    v2lapltau: &mut [f64],
    v2tau2: &mut [f64],
    param_b: f64,
    param_c1: f64,
    param_e1: f64,
    param_k0: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_b = f64x8::splat(param_b);
    let param_c1 = f64x8::splat(param_c1);
    let param_e1 = f64x8::splat(param_e1);
    let param_k0 = f64x8::splat(param_k0);
    let dens_threshold = f64x8::splat(dens_threshold);
    let zeta_threshold = f64x8::splat(zeta_threshold);
    let mut ip = 0usize;
    while ip < np {
        let m = (np - ip).min(8);
        let v_rho = load(rho, ip, np);
        let v_sigma = load(sigma, ip, np);
        let v_lapl = load(lapl, ip, np);
        let v_tau = load(tau, ip, np);
        let mut acc_zk = V_ZERO;
        let mut acc_vrho = V_ZERO;
        let mut acc_vsigma = V_ZERO;
        let mut acc_vlapl = V_ZERO;
        let mut acc_vtau = V_ZERO;
        let mut acc_v2rho2 = V_ZERO;
        let mut acc_v2rhosigma = V_ZERO;
        let mut acc_v2rholapl = V_ZERO;
        let mut acc_v2rhotau = V_ZERO;
        let mut acc_v2sigma2 = V_ZERO;
        let mut acc_v2sigmalapl = V_ZERO;
        let mut acc_v2sigmatau = V_ZERO;
        let mut acc_v2lapl2 = V_ZERO;
        let mut acc_v2lapltau = V_ZERO;
        let mut acc_v2tau2 = V_ZERO;
        {
            let t3 = (v_rho / f64x8::splat(2.0)).simd_le(dens_threshold);
            let t4 = f64x8::splat(M_CBRT3);
            let t5 = f64x8::splat(M_CBRTPI);
            let t7 = t4 / t5;
            let t8 = (f64x8::splat(1.0)).simd_le(zeta_threshold);
            let t9 = zeta_threshold - f64x8::splat(1.0);
            let t11 = ((t8).select(t9, (t8).select(-t9, f64x8::splat(0.0))));
            let t12 = f64x8::splat(1.0) + t11;
            let t14 = (simd::cbrt(zeta_threshold));
            let t16 = (simd::cbrt(t12));
            let t18 = (((t12).simd_le(zeta_threshold)).select(t14 * zeta_threshold, t16 * t12));
            let t19 = t7 * t18;
            let t20 = (simd::cbrt(v_rho));
            let t21 = f64x8::splat(M_CBRT2);
            let t22 = t21 * t21;
            let t23 = v_tau * t22;
            let t24 = t20 * t20;
            let t26 = f64x8::splat(1.0) / t24 / v_rho;
            let t28 = v_sigma * t22;
            let t29 = v_rho * v_rho;
            let t31 = f64x8::splat(1.0) / t24 / t29;
            let t34 = t23 * t26 - t28 * t31 / f64x8::splat(8.0);
            let t35 = f64x8::splat(M_CBRT6);
            let t37 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t38 = (simd::cbrt(t37));
            let t39 = t38 * t38;
            let t40 = f64x8::splat(1.0) / t39;
            let t44 = param_k0 * (f64x8::splat(1.0) - f64x8::splat(5.0) / f64x8::splat(9.0) * t34 * t35 * t40);
            let t45 = t34 * t34;
            let t47 = t35 * t35;
            let t49 = f64x8::splat(1.0) / t38 / t37;
            let t50 = t47 * t49;
            let t53 = f64x8::splat(1.0) + f64x8::splat(25.0) / f64x8::splat(81.0) * param_e1 * t45 * t50;
            let t54 = t53 * t53;
            let t55 = t45 * t45;
            let t57 = t37 * t37;
            let t59 = f64x8::splat(1.0) / t39 / t57;
            let t60 = t35 * t59;
            let t63 = t54 + f64x8::splat(1250.0) / f64x8::splat(2187.0) * param_c1 * t55 * t60;
            let t64 = ((t63).sqrt().sqrt());
            let t65 = f64x8::splat(1.0) / t64;
            let t67 = t44 * t65 + f64x8::splat(1.0);
            let t71 = v_sigma * v_sigma;
            let t73 = t29 * t29;
            let t74 = t73 * v_rho;
            let t76 = f64x8::splat(1.0) / t20 / t74;
            let t80 = f64x8::splat(1.0) + param_b * t47 * t49 * t71 * t21 * t76 / f64x8::splat(288.0);
            let t81 = (simd::pow(t80, f64x8::splat(1.0) / f64x8::splat(8.0)));
            let t82 = f64x8::splat(1.0) / t81;
            let t86 = ((t3).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t19 * t20 * t67 * t82));
            let tzk0 = f64x8::splat(2.0) * t86;
            acc_zk = tzk0;
            let t87 = f64x8::splat(1.0) / t24;
            let t94 = t29 * v_rho;
            let t96 = f64x8::splat(1.0) / t24 / t94;
            let t99 = -f64x8::splat(5.0) / f64x8::splat(3.0) * t23 * t31 + t28 * t96 / f64x8::splat(3.0);
            let t100 = param_k0 * t99;
            let t101 = t35 * t40;
            let t102 = t101 * t65;
            let t106 = f64x8::splat(1.0) / t64 / t63;
            let t107 = t53 * param_e1;
            let t108 = t107 * t34;
            let t113 = param_c1 * t45 * t34;
            let t117 = f64x8::splat(100.0) / f64x8::splat(81.0) * t108 * t50 * t99 + f64x8::splat(5000.0) / f64x8::splat(2187.0) * t113 * t60 * t99;
            let t118 = t106 * t117;
            let t121 = -f64x8::splat(5.0) / f64x8::splat(9.0) * t100 * t102 - t44 * t118 / f64x8::splat(4.0);
            let t126 = t73 * t29;
            let t127 = f64x8::splat(1.0) / t126;
            let t128 = t18 * t127;
            let t130 = t7 * t128 * t67;
            let t133 = f64x8::splat(1.0) / t81 / t80 * param_b;
            let t134 = t133 * t47;
            let t137 = t134 * t49 * t71 * t21;
            let t141 = ((t3).select(f64x8::splat(0.0), -t19 * t87 * t67 * t82 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t19 * t20 * t121 * t82 - t130 * t137 / f64x8::splat(1152.0)));
            let tvrho0 = f64x8::splat(2.0) * v_rho * t141 + f64x8::splat(2.0) * t86;
            acc_vrho = tvrho0;
            let t144 = param_k0 * t22;
            let t145 = t144 * t31;
            let t146 = t145 * t102;
            let t148 = t22 * t31;
            let t149 = t50 * t148;
            let t150 = t108 * t149;
            let t152 = t113 * t35;
            let t153 = t59 * t22;
            let t154 = t153 * t31;
            let t155 = t152 * t154;
            let t157 = -f64x8::splat(25.0) / f64x8::splat(162.0) * t150 - f64x8::splat(625.0) / f64x8::splat(2187.0) * t155;
            let t158 = t106 * t157;
            let t161 = f64x8::splat(5.0) / f64x8::splat(72.0) * t146 - t44 * t158 / f64x8::splat(4.0);
            let t166 = f64x8::splat(1.0) / t74;
            let t167 = t18 * t166;
            let t169 = t7 * t167 * t67;
            let t172 = t134 * t49 * v_sigma * t21;
            let t176 = ((t3).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t19 * t20 * t161 * t82 + t169 * t172 / f64x8::splat(3072.0)));
            let tvsigma0 = f64x8::splat(2.0) * v_rho * t176;
            acc_vsigma = tvsigma0;
            let tvlapl0 = f64x8::splat(0.0);
            acc_vlapl = tvlapl0;
            let t178 = t144 * t26;
            let t181 = t22 * t26;
            let t182 = t50 * t181;
            let t185 = t153 * t26;
            let t188 = f64x8::splat(100.0) / f64x8::splat(81.0) * t108 * t182 + f64x8::splat(5000.0) / f64x8::splat(2187.0) * t152 * t185;
            let t189 = t106 * t188;
            let t192 = -f64x8::splat(5.0) / f64x8::splat(9.0) * t178 * t102 - t44 * t189 / f64x8::splat(4.0);
            let t197 = ((t3).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t19 * t20 * t192 * t82));
            let tvtau0 = f64x8::splat(2.0) * v_rho * t197;
            acc_vtau = tvtau0;
            let t208 = t73 * t94;
            let t209 = f64x8::splat(1.0) / t208;
            let t210 = t18 * t209;
            let t212 = t7 * t210 * t67;
            let t218 = f64x8::splat(1.0) / t24 / t73;
            let t221 = f64x8::splat(40.0) / f64x8::splat(9.0) * t23 * t96 - f64x8::splat(11.0) / f64x8::splat(9.0) * t28 * t218;
            let t222 = param_k0 * t221;
            let t225 = t100 * t35;
            let t226 = t40 * t106;
            let t227 = t226 * t117;
            let t230 = t63 * t63;
            let t232 = f64x8::splat(1.0) / t64 / t230;
            let t233 = t117 * t117;
            let t234 = t232 * t233;
            let t237 = param_e1 * param_e1;
            let t238 = t237 * t45;
            let t239 = t99 * t99;
            let t240 = t60 * t239;
            let t247 = t50 * t221;
            let t250 = param_c1 * t45;
            let t256 = f64x8::splat(10000.0) / f64x8::splat(2187.0) * t238 * t240 + f64x8::splat(100.0) / f64x8::splat(81.0) * t107 * t239 * t47 * t49 + f64x8::splat(100.0) / f64x8::splat(81.0) * t108 * t247 + f64x8::splat(5000.0) / f64x8::splat(729.0) * t250 * t240 + f64x8::splat(5000.0) / f64x8::splat(2187.0) * t113 * t60 * t221;
            let t257 = t106 * t256;
            let t260 = -f64x8::splat(5.0) / f64x8::splat(9.0) * t222 * t102 + f64x8::splat(5.0) / f64x8::splat(18.0) * t225 * t227 + f64x8::splat(5.0) / f64x8::splat(16.0) * t44 * t234 - t44 * t257 / f64x8::splat(4.0);
            let t266 = t7 * t128 * t121;
            let t269 = t73 * t73;
            let t273 = t18 / t20 / t269 / t73;
            let t275 = t7 * t273 * t67;
            let t276 = t80 * t80;
            let t279 = param_b * param_b;
            let t280 = f64x8::splat(1.0) / t81 / t276 * t279;
            let t281 = t280 * t35;
            let t282 = t71 * t71;
            let t285 = t281 * t59 * t282 * t22;
            let t289 = ((t3).select(f64x8::splat(0.0), t19 * t26 * t67 * t82 / f64x8::splat(12.0) - t19 * t87 * t121 * t82 / f64x8::splat(4.0) + f64x8::splat(17.0) / f64x8::splat(3456.0) * t212 * t137 - f64x8::splat(3.0) / f64x8::splat(8.0) * t19 * t20 * t260 * t82 - t266 * t137 / f64x8::splat(576.0) - t275 * t285 / f64x8::splat(9216.0)));
            let tv2rho20 = f64x8::splat(2.0) * v_rho * t289 + f64x8::splat(4.0) * t141;
            acc_v2rho2 = tv2rho20;
            let t296 = t144 * t96;
            let t297 = t296 * t102;
            let t299 = t101 * t118;
            let t300 = t145 * t299;
            let t302 = t226 * t157;
            let t305 = t232 * t157;
            let t309 = t238 * t35;
            let t310 = t59 * t99;
            let t311 = t310 * t148;
            let t312 = t309 * t311;
            let t314 = t107 * t99;
            let t315 = t314 * t149;
            let t317 = t22 * t96;
            let t318 = t50 * t317;
            let t319 = t108 * t318;
            let t321 = t250 * t35;
            let t322 = t321 * t311;
            let t325 = t152 * t153 * t96;
            let t327 = -f64x8::splat(1250.0) / f64x8::splat(2187.0) * t312 - f64x8::splat(25.0) / f64x8::splat(162.0) * t315 + f64x8::splat(100.0) / f64x8::splat(243.0) * t319 - f64x8::splat(625.0) / f64x8::splat(729.0) * t322 + f64x8::splat(5000.0) / f64x8::splat(6561.0) * t325;
            let t328 = t106 * t327;
            let t331 = -f64x8::splat(5.0) / f64x8::splat(27.0) * t297 - f64x8::splat(5.0) / f64x8::splat(288.0) * t300 + f64x8::splat(5.0) / f64x8::splat(36.0) * t225 * t302 + f64x8::splat(5.0) / f64x8::splat(16.0) * t44 * t305 * t117 - t44 * t328 / f64x8::splat(4.0);
            let t337 = t7 * t128 * t161;
            let t343 = t7 * t167 * t121;
            let t349 = t18 / t20 / t269 / t94;
            let t351 = t7 * t349 * t67;
            let t352 = t71 * v_sigma;
            let t355 = t281 * t59 * t352 * t22;
            let t359 = ((t3).select(f64x8::splat(0.0), -t19 * t87 * t161 * t82 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t19 * t20 * t331 * t82 - t337 * t137 / f64x8::splat(1152.0) - f64x8::splat(5.0) / f64x8::splat(3072.0) * t130 * t172 + t343 * t172 / f64x8::splat(3072.0) + t351 * t355 / f64x8::splat(24576.0)));
            let tv2rhosigma0 = f64x8::splat(2.0) * v_rho * t359 + f64x8::splat(2.0) * t176;
            acc_v2rhosigma = tv2rhosigma0;
            let tv2rholapl0 = f64x8::splat(0.0);
            acc_v2rholapl = tv2rholapl0;
            let t369 = t226 * t188;
            let t372 = t232 * t188;
            let t376 = t310 * t181;
            let t385 = f64x8::splat(10000.0) / f64x8::splat(2187.0) * t309 * t376 + f64x8::splat(100.0) / f64x8::splat(81.0) * t314 * t182 - f64x8::splat(500.0) / f64x8::splat(243.0) * t150 + f64x8::splat(5000.0) / f64x8::splat(729.0) * t321 * t376 - f64x8::splat(25000.0) / f64x8::splat(6561.0) * t155;
            let t386 = t106 * t385;
            let t389 = f64x8::splat(25.0) / f64x8::splat(27.0) * t146 + f64x8::splat(5.0) / f64x8::splat(36.0) * t178 * t299 + f64x8::splat(5.0) / f64x8::splat(36.0) * t225 * t369 + f64x8::splat(5.0) / f64x8::splat(16.0) * t44 * t372 * t117 - t44 * t386 / f64x8::splat(4.0);
            let t395 = t7 * t128 * t192;
            let t399 = ((t3).select(f64x8::splat(0.0), -t19 * t87 * t192 * t82 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t19 * t20 * t389 * t82 - t395 * t137 / f64x8::splat(1152.0)));
            let tv2rhotau0 = f64x8::splat(2.0) * v_rho * t399 + f64x8::splat(2.0) * t197;
            acc_v2rhotau = tv2rhotau0;
            let t402 = t101 * t158;
            let t403 = t145 * t402;
            let t405 = t157 * t157;
            let t406 = t232 * t405;
            let t409 = t59 * t21;
            let t410 = t409 * t76;
            let t411 = t309 * t410;
            let t413 = t107 * t21;
            let t416 = t413 * t76 * t47 * t49;
            let t418 = t321 * t410;
            let t420 = f64x8::splat(625.0) / f64x8::splat(4374.0) * t411 + f64x8::splat(25.0) / f64x8::splat(648.0) * t416 + f64x8::splat(625.0) / f64x8::splat(2916.0) * t418;
            let t421 = t106 * t420;
            let t424 = -f64x8::splat(5.0) / f64x8::splat(144.0) * t403 + f64x8::splat(5.0) / f64x8::splat(16.0) * t44 * t406 - t44 * t421 / f64x8::splat(4.0);
            let t430 = t7 * t167 * t161;
            let t433 = t269 * t29;
            let t436 = t18 / t20 / t433;
            let t438 = t7 * t436 * t67;
            let t441 = t281 * t59 * t71 * t22;
            let t444 = t50 * t21;
            let t445 = t133 * t444;
            let t449 = ((t3).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t19 * t20 * t424 * t82 + t430 * t172 / f64x8::splat(1536.0) - t438 * t441 / f64x8::splat(65536.0) + t169 * t445 / f64x8::splat(3072.0)));
            let tv2sigma20 = f64x8::splat(2.0) * v_rho * t449;
            acc_v2sigma2 = tv2sigma20;
            let tv2sigmalapl0 = f64x8::splat(0.0);
            acc_v2sigmalapl = tv2sigmalapl0;
            let t453 = t101 * t189;
            let t454 = t145 * t453;
            let t460 = f64x8::splat(1.0) / t20 / t73;
            let t461 = t409 * t460;
            let t462 = t309 * t461;
            let t466 = t413 * t460 * t47 * t49;
            let t468 = t321 * t461;
            let t470 = -f64x8::splat(2500.0) / f64x8::splat(2187.0) * t462 - f64x8::splat(25.0) / f64x8::splat(81.0) * t466 - f64x8::splat(1250.0) / f64x8::splat(729.0) * t468;
            let t471 = t106 * t470;
            let t474 = f64x8::splat(5.0) / f64x8::splat(36.0) * t178 * t402 - f64x8::splat(5.0) / f64x8::splat(288.0) * t454 + f64x8::splat(5.0) / f64x8::splat(16.0) * t44 * t372 * t157 - t44 * t471 / f64x8::splat(4.0);
            let t480 = t7 * t167 * t192;
            let t484 = ((t3).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t19 * t20 * t474 * t82 + t480 * t172 / f64x8::splat(3072.0)));
            let tv2sigmatau0 = f64x8::splat(2.0) * v_rho * t484;
            acc_v2sigmatau = tv2sigmatau0;
            let tv2lapl20 = f64x8::splat(0.0);
            acc_v2lapl2 = tv2lapl20;
            let tv2lapltau0 = f64x8::splat(0.0);
            acc_v2lapltau = tv2lapltau0;
            let t488 = t188 * t188;
            let t489 = t232 * t488;
            let t493 = f64x8::splat(1.0) / t20 / t94;
            let t494 = t409 * t493;
            let t503 = f64x8::splat(20000.0) / f64x8::splat(2187.0) * t309 * t494 + f64x8::splat(200.0) / f64x8::splat(81.0) * t413 * t493 * t47 * t49 + f64x8::splat(10000.0) / f64x8::splat(729.0) * t321 * t494;
            let t504 = t106 * t503;
            let t507 = f64x8::splat(5.0) / f64x8::splat(18.0) * t178 * t453 + f64x8::splat(5.0) / f64x8::splat(16.0) * t44 * t489 - t44 * t504 / f64x8::splat(4.0);
            let t512 = ((t3).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t19 * t20 * t507 * t82));
            let tv2tau20 = f64x8::splat(2.0) * v_rho * t512;
            acc_v2tau2 = tv2tau20;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_vrho.into(); vrho[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_vsigma.into(); vsigma[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_vlapl.into(); vlapl[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_vtau.into(); vtau[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v2rho2.into(); v2rho2[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v2rhosigma.into(); v2rhosigma[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v2rholapl.into(); v2rholapl[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v2rhotau.into(); v2rhotau[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v2sigma2.into(); v2sigma2[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v2sigmalapl.into(); v2sigmalapl[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v2sigmatau.into(); v2sigmatau[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v2lapl2.into(); v2lapl2[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v2lapltau.into(); v2lapltau[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v2tau2.into(); v2tau2[ip..ip + m].copy_from_slice(&a[..m]); }
        ip += 8;
    }
}
