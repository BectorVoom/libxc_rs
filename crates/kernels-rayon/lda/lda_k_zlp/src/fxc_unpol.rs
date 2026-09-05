//! LDA_K_ZLP fxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_k_zlp.c`
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
pub fn lda_k_zlp_fxc_unpol(
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
            let t2 = t1 * t1;
            let t4 = (simd::cbrt(f64x8::splat(1.0) / f64x8::splat(M_PI)));
            let t5 = f64x8::splat(1.0) / t4;
            let t7 = f64x8::splat(M_CBRT4);
            let t8 = t2 * t5 * t7;
            let t10 = (simd::cbrt(zeta_threshold));
            let t11 = t10 * t10;
            let t13 = (((f64x8::splat(1.0)).simd_le(zeta_threshold)).select(t11 * zeta_threshold, f64x8::splat(1.0)));
            let t14 = (simd::cbrt(v_rho));
            let t15 = t14 * t14;
            let t16 = t13 * t15;
            let t17 = f64x8::splat(1.0) / t14;
            let t19 = f64x8::splat(1.0) + f64x8::splat(510.2040816326531) * t17;
            let t20 = (simd::ln(t19));
            let t23 = f64x8::splat(1.0) - f64x8::splat(0.00196) * t14 * t20;
            let t25 = t8 * t16 * t23;
            let tzk0 = f64x8::splat(1.0790666666666666) * t25;
            acc_zk = tzk0;
            let t27 = t15 * v_rho;
            let t29 = t27 * t2 * t5;
            let t30 = t7 * t13;
            let t35 = f64x8::splat(1.0) / t19;
            let t38 = -f64x8::splat(0.0006533333333333333) / t15 * t20 + f64x8::splat(0.3333333333333333) / v_rho * t35;
            let tvrho0 = f64x8::splat(1.7984444444444445) * t25 + f64x8::splat(1.0790666666666666) * t29 * t30 * t38;
            acc_vrho = tvrho0;
            let t42 = t13 * t17;
            let t52 = v_rho * v_rho;
            let t57 = f64x8::splat(1.0) / t14 / t52;
            let t58 = t19 * t19;
            let t59 = f64x8::splat(1.0) / t58;
            let t62 = f64x8::splat(0.00043555555555555557) / t27 * t20 - f64x8::splat(0.2222222222222222) / t52 * t35 + f64x8::splat(56.68934240362812) * t57 * t59;
            let tv2rho20 = f64x8::splat(1.198962962962963) * t8 * t42 * t23 + f64x8::splat(3.596888888888889) * t8 * t16 * t38 + f64x8::splat(1.0790666666666666) * t29 * t30 * t62;
            acc_v2rho2 = tv2rho20;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_vrho.into(); vrho[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v2rho2.into(); v2rho2[ip..ip + m].copy_from_slice(&a[..m]); }
        ip += 8;
    }
}
