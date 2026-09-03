//! HYB_LDA_XC_BN05 exc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/hyb_lda_xc_bn05.c`
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
pub fn hyb_lda_xc_bn05_exc_unpol(
    rho: &[f64],
    zk: &mut [f64],
    param_hyb_omega_0: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_hyb_omega_0 = f64x8::splat(param_hyb_omega_0);
    let dens_threshold = f64x8::splat(dens_threshold);
    let zeta_threshold = f64x8::splat(zeta_threshold);
    let mut ip = 0usize;
    while ip < np {
        let m = (np - ip).min(8);
        let v_rho = load(rho, ip, np);
        let mut acc_zk = V_ZERO;
        {
            let t1 = f64x8::splat(M_CBRT3);
            let t2 = f64x8::splat(1.0) / f64x8::splat(M_PI);
            let t3 = (simd::cbrt(t2));
            let t4 = t3 * t1;
            let t5 = f64x8::splat(M_CBRT4);
            let t6 = t5 * t5;
            let t7 = t6 * t4;
            let t8 = f64x8::splat(M_CBRT2);
            let t9 = t8 * t8;
            let t10 = (f64x8::splat(1.0)).simd_le(zeta_threshold);
            let t11 = (simd::cbrt(zeta_threshold));
            let t13 = ((t10).select(t11 * zeta_threshold, f64x8::splat(1.0)));
            let t14 = t13 * t9;
            let t15 = (simd::cbrt(v_rho));
            let t16 = (simd::cbrt(f64x8::splat(9.0)));
            let t17 = t16 * t16;
            let t18 = t3 * t3;
            let t20 = param_hyb_omega_0 * t18 * t17;
            let t21 = f64x8::splat(1.0) / t15;
            let t23 = ((t10).select(t11, f64x8::splat(1.0)));
            let t24 = f64x8::splat(1.0) / t23;
            let t27 = t24 * t21 * t1 * t20 / f64x8::splat(18.0);
            let t28 = (f64x8::splat(1.92)).simd_le(t27);
            let t29 = (f64x8::splat(1.92)).simd_lt(t27);
            let t30 = ((t29).select(t27, f64x8::splat(1.92)));
            let t31 = t30 * t30;
            let t34 = t31 * t31;
            let t35 = f64x8::splat(1.0) / t34;
            let t37 = t34 * t31;
            let t38 = f64x8::splat(1.0) / t37;
            let t40 = t34 * t34;
            let t41 = f64x8::splat(1.0) / t40;
            let t43 = t40 * t31;
            let t44 = f64x8::splat(1.0) / t43;
            let t46 = t40 * t34;
            let t47 = f64x8::splat(1.0) / t46;
            let t49 = t40 * t37;
            let t50 = f64x8::splat(1.0) / t49;
            let t52 = t40 * t40;
            let t53 = f64x8::splat(1.0) / t52;
            let t56 = f64x8::splat(1.0) / t52 / t31;
            let t59 = f64x8::splat(1.0) / t52 / t34;
            let t62 = f64x8::splat(1.0) / t52 / t37;
            let t65 = f64x8::splat(1.0) / t52 / t40;
            let t68 = f64x8::splat(1.0) / t52 / t43;
            let t71 = f64x8::splat(1.0) / t52 / t46;
            let t74 = f64x8::splat(1.0) / t52 / t49;
            let t76 = t52 * t52;
            let t77 = f64x8::splat(1.0) / t76;
            let t80 = f64x8::splat(1.0) / t76 / t31;
            let t83 = f64x8::splat(1.0) / t76 / t34;
            let t85 = f64x8::splat(1.0) / t31 / f64x8::splat(9.0) - t35 / f64x8::splat(30.0) + t38 / f64x8::splat(70.0) - t41 / f64x8::splat(135.0) + t44 / f64x8::splat(231.0) - t47 / f64x8::splat(364.0) + t50 / f64x8::splat(540.0) - t53 / f64x8::splat(765.0) + t56 / f64x8::splat(1045.0) - t59 / f64x8::splat(1386.0) + t62 / f64x8::splat(1794.0) - t65 / f64x8::splat(2275.0) + t68 / f64x8::splat(2835.0) - t71 / f64x8::splat(3480.0) + t74 / f64x8::splat(4216.0) - t77 / f64x8::splat(5049.0) + t80 / f64x8::splat(5985.0) - t83 / f64x8::splat(7030.0);
            let t86 = ((t29).select(f64x8::splat(1.92), t27));
            let t87 = (simd::atan2(f64x8::splat(1.0), t86));
            let t88 = t86 * t86;
            let t89 = t88 + f64x8::splat(3.0);
            let t90 = f64x8::splat(1.0) / t88;
            let t91 = f64x8::splat(1.0) + t90;
            let t92 = (simd::ln(t91));
            let t94 = -t92 * t89 + f64x8::splat(1.0);
            let t97 = t87 + t94 * t86 / f64x8::splat(4.0);
            let t101 = ((t28).select(t85, f64x8::splat(1.0) - f64x8::splat(8.0) / f64x8::splat(3.0) * t97 * t86));
            let t105 = f64x8::splat(3.0) / f64x8::splat(16.0) * t101 * t15 * t14 * t7;
            let t107 = t21 * t6 * t4;
            let t109 = f64x8::splat(1.0) + f64x8::splat(0.053425) * t107;
            let t110 = ((t107).sqrt());
            let t113 = ((t107) * (t107).sqrt());
            let t115 = t1 * t1;
            let t116 = t18 * t115;
            let t117 = t15 * t15;
            let t118 = f64x8::splat(1.0) / t117;
            let t120 = t118 * t5 * t116;
            let t122 = f64x8::splat(3.79785) * t110 + f64x8::splat(0.8969) * t107 + f64x8::splat(0.204775) * t113 + f64x8::splat(0.123235) * t120;
            let t125 = f64x8::splat(1.0) + f64x8::splat(16.081979498692537) / t122;
            let t126 = (simd::ln(t125));
            let t134 = f64x8::splat(1.0) / (f64x8::splat(2.0) * t8 - f64x8::splat(2.0)) * (f64x8::splat(2.0) * t13 - f64x8::splat(2.0));
            let t136 = f64x8::splat(1.0) + f64x8::splat(0.0278125) * t107;
            let t141 = f64x8::splat(5.1785) * t110 + f64x8::splat(0.905775) * t107 + f64x8::splat(0.1100325) * t113 + f64x8::splat(0.1241775) * t120;
            let t144 = f64x8::splat(1.0) + f64x8::splat(29.608749977793437) / t141;
            let t145 = (simd::ln(t144));
            let t149 = -f64x8::splat(0.0621814) * t126 * t109 + f64x8::splat(0.0197516734986138) * t145 * t136 * t134;
            let t152 = f64x8::splat(3.2) - f64x8::splat(0.225) * t107 + t120 / f64x8::splat(4.0);
            let t153 = f64x8::splat(1.0) / t152;
            let t155 = f64x8::splat(3.4602) * t153 * t149;
            let tzk0 = -t105 + t155;
            acc_zk = tzk0;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        ip += 8;
    }
}
