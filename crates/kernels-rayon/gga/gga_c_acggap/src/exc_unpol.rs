//! GGA_C_ACGGAP exc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_acggap.c`
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
pub fn gga_c_acggap_exc_unpol(
    rho: &[f64],
    sigma: &[f64],
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
            let t24 = t20 * t5 / t21;
            let t26 = f64x8::splat(3.79785) * t13 + f64x8::splat(0.8969) * t10 + f64x8::splat(0.204775) * t16 + f64x8::splat(0.123235) * t24;
            let t29 = f64x8::splat(1.0) + f64x8::splat(16.081979498692537) / t26;
            let t30 = (simd::ln(t29));
            let t32 = f64x8::splat(0.0621814) * t12 * t30;
            let t33 = (f64x8::splat(1.0)).simd_le(zeta_threshold);
            let t34 = (simd::cbrt(zeta_threshold));
            let t36 = ((t33).select(t34 * zeta_threshold, f64x8::splat(1.0)));
            let t39 = f64x8::splat(M_CBRT2);
            let t43 = (f64x8::splat(2.0) * t36 - f64x8::splat(2.0)) / (f64x8::splat(2.0) * t39 - f64x8::splat(2.0));
            let t45 = f64x8::splat(1.0) + f64x8::splat(0.0278125) * t10;
            let t50 = f64x8::splat(5.1785) * t13 + f64x8::splat(0.905775) * t10 + f64x8::splat(0.1100325) * t16 + f64x8::splat(0.1241775) * t24;
            let t53 = f64x8::splat(1.0) + f64x8::splat(29.608749977793437) / t50;
            let t54 = (simd::ln(t53));
            let t57 = f64x8::splat(0.0197516734986138) * t43 * t45 * t54;
            let t58 = (simd::ln(f64x8::splat(2.0)));
            let t59 = f64x8::splat(1.0) - t58;
            let t60 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t61 = f64x8::splat(1.0) / t60;
            let t62 = t59 * t61;
            let t63 = t34 * t34;
            let t64 = ((t33).select(t63, f64x8::splat(1.0)));
            let t65 = t64 * t64;
            let t66 = t65 * t64;
            let t68 = f64x8::splat(1.0) + f64x8::splat(0.0416675) * t10;
            let t72 = f64x8::splat(1.0) + f64x8::splat(0.125) * t4 * t9 * t68;
            let t74 = f64x8::splat(1.0) + f64x8::splat(0.0740825) * t10;
            let t78 = f64x8::splat(1.0) + f64x8::splat(0.125) * t4 * t9 * t74;
            let t79 = f64x8::splat(1.0) / t78;
            let t80 = t72 * t79;
            let t81 = v_rho * v_rho;
            let t83 = f64x8::splat(1.0) / t7 / t81;
            let t84 = v_sigma * t83;
            let t85 = f64x8::splat(1.0) / t65;
            let t86 = t39 * t85;
            let t87 = t84 * t86;
            let t88 = f64x8::splat(1.0) / t3;
            let t89 = t18 * t88;
            let t90 = ((v_sigma).sqrt());
            let t92 = f64x8::splat(1.0) / t7 / v_rho;
            let t94 = t39 * t39;
            let t95 = f64x8::splat(1.0) / t64;
            let t96 = t94 * t95;
            let t97 = f64x8::splat(1.0) / t13;
            let t98 = t96 * t97;
            let t99 = t90 * t92 * t98;
            let t101 = f64x8::splat(4.5) + t99 / f64x8::splat(4.0);
            let t102 = t5 * t101;
            let t104 = f64x8::splat(4.5) + f64x8::splat(0.36675) * t99;
            let t105 = f64x8::splat(1.0) / t104;
            let t107 = t89 * t102 * t105;
            let t110 = f64x8::splat(1.0) / t59;
            let t111 = t80 * t110;
            let t114 = f64x8::splat(1.0) / t66;
            let t115 = t60 * t114;
            let t117 = (simd::exp(-(-t32 + t57) * t110 * t115));
            let t118 = t117 - f64x8::splat(1.0);
            let t119 = f64x8::splat(1.0) / t118;
            let t120 = v_sigma * v_sigma;
            let t121 = t119 * t120;
            let t122 = t81 * t81;
            let t124 = f64x8::splat(1.0) / t21 / t122;
            let t125 = t121 * t124;
            let t126 = t111 * t125;
            let t127 = t65 * t65;
            let t128 = f64x8::splat(1.0) / t127;
            let t129 = t94 * t128;
            let t130 = t129 * t1;
            let t131 = f64x8::splat(1.0) / t19;
            let t132 = t131 * t6;
            let t133 = t101 * t101;
            let t134 = t104 * t104;
            let t135 = f64x8::splat(1.0) / t134;
            let t136 = t133 * t135;
            let t137 = t132 * t136;
            let t138 = t130 * t137;
            let t141 = t87 * t107 / f64x8::splat(96.0) + f64x8::splat(0.0002143700905903487) * t126 * t138;
            let t142 = t141 * t110;
            let t143 = t110 * t119;
            let t144 = t143 * t141;
            let t147 = f64x8::splat(1.0) + f64x8::splat(0.6585449182935511) * t80 * t144;
            let t148 = f64x8::splat(1.0) / t147;
            let t149 = t142 * t148;
            let t152 = f64x8::splat(1.0) + f64x8::splat(0.6585449182935511) * t80 * t149;
            let t153 = (simd::ln(t152));
            let t155 = t62 * t66 * t153;
            let tzk0 = -t32 + t57 + t155;
            acc_zk = tzk0;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        ip += 8;
    }
}
