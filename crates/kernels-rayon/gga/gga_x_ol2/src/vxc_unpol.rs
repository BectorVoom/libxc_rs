//! GGA_X_OL2 vxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_ol2.c`
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
pub fn gga_x_ol2_vxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    param_bb: f64,
    param_cc: f64,
    param_aa: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_bb = f64x8::splat(param_bb);
    let param_cc = f64x8::splat(param_cc);
    let param_aa = f64x8::splat(param_aa);
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
            let t4 = f64x8::splat(M_CBRTPI);
            let t6 = t3 / t4;
            let t7 = (f64x8::splat(1.0)).simd_le(zeta_threshold);
            let t8 = zeta_threshold - f64x8::splat(1.0);
            let t10 = ((t7).select(t8, (t7).select(-t8, f64x8::splat(0.0))));
            let t11 = f64x8::splat(1.0) + t10;
            let t13 = (simd::cbrt(zeta_threshold));
            let t15 = (simd::cbrt(t11));
            let t17 = (((t11).simd_le(zeta_threshold)).select(t13 * zeta_threshold, t15 * t11));
            let t18 = (simd::cbrt(v_rho));
            let t19 = t17 * t18;
            let t20 = param_bb * v_sigma;
            let t21 = f64x8::splat(M_CBRT2);
            let t22 = t21 * t21;
            let t23 = v_rho * v_rho;
            let t24 = t18 * t18;
            let t26 = f64x8::splat(1.0) / t24 / t23;
            let t27 = t22 * t26;
            let t30 = ((v_sigma).sqrt());
            let t31 = param_cc * t30;
            let t33 = f64x8::splat(1.0) / t18 / v_rho;
            let t38 = f64x8::splat(4.0) * t30 * t21 * t33 + t21;
            let t39 = f64x8::splat(1.0) / t38;
            let t40 = t21 * t33 * t39;
            let t42 = param_aa + f64x8::splat(0.013888888888888888) * t20 * t27 + t31 * t40;
            let t46 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t42));
            let tzk0 = f64x8::splat(2.0) * t46;
            acc_zk = tzk0;
            let t48 = t17 / t24;
            let t52 = t23 * v_rho;
            let t54 = f64x8::splat(1.0) / t24 / t52;
            let t55 = t22 * t54;
            let t61 = t21 / t18 / t23 * t39;
            let t64 = param_cc * v_sigma;
            let t65 = t38 * t38;
            let t66 = f64x8::splat(1.0) / t65;
            let t67 = t55 * t66;
            let t70 = -f64x8::splat(0.037037037037037035) * t20 * t55 - f64x8::splat(4.0) / f64x8::splat(3.0) * t31 * t61 + f64x8::splat(16.0) / f64x8::splat(3.0) * t64 * t67;
            let t75 = ((t2).select(f64x8::splat(0.0), -t6 * t48 * t42 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t70));
            let tvrho0 = f64x8::splat(2.0) * v_rho * t75 + f64x8::splat(2.0) * t46;
            acc_vrho = tvrho0;
            let t78 = param_bb * t22;
            let t81 = f64x8::splat(1.0) / t30;
            let t82 = param_cc * t81;
            let t85 = param_cc * t22;
            let t89 = f64x8::splat(0.013888888888888888) * t78 * t26 + t82 * t40 / f64x8::splat(2.0) - f64x8::splat(2.0) * t85 * t26 * t66;
            let t93 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t89));
            let tvsigma0 = f64x8::splat(2.0) * v_rho * t93;
            acc_vsigma = tvsigma0;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_vrho.into(); vrho[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_vsigma.into(); vsigma[ip..ip + m].copy_from_slice(&a[..m]); }
        ip += 8;
    }
}
