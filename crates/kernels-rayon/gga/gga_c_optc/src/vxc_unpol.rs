//! GGA_C_OPTC vxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_optc.c`
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
pub fn gga_c_optc_vxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    param_c1: f64,
    param_c2: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_c1 = f64x8::splat(param_c1);
    let param_c2 = f64x8::splat(param_c2);
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
        {
            let t1 = f64x8::splat(M_CBRT3);
            let t2 = f64x8::splat(1.0) / f64x8::splat(M_PI);
            let t3 = (simd::cbrt(t2));
            let t4 = t1 * t3;
            let t5 = f64x8::splat(M_CBRT4);
            let t6 = t5 * t5;
            let t7 = (simd::cbrt(v_rho));
            let t9 = t6 / t7;
            let t10 = t4 * t9;
            let t12 = f64x8::splat(1.0) + f64x8::splat(0.053425) * t10;
            let t13 = ((t10).sqrt());
            let t16 = ((t10) * (t10).sqrt());
            let t18 = t1 * t1;
            let t19 = t3 * t3;
            let t20 = t18 * t19;
            let t21 = t7 * t7;
            let t23 = t5 / t21;
            let t24 = t20 * t23;
            let t26 = f64x8::splat(3.79785) * t13 + f64x8::splat(0.8969) * t10 + f64x8::splat(0.204775) * t16 + f64x8::splat(0.123235) * t24;
            let t29 = f64x8::splat(1.0) + f64x8::splat(16.081824322151103) / t26;
            let t30 = (simd::ln(t29));
            let t32 = f64x8::splat(0.062182) * t12 * t30;
            let t33 = (f64x8::splat(1.0)).simd_le(zeta_threshold);
            let t34 = (simd::cbrt(zeta_threshold));
            let t35 = t34 * zeta_threshold;
            let t36 = ((t33).select(t35, f64x8::splat(1.0)));
            let t39 = f64x8::splat(M_CBRT2);
            let t42 = f64x8::splat(1.0) / (f64x8::splat(2.0) * t39 - f64x8::splat(2.0));
            let t43 = (f64x8::splat(2.0) * t36 - f64x8::splat(2.0)) * t42;
            let t45 = f64x8::splat(1.0) + f64x8::splat(0.0278125) * t10;
            let t50 = f64x8::splat(5.1785) * t13 + f64x8::splat(0.905775) * t10 + f64x8::splat(0.1100325) * t16 + f64x8::splat(0.1241775) * t24;
            let t53 = f64x8::splat(1.0) + f64x8::splat(29.608574643216677) / t50;
            let t54 = (simd::ln(t53));
            let t57 = f64x8::splat(0.019751789702565206) * t43 * t45 * t54;
            let t58 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t59 = (simd::cbrt(t58));
            let t60 = t59 * t59;
            let t61 = t18 * t60;
            let t62 = t34 * t34;
            let t63 = ((t33).select(t62, f64x8::splat(1.0)));
            let t64 = t63 * t63;
            let t65 = t64 * t63;
            let t66 = f64x8::splat(1.0) / t59;
            let t67 = t18 * t66;
            let t68 = v_rho * v_rho;
            let t70 = f64x8::splat(1.0) / t7 / t68;
            let t71 = v_sigma * t70;
            let t72 = t71 * t39;
            let t73 = f64x8::splat(1.0) / t64;
            let t75 = f64x8::splat(1.0) / t3;
            let t76 = t75 * t5;
            let t77 = t73 * t18 * t76;
            let t83 = f64x8::splat(1.0) / t60;
            let t84 = t1 * t83;
            let t87 = (simd::exp(-f64x8::splat(128.97460341341235) * (-t32 + t57) / t65 * t84));
            let t88 = t87 - f64x8::splat(1.0);
            let t89 = f64x8::splat(1.0) / t88;
            let t90 = t66 * t89;
            let t91 = v_sigma * v_sigma;
            let t92 = t68 * t68;
            let t94 = f64x8::splat(1.0) / t21 / t92;
            let t95 = t91 * t94;
            let t97 = t39 * t39;
            let t98 = t64 * t64;
            let t99 = f64x8::splat(1.0) / t98;
            let t100 = t97 * t99;
            let t101 = f64x8::splat(1.0) / t19;
            let t102 = t101 * t6;
            let t103 = t100 * t102;
            let t106 = t72 * t77 / f64x8::splat(96.0) + f64x8::splat(0.0027166129655589867) * t90 * t95 * t103;
            let t107 = t1 * t66;
            let t109 = t107 * t89 * v_sigma;
            let t110 = t70 * t39;
            let t112 = t73 * t75 * t5;
            let t116 = t18 * t83;
            let t117 = t88 * t88;
            let t118 = f64x8::splat(1.0) / t117;
            let t119 = t118 * t91;
            let t120 = t116 * t119;
            let t121 = t94 * t97;
            let t122 = t99 * t101;
            let t123 = t122 * t6;
            let t124 = t121 * t123;
            let t127 = f64x8::splat(1.0) + f64x8::splat(0.08693161489788757) * t109 * t110 * t112 + f64x8::splat(0.0075571056687546295) * t120 * t124;
            let t128 = f64x8::splat(1.0) / t127;
            let t132 = f64x8::splat(1.0) + f64x8::splat(2.7818116767324024) * t67 * t106 * t128;
            let t133 = (simd::ln(t132));
            let t137 = t2 * t59;
            let t140 = f64x8::splat(2.568) + f64x8::splat(5.8165) * t10 + f64x8::splat(0.00184725) * t24;
            let t143 = f64x8::splat(1000.0) + f64x8::splat(2180.75) * t10 + f64x8::splat(118.0) * t24;
            let t144 = f64x8::splat(1.0) / t143;
            let t146 = t140 * t144 - f64x8::splat(0.0018535714285714286);
            let t147 = t146 * t63;
            let t149 = t137 * t147 * v_sigma;
            let t150 = t2 * t5;
            let t151 = (simd::cbrt(f64x8::splat(9.0)));
            let t152 = t151 * t151;
            let t156 = f64x8::splat(1.0) / t21 / t68;
            let t158 = v_sigma * t39;
            let t162 = (simd::exp(-f64x8::splat(25.0) / f64x8::splat(18.0) * t150 * t152 * t3 * t156 * t64 * t158));
            let t163 = t76 * t162;
            let t164 = t110 * t163;
            let t168 = param_c1 * (-t32 + t57 + f64x8::splat(0.002584488143490343) * t61 * t65 * t133 + t149 * t164 / f64x8::splat(2.0));
            let t169 = param_c2 - param_c1;
            let t171 = t4 * t9 * t39;
            let t173 = f64x8::splat(1.0) + f64x8::splat(0.053425) * t171;
            let t174 = ((t171).sqrt());
            let t177 = ((t171) * (t171).sqrt());
            let t180 = t20 * t23 * t97;
            let t182 = f64x8::splat(3.79785) * t174 + f64x8::splat(0.8969) * t171 + f64x8::splat(0.204775) * t177 + f64x8::splat(0.123235) * t180;
            let t185 = f64x8::splat(1.0) + f64x8::splat(16.081824322151103) / t182;
            let t186 = (simd::ln(t185));
            let t188 = f64x8::splat(0.062182) * t173 * t186;
            let t189 = (f64x8::splat(2.0)).simd_le(zeta_threshold);
            let t191 = ((t189).select(t35, f64x8::splat(2.0) * t39));
            let t192 = (f64x8::splat(0.0)).simd_le(zeta_threshold);
            let t193 = ((t192).select(t35, f64x8::splat(0.0)));
            let t195 = (t191 + t193 - f64x8::splat(2.0)) * t42;
            let t197 = f64x8::splat(1.0) + f64x8::splat(0.05137) * t171;
            let t202 = f64x8::splat(7.05945) * t174 + f64x8::splat(1.549425) * t171 + f64x8::splat(0.420775) * t177 + f64x8::splat(0.1562925) * t180;
            let t205 = f64x8::splat(1.0) + f64x8::splat(32.1646831778707) / t202;
            let t206 = (simd::ln(t205));
            let t210 = f64x8::splat(1.0) + f64x8::splat(0.0278125) * t171;
            let t215 = f64x8::splat(5.1785) * t174 + f64x8::splat(0.905775) * t171 + f64x8::splat(0.1100325) * t177 + f64x8::splat(0.1241775) * t180;
            let t218 = f64x8::splat(1.0) + f64x8::splat(29.608574643216677) / t215;
            let t219 = (simd::ln(t218));
            let t220 = t210 * t219;
            let t223 = t195 * (-f64x8::splat(0.03109) * t197 * t206 + t188 - f64x8::splat(0.019751789702565206) * t220);
            let t225 = f64x8::splat(0.019751789702565206) * t195 * t220;
            let t226 = ((t189).select(t62, t97));
            let t227 = ((t192).select(t62, f64x8::splat(0.0)));
            let t229 = t226 / f64x8::splat(2.0) + t227 / f64x8::splat(2.0);
            let t230 = t229 * t229;
            let t231 = t230 * t229;
            let t232 = f64x8::splat(1.0) / t230;
            let t234 = t18 * t75;
            let t235 = t5 * t97;
            let t236 = t234 * t235;
            let t244 = (simd::exp(-f64x8::splat(128.97460341341235) * (-t188 + t223 + t225) / t231 * t84));
            let t245 = t244 - f64x8::splat(1.0);
            let t246 = f64x8::splat(1.0) / t245;
            let t247 = t66 * t246;
            let t249 = t230 * t230;
            let t250 = f64x8::splat(1.0) / t249;
            let t251 = t250 * t101;
            let t252 = t6 * t39;
            let t253 = t251 * t252;
            let t256 = t71 * t232 * t236 / f64x8::splat(96.0) + f64x8::splat(0.005433225931117973) * t247 * t95 * t253;
            let t258 = t107 * t246 * v_sigma;
            let t259 = t70 * t232;
            let t260 = t76 * t97;
            let t264 = t245 * t245;
            let t265 = f64x8::splat(1.0) / t264;
            let t266 = t265 * t91;
            let t267 = t116 * t266;
            let t268 = t94 * t250;
            let t269 = t102 * t39;
            let t270 = t268 * t269;
            let t273 = f64x8::splat(1.0) + f64x8::splat(0.08693161489788757) * t258 * t259 * t260 + f64x8::splat(0.015114211337509259) * t267 * t270;
            let t274 = f64x8::splat(1.0) / t273;
            let t278 = f64x8::splat(1.0) + f64x8::splat(2.7818116767324024) * t67 * t256 * t274;
            let t279 = (simd::ln(t278));
            let t285 = f64x8::splat(2.568) + f64x8::splat(5.8165) * t171 + f64x8::splat(0.00184725) * t180;
            let t288 = f64x8::splat(1000.0) + f64x8::splat(2180.75) * t171 + f64x8::splat(118.0) * t180;
            let t289 = f64x8::splat(1.0) / t288;
            let t291 = t285 * t289 - f64x8::splat(0.0018535714285714286);
            let t292 = t291 * t229;
            let t294 = t137 * t292 * v_sigma;
            let t302 = (simd::exp(-f64x8::splat(25.0) / f64x8::splat(9.0) * t150 * t152 * t3 * t156 * t230 * v_sigma));
            let t303 = t235 * t302;
            let t304 = t70 * t75 * t303;
            let t309 = ((t33).select(zeta_threshold, f64x8::splat(1.0)));
            let t310 = t169 * (-t188 + t223 + t225 + f64x8::splat(0.002584488143490343) * t61 * t231 * t279 + t294 * t304 / f64x8::splat(2.0)) * t309;
            let tzk0 = t168 + t310;
            acc_zk = tzk0;
            let t312 = f64x8::splat(1.0) / t7 / v_rho;
            let t313 = t6 * t312;
            let t316 = f64x8::splat(0.0011073577833333333) * t4 * t313 * t30;
            let t317 = t26 * t26;
            let t318 = f64x8::splat(1.0) / t317;
            let t319 = t12 * t318;
            let t321 = f64x8::splat(1.0) / t13 * t1;
            let t322 = t3 * t6;
            let t323 = t322 * t312;
            let t324 = t321 * t323;
            let t326 = t4 * t313;
            let t328 = ((t10).sqrt());
            let t329 = t328 * t1;
            let t330 = t329 * t323;
            let t334 = t5 / t21 / v_rho;
            let t335 = t20 * t334;
            let t337 = -f64x8::splat(0.632975) * t324 - f64x8::splat(0.29896666666666666) * t326 - f64x8::splat(0.1023875) * t330 - f64x8::splat(0.08215666666666667) * t335;
            let t338 = f64x8::splat(1.0) / t29;
            let t339 = t337 * t338;
            let t341 = f64x8::splat(1.0) * t319 * t339;
            let t342 = t43 * t1;
            let t346 = f64x8::splat(0.0001831155503675316) * t342 * t322 * t312 * t54;
            let t347 = t43 * t45;
            let t348 = t50 * t50;
            let t349 = f64x8::splat(1.0) / t348;
            let t354 = -f64x8::splat(0.8630833333333333) * t324 - f64x8::splat(0.301925) * t326 - f64x8::splat(0.05501625) * t330 - f64x8::splat(0.082785) * t335;
            let t356 = f64x8::splat(1.0) / t53;
            let t357 = t349 * t354 * t356;
            let t359 = f64x8::splat(0.5848223397455204) * t347 * t357;
            let t360 = t68 * v_rho;
            let t362 = f64x8::splat(1.0) / t7 / t360;
            let t363 = v_sigma * t362;
            let t364 = t363 * t39;
            let t367 = t98 * t65;
            let t368 = f64x8::splat(1.0) / t367;
            let t369 = t121 * t368;
            let t370 = t119 * t369;
            let t371 = t316 + t341 - t346 - t359;
            let t373 = t371 * t1 * t87;
            let t374 = t102 * t373;
            let t377 = t92 * v_rho;
            let t379 = f64x8::splat(1.0) / t21 / t377;
            let t380 = t91 * t379;
            let t384 = -f64x8::splat(7.0) / f64x8::splat(288.0) * t364 * t77 + f64x8::splat(0.03550031648908154) * t370 * t374 - f64x8::splat(0.012677527172608605) * t90 * t380 * t103;
            let t388 = t127 * t127;
            let t389 = f64x8::splat(1.0) / t388;
            let t390 = t106 * t389;
            let t391 = t18 * t118;
            let t392 = t391 * t72;
            let t393 = t98 * t63;
            let t395 = f64x8::splat(1.0) / t393 * t75;
            let t396 = t5 * t371;
            let t398 = t395 * t396 * t87;
            let t401 = t362 * t39;
            let t406 = f64x8::splat(1.0) / t59 / t58;
            let t408 = f64x8::splat(1.0) / t117 / t88;
            let t409 = t406 * t408;
            let t411 = t409 * t95 * t97;
            let t412 = t368 * t101;
            let t413 = t6 * t371;
            let t415 = t412 * t413 * t87;
            let t418 = t379 * t97;
            let t419 = t418 * t123;
            let t422 = f64x8::splat(1.1360101276506094) * t392 * t398 - f64x8::splat(0.2028404347617377) * t109 * t401 * t112 + f64x8::splat(5.848048239485272) * t411 * t415 - f64x8::splat(0.03526649312085494) * t120 * t419;
            let t426 = f64x8::splat(2.7818116767324024) * t67 * t384 * t128 - f64x8::splat(2.7818116767324024) * t67 * t390 * t422;
            let t428 = f64x8::splat(1.0) / t132;
            let t434 = -f64x8::splat(1.9388333333333334) * t326 - f64x8::splat(0.0012315) * t335;
            let t436 = t143 * t143;
            let t437 = f64x8::splat(1.0) / t436;
            let t438 = t140 * t437;
            let t441 = -f64x8::splat(726.9166666666666) * t326 - f64x8::splat(78.66666666666667) * t335;
            let t443 = t434 * t144 - t438 * t441;
            let t444 = t443 * t63;
            let t446 = t137 * t444 * v_sigma;
            let t449 = t401 * t163;
            let t453 = f64x8::splat(1.0) / t58 * t59;
            let t454 = t146 * t65;
            let t456 = t453 * t454 * t91;
            let t457 = t92 * t68;
            let t458 = f64x8::splat(1.0) / t457;
            let t460 = t6 * t152;
            let t461 = t460 * t162;
            let t462 = t458 * t97 * t461;
            let t466 = param_c1 * (t316 + t341 - t346 - t359 + f64x8::splat(0.002584488143490343) * t61 * t65 * t426 * t428 + t446 * t164 / f64x8::splat(2.0) - f64x8::splat(7.0) / f64x8::splat(6.0) * t149 * t449 + f64x8::splat(50.0) / f64x8::splat(27.0) * t456 * t462);
            let t467 = t4 * t6;
            let t468 = t312 * t39;
            let t471 = f64x8::splat(0.0011073577833333333) * t467 * t468 * t186;
            let t472 = t182 * t182;
            let t473 = f64x8::splat(1.0) / t472;
            let t474 = t173 * t473;
            let t477 = f64x8::splat(1.0) / t174 * t1 * t3;
            let t478 = t313 * t39;
            let t479 = t477 * t478;
            let t481 = t4 * t478;
            let t483 = ((t171).sqrt());
            let t485 = t483 * t1 * t3;
            let t486 = t485 * t478;
            let t489 = t20 * t334 * t97;
            let t491 = -f64x8::splat(0.632975) * t479 - f64x8::splat(0.29896666666666666) * t481 - f64x8::splat(0.1023875) * t486 - f64x8::splat(0.08215666666666667) * t489;
            let t492 = f64x8::splat(1.0) / t185;
            let t493 = t491 * t492;
            let t495 = f64x8::splat(1.0) * t474 * t493;
            let t499 = t202 * t202;
            let t500 = f64x8::splat(1.0) / t499;
            let t501 = t197 * t500;
            let t506 = -f64x8::splat(1.176575) * t479 - f64x8::splat(0.516475) * t481 - f64x8::splat(0.2103875) * t486 - f64x8::splat(0.104195) * t489;
            let t507 = f64x8::splat(1.0) / t205;
            let t508 = t506 * t507;
            let t514 = t215 * t215;
            let t515 = f64x8::splat(1.0) / t514;
            let t516 = t210 * t515;
            let t521 = -f64x8::splat(0.8630833333333333) * t479 - f64x8::splat(0.301925) * t481 - f64x8::splat(0.05501625) * t486 - f64x8::splat(0.082785) * t489;
            let t522 = f64x8::splat(1.0) / t218;
            let t523 = t521 * t522;
            let t527 = t195 * (f64x8::splat(0.0005323644333333333) * t467 * t468 * t206 + f64x8::splat(1.0) * t501 * t508 - t471 - t495 + f64x8::splat(0.0001831155503675316) * t467 * t468 * t219 + f64x8::splat(0.5848223397455204) * t516 * t523);
            let t528 = t195 * t4;
            let t529 = t39 * t219;
            let t532 = f64x8::splat(0.0001831155503675316) * t528 * t313 * t529;
            let t533 = t195 * t210;
            let t535 = t515 * t521 * t522;
            let t537 = f64x8::splat(0.5848223397455204) * t533 * t535;
            let t541 = t249 * t231;
            let t542 = f64x8::splat(1.0) / t541;
            let t544 = t94 * t542 * t101;
            let t545 = t266 * t544;
            let t546 = t471 + t495 + t527 - t532 - t537;
            let t548 = t546 * t1 * t244;
            let t549 = t252 * t548;
            let t555 = -f64x8::splat(7.0) / f64x8::splat(288.0) * t363 * t232 * t236 + f64x8::splat(0.07100063297816309) * t545 * t549 - f64x8::splat(0.02535505434521721) * t247 * t380 * t253;
            let t559 = t273 * t273;
            let t560 = f64x8::splat(1.0) / t559;
            let t561 = t256 * t560;
            let t562 = t18 * t265;
            let t563 = t249 * t229;
            let t564 = f64x8::splat(1.0) / t563;
            let t566 = t562 * t71 * t564;
            let t567 = t97 * t546;
            let t569 = t76 * t567 * t244;
            let t572 = t362 * t232;
            let t577 = f64x8::splat(1.0) / t264 / t245;
            let t578 = t406 * t577;
            let t580 = t578 * t95 * t542;
            let t581 = t39 * t546;
            let t583 = t102 * t581 * t244;
            let t587 = t379 * t250 * t269;
            let t590 = f64x8::splat(1.1360101276506094) * t566 * t569 - f64x8::splat(0.2028404347617377) * t258 * t572 * t260 + f64x8::splat(11.696096478970544) * t580 * t583 - f64x8::splat(0.07053298624170988) * t267 * t587;
            let t594 = f64x8::splat(2.7818116767324024) * t67 * t555 * t274 - f64x8::splat(2.7818116767324024) * t67 * t561 * t590;
            let t596 = f64x8::splat(1.0) / t278;
            let t602 = -f64x8::splat(1.9388333333333334) * t481 - f64x8::splat(0.0012315) * t489;
            let t604 = t288 * t288;
            let t605 = f64x8::splat(1.0) / t604;
            let t606 = t285 * t605;
            let t609 = -f64x8::splat(726.9166666666666) * t481 - f64x8::splat(78.66666666666667) * t489;
            let t611 = t602 * t289 - t606 * t609;
            let t612 = t611 * t229;
            let t614 = t137 * t612 * v_sigma;
            let t618 = t362 * t75 * t303;
            let t621 = t291 * t231;
            let t623 = t453 * t621 * t91;
            let t625 = t97 * t152;
            let t626 = t625 * t302;
            let t627 = t458 * t6 * t626;
            let t632 = t169 * (t471 + t495 + t527 - t532 - t537 + f64x8::splat(0.002584488143490343) * t61 * t231 * t594 * t596 + t614 * t304 / f64x8::splat(2.0) - f64x8::splat(7.0) / f64x8::splat(6.0) * t294 * t618 + f64x8::splat(100.0) / f64x8::splat(27.0) * t623 * t627) * t309;
            let tvrho0 = t168 + t310 + v_rho * (t466 + t632);
            acc_vrho = tvrho0;
            let t636 = t234 * t5;
            let t639 = v_sigma * t94;
            let t643 = t110 * t73 * t636 / f64x8::splat(96.0) + f64x8::splat(0.005433225931117973) * t90 * t639 * t103;
            let t650 = t39 * t73 * t76;
            let t653 = t118 * v_sigma;
            let t654 = t116 * t653;
            let t657 = f64x8::splat(0.08693161489788757) * t107 * t89 * t70 * t650 + f64x8::splat(0.015114211337509259) * t654 * t124;
            let t661 = f64x8::splat(2.7818116767324024) * t67 * t643 * t128 - f64x8::splat(2.7818116767324024) * t67 * t390 * t657;
            let t666 = t137 * t147;
            let t671 = f64x8::splat(1.0) / t377;
            let t673 = t671 * t97 * t461;
            let t677 = param_c1 * (f64x8::splat(0.002584488143490343) * t61 * t65 * t661 * t428 + t666 * t164 / f64x8::splat(2.0) - f64x8::splat(25.0) / f64x8::splat(36.0) * t453 * t454 * v_sigma * t673);
            let t684 = t259 * t18 * t260 / f64x8::splat(96.0) + f64x8::splat(0.010866451862235947) * t247 * t639 * t253;
            let t691 = t232 * t75 * t235;
            let t694 = t265 * v_sigma;
            let t695 = t116 * t694;
            let t698 = f64x8::splat(0.08693161489788757) * t107 * t246 * t70 * t691 + f64x8::splat(0.030228422675018518) * t695 * t270;
            let t702 = f64x8::splat(2.7818116767324024) * t67 * t684 * t274 - f64x8::splat(2.7818116767324024) * t67 * t561 * t698;
            let t707 = t137 * t292;
            let t713 = t671 * t6 * t626;
            let t718 = t169 * (f64x8::splat(0.002584488143490343) * t61 * t231 * t702 * t596 + t707 * t304 / f64x8::splat(2.0) - f64x8::splat(25.0) / f64x8::splat(18.0) * t453 * t621 * v_sigma * t713) * t309;
            let tvsigma0 = v_rho * (t677 + t718);
            acc_vsigma = tvsigma0;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_vrho.into(); vrho[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_vsigma.into(); vsigma[ip..ip + m].copy_from_slice(&a[..m]); }
        ip += 8;
    }
}
