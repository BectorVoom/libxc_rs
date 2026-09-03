//! GGA_K_OL1 vxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_k_ol1.c`
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
pub fn gga_k_ol1_vxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
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
        let v_sigma = load(sigma, ip, np);
        let mut acc_zk = V_ZERO;
        let mut acc_vrho = V_ZERO;
        let mut acc_vsigma = V_ZERO;
        {
            let t2 = (v_rho / f64x8::splat(2.0)).simd_le(dens_threshold);
            let t3 = f64x8::splat(M_CBRT3);
            let t4 = t3 * t3;
            let t5 = f64x8::splat(M_CBRTPI);
            let t7 = t4 * t5 * f64x8::splat(M_PI);
            let t8 = (f64x8::splat(1.0)).simd_le(zeta_threshold);
            let t9 = zeta_threshold - f64x8::splat(1.0);
            let t11 = ((t8).select(t9, (t8).select(-t9, f64x8::splat(0.0))));
            let t12 = f64x8::splat(1.0) + t11;
            let t14 = (simd::cbrt(zeta_threshold));
            let t15 = t14 * t14;
            let t17 = (simd::cbrt(t12));
            let t18 = t17 * t17;
            let t20 = (((t12).simd_le(zeta_threshold)).select(t15 * zeta_threshold, t18 * t12));
            let t21 = (simd::cbrt(v_rho));
            let t22 = t21 * t21;
            let t24 = f64x8::splat(M_CBRT2);
            let t25 = t24 * t24;
            let t26 = v_sigma * t25;
            let t27 = v_rho * v_rho;
            let t29 = f64x8::splat(1.0) / t22 / t27;
            let t32 = ((v_sigma).sqrt());
            let t33 = t25 * t32;
            let t35 = f64x8::splat(1.0) / t21 / v_rho;
            let t39 = f64x8::splat(M_CBRT6);
            let t41 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t42 = (simd::cbrt(t41));
            let t43 = t42 * t42;
            let t44 = f64x8::splat(1.0) / t43;
            let t47 = f64x8::splat(1.0) + f64x8::splat(5.0) / f64x8::splat(9.0) * (t26 * t29 / f64x8::splat(72.0) + f64x8::splat(0.00677) * t33 * t35) * t39 * t44;
            let t51 = ((t2).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t7 * t20 * t22 * t47));
            let tzk0 = f64x8::splat(2.0) * t51;
            acc_zk = tzk0;
            let t52 = f64x8::splat(1.0) / t21;
            let t57 = t7 * t20;
            let t58 = t27 * v_rho;
            let t60 = f64x8::splat(1.0) / t22 / t58;
            let t64 = f64x8::splat(1.0) / t21 / t27;
            let t67 = -t26 * t60 / f64x8::splat(27.0) - f64x8::splat(0.009026666666666667) * t33 * t64;
            let t69 = t39 * t44;
            let t74 = ((t2).select(f64x8::splat(0.0), t7 * t20 * t52 * t47 / f64x8::splat(10.0) + t57 * t22 * t67 * t69 / f64x8::splat(12.0)));
            let tvrho0 = f64x8::splat(2.0) * v_rho * t74 + f64x8::splat(2.0) * t51;
            acc_vrho = tvrho0;
            let t80 = t25 / t32;
            let t83 = t25 * t29 / f64x8::splat(72.0) + f64x8::splat(0.003385) * t80 * t35;
            let t88 = ((t2).select(f64x8::splat(0.0), t57 * t22 * t83 * t69 / f64x8::splat(12.0)));
            let tvsigma0 = f64x8::splat(2.0) * v_rho * t88;
            acc_vsigma = tvsigma0;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_vrho.into(); vrho[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_vsigma.into(); vsigma[ip..ip + m].copy_from_slice(&a[..m]); }
        ip += 8;
    }
}
