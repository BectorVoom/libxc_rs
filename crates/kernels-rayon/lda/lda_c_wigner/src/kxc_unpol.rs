//! LDA_C_WIGNER kxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_c_wigner.c`
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
pub fn lda_c_wigner_kxc_unpol(
    rho: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    v2rho2: &mut [f64],
    v3rho3: &mut [f64],
    param_a: f64,
    param_b: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_a = f64x8::splat(param_a);
    let param_b = f64x8::splat(param_b);
    let dens_threshold = f64x8::splat(dens_threshold);
    let zeta_threshold = f64x8::splat(zeta_threshold);
    let mut ip = 0usize;
    while ip < np {
        let m = (np - ip).min(8);
        let v_rho = load(rho, ip, np);
        let mut acc_zk = V_ZERO;
        let mut acc_vrho = V_ZERO;
        let mut acc_v2rho2 = V_ZERO;
        let mut acc_v3rho3 = V_ZERO;
        {
            let t1 = f64x8::splat(M_CBRT3);
            let t2 = f64x8::splat(1.0) / f64x8::splat(M_PI);
            let t3 = (simd::cbrt(t2));
            let t4 = t1 * t3;
            let t5 = f64x8::splat(M_CBRT4);
            let t6 = t5 * t5;
            let t7 = (simd::cbrt(v_rho));
            let t8 = f64x8::splat(1.0) / t7;
            let t12 = param_b + t4 * t6 * t8 / f64x8::splat(4.0);
            let tzk0 = param_a / t12;
            acc_zk = tzk0;
            let t15 = t12 * t12;
            let t16 = f64x8::splat(1.0) / t15;
            let tvrho0 = tzk0 + t8 * param_a * t16 * t4 * t6 / f64x8::splat(12.0);
            acc_vrho = tvrho0;
            let t22 = param_a * t16 * t1;
            let t23 = t3 * t6;
            let t28 = t7 * t7;
            let t33 = f64x8::splat(1.0) / t15 / t12;
            let t35 = t1 * t1;
            let t36 = t3 * t3;
            let tv2rho20 = t22 * t23 / t7 / v_rho / f64x8::splat(18.0) + f64x8::splat(1.0) / t28 / v_rho * param_a * t33 * t35 * t36 * t5 / f64x8::splat(18.0);
            acc_v2rho2 = tv2rho20;
            let t42 = param_a * t33 * t35;
            let t43 = t36 * t5;
            let t44 = v_rho * v_rho;
            let t55 = t44 * v_rho;
            let t58 = t15 * t15;
            let t59 = f64x8::splat(1.0) / t58;
            let tv3rho30 = -t42 * t43 / t28 / t44 / f64x8::splat(18.0) - f64x8::splat(2.0) / f64x8::splat(27.0) * t22 * t23 / t7 / t44 + f64x8::splat(1.0) / t55 * param_a * t59 * t2 / f64x8::splat(6.0);
            acc_v3rho3 = tv3rho30;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_vrho.into(); vrho[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v2rho2.into(); v2rho2[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v3rho3.into(); v3rho3[ip..ip + m].copy_from_slice(&a[..m]); }
        ip += 8;
    }
}
