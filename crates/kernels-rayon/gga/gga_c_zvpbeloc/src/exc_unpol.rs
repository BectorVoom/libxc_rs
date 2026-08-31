//! GGA_C_ZVPBELOC exc unpol kernel — explicit SIMD (exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_zvpbeloc.c`
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
pub fn gga_c_zvpbeloc_exc_unpol(
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
            let t1 = (simd::pow(f64x8::splat(4.0), f64x8::splat(1.0) / f64x8::splat(6.0)));
            let t2 = t1 * t1;
            let t3 = t2 * t2;
            let t4 = t3 * t1;
            let t5 = (simd::pow(f64x8::splat(3.0), f64x8::splat(1.0) / f64x8::splat(6.0)));
            let t6 = t4 * t5;
            let t7 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t8 = f64x8::splat(1.0) / t7;
            let t9 = (simd::pow(t8, f64x8::splat(1.0) / f64x8::splat(6.0)));
            let t10 = t6 * t9;
            let t11 = f64x8::splat(1.0) / f64x8::splat(M_PI);
            let t12 = (simd::cbrt(t11));
            let t13 = (simd::cbrt(v_rho));
            let t14 = f64x8::splat(1.0) / t13;
            let t17 = (((f64x8::splat(1e-20)).simd_lt(f64x8::splat(0.0))).select(f64x8::splat(0.0), f64x8::splat(1e-20)));
            let t19 = t10 * t12 * t14 * t17;
            let t21 = (simd::exp(-f64x8::splat(1.0) * t19));
            let t22 = f64x8::splat(M_CBRT3);
            let t23 = t22 * t12;
            let t24 = f64x8::splat(M_CBRT4);
            let t25 = t24 * t24;
            let t27 = t23 * t25 * t14;
            let t29 = f64x8::splat(1.0) + f64x8::splat(0.053425) * t27;
            let t30 = ((t27).sqrt());
            let t33 = ((t27) * (t27).sqrt());
            let t35 = t22 * t22;
            let t36 = t12 * t12;
            let t37 = t35 * t36;
            let t38 = t13 * t13;
            let t41 = t37 * t24 / t38;
            let t43 = f64x8::splat(3.79785) * t30 + f64x8::splat(0.8969) * t27 + f64x8::splat(0.204775) * t33 + f64x8::splat(0.123235) * t41;
            let t46 = f64x8::splat(1.0) + f64x8::splat(16.081979498692537) / t43;
            let t47 = (simd::ln(t46));
            let t49 = f64x8::splat(0.0621814) * t29 * t47;
            let t50 = (f64x8::splat(1.0)).simd_le(zeta_threshold);
            let t51 = (simd::cbrt(zeta_threshold));
            let t53 = ((t50).select(t51 * zeta_threshold, f64x8::splat(1.0)));
            let t56 = f64x8::splat(M_CBRT2);
            let t60 = (f64x8::splat(2.0) * t53 - f64x8::splat(2.0)) / (f64x8::splat(2.0) * t56 - f64x8::splat(2.0));
            let t62 = f64x8::splat(1.0) + f64x8::splat(0.0278125) * t27;
            let t67 = f64x8::splat(5.1785) * t30 + f64x8::splat(0.905775) * t27 + f64x8::splat(0.1100325) * t33 + f64x8::splat(0.1241775) * t41;
            let t70 = f64x8::splat(1.0) + f64x8::splat(29.608749977793437) / t67;
            let t71 = (simd::ln(t70));
            let t74 = f64x8::splat(0.0197516734986138) * t60 * t62 * t71;
            let t75 = (simd::ln(f64x8::splat(2.0)));
            let t76 = f64x8::splat(1.0) - t75;
            let t77 = t76 * t8;
            let t78 = t51 * t51;
            let t79 = ((t50).select(t78, f64x8::splat(1.0)));
            let t80 = t79 * t79;
            let t81 = t80 * t79;
            let t82 = v_rho * v_rho;
            let t84 = f64x8::splat(1.0) / t13 / t82;
            let t85 = v_sigma * t84;
            let t86 = f64x8::splat(1.0) / t80;
            let t87 = t56 * t86;
            let t89 = f64x8::splat(1.0) / t12;
            let t90 = t35 * t89;
            let t92 = (simd::exp(-t41 / f64x8::splat(4.0)));
            let t93 = f64x8::splat(1.0) - t92;
            let t94 = t24 * t93;
            let t95 = t90 * t94;
            let t98 = f64x8::splat(0.0375) + f64x8::splat(0.0008333333333333334) * t85 * t87 * t95;
            let t100 = t86 * t35;
            let t102 = t100 * t89 * t24;
            let t105 = f64x8::splat(1.0) / t76;
            let t106 = t98 * t105;
            let t109 = f64x8::splat(1.0) / t81;
            let t112 = (simd::exp(-(-t49 + t74) * t105 * t7 * t109));
            let t113 = t112 - f64x8::splat(1.0);
            let t114 = f64x8::splat(1.0) / t113;
            let t115 = t7 * t114;
            let t116 = v_sigma * v_sigma;
            let t117 = t115 * t116;
            let t118 = t106 * t117;
            let t119 = t82 * t82;
            let t121 = f64x8::splat(1.0) / t38 / t119;
            let t122 = t56 * t56;
            let t123 = t121 * t122;
            let t124 = t80 * t80;
            let t125 = f64x8::splat(1.0) / t124;
            let t127 = f64x8::splat(1.0) / t36;
            let t129 = t22 * t127 * t25;
            let t130 = t123 * t125 * t129;
            let t133 = t85 * t56 * t102 / f64x8::splat(96.0) + t118 * t130 / f64x8::splat(3072.0);
            let t134 = t98 * t133;
            let t135 = t105 * t7;
            let t136 = t115 * t133;
            let t138 = t106 * t136 + f64x8::splat(1.0);
            let t139 = f64x8::splat(1.0) / t138;
            let t140 = t135 * t139;
            let t142 = t134 * t140 + f64x8::splat(1.0);
            let t143 = (simd::ln(t142));
            let tzk0 = t21 * (t77 * t81 * t143 - t49 + t74);
            acc_zk = tzk0;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        ip += 8;
    }
}
