//! GGA_X_B88 vxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_b88.c`
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
pub fn gga_x_b88_vxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    param_beta: f64,
    param_gamma: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_beta = f64x8::splat(param_beta);
    let param_gamma = f64x8::splat(param_gamma);
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
            let t20 = t3 * t3;
            let t21 = param_beta * t20;
            let t23 = (simd::cbrt(f64x8::splat(1.0) / f64x8::splat(M_PI)));
            let t24 = f64x8::splat(1.0) / t23;
            let t25 = f64x8::splat(M_CBRT4);
            let t26 = t24 * t25;
            let t27 = t21 * t26;
            let t28 = f64x8::splat(M_CBRT2);
            let t29 = t28 * t28;
            let t30 = v_sigma * t29;
            let t31 = v_rho * v_rho;
            let t32 = t18 * t18;
            let t34 = f64x8::splat(1.0) / t32 / t31;
            let t35 = param_gamma * param_beta;
            let t36 = ((v_sigma).sqrt());
            let t37 = t35 * t36;
            let t39 = f64x8::splat(1.0) / t18 / v_rho;
            let t43 = (simd::ln(t36 * t28 * t39 + ((((t36 * t28 * t39) * (t36 * t28 * t39)) + f64x8::splat(1.0)).sqrt())));
            let t44 = t28 * t39 * t43;
            let t46 = t37 * t44 + f64x8::splat(1.0);
            let t47 = f64x8::splat(1.0) / t46;
            let t48 = t34 * t47;
            let t52 = f64x8::splat(1.0) + f64x8::splat(2.0) / f64x8::splat(9.0) * t27 * t30 * t48;
            let t56 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t52));
            let tzk0 = f64x8::splat(2.0) * t56;
            acc_zk = tzk0;
            let t58 = t17 / t32;
            let t62 = t31 * v_rho;
            let t64 = f64x8::splat(1.0) / t32 / t62;
            let t65 = t64 * t47;
            let t69 = t46 * t46;
            let t70 = f64x8::splat(1.0) / t69;
            let t71 = t34 * t70;
            let t75 = t28 / t18 / t31 * t43;
            let t77 = t35 * v_sigma;
            let t78 = t29 * t64;
            let t80 = t30 * t34 + f64x8::splat(1.0);
            let t81 = ((t80).sqrt());
            let t82 = f64x8::splat(1.0) / t81;
            let t83 = t78 * t82;
            let t86 = -f64x8::splat(4.0) / f64x8::splat(3.0) * t37 * t75 - f64x8::splat(4.0) / f64x8::splat(3.0) * t77 * t83;
            let t91 = -f64x8::splat(16.0) / f64x8::splat(27.0) * t27 * t30 * t65 - f64x8::splat(2.0) / f64x8::splat(9.0) * t27 * t30 * t71 * t86;
            let t96 = ((t2).select(f64x8::splat(0.0), -t6 * t58 * t52 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t91));
            let tvrho0 = f64x8::splat(2.0) * v_rho * t96 + f64x8::splat(2.0) * t56;
            acc_vrho = tvrho0;
            let t99 = t21 * t24;
            let t100 = t25 * t29;
            let t104 = t35 / t36;
            let t106 = t29 * t34;
            let t107 = t106 * t82;
            let t110 = t104 * t44 / f64x8::splat(2.0) + t35 * t107 / f64x8::splat(2.0);
            let t115 = -f64x8::splat(2.0) / f64x8::splat(9.0) * t27 * t30 * t71 * t110 + f64x8::splat(2.0) / f64x8::splat(9.0) * t99 * t100 * t48;
            let t119 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t115));
            let tvsigma0 = f64x8::splat(2.0) * v_rho * t119;
            acc_vsigma = tvsigma0;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_vrho.into(); vrho[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_vsigma.into(); vsigma[ip..ip + m].copy_from_slice(&a[..m]); }
        ip += 8;
    }
}
