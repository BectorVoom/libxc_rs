//! MGGA_C_RSCAN exc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_c_rscan.c`
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
pub fn mgga_c_rscan_exc_unpol(
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
            let t23 = f64x8::splat(1.0) / t22;
            let t25 = t21 * t6 * t23;
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
            let t87 = t8 * t86;
            let t88 = f64x8::splat(1.0) / t87;
            let t89 = t88 * t40;
            let t90 = f64x8::splat(1.0) / t66;
            let t92 = f64x8::splat(1.0) / t4;
            let t94 = t19 * t92 * t6;
            let t98 = f64x8::splat(1.0) + f64x8::splat(0.027439371595564633) * t85 * t89 * t90 * t94;
            let t99 = ((t98).sqrt().sqrt());
            let t101 = f64x8::splat(1.0) - f64x8::splat(1.0) / t99;
            let t104 = f64x8::splat(1.0) + f64x8::splat(1.0) * t101 * t81;
            let t105 = (simd::ln(t104));
            let t107 = t63 * t67 * t105;
            let t108 = t86 * t86;
            let t109 = t108 * v_rho;
            let t110 = t22 * v_rho;
            let t111 = f64x8::splat(1.0) / t110;
            let t113 = t22 * t86;
            let t114 = f64x8::splat(1.0) / t113;
            let t117 = v_tau * t111 - v_sigma * t114 / f64x8::splat(8.0);
            let t118 = (f64x8::splat(0.0)).simd_lt(t117);
            let t119 = ((t118).select(t117, f64x8::splat(0.0)));
            let t120 = t119 * t119;
            let t121 = t120 * t119;
            let t122 = t109 * t121;
            let t123 = f64x8::splat(M_CBRT6);
            let t124 = t123 * t123;
            let t125 = (simd::cbrt(t61));
            let t126 = t125 * t125;
            let t127 = t124 * t126;
            let t130 = t40 * t40;
            let t132 = f64x8::splat(3.0) / f64x8::splat(10.0) * t127 * t110 + f64x8::splat(0.0001) * t130;
            let t133 = t132 * t132;
            let t134 = t133 * t132;
            let t135 = f64x8::splat(1.0) / t134;
            let t136 = t86 * v_rho;
            let t137 = t8 * t136;
            let t140 = f64x8::splat(1.0) / t133 * t40;
            let t143 = f64x8::splat(2.0) * t137 * t120 * t140 + f64x8::splat(0.001);
            let t144 = f64x8::splat(1.0) / t143;
            let t145 = t135 * t144;
            let t147 = f64x8::splat(4.0) * t122 * t145;
            let t148 = (t147).simd_le(f64x8::splat(2.5));
            let t149 = (f64x8::splat(2.5)).simd_lt(t147);
            let t150 = ((t149).select(f64x8::splat(2.5), t147));
            let t152 = t150 * t150;
            let t154 = t152 * t150;
            let t156 = t152 * t152;
            let t158 = t156 * t150;
            let t160 = t156 * t152;
            let t165 = ((t149).select(t147, f64x8::splat(2.5)));
            let t166 = f64x8::splat(1.0) - t165;
            let t169 = (simd::exp(f64x8::splat(1.5) / t166));
            let t171 = ((t148).select(f64x8::splat(1.0) - f64x8::splat(0.64) * t150 - f64x8::splat(0.4352) * t152 - f64x8::splat(1.535685604549) * t154 + f64x8::splat(3.061560252175) * t156 - f64x8::splat(1.915710236206) * t158 + f64x8::splat(0.516884468372) * t160 - f64x8::splat(0.051848879792) * t156 * t154, -f64x8::splat(0.7) * t169));
            let t174 = f64x8::splat(1.0) + f64x8::splat(0.04445) * t14 + f64x8::splat(0.03138525) * t11;
            let t175 = f64x8::splat(1.0) / t174;
            let t178 = (simd::exp(f64x8::splat(1.0) * t175));
            let t179 = t178 - f64x8::splat(1.0);
            let t180 = f64x8::splat(1.0) / t126;
            let t181 = t123 * t180;
            let t182 = t130 * v_sigma;
            let t186 = f64x8::splat(1.0) + f64x8::splat(0.02133764210437636) * t181 * t182 * t114;
            let t187 = ((t186).sqrt().sqrt());
            let t189 = f64x8::splat(1.0) - f64x8::splat(1.0) / t187;
            let t191 = t179 * t189 + f64x8::splat(1.0);
            let t192 = (simd::ln(t191));
            let t198 = f64x8::splat(1.0) - f64x8::splat(2.363) * t41 * t39 * t43;
            let t200 = (-f64x8::splat(0.0285764) * t175 + f64x8::splat(0.0285764) * t192) * t198 + t33 - t58 - t107;
            let t201 = t171 * t200;
            let tzk0 = -t33 + t58 + t107 + t201;
            acc_zk = tzk0;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        ip += 8;
    }
}
