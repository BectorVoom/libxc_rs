//! GGA_C_OPTC exc unpol kernel — explicit SIMD (bit-exact).
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
pub fn gga_c_optc_exc_unpol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
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
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        ip += 8;
    }
}
