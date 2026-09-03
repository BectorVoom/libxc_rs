//! GGA_X_SSB_SW vxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_ssb_sw.c`
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
pub fn gga_x_ssb_sw_vxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    param_A: f64,
    param_B: f64,
    param_C: f64,
    param_D: f64,
    param_E: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_A = f64x8::splat(param_A);
    let param_B = f64x8::splat(param_B);
    let param_C = f64x8::splat(param_C);
    let param_D = f64x8::splat(param_D);
    let param_E = f64x8::splat(param_E);
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
            let t20 = f64x8::splat(M_CBRT6);
            let t22 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t23 = (simd::cbrt(t22));
            let t24 = t23 * t23;
            let t25 = f64x8::splat(1.0) / t24;
            let t26 = param_B * t20 * t25;
            let t27 = f64x8::splat(M_CBRT2);
            let t28 = t27 * t27;
            let t29 = v_sigma * t28;
            let t30 = v_rho * v_rho;
            let t31 = t18 * t18;
            let t33 = f64x8::splat(1.0) / t31 / t30;
            let t39 = f64x8::splat(1.0) + param_C * t20 * t25 * t29 * t33 / f64x8::splat(24.0);
            let t40 = f64x8::splat(1.0) / t39;
            let t46 = param_D * t20 * t25;
            let t47 = t20 * t20;
            let t50 = f64x8::splat(1.0) / t23 / t22;
            let t52 = v_sigma * v_sigma;
            let t54 = t30 * t30;
            let t55 = t54 * v_rho;
            let t57 = f64x8::splat(1.0) / t18 / t55;
            let t61 = f64x8::splat(1.0) + param_E * t47 * t50 * t52 * t27 * t57 / f64x8::splat(288.0);
            let t62 = f64x8::splat(1.0) / t61;
            let t67 = param_A + t26 * t29 * t33 * t40 / f64x8::splat(24.0) - t46 * t29 * t33 * t62 / f64x8::splat(24.0);
            let t71 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t67));
            let tzk0 = f64x8::splat(2.0) * t71;
            acc_zk = tzk0;
            let t73 = t17 / t31;
            let t77 = t30 * v_rho;
            let t79 = f64x8::splat(1.0) / t31 / t77;
            let t84 = param_B * t47;
            let t86 = t84 * t50 * t52;
            let t87 = t54 * t30;
            let t89 = f64x8::splat(1.0) / t18 / t87;
            let t91 = t39 * t39;
            let t92 = f64x8::splat(1.0) / t91;
            let t93 = t92 * param_C;
            let t94 = t27 * t89 * t93;
            let t101 = t22 * t22;
            let t102 = f64x8::splat(1.0) / t101;
            let t103 = param_D * t102;
            let t104 = t52 * v_sigma;
            let t105 = t103 * t104;
            let t106 = t54 * t54;
            let t107 = t106 * v_rho;
            let t108 = f64x8::splat(1.0) / t107;
            let t109 = t61 * t61;
            let t110 = f64x8::splat(1.0) / t109;
            let t112 = t108 * t110 * param_E;
            let t115 = -t26 * t29 * t79 * t40 / f64x8::splat(9.0) + t86 * t94 / f64x8::splat(108.0) + t46 * t29 * t79 * t62 / f64x8::splat(9.0) - t105 * t112 / f64x8::splat(108.0);
            let t120 = ((t2).select(f64x8::splat(0.0), -t6 * t73 * t67 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t115));
            let tvrho0 = f64x8::splat(2.0) * v_rho * t120 + f64x8::splat(2.0) * t71;
            acc_vrho = tvrho0;
            let t123 = t28 * t33;
            let t130 = t27 * t57 * t93;
            let t137 = f64x8::splat(1.0) / t106;
            let t139 = t137 * t110 * param_E;
            let t142 = t26 * t123 * t40 / f64x8::splat(24.0) - t84 * t50 * v_sigma * t130 / f64x8::splat(288.0) - t46 * t123 * t62 / f64x8::splat(24.0) + t103 * t52 * t139 / f64x8::splat(288.0);
            let t146 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t142));
            let tvsigma0 = f64x8::splat(2.0) * v_rho * t146;
            acc_vsigma = tvsigma0;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_vrho.into(); vrho[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_vsigma.into(); vsigma[ip..ip + m].copy_from_slice(&a[..m]); }
        ip += 8;
    }
}
