//! MGGA_C_REVSCAN exc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_c_revscan.c`
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
pub fn mgga_c_revscan_exc_unpol(
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
            let t11 = t5 * t7 / t8;
            let t13 = f64x8::splat(1.0) + f64x8::splat(0.053425) * t11;
            let t14 = ((t11).sqrt());
            let t17 = ((t11) * (t11).sqrt());
            let t19 = t2 * t2;
            let t20 = t4 * t4;
            let t21 = t19 * t20;
            let t22 = t8 * t8;
            let t25 = t21 * t6 / t22;
            let t27 = f64x8::splat(3.79785) * t14 + f64x8::splat(0.8969) * t11 + f64x8::splat(0.204775) * t17 + f64x8::splat(0.123235) * t25;
            let t30 = f64x8::splat(1.0) + f64x8::splat(16.081979498692537) / t27;
            let t31 = (simd::ln(t30));
            let t33 = f64x8::splat(0.0621814) * t13 * t31;
            let t34 = (f64x8::splat(1.0)).simd_le(zeta_threshold);
            let t35 = (simd::cbrt(zeta_threshold));
            let t37 = ((t34).select(t35 * zeta_threshold, f64x8::splat(1.0)));
            let t39 = f64x8::splat(2.0) * t37 - f64x8::splat(2.0);
            let t40 = f64x8::splat(M_CBRT2);
            let t41 = t40 - f64x8::splat(1.0);
            let t43 = f64x8::splat(1.0) / t41 / f64x8::splat(2.0);
            let t44 = t39 * t43;
            let t46 = f64x8::splat(1.0) + f64x8::splat(0.0278125) * t11;
            let t51 = f64x8::splat(5.1785) * t14 + f64x8::splat(0.905775) * t11 + f64x8::splat(0.1100325) * t17 + f64x8::splat(0.1241775) * t25;
            let t54 = f64x8::splat(1.0) + f64x8::splat(29.608749977793437) / t51;
            let t55 = (simd::ln(t54));
            let t58 = f64x8::splat(0.0197516734986138) * t44 * t46 * t55;
            let t59 = (simd::ln(f64x8::splat(2.0)));
            let t60 = f64x8::splat(1.0) - t59;
            let t61 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t63 = t60 / t61;
            let t64 = t35 * t35;
            let t65 = ((t34).select(t64, f64x8::splat(1.0)));
            let t66 = t65 * t65;
            let t67 = t66 * t65;
            let t69 = f64x8::splat(1.0) + f64x8::splat(0.025) * t11;
            let t71 = f64x8::splat(1.0) + f64x8::splat(0.04445) * t11;
            let t72 = f64x8::splat(1.0) / t71;
            let t73 = t69 * t72;
            let t74 = f64x8::splat(1.0) / t60;
            let t77 = f64x8::splat(1.0) / t67;
            let t78 = t61 * t77;
            let t80 = (simd::exp(-(-t33 + t58) * t74 * t78));
            let t81 = t80 - f64x8::splat(1.0);
            let t82 = f64x8::splat(1.0) / t81;
            let t83 = t74 * t82;
            let t84 = t83 * v_sigma;
            let t85 = t73 * t84;
            let t86 = v_rho * v_rho;
            let t88 = f64x8::splat(1.0) / t8 / t86;
            let t89 = t88 * t40;
            let t90 = f64x8::splat(1.0) / t66;
            let t92 = f64x8::splat(1.0) / t4;
            let t93 = t19 * t92;
            let t94 = t93 * t6;
            let t95 = t89 * t90 * t94;
            let t98 = f64x8::splat(1.0) + f64x8::splat(0.054878743191129266) * t85 * t95;
            let t99 = ((t98).sqrt().sqrt());
            let t102 = t69 * t69;
            let t103 = t71 * t71;
            let t104 = f64x8::splat(1.0) / t103;
            let t105 = t102 * t104;
            let t106 = t60 * t60;
            let t107 = f64x8::splat(1.0) / t106;
            let t108 = t81 * t81;
            let t109 = f64x8::splat(1.0) / t108;
            let t110 = t107 * t109;
            let t111 = v_sigma * v_sigma;
            let t112 = t110 * t111;
            let t113 = t105 * t112;
            let t114 = t86 * t86;
            let t116 = f64x8::splat(1.0) / t22 / t114;
            let t117 = t40 * t40;
            let t118 = t116 * t117;
            let t119 = t66 * t66;
            let t120 = f64x8::splat(1.0) / t119;
            let t121 = t118 * t120;
            let t122 = f64x8::splat(1.0) / t20;
            let t123 = t2 * t122;
            let t124 = t123 * t7;
            let t125 = t121 * t124;
            let t128 = f64x8::splat(1.0) + f64x8::splat(0.011293786703392187) * t113 * t125;
            let t129 = (simd::pow(t128, f64x8::splat(1.0) / f64x8::splat(8.0)));
            let t132 = f64x8::splat(1.0) - f64x8::splat(1.0) / t99 / f64x8::splat(2.0) - f64x8::splat(1.0) / t129 / f64x8::splat(2.0);
            let t135 = f64x8::splat(1.0) + f64x8::splat(1.0) * t132 * t81;
            let t136 = (simd::ln(t135));
            let t138 = t63 * t67 * t136;
            let t140 = f64x8::splat(1.0) / t22 / v_rho;
            let t143 = f64x8::splat(1.0) / t22 / t86;
            let t147 = f64x8::splat(M_CBRT6);
            let t149 = (simd::cbrt(t61));
            let t150 = t149 * t149;
            let t151 = f64x8::splat(1.0) / t150;
            let t152 = t151 * t117;
            let t154 = f64x8::splat(5.0) / f64x8::splat(9.0) * (v_tau * t140 - v_sigma * t143 / f64x8::splat(8.0)) * t147 * t152;
            let t155 = (t154).simd_le(f64x8::splat(1.0));
            let t156 = (simd::ln(f64x8::splat(f64::EPSILON)));
            let t159 = t156 / (-t156 + f64x8::splat(1.131));
            let t160 = (-t159).simd_lt(t154);
            let t161 = (t154).simd_lt(-t159);
            let t162 = ((t161).select(t154, -t159));
            let t163 = f64x8::splat(1.0) - t162;
            let t164 = f64x8::splat(1.0) / t163;
            let t167 = (simd::exp(-f64x8::splat(1.131) * t162 * t164));
            let t168 = ((t160).select(f64x8::splat(0.0), t167));
            let t170 = (simd::ln(f64x8::splat(0.7299270072992701) * f64x8::splat(f64::EPSILON)));
            let t173 = (-t170 + f64x8::splat(1.7)) / t170;
            let t174 = (t154).simd_lt(-t173);
            let t175 = ((t174).select(-t173, t154));
            let t176 = f64x8::splat(1.0) - t175;
            let t179 = (simd::exp(f64x8::splat(1.7) / t176));
            let t181 = ((t174).select(f64x8::splat(0.0), -f64x8::splat(1.37) * t179));
            let t182 = ((t155).select(t168, t181));
            let t185 = f64x8::splat(1.0) + f64x8::splat(0.033115) * t14 + f64x8::splat(0.04168) * t11;
            let t186 = f64x8::splat(1.0) / t185;
            let t189 = (simd::exp(f64x8::splat(1.0) * t186));
            let t190 = t189 - f64x8::splat(1.0);
            let t191 = t147 * t151;
            let t192 = t117 * v_sigma;
            let t196 = f64x8::splat(1.0) + f64x8::splat(0.04267528420875272) * t191 * t192 * t143;
            let t197 = ((t196).sqrt().sqrt());
            let t200 = t147 * t147;
            let t202 = f64x8::splat(1.0) / t149 / t61;
            let t203 = t200 * t202;
            let t204 = t40 * t111;
            let t205 = t114 * v_rho;
            let t207 = f64x8::splat(1.0) / t8 / t205;
            let t211 = f64x8::splat(1.0) + f64x8::splat(0.004552949705744548) * t203 * t204 * t207;
            let t212 = (simd::pow(t211, f64x8::splat(1.0) / f64x8::splat(8.0)));
            let t215 = f64x8::splat(1.0) - f64x8::splat(1.0) / t197 / f64x8::splat(2.0) - f64x8::splat(1.0) / t212 / f64x8::splat(2.0);
            let t217 = t190 * t215 + f64x8::splat(1.0);
            let t218 = (simd::ln(t217));
            let t224 = f64x8::splat(1.0) - f64x8::splat(2.363) * t41 * t39 * t43;
            let t226 = (-f64x8::splat(0.030197) * t186 + f64x8::splat(0.030197) * t218) * t224 + t33 - t58 - t138;
            let t227 = t182 * t226;
            let tzk0 = -t33 + t58 + t138 + t227;
            acc_zk = tzk0;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        ip += 8;
    }
}
