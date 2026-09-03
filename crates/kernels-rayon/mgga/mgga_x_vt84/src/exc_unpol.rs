//! MGGA_X_VT84 exc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_vt84.c`
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
pub fn mgga_x_vt84_exc_unpol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
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
        let v_sigma = load(sigma, ip, np);
        let v_lapl = load(lapl, ip, np);
        let v_tau = load(tau, ip, np);
        let mut acc_zk = V_ZERO;
        {
            let t3 = (v_rho / f64x8::splat(2.0)).simd_le(dens_threshold);
            let t4 = f64x8::splat(M_CBRT3);
            let t5 = f64x8::splat(M_CBRTPI);
            let t7 = t4 / t5;
            let t8 = (f64x8::splat(1.0)).simd_le(zeta_threshold);
            let t9 = zeta_threshold - f64x8::splat(1.0);
            let t11 = ((t8).select(t9, (t8).select(-t9, f64x8::splat(0.0))));
            let t12 = f64x8::splat(1.0) + t11;
            let t14 = (simd::cbrt(zeta_threshold));
            let t16 = (simd::cbrt(t12));
            let t18 = (((t12).simd_le(zeta_threshold)).select(t14 * zeta_threshold, t16 * t12));
            let t19 = (simd::cbrt(v_rho));
            let t20 = t18 * t19;
            let t21 = v_sigma * v_sigma;
            let t22 = t21 * v_sigma;
            let t23 = v_rho * v_rho;
            let t24 = t23 * v_rho;
            let t25 = f64x8::splat(1.0) / t24;
            let t26 = t22 * t25;
            let t27 = v_tau * v_tau;
            let t28 = t27 * v_tau;
            let t29 = f64x8::splat(1.0) / t28;
            let t30 = f64x8::splat(1.0) / t23;
            let t31 = t21 * t30;
            let t32 = f64x8::splat(1.0) / t27;
            let t33 = t31 * t32;
            let t35 = f64x8::splat(1.0) + t33 / f64x8::splat(64.0);
            let t36 = t35 * t35;
            let t37 = f64x8::splat(1.0) / t36;
            let t38 = t29 * t37;
            let t42 = f64x8::splat(M_CBRT6);
            let t43 = (f64x8::splat(10.0) / f64x8::splat(81.0) + f64x8::splat(0.00419826171875) * t26 * t38) * t42;
            let t44 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t45 = (simd::cbrt(t44));
            let t46 = t45 * t45;
            let t47 = f64x8::splat(1.0) / t46;
            let t48 = t43 * t47;
            let t49 = f64x8::splat(M_CBRT2);
            let t50 = t49 * t49;
            let t51 = v_sigma * t50;
            let t52 = t19 * t19;
            let t54 = f64x8::splat(1.0) / t52 / t23;
            let t55 = t51 * t54;
            let t58 = v_tau * t50;
            let t60 = f64x8::splat(1.0) / t52 / v_rho;
            let t63 = t58 * t60 - t55 / f64x8::splat(8.0);
            let t64 = t63 * t42;
            let t67 = f64x8::splat(5.0) / f64x8::splat(9.0) * t64 * t47 - f64x8::splat(1.0);
            let t68 = t47 * t67;
            let t71 = f64x8::splat(1.0) + f64x8::splat(0.2222222222222222) * t64 * t68;
            let t72 = ((t71).sqrt());
            let t73 = f64x8::splat(1.0) / t72;
            let t76 = t42 * t47;
            let t77 = t76 * t55;
            let t79 = f64x8::splat(9.0) / f64x8::splat(20.0) * t67 * t73 + t77 / f64x8::splat(36.0);
            let t80 = t79 * t79;
            let t83 = t42 * t42;
            let t85 = f64x8::splat(1.0) / t45 / t44;
            let t86 = t83 * t85;
            let t87 = t21 * t49;
            let t88 = t23 * t23;
            let t89 = t88 * v_rho;
            let t91 = f64x8::splat(1.0) / t19 / t89;
            let t93 = t86 * t87 * t91;
            let t95 = f64x8::splat(162.0) * t33 + f64x8::splat(100.0) * t93;
            let t96 = ((t95).sqrt());
            let t101 = t88 * t88;
            let t102 = f64x8::splat(1.0) / t101;
            let t105 = t48 * t55 / f64x8::splat(24.0) + f64x8::splat(146.0) / f64x8::splat(2025.0) * t80 - f64x8::splat(73.0) / f64x8::splat(97200.0) * t79 * t96 + f64x8::splat(5.301186990888923e-05) * t93 + f64x8::splat(0.0019577914932045744) * t33 + f64x8::splat(4.3721079261097765e-06) * t22 * t102;
            let t107 = f64x8::splat(1.0) + f64x8::splat(0.05873374479613724) * t77;
            let t108 = t107 * t107;
            let t109 = f64x8::splat(1.0) / t108;
            let t110 = t105 * t109;
            let t112 = (simd::exp(-f64x8::splat(0.0001863) * t110));
            let t113 = f64x8::splat(1.0) + t110;
            let t114 = f64x8::splat(1.0) / t113;
            let t115 = t112 * t114;
            let t117 = t105 * t105;
            let t118 = t108 * t108;
            let t119 = f64x8::splat(1.0) / t118;
            let t122 = (simd::exp(-f64x8::splat(0.00150903) * t117 * t119));
            let t123 = f64x8::splat(1.0) - t122;
            let t124 = f64x8::splat(1.0) / t105;
            let t127 = f64x8::splat(10.0) / f64x8::splat(81.0) * t124 * t108 - f64x8::splat(1.0);
            let t129 = t110 * t115 + t123 * t127 + f64x8::splat(1.0);
            let t133 = ((t3).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t7 * t20 * t129));
            let tzk0 = f64x8::splat(2.0) * t133;
            acc_zk = tzk0;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        ip += 8;
    }
}
