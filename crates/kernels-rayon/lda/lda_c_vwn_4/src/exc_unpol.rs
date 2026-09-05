//! LDA_C_VWN_4 exc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_c_vwn_4.c`
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
pub fn lda_c_vwn_4_exc_unpol(
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
            let t59 = f64x8::splat(M_CBRT2);
            let t60 = t59 - f64x8::splat(1.0);
            let t65 = f64x8::splat(9.0) * t56 - f64x8::splat(9.0);
            let t67 = t33 * (t40 + f64x8::splat(0.32323836906055065) * t44 + f64x8::splat(0.021608710360898266) * t49) * t65 / f64x8::splat(24.0);
            let tzk0 = t20 + t25 + t31 - t67;
            acc_zk = tzk0;
        }
        store_add(zk, ip, m, acc_zk);
        ip += 8;
    }
}
