//! LDA_XC_ZLP fxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_xc_zlp.c`
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
pub fn lda_xc_zlp_fxc_unpol(
    rho: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    v2rho2: &mut [f64],
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
        let mut acc_vrho = V_ZERO;
        let mut acc_v2rho2 = V_ZERO;
        {
            let t1 = (simd::cbrt(v_rho));
            let t4 = f64x8::splat(1.0) + f64x8::splat(105.5562709925034) / t1;
            let t5 = (simd::ln(t4));
            let t8 = f64x8::splat(1.0) - f64x8::splat(0.00947362) * t5 * t1;
            let t9 = t8 * t1;
            let tzk0 = -f64x8::splat(0.93222) * t9;
            acc_zk = tzk0;
            let t12 = t1 * v_rho;
            let t14 = f64x8::splat(1.0) / t4;
            let t17 = t1 * t1;
            let t18 = f64x8::splat(1.0) / t17;
            let t21 = f64x8::splat(0.3333333333333333) / v_rho * t14 - f64x8::splat(0.0031578733333333334) * t5 * t18;
            let tvrho0 = -f64x8::splat(1.24296) * t9 - f64x8::splat(0.93222) * t12 * t21;
            acc_vrho = tvrho0;
            let t28 = v_rho * v_rho;
            let t34 = t4 * t4;
            let t35 = f64x8::splat(1.0) / t34;
            let t39 = f64x8::splat(1.0) / t17 / v_rho;
            let t42 = -f64x8::splat(0.2222222222222222) / t28 * t14 + f64x8::splat(11.728474554722599) / t1 / t28 * t35 + f64x8::splat(0.002105248888888889) * t5 * t39;
            let tv2rho20 = -f64x8::splat(2.48592) * t21 * t1 - f64x8::splat(0.41432) * t8 * t18 - f64x8::splat(0.93222) * t12 * t42;
            acc_v2rho2 = tv2rho20;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_vrho.into(); vrho[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v2rho2.into(); v2rho2[ip..ip + m].copy_from_slice(&a[..m]); }
        ip += 8;
    }
}
