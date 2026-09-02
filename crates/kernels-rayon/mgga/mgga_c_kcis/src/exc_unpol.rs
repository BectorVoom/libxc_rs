//! MGGA_C_KCIS exc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_c_kcis.c`
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
pub fn mgga_c_kcis_exc_unpol(
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
            let t24 = t6 * t23;
            let t25 = t21 * t24;
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
            let t60 = t40 * v_sigma;
            let t61 = v_rho * v_rho;
            let t63 = f64x8::splat(1.0) / t8 / t61;
            let t64 = t60 * t63;
            let t65 = f64x8::splat(1.0) / t4;
            let t66 = t19 * t65;
            let t67 = (f64x8::splat(0.0)).simd_lt(t59);
            let t69 = ((t67).select(t59, -t59));
            let t70 = f64x8::splat(1.0) / t69;
            let t71 = t6 * t70;
            let t72 = t66 * t71;
            let t75 = f64x8::splat(1.0) + t64 * t72 / f64x8::splat(96.0);
            let t76 = (simd::ln(t75));
            let t78 = f64x8::splat(1.0) + f64x8::splat(0.066725) * t76;
            let t79 = f64x8::splat(1.0) / t78;
            let t81 = f64x8::splat(1.0) / t20;
            let t82 = t2 * t81;
            let t83 = t82 * t7;
            let t85 = f64x8::splat(1.0) / t8 / v_rho;
            let t86 = f64x8::splat(1.0) / v_rho;
            let t89 = f64x8::splat(1.07924) + f64x8::splat(0.03964) * t14 + f64x8::splat(0.0123825) * t11;
            let t92 = f64x8::splat(1.0) + t14 * t89 / f64x8::splat(2.0);
            let t93 = t92 * t92;
            let t94 = f64x8::splat(1.0) / t93;
            let t99 = t2 * t4 * t3;
            let t100 = t7 * t85;
            let t104 = t19 * t20 * t3;
            let t106 = f64x8::splat(1.0) / t22 / v_rho;
            let t107 = t6 * t106;
            let t110 = f64x8::splat(1.0) / t61;
            let t112 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t115 = t2 * t4 / t112;
            let t116 = t7 * t63;
            let t119 = -f64x8::splat(0.005977859662531589) * t86 + f64x8::splat(0.001317375) * t99 * t100 - f64x8::splat(0.00023775) * t104 * t107 + f64x8::splat(6.474423634745383e-06) * t110 - f64x8::splat(5.40140625e-07) * t115 * t116;
            let t121 = f64x8::splat(0.0011713266981940448) * t86 * t94 - t59 * t119;
            let t122 = t85 * t121;
            let t123 = (simd::pow(f64x8::splat(4.0), f64x8::splat(1.0) / f64x8::splat(6.0)));
            let t124 = t82 * t123;
            let t125 = t14 * t11;
            let t126 = t22 * t125;
            let t127 = f64x8::splat(1.0) / t92;
            let t131 = t59 * t59;
            let t133 = f64x8::splat(0.0019711289) * t124 * t126 * t127 - f64x8::splat(2.0) * t131;
            let t134 = f64x8::splat(1.0) / t133;
            let t135 = t134 * v_sigma;
            let t137 = t83 * t122 * t135;
            let t139 = t59 * t79 + f64x8::splat(0.009949166666666667) * t137;
            let t140 = ((f64x8::splat(4.0)).sqrt());
            let t141 = t59 * t140;
            let t142 = t125 * t127;
            let t145 = t7 * t22;
            let t149 = f64x8::splat(0.00619125) * t141 * t142 - f64x8::splat(0.07959333333333334) * t82 * t145 * t119;
            let t150 = t149 * t134;
            let t151 = v_sigma * t110;
            let t152 = t150 * t151;
            let t154 = t121 * t134;
            let t155 = v_sigma * v_sigma;
            let t156 = t61 * t61;
            let t157 = f64x8::splat(1.0) / t156;
            let t158 = t155 * t157;
            let t159 = t154 * t158;
            let t161 = f64x8::splat(1.0) + t152 / f64x8::splat(8.0) - t159 / f64x8::splat(64.0);
            let t162 = f64x8::splat(1.0) / t161;
            let t163 = t139 * t162;
            let t166 = (((f64x8::splat(2.0)).simd_le(zeta_threshold)).select(t36, f64x8::splat(2.0) * t40));
            let t168 = (((f64x8::splat(0.0)).simd_le(zeta_threshold)).select(t36, f64x8::splat(0.0)));
            let t170 = (t166 + t168 - f64x8::splat(2.0)) * t43;
            let t172 = f64x8::splat(1.0) + f64x8::splat(0.05137) * t11;
            let t177 = f64x8::splat(7.05945) * t14 + f64x8::splat(1.549425) * t11 + f64x8::splat(0.420775) * t17 + f64x8::splat(0.1562925) * t25;
            let t180 = f64x8::splat(1.0) + f64x8::splat(32.1646831778707) / t177;
            let t181 = (simd::ln(t180));
            let t189 = -t33 + t170 * (-f64x8::splat(0.03109) * t172 * t181 + t33 - f64x8::splat(0.019751789702565206) * t56) + f64x8::splat(0.019751789702565206) * t170 * t56;
            let t190 = v_sigma * t63;
            let t191 = t190 * t19;
            let t192 = t65 * t6;
            let t193 = (f64x8::splat(0.0)).simd_lt(t189);
            let t195 = ((t193).select(t189, -t189));
            let t196 = f64x8::splat(1.0) / t195;
            let t197 = t192 * t196;
            let t200 = f64x8::splat(1.0) + t191 * t197 / f64x8::splat(96.0);
            let t201 = (simd::ln(t200));
            let t203 = f64x8::splat(1.0) + f64x8::splat(0.066725) * t201;
            let t204 = f64x8::splat(1.0) / t203;
            let t207 = t189 * t204 + f64x8::splat(0.0069644166666666665) * t137;
            let t210 = f64x8::splat(1.0) + f64x8::splat(0.1875) * t152 - f64x8::splat(0.04046875) * t159;
            let t211 = f64x8::splat(1.0) / t210;
            let t214 = t44 * (t207 * t211 - t163);
            let t215 = v_sigma * t86;
            let t216 = f64x8::splat(1.0) / v_tau;
            let t217 = ((t34).select(zeta_threshold, f64x8::splat(1.0)));
            let t218 = t216 * t217;
            let t220 = t5 * t10 * t40;
            let t222 = f64x8::splat(1.0) + f64x8::splat(0.053425) * t220;
            let t223 = ((t220).sqrt());
            let t226 = ((t220) * (t220).sqrt());
            let t228 = t40 * t40;
            let t230 = t21 * t24 * t228;
            let t232 = f64x8::splat(3.79785) * t223 + f64x8::splat(0.8969) * t220 + f64x8::splat(0.204775) * t226 + f64x8::splat(0.123235) * t230;
            let t235 = f64x8::splat(1.0) + f64x8::splat(16.081824322151103) / t232;
            let t236 = (simd::ln(t235));
            let t238 = f64x8::splat(0.062182) * t222 * t236;
            let t240 = f64x8::splat(1.0) + f64x8::splat(0.0278125) * t220;
            let t245 = f64x8::splat(5.1785) * t223 + f64x8::splat(0.905775) * t220 + f64x8::splat(0.1100325) * t226 + f64x8::splat(0.1241775) * t230;
            let t248 = f64x8::splat(1.0) + f64x8::splat(29.608574643216677) / t245;
            let t249 = (simd::ln(t248));
            let t250 = t240 * t249;
            let t253 = -t238 + f64x8::splat(0.019751789702565206) * t44 * t250;
            let t254 = (f64x8::splat(0.0)).simd_lt(t253);
            let t256 = ((t254).select(t253, -t253));
            let t257 = f64x8::splat(1.0) / t256;
            let t258 = t228 * t257;
            let t259 = t192 * t258;
            let t262 = f64x8::splat(1.0) + t191 * t259 / f64x8::splat(96.0);
            let t263 = (simd::ln(t262));
            let t265 = f64x8::splat(1.0) + f64x8::splat(0.066725) * t263;
            let t266 = f64x8::splat(1.0) / t265;
            let t268 = t82 * t100;
            let t271 = f64x8::splat(1.07924) + f64x8::splat(0.03964) * t223 + f64x8::splat(0.0123825) * t220;
            let t274 = f64x8::splat(1.0) + t223 * t271 / f64x8::splat(2.0);
            let t275 = t274 * t274;
            let t276 = f64x8::splat(1.0) / t275;
            let t280 = t100 * t40;
            let t283 = t107 * t228;
            let t287 = t116 * t40;
            let t290 = -f64x8::splat(0.011955719325063178) * t86 + f64x8::splat(0.00263475) * t99 * t280 - f64x8::splat(0.0004755) * t104 * t283 + f64x8::splat(2.5897694538981533e-05) * t110 - f64x8::splat(2.1605625e-06) * t115 * t287;
            let t292 = f64x8::splat(0.0023426533963880895) * t86 * t276 - t253 * t290;
            let t293 = t40 * t292;
            let t294 = t22 * t40;
            let t295 = t223 * t220;
            let t296 = f64x8::splat(1.0) / t274;
            let t297 = t295 * t296;
            let t301 = t253 * t253;
            let t303 = f64x8::splat(0.00098556445) * t124 * t294 * t297 - f64x8::splat(2.0) * t301;
            let t304 = f64x8::splat(1.0) / t303;
            let t305 = t304 * v_sigma;
            let t306 = t293 * t305;
            let t307 = t268 * t306;
            let t309 = t253 * t266 + f64x8::splat(0.0049745833333333335) * t307;
            let t310 = t253 * t140;
            let t316 = f64x8::splat(0.00619125) * t310 * t297 - f64x8::splat(0.03979666666666667) * t83 * t294 * t290;
            let t317 = t316 * t304;
            let t318 = t317 * t151;
            let t320 = t292 * t304;
            let t321 = t320 * t158;
            let t323 = f64x8::splat(1.0) + t318 / f64x8::splat(8.0) - t321 / f64x8::splat(64.0);
            let t324 = f64x8::splat(1.0) / t323;
            let t325 = t309 * t324;
            let t327 = f64x8::splat(1.0) + f64x8::splat(0.05137) * t220;
            let t332 = f64x8::splat(7.05945) * t223 + f64x8::splat(1.549425) * t220 + f64x8::splat(0.420775) * t226 + f64x8::splat(0.1562925) * t230;
            let t335 = f64x8::splat(1.0) + f64x8::splat(32.1646831778707) / t332;
            let t336 = (simd::ln(t335));
            let t344 = -t238 + t170 * (-f64x8::splat(0.03109) * t327 * t336 + t238 - f64x8::splat(0.019751789702565206) * t250) + f64x8::splat(0.019751789702565206) * t170 * t250;
            let t345 = (f64x8::splat(0.0)).simd_lt(t344);
            let t347 = ((t345).select(t344, -t344));
            let t348 = f64x8::splat(1.0) / t347;
            let t349 = t6 * t348;
            let t350 = t66 * t349;
            let t353 = f64x8::splat(1.0) + t64 * t350 / f64x8::splat(96.0);
            let t354 = (simd::ln(t353));
            let t356 = f64x8::splat(1.0) + f64x8::splat(0.066725) * t354;
            let t357 = f64x8::splat(1.0) / t356;
            let t360 = t344 * t357 + f64x8::splat(0.0034822083333333332) * t307;
            let t363 = f64x8::splat(1.0) + f64x8::splat(0.1875) * t318 - f64x8::splat(0.04046875) * t321;
            let t364 = f64x8::splat(1.0) / t363;
            let t368 = t325 + t170 * (t360 * t364 - t325);
            let t369 = t218 * t368;
            let t371 = t215 * t369 / f64x8::splat(8.0);
            let tzk0 = t163 + t214 - t371;
            acc_zk = tzk0;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        ip += 8;
    }
}
