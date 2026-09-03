//! MGGA_C_PKZB exc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_c_pkzb.c`
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
pub fn mgga_c_pkzb_exc_unpol(
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
            let t2 = v_sigma * v_sigma;
            let t3 = v_rho * v_rho;
            let t4 = f64x8::splat(1.0) / t3;
            let t5 = t2 * t4;
            let t6 = v_tau * v_tau;
            let t7 = f64x8::splat(1.0) / t6;
            let t10 = f64x8::splat(1.0) + f64x8::splat(0.00828125) * t5 * t7;
            let t11 = f64x8::splat(M_CBRT3);
            let t12 = f64x8::splat(1.0) / f64x8::splat(M_PI);
            let t13 = (simd::cbrt(t12));
            let t14 = t11 * t13;
            let t15 = f64x8::splat(M_CBRT4);
            let t16 = t15 * t15;
            let t17 = (simd::cbrt(v_rho));
            let t18 = f64x8::splat(1.0) / t17;
            let t20 = t14 * t16 * t18;
            let t22 = f64x8::splat(1.0) + f64x8::splat(0.053425) * t20;
            let t23 = ((t20).sqrt());
            let t26 = ((t20) * (t20).sqrt());
            let t28 = t11 * t11;
            let t29 = t13 * t13;
            let t30 = t28 * t29;
            let t31 = t17 * t17;
            let t32 = f64x8::splat(1.0) / t31;
            let t34 = t30 * t15 * t32;
            let t36 = f64x8::splat(3.79785) * t23 + f64x8::splat(0.8969) * t20 + f64x8::splat(0.204775) * t26 + f64x8::splat(0.123235) * t34;
            let t39 = f64x8::splat(1.0) + f64x8::splat(16.081979498692537) / t36;
            let t40 = (simd::ln(t39));
            let t42 = f64x8::splat(0.0621814) * t22 * t40;
            let t43 = (f64x8::splat(1.0)).simd_le(zeta_threshold);
            let t44 = (simd::cbrt(zeta_threshold));
            let t45 = t44 * zeta_threshold;
            let t46 = ((t43).select(t45, f64x8::splat(1.0)));
            let t49 = f64x8::splat(M_CBRT2);
            let t52 = f64x8::splat(1.0) / (f64x8::splat(2.0) * t49 - f64x8::splat(2.0));
            let t53 = (f64x8::splat(2.0) * t46 - f64x8::splat(2.0)) * t52;
            let t55 = f64x8::splat(1.0) + f64x8::splat(0.0278125) * t20;
            let t60 = f64x8::splat(5.1785) * t23 + f64x8::splat(0.905775) * t20 + f64x8::splat(0.1100325) * t26 + f64x8::splat(0.1241775) * t34;
            let t63 = f64x8::splat(1.0) + f64x8::splat(29.608749977793437) / t60;
            let t64 = (simd::ln(t63));
            let t67 = f64x8::splat(0.0197516734986138) * t53 * t55 * t64;
            let t68 = (simd::ln(f64x8::splat(2.0)));
            let t69 = f64x8::splat(1.0) - t68;
            let t70 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t71 = f64x8::splat(1.0) / t70;
            let t72 = t69 * t71;
            let t73 = t44 * t44;
            let t74 = ((t43).select(t73, f64x8::splat(1.0)));
            let t75 = t74 * t74;
            let t76 = t75 * t74;
            let t78 = f64x8::splat(1.0) / t17 / t3;
            let t79 = v_sigma * t78;
            let t81 = f64x8::splat(1.0) / t75;
            let t83 = f64x8::splat(1.0) / t13;
            let t84 = t83 * t15;
            let t85 = t81 * t28 * t84;
            let t88 = f64x8::splat(1.0) / t69;
            let t91 = f64x8::splat(1.0) / t76;
            let t92 = t70 * t91;
            let t94 = (simd::exp(-(-t42 + t67) * t88 * t92));
            let t95 = t94 - f64x8::splat(1.0);
            let t96 = f64x8::splat(1.0) / t95;
            let t97 = t88 * t96;
            let t98 = t3 * t3;
            let t100 = f64x8::splat(1.0) / t31 / t98;
            let t101 = t2 * t100;
            let t103 = t49 * t49;
            let t104 = t75 * t75;
            let t105 = f64x8::splat(1.0) / t104;
            let t106 = t103 * t105;
            let t107 = f64x8::splat(1.0) / t29;
            let t108 = t11 * t107;
            let t109 = t108 * t16;
            let t110 = t106 * t109;
            let t113 = t79 * t49 * t85 / f64x8::splat(96.0) + f64x8::splat(0.0002143700905903487) * t97 * t101 * t110;
            let t114 = t113 * t88;
            let t117 = f64x8::splat(1.0) + f64x8::splat(0.6585449182935511) * t97 * t113;
            let t118 = f64x8::splat(1.0) / t117;
            let t121 = f64x8::splat(1.0) + f64x8::splat(0.6585449182935511) * t114 * t118;
            let t122 = (simd::ln(t121));
            let t125 = t72 * t76 * t122 - t42 + t67;
            let t126 = t10 * t125;
            let t129 = ((v_rho / f64x8::splat(2.0)).simd_le(dens_threshold)) | (t43);
            let t130 = t14 * t16;
            let t133 = ((t43).select(f64x8::splat(1.0) / t44, f64x8::splat(1.0)));
            let t135 = t130 * t18 * t49 * t133;
            let t137 = f64x8::splat(1.0) + f64x8::splat(0.053425) * t135;
            let t138 = ((t135).sqrt());
            let t141 = ((t135) * (t135).sqrt());
            let t143 = t30 * t15;
            let t145 = t133 * t133;
            let t147 = t143 * t32 * t103 * t145;
            let t149 = f64x8::splat(3.79785) * t138 + f64x8::splat(0.8969) * t135 + f64x8::splat(0.204775) * t141 + f64x8::splat(0.123235) * t147;
            let t152 = f64x8::splat(1.0) + f64x8::splat(16.081979498692537) / t149;
            let t153 = (simd::ln(t152));
            let t155 = f64x8::splat(0.0621814) * t137 * t153;
            let t156 = (f64x8::splat(2.0)).simd_le(zeta_threshold);
            let t158 = ((t156).select(t45, f64x8::splat(2.0) * t49));
            let t159 = (f64x8::splat(0.0)).simd_le(zeta_threshold);
            let t160 = ((t159).select(t45, f64x8::splat(0.0)));
            let t162 = (t158 + t160 - f64x8::splat(2.0)) * t52;
            let t164 = f64x8::splat(1.0) + f64x8::splat(0.05137) * t135;
            let t169 = f64x8::splat(7.05945) * t138 + f64x8::splat(1.549425) * t135 + f64x8::splat(0.420775) * t141 + f64x8::splat(0.1562925) * t147;
            let t172 = f64x8::splat(1.0) + f64x8::splat(32.16395899738507) / t169;
            let t173 = (simd::ln(t172));
            let t177 = f64x8::splat(1.0) + f64x8::splat(0.0278125) * t135;
            let t182 = f64x8::splat(5.1785) * t138 + f64x8::splat(0.905775) * t135 + f64x8::splat(0.1100325) * t141 + f64x8::splat(0.1241775) * t147;
            let t185 = f64x8::splat(1.0) + f64x8::splat(29.608749977793437) / t182;
            let t186 = (simd::ln(t185));
            let t187 = t177 * t186;
            let t190 = t162 * (-f64x8::splat(0.0310907) * t164 * t173 + t155 - f64x8::splat(0.0197516734986138) * t187);
            let t192 = f64x8::splat(0.0197516734986138) * t162 * t187;
            let t193 = ((t156).select(t73, t103));
            let t194 = ((t159).select(t73, f64x8::splat(0.0)));
            let t196 = t193 / f64x8::splat(2.0) + t194 / f64x8::splat(2.0);
            let t197 = t196 * t196;
            let t198 = t197 * t196;
            let t199 = f64x8::splat(1.0) / t197;
            let t200 = t199 * t28;
            let t204 = t84 * t103 / t133;
            let t209 = f64x8::splat(1.0) / t198;
            let t210 = t70 * t209;
            let t212 = (simd::exp(-(-t155 + t190 + t192) * t88 * t210));
            let t213 = t212 - f64x8::splat(1.0);
            let t214 = f64x8::splat(1.0) / t213;
            let t215 = t88 * t214;
            let t216 = t197 * t197;
            let t217 = f64x8::splat(1.0) / t216;
            let t220 = t16 * t49;
            let t221 = f64x8::splat(1.0) / t145;
            let t222 = t220 * t221;
            let t223 = t108 * t222;
            let t226 = t79 * t200 * t204 / f64x8::splat(96.0) + f64x8::splat(0.0004287401811806974) * t215 * t101 * t217 * t223;
            let t227 = t226 * t88;
            let t230 = f64x8::splat(1.0) + f64x8::splat(0.6585449182935511) * t215 * t226;
            let t231 = f64x8::splat(1.0) / t230;
            let t234 = f64x8::splat(1.0) + f64x8::splat(0.6585449182935511) * t227 * t231;
            let t235 = (simd::ln(t234));
            let t239 = ((t43).select(zeta_threshold, f64x8::splat(1.0)));
            let t242 = ((t129).select(f64x8::splat(0.0), (t72 * t198 * t235 - t155 + t190 + t192) * t239 / f64x8::splat(2.0)));
            let t243 = t7 * t242;
            let t245 = f64x8::splat(0.0478125) * t5 * t243;
            let tzk0 = t126 - t245;
            acc_zk = tzk0;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        ip += 8;
    }
}
