//! LDA_C_VWN_RPA exc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_c_vwn_rpa.c`
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
pub fn lda_c_vwn_rpa_exc_unpol(
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
            let t14 = t11 + f64x8::splat(6.536) * t12 + f64x8::splat(42.7198);
            let t15 = f64x8::splat(1.0) / t14;
            let t19 = (simd::ln(t4 * t9 * t15 / f64x8::splat(4.0)));
            let t21 = t12 + f64x8::splat(13.072);
            let t24 = (simd::atan(f64x8::splat(0.0448998886412873) / t21));
            let t26 = t12 / f64x8::splat(2.0);
            let t27 = t26 + f64x8::splat(0.409286);
            let t28 = t27 * t27;
            let t30 = (simd::ln(t28 * t15));
            let t34 = (simd::cbrt(zeta_threshold));
            let t36 = (((f64x8::splat(1.0)).simd_le(zeta_threshold)).select(t34 * zeta_threshold, f64x8::splat(1.0)));
            let t38 = f64x8::splat(2.0) * t36 - f64x8::splat(2.0);
            let t39 = f64x8::splat(M_CBRT2);
            let t42 = f64x8::splat(1.0) / (f64x8::splat(2.0) * t39 - f64x8::splat(2.0));
            let t44 = -t38 * t42 + f64x8::splat(1.0);
            let t45 = (f64x8::splat(0.0310907) * t19 + f64x8::splat(20.521972937837504) * t24 + f64x8::splat(0.004431373767749538) * t30) * t44;
            let t47 = t11 + f64x8::splat(10.06155) * t12 + f64x8::splat(101.578);
            let t48 = f64x8::splat(1.0) / t47;
            let t52 = (simd::ln(t4 * t9 * t48 / f64x8::splat(4.0)));
            let t54 = t12 + f64x8::splat(20.1231);
            let t57 = (simd::atan(f64x8::splat(1.171685277708993) / t54));
            let t59 = t26 + f64x8::splat(0.743294);
            let t60 = t59 * t59;
            let t62 = (simd::ln(t60 * t48));
            let t66 = (f64x8::splat(0.01554535) * t52 + f64x8::splat(0.6188180297906063) * t57 + f64x8::splat(0.002667310007273315) * t62) * t38 * t42;
            let tzk0 = t45 + t66;
            acc_zk = tzk0;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        ip += 8;
    }
}
