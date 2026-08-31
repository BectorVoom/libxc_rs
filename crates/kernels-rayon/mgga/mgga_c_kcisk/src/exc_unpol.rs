//! MGGA_C_KCISK exc unpol kernel — explicit SIMD (exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_c_kcisk.c`
//! by tools/translate_rayon/from_maple.py, then rewritten to
//! `wide::f64x8` by simd.py (exact math). Eight grid points per step; every lane runs maple2c's expression
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
pub fn mgga_c_kcisk_exc_unpol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
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
        {
            let t2 = f64x8::splat(M_CBRT3);
            let t3 = f64x8::splat(1.0) / f64x8::splat(M_PI);
            let t4 = (simd::cbrt(t3));
            let t5 = t2 * t4;
            let t6 = f64x8::splat(M_CBRT4);
            let t7 = t6 * t6;
            let t8 = (simd::cbrt(v_rho));
            let t9 = f64x8::splat(1.0) / t8;
            let t10 = t7 * t9;
            let t11 = t5 * t10;
            let t13 = f64x8::splat(1.0) + f64x8::splat(0.053425) * t11;
            let t14 = ((t11).sqrt());
            let t17 = ((t11) * (t11).sqrt());
            let t19 = t2 * t2;
            let t20 = t4 * t4;
            let t21 = t19 * t20;
            let t22 = t8 * t8;
            let t23 = f64x8::splat(1.0) / t22;
            let t25 = t21 * t6 * t23;
            let t27 = f64x8::splat(3.79785) * t14 + f64x8::splat(0.8969) * t11 + f64x8::splat(0.204775) * t17 + f64x8::splat(0.123235) * t25;
            let t30 = f64x8::splat(1.0) + f64x8::splat(16.081824322151103) / t27;
            let t31 = (simd::ln(t30));
            let t33 = f64x8::splat(0.062182) * t13 * t31;
            let t34 = (f64x8::splat(1.0)).simd_le(zeta_threshold);
            let t35 = (simd::cbrt(zeta_threshold));
            let t36 = t35 * zeta_threshold;
            let t37 = ((t34).select(t36, f64x8::splat(1.0)));
            let t40 = f64x8::splat(M_CBRT2);
            let t43 = f64x8::splat(1.0) / (f64x8::splat(2.0) * t40 - f64x8::splat(2.0));
            let t44 = (f64x8::splat(2.0) * t37 - f64x8::splat(2.0)) * t43;
            let t46 = f64x8::splat(1.0) + f64x8::splat(0.0278125) * t11;
            let t51 = f64x8::splat(5.1785) * t14 + f64x8::splat(0.905775) * t11 + f64x8::splat(0.1100325) * t17 + f64x8::splat(0.1241775) * t25;
            let t54 = f64x8::splat(1.0) + f64x8::splat(29.608574643216677) / t51;
            let t55 = (simd::ln(t54));
            let t56 = t46 * t55;
            let t59 = -t33 + f64x8::splat(0.019751789702565206) * t44 * t56;
            let t61 = f64x8::splat(M_CBRT6);
            let t62 = t61 * t61;
            let t63 = t40 * t3 * t62;
            let t64 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t65 = (simd::cbrt(t64));
            let t66 = f64x8::splat(1.0) / t65;
            let t67 = t40 * t2;
            let t68 = t4 * t7;
            let t69 = t68 * t9;
            let t70 = t67 * t69;
            let t72 = f64x8::splat(1.0) + f64x8::splat(0.053425) * t70;
            let t73 = ((t70).sqrt());
            let t76 = ((t70) * (t70).sqrt());
            let t78 = t40 * t40;
            let t79 = t78 * t19;
            let t80 = t20 * t6;
            let t81 = t80 * t23;
            let t82 = t79 * t81;
            let t84 = f64x8::splat(3.79785) * t73 + f64x8::splat(0.8969) * t70 + f64x8::splat(0.204775) * t76 + f64x8::splat(0.123235) * t82;
            let t87 = f64x8::splat(1.0) + f64x8::splat(16.081824322151103) / t84;
            let t88 = (simd::ln(t87));
            let t90 = f64x8::splat(0.062182) * t72 * t88;
            let t92 = f64x8::splat(1.0) + f64x8::splat(0.0278125) * t70;
            let t97 = f64x8::splat(5.1785) * t73 + f64x8::splat(0.905775) * t70 + f64x8::splat(0.1100325) * t76 + f64x8::splat(0.1241775) * t82;
            let t100 = f64x8::splat(1.0) + f64x8::splat(29.608574643216677) / t97;
            let t101 = (simd::ln(t100));
            let t102 = t92 * t101;
            let t105 = -t90 + f64x8::splat(0.019751789702565206) * t44 * t102;
            let t106 = t66 * t105;
            let t109 = f64x8::splat(10.0) / f64x8::splat(9.0) * t63 * t106 * t9;
            let t110 = (t109).simd_lt(-f64x8::splat(0.066725));
            let t112 = ((t110).select(f64x8::splat(0.0), f64x8::splat(0.066725) + t109));
            let t113 = t112 * t40;
            let t114 = v_rho * v_rho;
            let t116 = f64x8::splat(1.0) / t8 / t114;
            let t117 = v_sigma * t116;
            let t118 = t113 * t117;
            let t119 = f64x8::splat(1.0) / t4;
            let t120 = t19 * t119;
            let t121 = (f64x8::splat(0.0)).simd_lt(t59);
            let t123 = ((t121).select(t59, -t59));
            let t124 = f64x8::splat(1.0) / t123;
            let t125 = t6 * t124;
            let t126 = t120 * t125;
            let t129 = f64x8::splat(1.0) + f64x8::splat(0.05397236614853195) * t118 * t126;
            let t130 = (simd::ln(t129));
            let t132 = f64x8::splat(1.0) + f64x8::splat(0.193) * t130;
            let t133 = f64x8::splat(1.0) / t132;
            let t135 = f64x8::splat(1.0) / t20;
            let t136 = t2 * t135;
            let t137 = t136 * t7;
            let t139 = f64x8::splat(1.0) / t8 / v_rho;
            let t140 = f64x8::splat(1.0) / v_rho;
            let t143 = f64x8::splat(1.07924) + f64x8::splat(0.03964) * t14 + f64x8::splat(0.0123825) * t11;
            let t146 = f64x8::splat(1.0) + t14 * t143 / f64x8::splat(2.0);
            let t147 = t146 * t146;
            let t148 = f64x8::splat(1.0) / t147;
            let t152 = t4 * t3;
            let t153 = t2 * t152;
            let t154 = t7 * t139;
            let t157 = t20 * t3;
            let t158 = t19 * t157;
            let t160 = f64x8::splat(1.0) / t22 / v_rho;
            let t161 = t6 * t160;
            let t164 = f64x8::splat(1.0) / t114;
            let t167 = t4 / t64;
            let t168 = t2 * t167;
            let t169 = t7 * t116;
            let t172 = -f64x8::splat(0.005977859662531589) * t140 + f64x8::splat(0.001317375) * t153 * t154 - f64x8::splat(0.00023775) * t158 * t161 + f64x8::splat(6.474423634745383e-06) * t164 - f64x8::splat(5.40140625e-07) * t168 * t169;
            let t174 = f64x8::splat(0.0011713266981940448) * t140 * t148 - t59 * t172;
            let t175 = t139 * t174;
            let t176 = (simd::pow(f64x8::splat(4.0), f64x8::splat(1.0) / f64x8::splat(6.0)));
            let t177 = t136 * t176;
            let t178 = t14 * t11;
            let t179 = t22 * t178;
            let t180 = f64x8::splat(1.0) / t146;
            let t184 = t59 * t59;
            let t186 = f64x8::splat(0.0019711289) * t177 * t179 * t180 - f64x8::splat(2.0) * t184;
            let t187 = f64x8::splat(1.0) / t186;
            let t188 = t187 * v_sigma;
            let t190 = t137 * t175 * t188;
            let t192 = t59 * t133 + f64x8::splat(0.009949166666666667) * t190;
            let t193 = ((f64x8::splat(4.0)).sqrt());
            let t194 = t59 * t193;
            let t195 = t178 * t180;
            let t198 = t7 * t22;
            let t202 = f64x8::splat(0.00619125) * t194 * t195 - f64x8::splat(0.07959333333333334) * t136 * t198 * t172;
            let t203 = t202 * t187;
            let t204 = v_sigma * t164;
            let t205 = t203 * t204;
            let t207 = t174 * t187;
            let t208 = v_sigma * v_sigma;
            let t209 = t114 * t114;
            let t210 = f64x8::splat(1.0) / t209;
            let t211 = t208 * t210;
            let t212 = t207 * t211;
            let t214 = f64x8::splat(1.0) + t205 / f64x8::splat(8.0) - t212 / f64x8::splat(64.0);
            let t215 = f64x8::splat(1.0) / t214;
            let t216 = t192 * t215;
            let t219 = (((f64x8::splat(2.0)).simd_le(zeta_threshold)).select(t36, f64x8::splat(2.0) * t40));
            let t221 = (((f64x8::splat(0.0)).simd_le(zeta_threshold)).select(t36, f64x8::splat(0.0)));
            let t223 = (t219 + t221 - f64x8::splat(2.0)) * t43;
            let t225 = f64x8::splat(1.0) + f64x8::splat(0.05137) * t11;
            let t230 = f64x8::splat(7.05945) * t14 + f64x8::splat(1.549425) * t11 + f64x8::splat(0.420775) * t17 + f64x8::splat(0.1562925) * t25;
            let t233 = f64x8::splat(1.0) + f64x8::splat(32.1646831778707) / t230;
            let t234 = (simd::ln(t233));
            let t242 = -t33 + t223 * (-f64x8::splat(0.03109) * t225 * t234 + t33 - f64x8::splat(0.019751789702565206) * t56) + f64x8::splat(0.019751789702565206) * t223 * t56;
            let t243 = t3 * t62;
            let t244 = t66 * t59;
            let t247 = f64x8::splat(10.0) / f64x8::splat(9.0) * t243 * t244 * t9;
            let t248 = (t247).simd_lt(-f64x8::splat(0.066725));
            let t250 = ((t248).select(f64x8::splat(0.0), f64x8::splat(0.066725) + t247));
            let t251 = t250 * v_sigma;
            let t253 = (f64x8::splat(0.0)).simd_lt(t242);
            let t255 = ((t253).select(t242, -t242));
            let t256 = f64x8::splat(1.0) / t255;
            let t257 = t6 * t256;
            let t258 = t120 * t257;
            let t261 = f64x8::splat(1.0) + f64x8::splat(0.05397236614853195) * t251 * t116 * t258;
            let t262 = (simd::ln(t261));
            let t264 = f64x8::splat(1.0) + f64x8::splat(0.193) * t262;
            let t265 = f64x8::splat(1.0) / t264;
            let t268 = t242 * t265 + f64x8::splat(0.0069644166666666665) * t190;
            let t271 = f64x8::splat(1.0) + f64x8::splat(0.1875) * t205 - f64x8::splat(0.04046875) * t212;
            let t272 = f64x8::splat(1.0) / t271;
            let t275 = t44 * (t268 * t272 - t216);
            let t276 = v_sigma * t140;
            let t277 = f64x8::splat(1.0) / v_tau;
            let t278 = ((t34).select(zeta_threshold, f64x8::splat(1.0)));
            let t279 = t277 * t278;
            let t281 = t78 * t3 * t62;
            let t282 = t78 * t2;
            let t283 = t282 * t69;
            let t285 = f64x8::splat(1.0) + f64x8::splat(0.053425) * t283;
            let t286 = ((t283).sqrt());
            let t289 = ((t283) * (t283).sqrt());
            let t291 = t40 * t19;
            let t292 = t291 * t81;
            let t294 = f64x8::splat(3.79785) * t286 + f64x8::splat(0.8969) * t283 + f64x8::splat(0.204775) * t289 + f64x8::splat(0.24647) * t292;
            let t297 = f64x8::splat(1.0) + f64x8::splat(16.081824322151103) / t294;
            let t298 = (simd::ln(t297));
            let t302 = f64x8::splat(1.0) + f64x8::splat(0.0278125) * t283;
            let t307 = f64x8::splat(5.1785) * t286 + f64x8::splat(0.905775) * t283 + f64x8::splat(0.1100325) * t289 + f64x8::splat(0.248355) * t292;
            let t310 = f64x8::splat(1.0) + f64x8::splat(29.608574643216677) / t307;
            let t311 = (simd::ln(t310));
            let t316 = t66 * (-f64x8::splat(0.062182) * t285 * t298 + f64x8::splat(0.019751789702565206) * t44 * t302 * t311);
            let t319 = f64x8::splat(10.0) / f64x8::splat(9.0) * t281 * t316 * t9;
            let t320 = (t319).simd_lt(-f64x8::splat(0.066725));
            let t322 = ((t320).select(f64x8::splat(0.0), f64x8::splat(0.066725) + t319));
            let t323 = t322 * v_sigma;
            let t324 = t116 * t78;
            let t325 = t323 * t324;
            let t326 = (f64x8::splat(0.0)).simd_lt(t105);
            let t328 = ((t326).select(t105, -t105));
            let t329 = f64x8::splat(1.0) / t328;
            let t330 = t6 * t329;
            let t331 = t120 * t330;
            let t334 = f64x8::splat(1.0) + f64x8::splat(0.05397236614853195) * t325 * t331;
            let t335 = (simd::ln(t334));
            let t337 = f64x8::splat(1.0) + f64x8::splat(0.193) * t335;
            let t338 = f64x8::splat(1.0) / t337;
            let t340 = t135 * t7;
            let t341 = t67 * t340;
            let t344 = f64x8::splat(1.07924) + f64x8::splat(0.03964) * t73 + f64x8::splat(0.0123825) * t70;
            let t347 = f64x8::splat(1.0) + t73 * t344 / f64x8::splat(2.0);
            let t348 = t347 * t347;
            let t349 = f64x8::splat(1.0) / t348;
            let t353 = t152 * t7;
            let t357 = t157 * t6;
            let t362 = t167 * t7;
            let t366 = -f64x8::splat(0.011955719325063178) * t140 + f64x8::splat(0.00263475) * t67 * t353 * t139 - f64x8::splat(0.0004755) * t79 * t357 * t160 + f64x8::splat(2.5897694538981533e-05) * t164 - f64x8::splat(2.1605625e-06) * t67 * t362 * t116;
            let t368 = f64x8::splat(0.0023426533963880895) * t140 * t349 - t105 * t366;
            let t369 = t139 * t368;
            let t370 = t67 * t135;
            let t371 = t176 * t22;
            let t372 = t73 * t70;
            let t373 = f64x8::splat(1.0) / t347;
            let t374 = t372 * t373;
            let t378 = t105 * t105;
            let t380 = f64x8::splat(0.00098556445) * t370 * t371 * t374 - f64x8::splat(2.0) * t378;
            let t381 = f64x8::splat(1.0) / t380;
            let t382 = t381 * v_sigma;
            let t384 = t341 * t369 * t382;
            let t386 = t105 * t338 + f64x8::splat(0.0049745833333333335) * t384;
            let t387 = t105 * t193;
            let t393 = f64x8::splat(0.00619125) * t387 * t374 - f64x8::splat(0.03979666666666667) * t370 * t198 * t366;
            let t394 = t393 * t381;
            let t395 = t394 * t204;
            let t397 = t368 * t381;
            let t398 = t397 * t211;
            let t400 = f64x8::splat(1.0) + t395 / f64x8::splat(8.0) - t398 / f64x8::splat(64.0);
            let t401 = f64x8::splat(1.0) / t400;
            let t402 = t386 * t401;
            let t404 = f64x8::splat(1.0) + f64x8::splat(0.05137) * t70;
            let t409 = f64x8::splat(7.05945) * t73 + f64x8::splat(1.549425) * t70 + f64x8::splat(0.420775) * t76 + f64x8::splat(0.1562925) * t82;
            let t412 = f64x8::splat(1.0) + f64x8::splat(32.1646831778707) / t409;
            let t413 = (simd::ln(t412));
            let t421 = -t90 + t223 * (-f64x8::splat(0.03109) * t404 * t413 + t90 - f64x8::splat(0.019751789702565206) * t102) + f64x8::splat(0.019751789702565206) * t223 * t102;
            let t422 = (f64x8::splat(0.0)).simd_lt(t421);
            let t424 = ((t422).select(t421, -t421));
            let t425 = f64x8::splat(1.0) / t424;
            let t426 = t6 * t425;
            let t427 = t120 * t426;
            let t430 = f64x8::splat(1.0) + f64x8::splat(0.05397236614853195) * t118 * t427;
            let t431 = (simd::ln(t430));
            let t433 = f64x8::splat(1.0) + f64x8::splat(0.193) * t431;
            let t434 = f64x8::splat(1.0) / t433;
            let t437 = t421 * t434 + f64x8::splat(0.0034822083333333332) * t384;
            let t440 = f64x8::splat(1.0) + f64x8::splat(0.1875) * t395 - f64x8::splat(0.04046875) * t398;
            let t441 = f64x8::splat(1.0) / t440;
            let t445 = t402 + t223 * (t437 * t441 - t402);
            let t446 = t279 * t445;
            let t448 = t276 * t446 / f64x8::splat(8.0);
            let tzk0 = t216 + t275 - t448;
            acc_zk = tzk0;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        ip += 8;
    }
}
