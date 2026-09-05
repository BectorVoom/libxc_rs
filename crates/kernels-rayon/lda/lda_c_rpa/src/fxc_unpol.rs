//! LDA_C_RPA fxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_c_rpa.c`
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
pub fn lda_c_rpa_fxc_unpol(
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
            let t1 = f64x8::splat(M_CBRT3);
            let t3 = (simd::cbrt(f64x8::splat(1.0) / f64x8::splat(M_PI)));
            let t4 = t1 * t3;
            let t5 = f64x8::splat(M_CBRT4);
            let t6 = t5 * t5;
            let t7 = (simd::cbrt(v_rho));
            let t9 = t6 / t7;
            let t10 = t4 * t9;
            let t12 = (simd::ln(t10 / f64x8::splat(4.0)));
            let t13 = f64x8::splat(0.0311) * t12;
            let t16 = f64x8::splat(0.00225) * t4 * t9 * t12;
            let t17 = f64x8::splat(0.00425) * t10;
            let tzk0 = t13 - f64x8::splat(0.048) + t16 - t17;
            acc_zk = tzk0;
            let t18 = f64x8::splat(1.0) / v_rho;
            let t22 = t6 / t7 / v_rho;
            let t24 = t4 * t22 * t12;
            let t26 = t4 * t22;
            let tvrho0 = t13 - f64x8::splat(0.048) + t16 - t17 + v_rho * (-f64x8::splat(0.010366666666666666) * t18 - f64x8::splat(0.00075) * t24 + f64x8::splat(0.0006666666666666666) * t26);
            acc_vrho = tvrho0;
            let t33 = v_rho * v_rho;
            let t34 = f64x8::splat(1.0) / t33;
            let t38 = t6 / t7 / t33;
            let t40 = t4 * t38 * t12;
            let t42 = t4 * t38;
            let tv2rho20 = -f64x8::splat(0.020733333333333333) * t18 - f64x8::splat(0.0015) * t24 + f64x8::splat(0.0013333333333333333) * t26 + v_rho * (f64x8::splat(0.010366666666666666) * t34 + f64x8::splat(0.001) * t40 - f64x8::splat(0.0006388888888888889) * t42);
            acc_v2rho2 = tv2rho20;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_vrho.into(); vrho[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v2rho2.into(); v2rho2[ip..ip + m].copy_from_slice(&a[..m]); }
        ip += 8;
    }
}
