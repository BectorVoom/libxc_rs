//! LDA_C_VWN_3 exc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_c_vwn_3.c`
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
pub fn lda_c_vwn_3_exc_unpol(
    rho: &[f64],
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
        let mut acc_zk = V_ZERO;
        {
            let t1 = f64x8::splat(M_CBRT3);
            let t2 = f64x8::splat(1.0) / f64x8::splat(M_PI);
            let t3 = (simd::cbrt(t2));
            let t4 = t1 * t3;
            let t5 = f64x8::splat(M_CBRT4);
            let t6 = t5 * t5;
            let t7 = (simd::cbrt(v_rho));
            let t8 = f64x8::splat(1.0) / t7;
            let t9 = t6 * t8;
            let t10 = t4 * t9;
            let t11 = t10 / f64x8::splat(4.0);
            let t12 = ((t10).sqrt());
            let t14 = t11 + f64x8::splat(1.86372) * t12 + f64x8::splat(12.9352);
            let t15 = f64x8::splat(1.0) / t14;
            let t19 = (simd::ln(t4 * t9 * t15 / f64x8::splat(4.0)));
            let t20 = f64x8::splat(0.0310907) * t19;
            let t21 = t12 + f64x8::splat(3.72744);
            let t24 = (simd::atan(f64x8::splat(6.15199081975908) / t21));
            let t25 = f64x8::splat(0.038783294878113016) * t24;
            let t26 = t12 / f64x8::splat(2.0);
            let t27 = t26 + f64x8::splat(0.10498);
            let t28 = t27 * t27;
            let t30 = (simd::ln(t28 * t15));
            let t31 = f64x8::splat(0.0009690227711544374) * t30;
            let t33 = t11 + f64x8::splat(3.53021) * t12 + f64x8::splat(18.0578);
            let t34 = f64x8::splat(1.0) / t33;
            let t38 = (simd::ln(t4 * t9 * t34 / f64x8::splat(4.0)));
            let t40 = t12 + f64x8::splat(7.06042);
            let t43 = (simd::atan(f64x8::splat(4.730926909560113) / t40));
            let t45 = t26 + f64x8::splat(0.325);
            let t46 = t45 * t45;
            let t48 = (simd::ln(t46 * t34));
            let t50 = f64x8::splat(0.01554535) * t38 + f64x8::splat(0.05249139316978094) * t43 + f64x8::splat(0.0022478670955426118) * t48 - t20 - t25 - t31;
            let t52 = t11 + f64x8::splat(10.06155) * t12 + f64x8::splat(101.578);
            let t53 = f64x8::splat(1.0) / t52;
            let t57 = (simd::ln(t4 * t9 * t53 / f64x8::splat(4.0)));
            let t59 = t12 + f64x8::splat(20.1231);
            let t62 = (simd::atan(f64x8::splat(1.171685277708993) / t59));
            let t64 = t26 + f64x8::splat(0.743294);
            let t65 = t64 * t64;
            let t67 = (simd::ln(t65 * t53));
            let t70 = t11 + f64x8::splat(6.536) * t12 + f64x8::splat(42.7198);
            let t71 = f64x8::splat(1.0) / t70;
            let t75 = (simd::ln(t4 * t9 * t71 / f64x8::splat(4.0)));
            let t77 = t12 + f64x8::splat(13.072);
            let t80 = (simd::atan(f64x8::splat(0.0448998886412873) / t77));
            let t82 = t26 + f64x8::splat(0.409286);
            let t83 = t82 * t82;
            let t85 = (simd::ln(t83 * t71));
            let t87 = f64x8::splat(0.01554535) * t57 + f64x8::splat(0.6188180297906063) * t62 + f64x8::splat(0.002667310007273315) * t67 - f64x8::splat(0.0310907) * t75 - f64x8::splat(20.521972937837504) * t80 - f64x8::splat(0.004431373767749538) * t85;
            let t88 = f64x8::splat(1.0) / t87;
            let t90 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t91 = f64x8::splat(1.0) / t90;
            let t92 = t50 * t88 * t91;
            let t94 = t11 + f64x8::splat(0.534175) * t12 + f64x8::splat(11.4813);
            let t95 = f64x8::splat(1.0) / t94;
            let t99 = (simd::ln(t4 * t9 * t95 / f64x8::splat(4.0)));
            let t100 = t12 + f64x8::splat(1.06835);
            let t103 = (simd::atan(f64x8::splat(6.692072046645942) / t100));
            let t105 = t26 + f64x8::splat(0.228344);
            let t106 = t105 * t105;
            let t108 = (simd::ln(t106 * t95));
            let t110 = t99 + f64x8::splat(0.32323836906055065) * t103 + f64x8::splat(0.021608710360898266) * t108;
            let t112 = (simd::cbrt(zeta_threshold));
            let t114 = (((f64x8::splat(1.0)).simd_le(zeta_threshold)).select(t112 * zeta_threshold, f64x8::splat(1.0)));
            let t116 = f64x8::splat(2.0) * t114 - f64x8::splat(2.0);
            let t118 = f64x8::splat(M_CBRT2);
            let t119 = t118 - f64x8::splat(1.0);
            let t121 = f64x8::splat(1.0) / t119 / f64x8::splat(2.0);
            let t122 = f64x8::splat(9.0) * t119;
            let t123 = t121 * t122;
            let t124 = t110 * t116 * t123;
            let t126 = t92 * t124 / f64x8::splat(24.0);
            let tzk0 = t20 + t25 + t31 - t126;
            acc_zk = tzk0;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        ip += 8;
    }
}
