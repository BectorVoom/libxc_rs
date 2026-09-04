//! MGGA_X_REVTM fxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_revtm.c`
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
pub fn mgga_x_revtm_fxc_unpol(
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
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
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
            let t19 = (simd::cbrt(v_rho));
            let t20 = t18 * t19;
            let t21 = f64x8::splat(1.0) / v_rho;
            let t22 = v_sigma * t21;
            let t23 = f64x8::splat(1.0) / v_tau;
            let t25 = t22 * t23 / f64x8::splat(8.0);
            let t26 = (t25).simd_lt(f64x8::splat(1.0));
            let t27 = ((t26).select(t25, f64x8::splat(1.0)));
            let t28 = t27 * t27;
            let t29 = t28 * t27;
            let t31 = t28 + f64x8::splat(3.0) * t29;
            let t32 = f64x8::splat(1.0) + t29;
            let t33 = t32 * t32;
            let t34 = f64x8::splat(1.0) / t33;
            let t35 = t31 * t34;
            let t36 = f64x8::splat(M_CBRT6);
            let t37 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t38 = (simd::cbrt(t37));
            let t39 = t38 * t38;
            let t40 = f64x8::splat(1.0) / t39;
            let t41 = t36 * t40;
            let t42 = f64x8::splat(M_CBRT2);
            let t43 = t42 * t42;
            let t44 = v_sigma * t43;
            let t45 = v_rho * v_rho;
            let t46 = t19 * t19;
            let t48 = f64x8::splat(1.0) / t46 / t45;
            let t49 = t44 * t48;
            let t50 = t41 * t49;
            let t52 = t36 * t36;
            let t54 = f64x8::splat(1.0) / t38 / t37;
            let t55 = t52 * t54;
            let t56 = v_sigma * v_sigma;
            let t57 = t56 * t42;
            let t58 = t45 * t45;
            let t59 = t58 * v_rho;
            let t61 = f64x8::splat(1.0) / t19 / t59;
            let t65 = f64x8::splat(1.0) + f64x8::splat(0.1504548888888889) * t50 + f64x8::splat(0.00537989809245259) * t55 * t57 * t61;
            let t66 = (simd::pow(t65, f64x8::splat(1.0) / f64x8::splat(5.0)));
            let t69 = v_tau * t43;
            let t71 = f64x8::splat(1.0) / t46 / v_rho;
            let t72 = t69 * t71;
            let t81 = f64x8::splat(1.0) + f64x8::splat(0.06394332777777778) * t50 - f64x8::splat(5.0) / f64x8::splat(9.0) * (f64x8::splat(0.14554132) * t72 + f64x8::splat(0.256337604) * t52 * t39 + f64x8::splat(0.011867481666666667) * t49) * t36 * t40;
            let t82 = t66 * t66;
            let t83 = f64x8::splat(1.0) / t82;
            let t86 = f64x8::splat(1.0) / t66 + f64x8::splat(7.0) / f64x8::splat(9.0) * t81 * t83;
            let t88 = f64x8::splat(1.0) - t35;
            let t91 = (f64x8::splat(10.0) / f64x8::splat(81.0) + f64x8::splat(25.0) / f64x8::splat(8748.0) * t50) * t36;
            let t92 = t91 * t40;
            let t96 = t72 - t49 / f64x8::splat(8.0);
            let t97 = t96 * t36;
            let t100 = f64x8::splat(5.0) / f64x8::splat(9.0) * t97 * t40 - f64x8::splat(1.0);
            let t101 = t40 * t100;
            let t104 = f64x8::splat(1.0) + f64x8::splat(0.2222222222222222) * t97 * t101;
            let t105 = ((t104).sqrt());
            let t106 = f64x8::splat(1.0) / t105;
            let t110 = f64x8::splat(9.0) / f64x8::splat(20.0) * t100 * t106 + t50 / f64x8::splat(36.0);
            let t111 = t110 * t110;
            let t113 = t110 * t27;
            let t114 = f64x8::splat(1.0) - t27;
            let t117 = f64x8::splat(1.0) + f64x8::splat(5.0) / f64x8::splat(12.0) * t92 * t49 + f64x8::splat(292.0) / f64x8::splat(405.0) * t111 - f64x8::splat(146.0) / f64x8::splat(135.0) * t113 * t114;
            let t118 = (simd::pow(t117, f64x8::splat(1.0) / f64x8::splat(10.0)));
            let t120 = t88 * t118 + t35 * t86;
            let t124 = ((t3).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t7 * t20 * t120));
            let tzk0 = f64x8::splat(2.0) * t124;
            acc_zk = tzk0;
            let t126 = t18 / t46;
            let t130 = f64x8::splat(1.0) / t45;
            let t131 = v_sigma * t130;
            let t134 = ((t26).select(-t131 * t23 / f64x8::splat(8.0), f64x8::splat(0.0)));
            let t135 = t27 * t134;
            let t137 = t28 * t134;
            let t139 = f64x8::splat(2.0) * t135 + f64x8::splat(9.0) * t137;
            let t140 = t139 * t34;
            let t143 = f64x8::splat(1.0) / t33 / t32;
            let t144 = t31 * t143;
            let t145 = t86 * t28;
            let t146 = t145 * t134;
            let t150 = f64x8::splat(1.0) / t66 / t65;
            let t151 = t45 * v_rho;
            let t153 = f64x8::splat(1.0) / t46 / t151;
            let t154 = t44 * t153;
            let t155 = t41 * t154;
            let t157 = t58 * t45;
            let t159 = f64x8::splat(1.0) / t19 / t157;
            let t161 = t55 * t57 * t159;
            let t163 = -f64x8::splat(0.40121303703703703) * t155 - f64x8::splat(0.028692789826413812) * t161;
            let t167 = t69 * t48;
            let t174 = -f64x8::splat(0.17051554074074074) * t155 - f64x8::splat(5.0) / f64x8::splat(9.0) * (-f64x8::splat(0.24256886666666666) * t167 - f64x8::splat(0.031646617777777775) * t154) * t36 * t40;
            let t178 = f64x8::splat(1.0) / t82 / t65;
            let t179 = t81 * t178;
            let t182 = -t150 * t163 / f64x8::splat(5.0) + f64x8::splat(7.0) / f64x8::splat(9.0) * t174 * t83 - f64x8::splat(14.0) / f64x8::splat(45.0) * t179 * t163;
            let t186 = f64x8::splat(6.0) * t144 * t137 - t140;
            let t188 = t118 * t118;
            let t189 = t188 * t188;
            let t190 = t189 * t189;
            let t191 = t190 * t118;
            let t192 = f64x8::splat(1.0) / t191;
            let t193 = t88 * t192;
            let t199 = -f64x8::splat(5.0) / f64x8::splat(3.0) * t167 + t154 / f64x8::splat(3.0);
            let t200 = t199 * t36;
            let t201 = t40 * t106;
            let t205 = f64x8::splat(1.0) / t105 / t104;
            let t206 = t100 * t205;
            let t209 = t96 * t52;
            let t210 = t54 * t199;
            let t213 = f64x8::splat(0.2222222222222222) * t200 * t101 + f64x8::splat(0.12345679012345678) * t209 * t210;
            let t217 = t200 * t201 / f64x8::splat(4.0) - f64x8::splat(9.0) / f64x8::splat(40.0) * t206 * t213 - f64x8::splat(2.0) / f64x8::splat(27.0) * t155;
            let t220 = t217 * t27;
            let t223 = t110 * t134;
            let t228 = -f64x8::splat(125.0) / f64x8::splat(19683.0) * t161 - f64x8::splat(10.0) / f64x8::splat(9.0) * t92 * t154 + f64x8::splat(584.0) / f64x8::splat(405.0) * t110 * t217 - f64x8::splat(146.0) / f64x8::splat(135.0) * t220 * t114 - f64x8::splat(146.0) / f64x8::splat(135.0) * t223 * t114 + f64x8::splat(146.0) / f64x8::splat(135.0) * t113 * t134;
            let t231 = t140 * t86 - f64x8::splat(6.0) * t144 * t146 + t35 * t182 + t186 * t118 + t193 * t228 / f64x8::splat(10.0);
            let t236 = ((t3).select(f64x8::splat(0.0), -t7 * t126 * t120 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t7 * t20 * t231));
            let tvrho0 = f64x8::splat(2.0) * v_rho * t236 + f64x8::splat(2.0) * t124;
            acc_vrho = tvrho0;
            let t241 = ((t26).select(t21 * t23 / f64x8::splat(8.0), f64x8::splat(0.0)));
            let t242 = t27 * t241;
            let t244 = t28 * t241;
            let t246 = f64x8::splat(2.0) * t242 + f64x8::splat(9.0) * t244;
            let t247 = t246 * t34;
            let t249 = t145 * t241;
            let t252 = t43 * t48;
            let t253 = t41 * t252;
            let t255 = v_sigma * t42;
            let t257 = t55 * t255 * t61;
            let t259 = f64x8::splat(0.1504548888888889) * t253 + f64x8::splat(0.01075979618490518) * t257;
            let t267 = -t150 * t259 / f64x8::splat(5.0) + f64x8::splat(0.04460577520576132) * t41 * t252 * t83 - f64x8::splat(14.0) / f64x8::splat(45.0) * t179 * t259;
            let t271 = f64x8::splat(6.0) * t144 * t244 - t247;
            let t274 = t40 * t43;
            let t279 = t41 * t252 * t106;
            let t281 = t41 * t100;
            let t282 = t252 * t281;
            let t284 = t54 * t43;
            let t286 = t209 * t284 * t48;
            let t288 = -f64x8::splat(0.027777777777777776) * t282 - f64x8::splat(0.015432098765432098) * t286;
            let t292 = -t279 / f64x8::splat(32.0) - f64x8::splat(9.0) / f64x8::splat(40.0) * t206 * t288 + t253 / f64x8::splat(36.0);
            let t295 = t292 * t27;
            let t298 = t110 * t241;
            let t303 = f64x8::splat(125.0) / f64x8::splat(52488.0) * t257 + f64x8::splat(5.0) / f64x8::splat(12.0) * t91 * t274 * t48 + f64x8::splat(584.0) / f64x8::splat(405.0) * t110 * t292 - f64x8::splat(146.0) / f64x8::splat(135.0) * t295 * t114 - f64x8::splat(146.0) / f64x8::splat(135.0) * t298 * t114 + f64x8::splat(146.0) / f64x8::splat(135.0) * t113 * t241;
            let t306 = t247 * t86 - f64x8::splat(6.0) * t144 * t249 + t35 * t267 + t271 * t118 + t193 * t303 / f64x8::splat(10.0);
            let t310 = ((t3).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t7 * t20 * t306));
            let tvsigma0 = f64x8::splat(2.0) * v_rho * t310;
            acc_vsigma = tvsigma0;
            let tvlapl0 = f64x8::splat(0.0);
            acc_vlapl = tvlapl0;
            let t312 = v_tau * v_tau;
            let t313 = f64x8::splat(1.0) / t312;
            let t316 = ((t26).select(-t22 * t313 / f64x8::splat(8.0), f64x8::splat(0.0)));
            let t317 = t27 * t316;
            let t319 = t28 * t316;
            let t321 = f64x8::splat(2.0) * t317 + f64x8::splat(9.0) * t319;
            let t322 = t321 * t34;
            let t324 = t145 * t316;
            let t327 = t35 * t43;
            let t329 = t40 * t83;
            let t330 = t71 * t36 * t329;
            let t335 = f64x8::splat(6.0) * t144 * t319 - t322;
            let t337 = t43 * t71;
            let t346 = f64x8::splat(0.2222222222222222) * t337 * t281 + f64x8::splat(0.12345679012345678) * t209 * t284 * t71;
            let t349 = t337 * t41 * t106 / f64x8::splat(4.0) - f64x8::splat(9.0) / f64x8::splat(40.0) * t206 * t346;
            let t352 = t349 * t27;
            let t355 = t110 * t316;
            let t360 = f64x8::splat(584.0) / f64x8::splat(405.0) * t110 * t349 - f64x8::splat(146.0) / f64x8::splat(135.0) * t352 * t114 - f64x8::splat(146.0) / f64x8::splat(135.0) * t355 * t114 + f64x8::splat(146.0) / f64x8::splat(135.0) * t113 * t316;
            let t363 = t322 * t86 - f64x8::splat(6.0) * t144 * t324 - f64x8::splat(0.06288822469135802) * t327 * t330 + t335 * t118 + t193 * t360 / f64x8::splat(10.0);
            let t367 = ((t3).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t7 * t20 * t363));
            let tvtau0 = f64x8::splat(2.0) * v_rho * t367;
            acc_vtau = tvtau0;
            let t370 = t18 * t71;
            let t377 = t134 * t134;
            let t379 = f64x8::splat(1.0) / t151;
            let t380 = v_sigma * t379;
            let t383 = ((t26).select(t380 * t23 / f64x8::splat(4.0), f64x8::splat(0.0)));
            let t386 = t27 * t377;
            let t388 = t28 * t383;
            let t390 = f64x8::splat(2.0) * t27 * t383 + f64x8::splat(2.0) * t377 + f64x8::splat(18.0) * t386 + f64x8::splat(9.0) * t388;
            let t391 = t390 * t34;
            let t393 = t139 * t143;
            let t398 = t33 * t33;
            let t399 = f64x8::splat(1.0) / t398;
            let t400 = t31 * t399;
            let t401 = t28 * t28;
            let t402 = t86 * t401;
            let t403 = t402 * t377;
            let t406 = t182 * t28;
            let t407 = t406 * t134;
            let t410 = t86 * t27;
            let t411 = t410 * t377;
            let t414 = t145 * t383;
            let t417 = t65 * t65;
            let t419 = f64x8::splat(1.0) / t66 / t417;
            let t420 = t163 * t163;
            let t424 = f64x8::splat(1.0) / t46 / t58;
            let t425 = t44 * t424;
            let t426 = t41 * t425;
            let t428 = t58 * t151;
            let t430 = f64x8::splat(1.0) / t19 / t428;
            let t432 = t55 * t57 * t430;
            let t434 = f64x8::splat(1.4711144691358025) * t426 + f64x8::splat(0.18172100223395413) * t432;
            let t438 = t69 * t153;
            let t445 = f64x8::splat(0.625223649382716) * t426 - f64x8::splat(5.0) / f64x8::splat(9.0) * (f64x8::splat(0.6468503111111111) * t438 + f64x8::splat(0.11603759851851851) * t425) * t36 * t40;
            let t448 = t174 * t178;
            let t452 = f64x8::splat(1.0) / t82 / t417;
            let t453 = t81 * t452;
            let t458 = f64x8::splat(6.0) / f64x8::splat(25.0) * t419 * t420 - t150 * t434 / f64x8::splat(5.0) + f64x8::splat(7.0) / f64x8::splat(9.0) * t445 * t83 - f64x8::splat(28.0) / f64x8::splat(45.0) * t448 * t163 + f64x8::splat(98.0) / f64x8::splat(225.0) * t453 * t420 - f64x8::splat(14.0) / f64x8::splat(45.0) * t179 * t434;
            let t462 = t401 * t377;
            let t469 = f64x8::splat(12.0) * t393 * t137 + f64x8::splat(12.0) * t144 * t386 + f64x8::splat(6.0) * t144 * t388 - f64x8::splat(54.0) * t400 * t462 - t391;
            let t471 = t186 * t192;
            let t475 = f64x8::splat(1.0) / t191 / t117;
            let t476 = t88 * t475;
            let t477 = t228 * t228;
            let t483 = t217 * t217;
            let t487 = f64x8::splat(40.0) / f64x8::splat(9.0) * t438 - f64x8::splat(11.0) / f64x8::splat(9.0) * t425;
            let t488 = t487 * t36;
            let t491 = t40 * t205;
            let t492 = t491 * t213;
            let t495 = t104 * t104;
            let t497 = f64x8::splat(1.0) / t105 / t495;
            let t498 = t100 * t497;
            let t499 = t213 * t213;
            let t504 = t199 * t199;
            let t511 = f64x8::splat(0.2222222222222222) * t488 * t101 + f64x8::splat(0.24691358024691357) * t504 * t52 * t54 + f64x8::splat(0.12345679012345678) * t209 * t54 * t487;
            let t515 = t488 * t201 / f64x8::splat(4.0) - t200 * t492 / f64x8::splat(4.0) + f64x8::splat(27.0) / f64x8::splat(80.0) * t498 * t499 - f64x8::splat(9.0) / f64x8::splat(40.0) * t206 * t511 + f64x8::splat(22.0) / f64x8::splat(81.0) * t426;
            let t518 = t515 * t27;
            let t521 = t217 * t134;
            let t526 = t110 * t383;
            let t533 = f64x8::splat(125.0) / f64x8::splat(2187.0) * t432 + f64x8::splat(110.0) / f64x8::splat(27.0) * t92 * t425 + f64x8::splat(584.0) / f64x8::splat(405.0) * t483 + f64x8::splat(584.0) / f64x8::splat(405.0) * t110 * t515 - f64x8::splat(146.0) / f64x8::splat(135.0) * t518 * t114 - f64x8::splat(292.0) / f64x8::splat(135.0) * t521 * t114 + f64x8::splat(292.0) / f64x8::splat(135.0) * t220 * t134 - f64x8::splat(146.0) / f64x8::splat(135.0) * t526 * t114 + f64x8::splat(292.0) / f64x8::splat(135.0) * t110 * t377 + f64x8::splat(146.0) / f64x8::splat(135.0) * t113 * t383;
            let t536 = t391 * t86 - f64x8::splat(12.0) * t393 * t146 + f64x8::splat(2.0) * t140 * t182 + f64x8::splat(54.0) * t400 * t403 - f64x8::splat(12.0) * t144 * t407 - f64x8::splat(12.0) * t144 * t411 - f64x8::splat(6.0) * t144 * t414 + t35 * t458 + t469 * t118 + t471 * t228 / f64x8::splat(5.0) - f64x8::splat(9.0) / f64x8::splat(100.0) * t476 * t477 + t193 * t533 / f64x8::splat(10.0);
            let t541 = ((t3).select(f64x8::splat(0.0), t7 * t370 * t120 / f64x8::splat(12.0) - t7 * t126 * t231 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t7 * t20 * t536));
            let tv2rho20 = f64x8::splat(2.0) * v_rho * t541 + f64x8::splat(4.0) * t236;
            acc_v2rho2 = tv2rho20;
            let t547 = t134 * t241;
            let t551 = ((t26).select(-t130 * t23 / f64x8::splat(8.0), f64x8::splat(0.0)));
            let t552 = t27 * t551;
            let t554 = t242 * t134;
            let t556 = t28 * t551;
            let t558 = f64x8::splat(2.0) * t547 + f64x8::splat(2.0) * t552 + f64x8::splat(18.0) * t554 + f64x8::splat(9.0) * t556;
            let t559 = t558 * t34;
            let t561 = t246 * t143;
            let t567 = t400 * t86;
            let t568 = t401 * t241;
            let t569 = t568 * t134;
            let t572 = t406 * t241;
            let t575 = t144 * t86;
            let t578 = t145 * t551;
            let t582 = t267 * t28;
            let t583 = t582 * t134;
            let t586 = t419 * t259;
            let t589 = t43 * t153;
            let t590 = t41 * t589;
            let t593 = t55 * t255 * t159;
            let t595 = -f64x8::splat(0.40121303703703703) * t590 - f64x8::splat(0.057385579652827624) * t593;
            let t601 = t41 * t43;
            let t602 = t48 * t178;
            let t608 = t259 * t163;
            let t613 = f64x8::splat(6.0) / f64x8::splat(25.0) * t586 * t163 - t150 * t595 / f64x8::splat(5.0) - f64x8::splat(0.11894873388203017) * t41 * t589 * t83 - f64x8::splat(0.017842310082304528) * t601 * t602 * t163 - f64x8::splat(14.0) / f64x8::splat(45.0) * t448 * t259 + f64x8::splat(98.0) / f64x8::splat(225.0) * t453 * t608 - f64x8::splat(14.0) / f64x8::splat(45.0) * t179 * t595;
            let t625 = f64x8::splat(6.0) * t561 * t137 + f64x8::splat(12.0) * t144 * t554 + f64x8::splat(6.0) * t144 * t556 + f64x8::splat(6.0) * t393 * t244 - f64x8::splat(54.0) * t400 * t569 - t559;
            let t627 = t271 * t192;
            let t632 = t303 * t228;
            let t642 = t41 * t589 * t106;
            let t644 = t48 * t205;
            let t646 = t601 * t644 * t213;
            let t648 = t491 * t288;
            let t651 = t288 * t213;
            let t654 = t589 * t281;
            let t656 = t55 * t199;
            let t657 = t252 * t656;
            let t660 = t209 * t284 * t153;
            let t662 = f64x8::splat(0.07407407407407407) * t654 - f64x8::splat(0.030864197530864196) * t657 + f64x8::splat(0.0411522633744856) * t660;
            let t666 = t642 / f64x8::splat(12.0) + t646 / f64x8::splat(64.0) - t200 * t648 / f64x8::splat(8.0) + f64x8::splat(27.0) / f64x8::splat(80.0) * t498 * t651 - f64x8::splat(9.0) / f64x8::splat(40.0) * t206 * t662 - f64x8::splat(2.0) / f64x8::splat(27.0) * t590;
            let t669 = t666 * t27;
            let t672 = t292 * t134;
            let t677 = t217 * t241;
            let t680 = t110 * t551;
            let t689 = -f64x8::splat(125.0) / f64x8::splat(6561.0) * t593 - f64x8::splat(10.0) / f64x8::splat(9.0) * t91 * t274 * t153 + f64x8::splat(584.0) / f64x8::splat(405.0) * t217 * t292 + f64x8::splat(584.0) / f64x8::splat(405.0) * t110 * t666 - f64x8::splat(146.0) / f64x8::splat(135.0) * t669 * t114 - f64x8::splat(146.0) / f64x8::splat(135.0) * t672 * t114 + f64x8::splat(146.0) / f64x8::splat(135.0) * t295 * t134 - f64x8::splat(146.0) / f64x8::splat(135.0) * t677 * t114 - f64x8::splat(146.0) / f64x8::splat(135.0) * t680 * t114 + f64x8::splat(292.0) / f64x8::splat(135.0) * t298 * t134 + f64x8::splat(146.0) / f64x8::splat(135.0) * t220 * t241 + f64x8::splat(146.0) / f64x8::splat(135.0) * t113 * t551;
            let t692 = t559 * t86 - f64x8::splat(6.0) * t561 * t146 + t247 * t182 - f64x8::splat(6.0) * t393 * t249 + f64x8::splat(54.0) * t567 * t569 - f64x8::splat(6.0) * t144 * t572 - f64x8::splat(12.0) * t575 * t554 - f64x8::splat(6.0) * t144 * t578 + t140 * t267 - f64x8::splat(6.0) * t144 * t583 + t35 * t613 + t625 * t118 + t627 * t228 / f64x8::splat(10.0) + t471 * t303 / f64x8::splat(10.0) - f64x8::splat(9.0) / f64x8::splat(100.0) * t476 * t632 + t193 * t689 / f64x8::splat(10.0);
            let t697 = ((t3).select(f64x8::splat(0.0), -t7 * t126 * t306 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t7 * t20 * t692));
            let tv2rhosigma0 = f64x8::splat(2.0) * v_rho * t697 + f64x8::splat(2.0) * t310;
            acc_v2rhosigma = tv2rhosigma0;
            let tv2rholapl0 = f64x8::splat(0.0);
            acc_v2rholapl = tv2rholapl0;
            let t703 = t134 * t316;
            let t707 = ((t26).select(t131 * t313 / f64x8::splat(8.0), f64x8::splat(0.0)));
            let t708 = t27 * t707;
            let t710 = t317 * t134;
            let t712 = t28 * t707;
            let t714 = f64x8::splat(2.0) * t703 + f64x8::splat(2.0) * t708 + f64x8::splat(18.0) * t710 + f64x8::splat(9.0) * t712;
            let t715 = t714 * t34;
            let t717 = t321 * t143;
            let t723 = t401 * t316;
            let t724 = t723 * t134;
            let t727 = t406 * t316;
            let t732 = t145 * t707;
            let t735 = t140 * t43;
            let t738 = t144 * t337;
            let t739 = t83 * t28;
            let t741 = t41 * t739 * t134;
            let t745 = t48 * t36 * t329;
            let t748 = t35 * t337;
            let t750 = t41 * t178 * t163;
            let t763 = f64x8::splat(6.0) * t717 * t137 + f64x8::splat(12.0) * t144 * t710 + f64x8::splat(6.0) * t144 * t712 + f64x8::splat(6.0) * t393 * t319 - f64x8::splat(54.0) * t400 * t724 - t715;
            let t765 = t335 * t192;
            let t770 = t360 * t228;
            let t776 = t337 * t36;
            let t779 = t491 * t346;
            let t782 = t346 * t213;
            let t789 = -f64x8::splat(0.37037037037037035) * t282 + f64x8::splat(0.24691358024691357) * t337 * t656 - f64x8::splat(0.205761316872428) * t286;
            let t792 = -f64x8::splat(5.0) / f64x8::splat(12.0) * t279 - t776 * t492 / f64x8::splat(8.0) - t200 * t779 / f64x8::splat(8.0) + f64x8::splat(27.0) / f64x8::splat(80.0) * t498 * t782 - f64x8::splat(9.0) / f64x8::splat(40.0) * t206 * t789;
            let t795 = t792 * t27;
            let t798 = t349 * t134;
            let t803 = t217 * t316;
            let t806 = t110 * t707;
            let t815 = f64x8::splat(584.0) / f64x8::splat(405.0) * t217 * t349 + f64x8::splat(584.0) / f64x8::splat(405.0) * t110 * t792 - f64x8::splat(146.0) / f64x8::splat(135.0) * t795 * t114 - f64x8::splat(146.0) / f64x8::splat(135.0) * t798 * t114 + f64x8::splat(146.0) / f64x8::splat(135.0) * t352 * t134 - f64x8::splat(146.0) / f64x8::splat(135.0) * t803 * t114 - f64x8::splat(146.0) / f64x8::splat(135.0) * t806 * t114 + f64x8::splat(292.0) / f64x8::splat(135.0) * t355 * t134 + f64x8::splat(146.0) / f64x8::splat(135.0) * t220 * t316 + f64x8::splat(146.0) / f64x8::splat(135.0) * t113 * t707;
            let t818 = t715 * t86 - f64x8::splat(6.0) * t717 * t146 + t322 * t182 - f64x8::splat(6.0) * t393 * t324 + f64x8::splat(54.0) * t567 * t724 - f64x8::splat(6.0) * t144 * t727 - f64x8::splat(12.0) * t575 * t710 - f64x8::splat(6.0) * t144 * t732 - f64x8::splat(0.06288822469135802) * t735 * t330 + f64x8::splat(0.37732934814814817) * t738 * t741 + f64x8::splat(0.10481370781893004) * t327 * t745 + f64x8::splat(0.02515528987654321) * t748 * t750 + t763 * t118 + t765 * t228 / f64x8::splat(10.0) + t471 * t360 / f64x8::splat(10.0) - f64x8::splat(9.0) / f64x8::splat(100.0) * t476 * t770 + t193 * t815 / f64x8::splat(10.0);
            let t823 = ((t3).select(f64x8::splat(0.0), -t7 * t126 * t363 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t7 * t20 * t818));
            let tv2rhotau0 = f64x8::splat(2.0) * v_rho * t823 + f64x8::splat(2.0) * t367;
            acc_v2rhotau = tv2rhotau0;
            let t826 = t241 * t241;
            let t828 = ((t26).select(f64x8::splat(0.0), f64x8::splat(0.0)));
            let t829 = t27 * t828;
            let t830 = f64x8::splat(2.0) * t829;
            let t831 = t27 * t826;
            let t833 = t28 * t828;
            let t834 = f64x8::splat(9.0) * t833;
            let t835 = f64x8::splat(2.0) * t826 + t830 + f64x8::splat(18.0) * t831 + t834;
            let t836 = t835 * t34;
            let t842 = t402 * t826;
            let t845 = t582 * t241;
            let t848 = t410 * t826;
            let t851 = t145 * t828;
            let t853 = f64x8::splat(6.0) * t144 * t851;
            let t854 = t259 * t259;
            let t857 = t150 * t52;
            let t858 = t54 * t42;
            let t859 = t858 * t61;
            let t867 = t179 * t52;
            let t870 = f64x8::splat(6.0) / f64x8::splat(25.0) * t419 * t854 - f64x8::splat(0.0021519592369810357) * t857 * t859 - f64x8::splat(0.035684620164609056) * t601 * t602 * t259 + f64x8::splat(98.0) / f64x8::splat(225.0) * t453 * t854 - f64x8::splat(0.0033474921464149445) * t867 * t859;
            let t874 = t401 * t826;
            let t880 = f64x8::splat(6.0) * t144 * t833;
            let t881 = f64x8::splat(12.0) * t144 * t831 + f64x8::splat(12.0) * t561 * t244 - f64x8::splat(54.0) * t400 * t874 - t836 + t880;
            let t885 = t303 * t303;
            let t888 = t42 * t61;
            let t889 = t55 * t888;
            let t891 = t292 * t292;
            let t894 = t601 * t644 * t288;
            let t896 = t288 * t288;
            let t899 = t206 * t52;
            let t900 = t899 * t859;
            let t902 = t894 / f64x8::splat(32.0) + f64x8::splat(27.0) / f64x8::splat(80.0) * t498 * t896 - f64x8::splat(0.001736111111111111) * t900;
            let t905 = t902 * t27;
            let t908 = t292 * t241;
            let t913 = t110 * t828;
            let t915 = f64x8::splat(146.0) / f64x8::splat(135.0) * t913 * t114;
            let t919 = f64x8::splat(146.0) / f64x8::splat(135.0) * t113 * t828;
            let t920 = f64x8::splat(125.0) / f64x8::splat(26244.0) * t889 + f64x8::splat(584.0) / f64x8::splat(405.0) * t891 + f64x8::splat(584.0) / f64x8::splat(405.0) * t110 * t902 - f64x8::splat(146.0) / f64x8::splat(135.0) * t905 * t114 - f64x8::splat(292.0) / f64x8::splat(135.0) * t908 * t114 + f64x8::splat(292.0) / f64x8::splat(135.0) * t295 * t241 - t915 + f64x8::splat(292.0) / f64x8::splat(135.0) * t110 * t826 + t919;
            let t923 = t836 * t86 - f64x8::splat(12.0) * t561 * t249 + f64x8::splat(2.0) * t247 * t267 + f64x8::splat(54.0) * t400 * t842 - f64x8::splat(12.0) * t144 * t845 - f64x8::splat(12.0) * t144 * t848 - t853 + t35 * t870 + t881 * t118 + t627 * t303 / f64x8::splat(5.0) - f64x8::splat(9.0) / f64x8::splat(100.0) * t476 * t885 + t193 * t920 / f64x8::splat(10.0);
            let t927 = ((t3).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t7 * t20 * t923));
            let tv2sigma20 = f64x8::splat(2.0) * v_rho * t927;
            acc_v2sigma2 = tv2sigma20;
            let tv2sigmalapl0 = f64x8::splat(0.0);
            acc_v2sigmalapl = tv2sigmalapl0;
            let t929 = t241 * t316;
            let t933 = ((t26).select(-t21 * t313 / f64x8::splat(8.0), f64x8::splat(0.0)));
            let t934 = t27 * t933;
            let t936 = t317 * t241;
            let t938 = t28 * t933;
            let t940 = f64x8::splat(2.0) * t929 + f64x8::splat(2.0) * t934 + f64x8::splat(18.0) * t936 + f64x8::splat(9.0) * t938;
            let t941 = t940 * t34;
            let t948 = t723 * t241;
            let t951 = t582 * t316;
            let t956 = t145 * t933;
            let t959 = t247 * t43;
            let t963 = t41 * t739 * t241;
            let t967 = t41 * t178 * t259;
            let t980 = f64x8::splat(12.0) * t144 * t936 + f64x8::splat(6.0) * t144 * t938 + f64x8::splat(6.0) * t717 * t244 + f64x8::splat(6.0) * t561 * t319 - f64x8::splat(54.0) * t400 * t948 - t941;
            let t986 = t360 * t303;
            let t994 = t601 * t644 * t346;
            let t996 = t346 * t288;
            let t999 = t206 * t42;
            let t1001 = f64x8::splat(1.0) / t19 / t58;
            let t1002 = t1001 * t52;
            let t1004 = t999 * t1002 * t54;
            let t1006 = -t776 * t648 / f64x8::splat(8.0) + t994 / f64x8::splat(64.0) + f64x8::splat(27.0) / f64x8::splat(80.0) * t498 * t996 + f64x8::splat(0.013888888888888888) * t1004;
            let t1009 = t1006 * t27;
            let t1012 = t349 * t241;
            let t1017 = t292 * t316;
            let t1020 = t110 * t933;
            let t1029 = f64x8::splat(584.0) / f64x8::splat(405.0) * t292 * t349 + f64x8::splat(584.0) / f64x8::splat(405.0) * t110 * t1006 - f64x8::splat(146.0) / f64x8::splat(135.0) * t1009 * t114 - f64x8::splat(146.0) / f64x8::splat(135.0) * t1012 * t114 + f64x8::splat(146.0) / f64x8::splat(135.0) * t352 * t241 - f64x8::splat(146.0) / f64x8::splat(135.0) * t1017 * t114 - f64x8::splat(146.0) / f64x8::splat(135.0) * t1020 * t114 + f64x8::splat(292.0) / f64x8::splat(135.0) * t355 * t241 + f64x8::splat(146.0) / f64x8::splat(135.0) * t295 * t316 + f64x8::splat(146.0) / f64x8::splat(135.0) * t113 * t933;
            let t1032 = t941 * t86 - f64x8::splat(6.0) * t717 * t249 + t322 * t267 - f64x8::splat(6.0) * t561 * t324 + f64x8::splat(54.0) * t567 * t948 - f64x8::splat(6.0) * t144 * t951 - f64x8::splat(12.0) * t575 * t936 - f64x8::splat(6.0) * t144 * t956 - f64x8::splat(0.06288822469135802) * t959 * t330 + f64x8::splat(0.37732934814814817) * t738 * t963 + f64x8::splat(0.02515528987654321) * t748 * t967 + t980 * t118 + t765 * t303 / f64x8::splat(10.0) + t627 * t360 / f64x8::splat(10.0) - f64x8::splat(9.0) / f64x8::splat(100.0) * t476 * t986 + t193 * t1029 / f64x8::splat(10.0);
            let t1036 = ((t3).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t7 * t20 * t1032));
            let tv2sigmatau0 = f64x8::splat(2.0) * v_rho * t1036;
            acc_v2sigmatau = tv2sigmatau0;
            let tv2lapl20 = f64x8::splat(0.0);
            acc_v2lapl2 = tv2lapl20;
            let tv2lapltau0 = f64x8::splat(0.0);
            acc_v2lapltau = tv2lapltau0;
            let t1038 = t316 * t316;
            let t1041 = f64x8::splat(1.0) / t312 / v_tau;
            let t1044 = ((t26).select(t22 * t1041 / f64x8::splat(4.0), f64x8::splat(0.0)));
            let t1045 = t27 * t1044;
            let t1047 = t27 * t1038;
            let t1049 = t28 * t1044;
            let t1051 = f64x8::splat(2.0) * t1038 + f64x8::splat(2.0) * t1045 + f64x8::splat(18.0) * t1047 + f64x8::splat(9.0) * t1049;
            let t1052 = t1051 * t34;
            let t1056 = t322 * t43;
            let t1059 = t402 * t1038;
            let t1063 = t41 * t739 * t316;
            let t1066 = t410 * t1038;
            let t1069 = t145 * t1044;
            let t1074 = t401 * t1038;
            let t1081 = f64x8::splat(12.0) * t144 * t1047 + f64x8::splat(6.0) * t144 * t1049 - f64x8::splat(54.0) * t400 * t1074 + f64x8::splat(12.0) * t717 * t319 - t1052;
            let t1085 = t360 * t360;
            let t1088 = t349 * t349;
            let t1092 = t346 * t346;
            let t1096 = f64x8::splat(1.0) / t19 / t151;
            let t1097 = t1096 * t52;
            let t1101 = -t776 * t779 / f64x8::splat(4.0) + f64x8::splat(27.0) / f64x8::splat(80.0) * t498 * t1092 - f64x8::splat(0.1111111111111111) * t999 * t1097 * t54;
            let t1104 = t1101 * t27;
            let t1107 = t349 * t316;
            let t1112 = t110 * t1044;
            let t1119 = f64x8::splat(584.0) / f64x8::splat(405.0) * t1088 + f64x8::splat(584.0) / f64x8::splat(405.0) * t110 * t1101 - f64x8::splat(146.0) / f64x8::splat(135.0) * t1104 * t114 - f64x8::splat(292.0) / f64x8::splat(135.0) * t1107 * t114 + f64x8::splat(292.0) / f64x8::splat(135.0) * t352 * t316 - f64x8::splat(146.0) / f64x8::splat(135.0) * t1112 * t114 + f64x8::splat(292.0) / f64x8::splat(135.0) * t110 * t1038 + f64x8::splat(146.0) / f64x8::splat(135.0) * t113 * t1044;
            let t1122 = t1052 * t86 - f64x8::splat(12.0) * t717 * t324 - f64x8::splat(0.12577644938271604) * t1056 * t330 + f64x8::splat(54.0) * t400 * t1059 + f64x8::splat(0.7546586962962963) * t738 * t1063 - f64x8::splat(12.0) * t144 * t1066 - f64x8::splat(6.0) * t144 * t1069 + t1081 * t118 + t765 * t360 / f64x8::splat(5.0) - f64x8::splat(9.0) / f64x8::splat(100.0) * t476 * t1085 + t193 * t1119 / f64x8::splat(10.0);
            let t1126 = ((t3).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t7 * t20 * t1122));
            let tv2tau20 = f64x8::splat(2.0) * v_rho * t1126;
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
