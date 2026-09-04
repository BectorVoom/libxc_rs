//! GGA_X_ITYH_OPTX fxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_ityh_optx.c`
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
pub fn gga_x_ityh_optx_fxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    v2rho2: &mut [f64],
    v2rhosigma: &mut [f64],
    v2sigma2: &mut [f64],
    param_b: f64,
    param_a: f64,
    param_hyb_omega_0: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_b = f64x8::splat(param_b);
    let param_a = f64x8::splat(param_a);
    let param_hyb_omega_0 = f64x8::splat(param_hyb_omega_0);
    let dens_threshold = f64x8::splat(dens_threshold);
    let zeta_threshold = f64x8::splat(zeta_threshold);
    let mut ip = 0usize;
    while ip < np {
        let m = (np - ip).min(8);
        let v_rho = load(rho, ip, np);
        let v_sigma = load(sigma, ip, np);
        let mut acc_zk = V_ZERO;
        let mut acc_vrho = V_ZERO;
        let mut acc_vsigma = V_ZERO;
        let mut acc_v2rho2 = V_ZERO;
        let mut acc_v2rhosigma = V_ZERO;
        let mut acc_v2sigma2 = V_ZERO;
        {
            let t2 = (v_rho / f64x8::splat(2.0)).simd_le(dens_threshold);
            let t3 = f64x8::splat(M_CBRT3);
            let t4 = f64x8::splat(M_CBRTPI);
            let t7 = (f64x8::splat(1.0)).simd_le(zeta_threshold);
            let t8 = zeta_threshold - f64x8::splat(1.0);
            let t10 = ((t7).select(t8, (t7).select(-t8, f64x8::splat(0.0))));
            let t11 = f64x8::splat(1.0) + t10;
            let t13 = (simd::cbrt(zeta_threshold));
            let t15 = (simd::cbrt(t11));
            let t17 = (((t11).simd_le(zeta_threshold)).select(t13 * zeta_threshold, t15 * t11));
            let t18 = t3 / t4 * t17;
            let t19 = (simd::cbrt(v_rho));
            let t20 = t3 * t3;
            let t22 = f64x8::splat(1.0) / f64x8::splat(M_PI);
            let t23 = (simd::cbrt(t22));
            let t24 = f64x8::splat(1.0) / t23;
            let t25 = f64x8::splat(M_CBRT4);
            let t26 = t24 * t25;
            let t27 = v_sigma * v_sigma;
            let t28 = param_b * t27;
            let t29 = f64x8::splat(M_CBRT2);
            let t30 = v_rho * v_rho;
            let t31 = t30 * t30;
            let t32 = t31 * v_rho;
            let t34 = f64x8::splat(1.0) / t19 / t32;
            let t36 = t29 * t29;
            let t38 = t19 * t19;
            let t40 = f64x8::splat(1.0) / t38 / t30;
            let t43 = f64x8::splat(1.0) + f64x8::splat(6.0) * v_sigma * t36 * t40;
            let t44 = t43 * t43;
            let t45 = f64x8::splat(1.0) / t44;
            let t46 = t29 * t34 * t45;
            let t49 = param_a + f64x8::splat(72.0) * t28 * t46;
            let t52 = f64x8::splat(M_PI) * t20 * t26 / t49;
            let t53 = ((t52).sqrt());
            let t55 = param_hyb_omega_0 / t53;
            let t56 = t11 * v_rho;
            let t57 = (simd::cbrt(t56));
            let t58 = f64x8::splat(1.0) / t57;
            let t59 = t29 * t58;
            let t61 = t55 * t59 / f64x8::splat(2.0);
            let t62 = (f64x8::splat(1.35)).simd_le(t61);
            let t63 = (f64x8::splat(1.35)).simd_lt(t61);
            let t64 = ((t63).select(t61, f64x8::splat(1.35)));
            let t65 = t64 * t64;
            let t68 = t65 * t65;
            let t69 = f64x8::splat(1.0) / t68;
            let t71 = t68 * t65;
            let t72 = f64x8::splat(1.0) / t71;
            let t74 = t68 * t68;
            let t75 = f64x8::splat(1.0) / t74;
            let t78 = f64x8::splat(1.0) / t74 / t65;
            let t81 = f64x8::splat(1.0) / t74 / t68;
            let t84 = f64x8::splat(1.0) / t74 / t71;
            let t86 = t74 * t74;
            let t87 = f64x8::splat(1.0) / t86;
            let t90 = ((t63).select(f64x8::splat(1.35), t61));
            let t91 = ((f64x8::splat(M_PI)).sqrt());
            let t92 = f64x8::splat(1.0) / t90;
            let t94 = (simd::erf(t92 / f64x8::splat(2.0)));
            let t96 = t90 * t90;
            let t97 = f64x8::splat(1.0) / t96;
            let t99 = (simd::exp(-t97 / f64x8::splat(4.0)));
            let t100 = t99 - f64x8::splat(1.0);
            let t103 = t99 - f64x8::splat(3.0) / f64x8::splat(2.0) - f64x8::splat(2.0) * t96 * t100;
            let t106 = f64x8::splat(2.0) * t90 * t103 + t91 * t94;
            let t110 = ((t62).select(f64x8::splat(1.0) / t65 / f64x8::splat(36.0) - t69 / f64x8::splat(960.0) + t72 / f64x8::splat(26880.0) - t75 / f64x8::splat(829440.0) + t78 / f64x8::splat(28385280.0) - t81 / f64x8::splat(1073479680.0) + t84 / f64x8::splat(44590694400.0) - t87 / f64x8::splat(2021444812800.0), f64x8::splat(1.0) - f64x8::splat(8.0) / f64x8::splat(3.0) * t90 * t106));
            let t111 = t19 * t110;
            let t115 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t18 * t111 * t49));
            let tzk0 = f64x8::splat(2.0) * t115;
            acc_zk = tzk0;
            let t116 = f64x8::splat(1.0) / t38;
            let t117 = t116 * t110;
            let t121 = t65 * t64;
            let t122 = f64x8::splat(1.0) / t121;
            let t125 = param_hyb_omega_0 / t53 / t52;
            let t127 = t125 * t59 * f64x8::splat(M_PI);
            let t128 = t20 * t24;
            let t129 = t49 * t49;
            let t130 = f64x8::splat(1.0) / t129;
            let t131 = t25 * t130;
            let t132 = t31 * t30;
            let t134 = f64x8::splat(1.0) / t19 / t132;
            let t136 = t29 * t134 * t45;
            let t140 = param_b * t27 * v_sigma;
            let t141 = t31 * t31;
            let t142 = t141 * v_rho;
            let t143 = f64x8::splat(1.0) / t142;
            let t145 = f64x8::splat(1.0) / t44 / t43;
            let t146 = t143 * t145;
            let t149 = -f64x8::splat(384.0) * t28 * t136 + f64x8::splat(4608.0) * t140 * t146;
            let t155 = f64x8::splat(1.0) / t57 / t56;
            let t156 = t29 * t155;
            let t160 = t127 * t128 * t131 * t149 / f64x8::splat(4.0) - t55 * t156 * t11 / f64x8::splat(6.0);
            let t161 = ((t63).select(t160, f64x8::splat(0.0)));
            let t164 = t68 * t64;
            let t165 = f64x8::splat(1.0) / t164;
            let t168 = t68 * t121;
            let t169 = f64x8::splat(1.0) / t168;
            let t173 = f64x8::splat(1.0) / t74 / t64;
            let t177 = f64x8::splat(1.0) / t74 / t121;
            let t181 = f64x8::splat(1.0) / t74 / t164;
            let t185 = f64x8::splat(1.0) / t74 / t168;
            let t189 = f64x8::splat(1.0) / t86 / t64;
            let t193 = ((t63).select(f64x8::splat(0.0), t160));
            let t195 = t99 * t97;
            let t199 = t96 * t90;
            let t200 = f64x8::splat(1.0) / t199;
            let t204 = t90 * t100;
            let t209 = t200 * t193 * t99 / f64x8::splat(2.0) - f64x8::splat(4.0) * t204 * t193 - t92 * t193 * t99;
            let t212 = f64x8::splat(2.0) * t193 * t103 - t195 * t193 + f64x8::splat(2.0) * t90 * t209;
            let t216 = ((t62).select(-t122 * t161 / f64x8::splat(18.0) + t165 * t161 / f64x8::splat(240.0) - t169 * t161 / f64x8::splat(4480.0) + t173 * t161 / f64x8::splat(103680.0) - t177 * t161 / f64x8::splat(2838528.0) + t181 * t161 / f64x8::splat(89456640.0) - t185 * t161 / f64x8::splat(3185049600.0) + t189 * t161 / f64x8::splat(126340300800.0), -f64x8::splat(8.0) / f64x8::splat(3.0) * t193 * t106 - f64x8::splat(8.0) / f64x8::splat(3.0) * t90 * t212));
            let t217 = t19 * t216;
            let t225 = ((t2).select(f64x8::splat(0.0), -t18 * t117 * t49 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t18 * t217 * t49 - f64x8::splat(3.0) / f64x8::splat(8.0) * t18 * t111 * t149));
            let tvrho0 = f64x8::splat(2.0) * v_rho * t225 + f64x8::splat(2.0) * t115;
            acc_vrho = tvrho0;
            let t228 = param_b * v_sigma;
            let t231 = f64x8::splat(1.0) / t141;
            let t232 = t231 * t145;
            let t235 = f64x8::splat(144.0) * t228 * t46 - f64x8::splat(1728.0) * t28 * t232;
            let t239 = t127 * t128 * t131 * t235 / f64x8::splat(4.0);
            let t240 = ((t63).select(t239, f64x8::splat(0.0)));
            let t243 = t165 * t240;
            let t245 = t169 * t240;
            let t247 = t173 * t240;
            let t249 = t177 * t240;
            let t251 = t181 * t240;
            let t253 = t185 * t240;
            let t255 = t189 * t240;
            let t258 = ((t63).select(f64x8::splat(0.0), t239));
            let t270 = t200 * t258 * t99 / f64x8::splat(2.0) - f64x8::splat(4.0) * t204 * t258 - t92 * t258 * t99;
            let t273 = f64x8::splat(2.0) * t258 * t103 - t195 * t258 + f64x8::splat(2.0) * t90 * t270;
            let t277 = ((t62).select(-t122 * t240 / f64x8::splat(18.0) + t243 / f64x8::splat(240.0) - t245 / f64x8::splat(4480.0) + t247 / f64x8::splat(103680.0) - t249 / f64x8::splat(2838528.0) + t251 / f64x8::splat(89456640.0) - t253 / f64x8::splat(3185049600.0) + t255 / f64x8::splat(126340300800.0), -f64x8::splat(8.0) / f64x8::splat(3.0) * t258 * t106 - f64x8::splat(8.0) / f64x8::splat(3.0) * t90 * t273));
            let t278 = t19 * t277;
            let t285 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t18 * t111 * t235 - f64x8::splat(3.0) / f64x8::splat(8.0) * t18 * t278 * t49));
            let tvsigma0 = f64x8::splat(2.0) * v_rho * t285;
            acc_vsigma = tvsigma0;
            let t289 = f64x8::splat(1.0) / t38 / v_rho;
            let t290 = t289 * t110;
            let t294 = t116 * t216;
            let t301 = t161 * t161;
            let t304 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t306 = t23 * t23;
            let t307 = f64x8::splat(1.0) / t306;
            let t308 = t25 * t25;
            let t309 = t307 * t308;
            let t315 = param_hyb_omega_0 / t53 / t304 / t3 / t309 / t130 / f64x8::splat(3.0);
            let t317 = t315 * t59 * t304;
            let t318 = t3 * t307;
            let t319 = t129 * t129;
            let t320 = f64x8::splat(1.0) / t319;
            let t321 = t308 * t320;
            let t322 = t149 * t149;
            let t328 = t125 * t156 * f64x8::splat(M_PI);
            let t329 = t128 * t25;
            let t330 = t130 * t149;
            let t335 = t129 * t49;
            let t336 = f64x8::splat(1.0) / t335;
            let t337 = t25 * t336;
            let t342 = t30 * v_rho;
            let t343 = t31 * t342;
            let t345 = f64x8::splat(1.0) / t19 / t343;
            let t347 = t29 * t345 * t45;
            let t350 = t141 * t30;
            let t351 = f64x8::splat(1.0) / t350;
            let t352 = t351 * t145;
            let t355 = t27 * t27;
            let t356 = param_b * t355;
            let t357 = t141 * t31;
            let t359 = f64x8::splat(1.0) / t38 / t357;
            let t360 = t44 * t44;
            let t361 = f64x8::splat(1.0) / t360;
            let t363 = t359 * t361 * t36;
            let t366 = f64x8::splat(2432.0) * t28 * t347 - f64x8::splat(66048.0) * t140 * t352 + f64x8::splat(221184.0) * t356 * t363;
            let t371 = t11 * t11;
            let t374 = f64x8::splat(1.0) / t57 / t371 / t30;
            let t375 = t29 * t374;
            let t379 = f64x8::splat(9.0) / f64x8::splat(8.0) * t317 * t318 * t321 * t322 - t328 * t329 * t330 * t11 / f64x8::splat(6.0) - t127 * t128 * t337 * t322 / f64x8::splat(2.0) + t127 * t128 * t131 * t366 / f64x8::splat(4.0) + f64x8::splat(2.0) / f64x8::splat(9.0) * t55 * t375 * t371;
            let t380 = ((t63).select(t379, f64x8::splat(0.0)));
            let t408 = f64x8::splat(1.0) / t86 / t65;
            let t413 = t69 * t301 / f64x8::splat(6.0) - t122 * t380 / f64x8::splat(18.0) - t72 * t301 / f64x8::splat(48.0) + t165 * t380 / f64x8::splat(240.0) + t75 * t301 / f64x8::splat(640.0) - t169 * t380 / f64x8::splat(4480.0) - t78 * t301 / f64x8::splat(11520.0) + t173 * t380 / f64x8::splat(103680.0) + t81 * t301 / f64x8::splat(258048.0) - t177 * t380 / f64x8::splat(2838528.0) - t84 * t301 / f64x8::splat(6881280.0) + t181 * t380 / f64x8::splat(89456640.0) + t87 * t301 / f64x8::splat(212336640.0) - t185 * t380 / f64x8::splat(3185049600.0) - t408 * t301 / f64x8::splat(7431782400.0) + t189 * t380 / f64x8::splat(126340300800.0);
            let t414 = ((t63).select(f64x8::splat(0.0), t379));
            let t419 = t96 * t96;
            let t421 = f64x8::splat(1.0) / t419 / t90;
            let t422 = t193 * t193;
            let t423 = t421 * t422;
            let t426 = t99 * t200;
            let t434 = f64x8::splat(1.0) / t419;
            let t442 = f64x8::splat(1.0) / t419 / t96;
            let t443 = t442 * t422;
            let t454 = -f64x8::splat(2.0) * t434 * t422 * t99 + t200 * t414 * t99 / f64x8::splat(2.0) + t443 * t99 / f64x8::splat(4.0) - f64x8::splat(4.0) * t422 * t100 - t97 * t422 * t99 - f64x8::splat(4.0) * t204 * t414 - t92 * t414 * t99;
            let t457 = -t423 * t99 / f64x8::splat(2.0) + f64x8::splat(2.0) * t426 * t422 - t195 * t414 + f64x8::splat(2.0) * t414 * t103 + f64x8::splat(4.0) * t193 * t209 + f64x8::splat(2.0) * t90 * t454;
            let t461 = ((t62).select(t413, -f64x8::splat(8.0) / f64x8::splat(3.0) * t414 * t106 - f64x8::splat(16.0) / f64x8::splat(3.0) * t193 * t212 - f64x8::splat(8.0) / f64x8::splat(3.0) * t90 * t457));
            let t462 = t19 * t461;
            let t473 = ((t2).select(f64x8::splat(0.0), t18 * t290 * t49 / f64x8::splat(12.0) - t18 * t294 * t49 / f64x8::splat(4.0) - t18 * t117 * t149 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t18 * t462 * t49 - f64x8::splat(3.0) / f64x8::splat(4.0) * t18 * t217 * t149 - f64x8::splat(3.0) / f64x8::splat(8.0) * t18 * t111 * t366));
            let tv2rho20 = f64x8::splat(2.0) * v_rho * t473 + f64x8::splat(4.0) * t225;
            acc_v2rho2 = tv2rho20;
            let t476 = t116 * t277;
            let t480 = t69 * t240;
            let t483 = t318 * t308;
            let t484 = t320 * t235;
            let t489 = t130 * t235;
            let t494 = t336 * t235;
            let t503 = t141 * t342;
            let t505 = f64x8::splat(1.0) / t38 / t503;
            let t507 = t505 * t361 * t36;
            let t510 = -f64x8::splat(768.0) * t228 * t136 + f64x8::splat(23040.0) * t28 * t146 - f64x8::splat(82944.0) * t140 * t507;
            let t515 = f64x8::splat(9.0) / f64x8::splat(8.0) * t317 * t483 * t484 * t149 - t328 * t329 * t489 * t11 / f64x8::splat(12.0) - t127 * t329 * t494 * t149 / f64x8::splat(2.0) + t127 * t128 * t131 * t510 / f64x8::splat(4.0);
            let t516 = ((t63).select(t515, f64x8::splat(0.0)));
            let t519 = t72 * t240;
            let t522 = t165 * t516;
            let t524 = t75 * t240;
            let t527 = t169 * t516;
            let t529 = t78 * t240;
            let t532 = t173 * t516;
            let t534 = t81 * t240;
            let t537 = t177 * t516;
            let t539 = t84 * t240;
            let t542 = t181 * t516;
            let t544 = t87 * t240;
            let t547 = t185 * t516;
            let t549 = t408 * t240;
            let t552 = t189 * t516;
            let t554 = t480 * t161 / f64x8::splat(6.0) - t122 * t516 / f64x8::splat(18.0) - t519 * t161 / f64x8::splat(48.0) + t522 / f64x8::splat(240.0) + t524 * t161 / f64x8::splat(640.0) - t527 / f64x8::splat(4480.0) - t529 * t161 / f64x8::splat(11520.0) + t532 / f64x8::splat(103680.0) + t534 * t161 / f64x8::splat(258048.0) - t537 / f64x8::splat(2838528.0) - t539 * t161 / f64x8::splat(6881280.0) + t542 / f64x8::splat(89456640.0) + t544 * t161 / f64x8::splat(212336640.0) - t547 / f64x8::splat(3185049600.0) - t549 * t161 / f64x8::splat(7431782400.0) + t552 / f64x8::splat(126340300800.0);
            let t555 = ((t63).select(f64x8::splat(0.0), t515));
            let t559 = t421 * t193;
            let t560 = t99 * t258;
            let t563 = t258 * t193;
            let t573 = t434 * t258;
            let t574 = t99 * t193;
            let t580 = t442 * t258;
            let t583 = t193 * t100;
            let t586 = t97 * t193;
            let t592 = -f64x8::splat(2.0) * t573 * t574 + t200 * t555 * t99 / f64x8::splat(2.0) + t580 * t574 / f64x8::splat(4.0) - f64x8::splat(4.0) * t583 * t258 - t586 * t560 - f64x8::splat(4.0) * t204 * t555 - t92 * t555 * t99;
            let t595 = -t559 * t560 / f64x8::splat(2.0) + f64x8::splat(2.0) * t426 * t563 - t195 * t555 + f64x8::splat(2.0) * t555 * t103 + f64x8::splat(2.0) * t258 * t209 + f64x8::splat(2.0) * t193 * t270 + f64x8::splat(2.0) * t90 * t592;
            let t599 = ((t62).select(t554, -f64x8::splat(8.0) / f64x8::splat(3.0) * t555 * t106 - f64x8::splat(8.0) / f64x8::splat(3.0) * t193 * t273 - f64x8::splat(8.0) / f64x8::splat(3.0) * t258 * t212 - f64x8::splat(8.0) / f64x8::splat(3.0) * t90 * t595));
            let t600 = t19 * t599;
            let t617 = ((t2).select(f64x8::splat(0.0), -t18 * t476 * t49 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t18 * t600 * t49 - f64x8::splat(3.0) / f64x8::splat(8.0) * t18 * t278 * t149 - t18 * t117 * t235 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t18 * t217 * t235 - f64x8::splat(3.0) / f64x8::splat(8.0) * t18 * t111 * t510));
            let tv2rhosigma0 = f64x8::splat(2.0) * v_rho * t617 + f64x8::splat(2.0) * t285;
            acc_v2rhosigma = tv2rhosigma0;
            let t620 = t240 * t240;
            let t623 = t235 * t235;
            let t632 = param_b * t29;
            let t639 = f64x8::splat(1.0) / t38 / t350;
            let t641 = t639 * t361 * t36;
            let t644 = f64x8::splat(144.0) * t632 * t34 * t45 - f64x8::splat(6912.0) * t228 * t232 + f64x8::splat(31104.0) * t28 * t641;
            let t649 = f64x8::splat(9.0) / f64x8::splat(8.0) * t317 * t318 * t321 * t623 - t127 * t128 * t337 * t623 / f64x8::splat(2.0) + t127 * t128 * t131 * t644 / f64x8::splat(4.0);
            let t650 = ((t63).select(t649, f64x8::splat(0.0)));
            let t653 = t72 * t620;
            let t655 = t165 * t650;
            let t657 = t75 * t620;
            let t659 = t169 * t650;
            let t661 = t78 * t620;
            let t663 = t173 * t650;
            let t665 = t81 * t620;
            let t667 = t177 * t650;
            let t669 = t84 * t620;
            let t671 = t181 * t650;
            let t673 = t87 * t620;
            let t675 = t185 * t650;
            let t677 = t408 * t620;
            let t679 = t189 * t650;
            let t681 = t69 * t620 / f64x8::splat(6.0) - t122 * t650 / f64x8::splat(18.0) - t653 / f64x8::splat(48.0) + t655 / f64x8::splat(240.0) + t657 / f64x8::splat(640.0) - t659 / f64x8::splat(4480.0) - t661 / f64x8::splat(11520.0) + t663 / f64x8::splat(103680.0) + t665 / f64x8::splat(258048.0) - t667 / f64x8::splat(2838528.0) - t669 / f64x8::splat(6881280.0) + t671 / f64x8::splat(89456640.0) + t673 / f64x8::splat(212336640.0) - t675 / f64x8::splat(3185049600.0) - t677 / f64x8::splat(7431782400.0) + t679 / f64x8::splat(126340300800.0);
            let t682 = ((t63).select(f64x8::splat(0.0), t649));
            let t687 = t258 * t258;
            let t688 = t421 * t687;
            let t704 = t442 * t687;
            let t715 = -f64x8::splat(2.0) * t434 * t687 * t99 + t200 * t682 * t99 / f64x8::splat(2.0) + t704 * t99 / f64x8::splat(4.0) - f64x8::splat(4.0) * t687 * t100 - t97 * t687 * t99 - f64x8::splat(4.0) * t204 * t682 - t92 * t682 * t99;
            let t718 = -t688 * t99 / f64x8::splat(2.0) + f64x8::splat(2.0) * t426 * t687 - t195 * t682 + f64x8::splat(2.0) * t682 * t103 + f64x8::splat(4.0) * t258 * t270 + f64x8::splat(2.0) * t90 * t715;
            let t722 = ((t62).select(t681, -f64x8::splat(8.0) / f64x8::splat(3.0) * t682 * t106 - f64x8::splat(16.0) / f64x8::splat(3.0) * t258 * t273 - f64x8::splat(8.0) / f64x8::splat(3.0) * t90 * t718));
            let t723 = t19 * t722;
            let t734 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t18 * t723 * t49 - f64x8::splat(3.0) / f64x8::splat(4.0) * t18 * t278 * t235 - f64x8::splat(3.0) / f64x8::splat(8.0) * t18 * t111 * t644));
            let tv2sigma20 = f64x8::splat(2.0) * v_rho * t734;
            acc_v2sigma2 = tv2sigma20;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_vrho.into(); vrho[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_vsigma.into(); vsigma[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v2rho2.into(); v2rho2[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v2rhosigma.into(); v2rhosigma[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v2sigma2.into(); v2sigma2[ip..ip + m].copy_from_slice(&a[..m]); }
        ip += 8;
    }
}
