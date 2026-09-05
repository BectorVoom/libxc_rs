//! LDA_C_VWN_2 exc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_c_vwn_2.c`
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

/// Accumulate 8 consecutive grid points into an output array.
///
/// `+=`, not `=`. The scalar kernel writes `out[ip] += v`; a plain store is a
/// different operation in two ways. It keeps the sign of a negative zero where
/// `0.0 + -0.0` gives `+0.0` -- a bit difference the fingerprint gate reports
/// as a rejection even though no value changed (`gga_x_pbepow fxc` was
/// rejected on exactly this, 273 of 200,000 `v2sigma2` elements) -- and it
/// would discard whatever a caller had already put in the buffer.
#[inline(always)]
fn store_add(s: &mut [f64], ip: usize, m: usize, acc: f64x8) {
    let a: [f64; 8] = acc.into();
    if m == 8 {
        let mut b = [0.0f64; 8];
        b.copy_from_slice(&s[ip..ip + 8]);
        let r: [f64; 8] = (f64x8::new(b) + acc).into();
        s[ip..ip + 8].copy_from_slice(&r);
    } else {
        for k in 0..m {
            s[ip + k] += a[k];
        }
    }
}

#[allow(unused_variables, non_snake_case)]
pub fn lda_c_vwn_2_exc_unpol(
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
            let t32 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t33 = f64x8::splat(1.0) / t32;
            let t35 = t11 + f64x8::splat(0.534175) * t12 + f64x8::splat(11.4813);
            let t36 = f64x8::splat(1.0) / t35;
            let t40 = (simd::ln(t4 * t9 * t36 / f64x8::splat(4.0)));
            let t41 = t12 + f64x8::splat(1.06835);
            let t44 = (simd::atan(f64x8::splat(6.692072046645942) / t41));
            let t46 = t26 + f64x8::splat(0.228344);
            let t47 = t46 * t46;
            let t49 = (simd::ln(t47 * t36));
            let t54 = (simd::cbrt(zeta_threshold));
            let t56 = (((f64x8::splat(1.0)).simd_le(zeta_threshold)).select(t54 * zeta_threshold, f64x8::splat(1.0)));
            let t58 = f64x8::splat(2.0) * t56 - f64x8::splat(2.0);
            let t59 = f64x8::splat(M_CBRT2);
            let t60 = t59 - f64x8::splat(1.0);
            let t62 = f64x8::splat(1.0) / t60 / f64x8::splat(2.0);
            let t65 = f64x8::splat(9.0) * t58 * t62 * t60;
            let t67 = t33 * (t40 + f64x8::splat(0.32323836906055065) * t44 + f64x8::splat(0.021608710360898266) * t49) * t65 / f64x8::splat(24.0);
            let t69 = t11 + f64x8::splat(10.06155) * t12 + f64x8::splat(101.578);
            let t70 = f64x8::splat(1.0) / t69;
            let t74 = (simd::ln(t4 * t9 * t70 / f64x8::splat(4.0)));
            let t76 = t12 + f64x8::splat(20.1231);
            let t79 = (simd::atan(f64x8::splat(1.171685277708993) / t76));
            let t81 = t26 + f64x8::splat(0.743294);
            let t82 = t81 * t81;
            let t84 = (simd::ln(t82 * t70));
            let t87 = t11 + f64x8::splat(6.536) * t12 + f64x8::splat(42.7198);
            let t88 = f64x8::splat(1.0) / t87;
            let t92 = (simd::ln(t4 * t9 * t88 / f64x8::splat(4.0)));
            let t94 = t12 + f64x8::splat(13.072);
            let t97 = (simd::atan(f64x8::splat(0.0448998886412873) / t94));
            let t99 = t26 + f64x8::splat(0.409286);
            let t100 = t99 * t99;
            let t102 = (simd::ln(t100 * t88));
            let t106 = (f64x8::splat(0.01554535) * t74 + f64x8::splat(0.6188180297906063) * t79 + f64x8::splat(0.002667310007273315) * t84 - f64x8::splat(0.0310907) * t92 - f64x8::splat(20.521972937837504) * t97 - f64x8::splat(0.004431373767749538) * t102) * t58 * t62;
            let t108 = t11 + f64x8::splat(3.53021) * t12 + f64x8::splat(18.0578);
            let t109 = f64x8::splat(1.0) / t108;
            let t113 = (simd::ln(t4 * t9 * t109 / f64x8::splat(4.0)));
            let t115 = t12 + f64x8::splat(7.06042);
            let t118 = (simd::atan(f64x8::splat(4.730926909560113) / t115));
            let t120 = t26 + f64x8::splat(0.325);
            let t121 = t120 * t120;
            let t123 = (simd::ln(t121 * t109));
            let t127 = (f64x8::splat(0.01554535) * t113 + f64x8::splat(0.05249139316978094) * t118 + f64x8::splat(0.0022478670955426118) * t123 - t20 - t25 - t31) * t58 * t62;
            let tzk0 = t20 + t25 + t31 - t67 - t106 + t127;
            acc_zk = tzk0;
        }
        store_add(zk, ip, m, acc_zk);
        ip += 8;
    }
}
