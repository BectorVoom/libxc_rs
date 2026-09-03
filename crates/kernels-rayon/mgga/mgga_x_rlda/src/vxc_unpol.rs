//! MGGA_X_RLDA vxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_rlda.c`
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
pub fn mgga_x_rlda_vxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    vlapl: &mut [f64],
    vtau: &mut [f64],
    param_prefactor: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_prefactor = f64x8::splat(param_prefactor);
    let dens_threshold = f64x8::splat(dens_threshold);
    let zeta_threshold = f64x8::splat(zeta_threshold);
    let mut ip = 0usize;
    while ip < np {
        let m = (np - ip).min(8);
        let v_rho = load(rho, ip, np);
        let v_sigma = load(sigma, ip, np);
        let v_lapl = load(lapl, ip, np);
        let v_tau = load(tau, ip, np);
        let mut acc_zk = V_ZERO;
        let mut acc_vrho = V_ZERO;
        let mut acc_vsigma = V_ZERO;
        let mut acc_vlapl = V_ZERO;
        let mut acc_vtau = V_ZERO;
        {
            let t3 = (v_rho / f64x8::splat(2.0)).simd_le(dens_threshold);
            let t4 = f64x8::splat(M_CBRTPI);
            let t5 = t4 * t4;
            let t6 = (f64x8::splat(1.0)).simd_le(zeta_threshold);
            let t7 = zeta_threshold - f64x8::splat(1.0);
            let t9 = ((t6).select(t7, (t6).select(-t7, f64x8::splat(0.0))));
            let t10 = f64x8::splat(1.0) + t9;
            let t12 = (simd::cbrt(zeta_threshold));
            let t14 = (simd::cbrt(t10));
            let t16 = (((t10).simd_le(zeta_threshold)).select(t12 * zeta_threshold, t14 * t10));
            let t17 = t5 * t16;
            let t18 = (simd::cbrt(v_rho));
            let t21 = (simd::cbrt(f64x8::splat(1.0) / f64x8::splat(M_PI)));
            let t22 = f64x8::splat(1.0) / t21;
            let t23 = param_prefactor * t22;
            let t24 = f64x8::splat(M_CBRT4);
            let t25 = f64x8::splat(M_CBRT2);
            let t26 = t25 * t25;
            let t27 = v_tau * t26;
            let t28 = t18 * t18;
            let t30 = f64x8::splat(1.0) / t28 / v_rho;
            let t33 = v_lapl * t26;
            let t36 = f64x8::splat(2.0) * t27 * t30 - t33 * t30 / f64x8::splat(4.0);
            let t39 = t23 * t24 / t36;
            let t42 = ((t3).select(f64x8::splat(0.0), -f64x8::splat(15.0) / f64x8::splat(16.0) * t17 * t18 * t39));
            let tzk0 = f64x8::splat(2.0) * t42;
            acc_zk = tzk0;
            let t43 = f64x8::splat(1.0) / t28;
            let t48 = t17 * t18 * param_prefactor;
            let t49 = t22 * t24;
            let t50 = t36 * t36;
            let t51 = f64x8::splat(1.0) / t50;
            let t52 = v_rho * v_rho;
            let t54 = f64x8::splat(1.0) / t28 / t52;
            let t59 = -f64x8::splat(10.0) / f64x8::splat(3.0) * t27 * t54 + f64x8::splat(5.0) / f64x8::splat(12.0) * t33 * t54;
            let t61 = t49 * t51 * t59;
            let t65 = ((t3).select(f64x8::splat(0.0), -f64x8::splat(5.0) / f64x8::splat(16.0) * t17 * t43 * t39 + f64x8::splat(15.0) / f64x8::splat(16.0) * t48 * t61));
            let tvrho0 = f64x8::splat(2.0) * v_rho * t65 + f64x8::splat(2.0) * t42;
            acc_vrho = tvrho0;
            let tvsigma0 = f64x8::splat(0.0);
            acc_vsigma = tvsigma0;
            let t70 = f64x8::splat(1.0) / t18 / v_rho * param_prefactor;
            let t71 = t17 * t70;
            let t73 = t49 * t51 * t26;
            let t74 = t71 * t73;
            let t76 = ((t3).select(f64x8::splat(0.0), -f64x8::splat(15.0) / f64x8::splat(64.0) * t74));
            let tvlapl0 = f64x8::splat(2.0) * v_rho * t76;
            acc_vlapl = tvlapl0;
            let t79 = ((t3).select(f64x8::splat(0.0), f64x8::splat(15.0) / f64x8::splat(8.0) * t74));
            let tvtau0 = f64x8::splat(2.0) * v_rho * t79;
            acc_vtau = tvtau0;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_vrho.into(); vrho[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_vsigma.into(); vsigma[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_vlapl.into(); vlapl[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_vtau.into(); vtau[ip..ip + m].copy_from_slice(&a[..m]); }
        ip += 8;
    }
}
