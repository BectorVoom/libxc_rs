//! MGGA_X_FT98 exc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_ft98.c`
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
pub fn mgga_x_ft98_exc_unpol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    param_a: f64,
    param_a1: f64,
    param_a2: f64,
    param_b: f64,
    param_b1: f64,
    param_b2: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_a = f64x8::splat(param_a);
    let param_a1 = f64x8::splat(param_a1);
    let param_a2 = f64x8::splat(param_a2);
    let param_b = f64x8::splat(param_b);
    let param_b1 = f64x8::splat(param_b1);
    let param_b2 = f64x8::splat(param_b2);
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
            let t6 = f64x8::splat(1.0) / t5;
            let t7 = t4 * t6;
            let t8 = (f64x8::splat(1.0)).simd_le(zeta_threshold);
            let t9 = zeta_threshold - f64x8::splat(1.0);
            let t11 = ((t8).select(t9, (t8).select(-t9, f64x8::splat(0.0))));
            let t12 = f64x8::splat(1.0) + t11;
            let t14 = (simd::cbrt(zeta_threshold));
            let t16 = (simd::cbrt(t12));
            let t18 = (((t12).simd_le(zeta_threshold)).select(t14 * zeta_threshold, t16 * t12));
            let t19 = (simd::cbrt(v_rho));
            let t20 = t18 * t19;
            let t21 = param_a1 * v_sigma;
            let t22 = f64x8::splat(M_CBRT2);
            let t23 = t22 * t22;
            let t24 = v_rho * v_rho;
            let t25 = t19 * t19;
            let t27 = f64x8::splat(1.0) / t25 / t24;
            let t28 = t23 * t27;
            let t30 = t21 * t28 + f64x8::splat(1.0);
            let t31 = ((t30).sqrt());
            let t32 = param_a * t31;
            let t33 = param_b1 * v_sigma;
            let t35 = t28 * t33 + f64x8::splat(1.0);
            let t36 = ((t35).sqrt().sqrt());
            let t37 = t36 * t36;
            let t38 = t37 * t36;
            let t39 = f64x8::splat(1.0) / t38;
            let t40 = t32 * t39;
            let t41 = v_sigma * t23;
            let t42 = t41 * t27;
            let t44 = v_lapl * t23;
            let t46 = f64x8::splat(1.0) / t25 / v_rho;
            let t48 = -t44 * t46 + t42;
            let t49 = t48 * t48;
            let t50 = param_a2 * t49;
            let t51 = f64x8::splat(1.0) + t42;
            let t52 = t51 * t51;
            let t53 = f64x8::splat(1.0) / t52;
            let t56 = param_b * (t50 * t53 + f64x8::splat(1.0));
            let t57 = param_b2 * param_b2;
            let t59 = ((t57 + f64x8::splat(1.0)).sqrt());
            let t60 = t59 - param_b2;
            let t61 = v_sigma * v_sigma;
            let t62 = t61 * t22;
            let t63 = t24 * t24;
            let t64 = t63 * v_rho;
            let t66 = f64x8::splat(1.0) / t19 / t64;
            let t67 = t62 * t66;
            let t68 = f64x8::splat(2.0) * t67;
            let t69 = v_lapl * v_lapl;
            let t70 = t69 * t22;
            let t71 = t24 * v_rho;
            let t73 = f64x8::splat(1.0) / t19 / t71;
            let t74 = t70 * t73;
            let t75 = f64x8::splat(2.0) * t74;
            let t76 = t68 - t75 - param_b2;
            let t77 = ((f64x8::splat(f64::EPSILON)).sqrt().sqrt());
            let t78 = f64x8::splat(1.0) / t77;
            let t79 = (t76).simd_lt(-t78);
            let t85 = t76 * t76;
            let t86 = t85 * t76;
            let t87 = f64x8::splat(1.0) / t86;
            let t89 = t85 * t85;
            let t90 = t89 * t76;
            let t91 = f64x8::splat(1.0) / t90;
            let t96 = (((f64x8::splat(0.0)).simd_lt(t76)).select(t76, -t76));
            let t97 = (t96).simd_lt(t77);
            let t100 = t89 * t85;
            let t102 = t89 * t89;
            let t105 = (-t78).simd_lt(t76);
            let t106 = ((t105).select(t76, -t78));
            let t107 = t106 * t106;
            let t108 = f64x8::splat(1.0) + t107;
            let t109 = ((t108).sqrt());
            let t110 = t106 + t109;
            let t112 = ((t79).select(-f64x8::splat(4.0) * t67 + f64x8::splat(4.0) * t74 + f64x8::splat(2.0) * param_b2 - f64x8::splat(1.0) / t76 / f64x8::splat(2.0) + t87 / f64x8::splat(8.0) - t91 / f64x8::splat(16.0), (t97).select(f64x8::splat(1.0) - t68 + t75 + param_b2 + t85 / f64x8::splat(2.0) - t89 / f64x8::splat(8.0) + t100 / f64x8::splat(16.0) - f64x8::splat(5.0) / f64x8::splat(128.0) * t102, f64x8::splat(1.0) / t110)));
            let t114 = t112 * t60 + f64x8::splat(1.0);
            let t115 = t22 - f64x8::splat(1.0);
            let t116 = t115 * t60;
            let t118 = t112 * t116 + f64x8::splat(1.0);
            let t119 = t118 * t118;
            let t120 = t119 * t118;
            let t121 = f64x8::splat(1.0) / t120;
            let t122 = t114 * t121;
            let t123 = t122 * t49;
            let t125 = t123 * t56 + t40 * t42 + f64x8::splat(1.0);
            let t126 = t4 * t4;
            let t127 = f64x8::splat(1.0) / f64x8::splat(M_PI);
            let t128 = (simd::cbrt(t127));
            let t129 = t128 * t128;
            let t130 = t126 * t129;
            let t131 = f64x8::splat(M_CBRT4);
            let t133 = param_b * v_sigma;
            let t137 = f64x8::splat(1.0) + f64x8::splat(81.0) / f64x8::splat(4.0) * t130 * t131 * t133 * t28;
            let t138 = f64x8::splat(1.0) / t137;
            let t139 = t125 * t138;
            let t140 = ((t139).sqrt());
            let t144 = ((t3).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t7 * t20 * t140));
            let tzk0 = f64x8::splat(2.0) * t144;
            acc_zk = tzk0;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        ip += 8;
    }
}
