//! MGGA_K_PC07 exc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_k_pc07.c`
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
pub fn mgga_k_pc07_exc_unpol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
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
        let v_sigma = load(sigma, ip, np);
        let v_lapl = load(lapl, ip, np);
        let v_tau = load(tau, ip, np);
        let mut acc_zk = V_ZERO;
        {
            let t3 = (v_rho / f64x8::splat(2.0)).simd_le(dens_threshold);
            let t4 = f64x8::splat(M_CBRT3);
            let t5 = t4 * t4;
            let t6 = f64x8::splat(M_CBRTPI);
            let t8 = t5 * t6 * f64x8::splat(M_PI);
            let t9 = (f64x8::splat(1.0)).simd_le(zeta_threshold);
            let t10 = zeta_threshold - f64x8::splat(1.0);
            let t12 = ((t9).select(t10, (t9).select(-t10, f64x8::splat(0.0))));
            let t13 = f64x8::splat(1.0) + t12;
            let t15 = (simd::cbrt(zeta_threshold));
            let t16 = t15 * t15;
            let t18 = (simd::cbrt(t13));
            let t19 = t18 * t18;
            let t21 = (((t13).simd_le(zeta_threshold)).select(t16 * zeta_threshold, t19 * t13));
            let t22 = (simd::cbrt(v_rho));
            let t23 = t22 * t22;
            let t24 = t21 * t23;
            let t25 = f64x8::splat(M_CBRT6);
            let t26 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t27 = (simd::cbrt(t26));
            let t28 = t27 * t27;
            let t29 = f64x8::splat(1.0) / t28;
            let t30 = t25 * t29;
            let t31 = f64x8::splat(M_CBRT2);
            let t32 = t31 * t31;
            let t33 = v_sigma * t32;
            let t34 = v_rho * v_rho;
            let t36 = f64x8::splat(1.0) / t23 / t34;
            let t38 = t30 * t33 * t36;
            let t39 = f64x8::splat(5.0) / f64x8::splat(72.0) * t38;
            let t41 = v_lapl * t32;
            let t43 = f64x8::splat(1.0) / t23 / v_rho;
            let t47 = t25 * t25;
            let t49 = f64x8::splat(1.0) / t27 / t26;
            let t50 = t47 * t49;
            let t51 = v_lapl * v_lapl;
            let t52 = t51 * t31;
            let t53 = t34 * v_rho;
            let t55 = f64x8::splat(1.0) / t22 / t53;
            let t58 = t50 * t52 * t55 / f64x8::splat(2916.0);
            let t59 = t50 * v_sigma;
            let t60 = t34 * t34;
            let t62 = f64x8::splat(1.0) / t22 / t60;
            let t63 = t31 * t62;
            let t64 = t63 * v_lapl;
            let t66 = t59 * t64 / f64x8::splat(2592.0);
            let t67 = v_sigma * v_sigma;
            let t68 = t67 * t31;
            let t69 = t60 * v_rho;
            let t71 = f64x8::splat(1.0) / t22 / t69;
            let t74 = t50 * t68 * t71 / f64x8::splat(8748.0);
            let t75 = f64x8::splat(1.0) + f64x8::splat(5.0) / f64x8::splat(648.0) * t38 + f64x8::splat(5.0) / f64x8::splat(54.0) * t30 * t41 * t43 + t58 - t66 + t74;
            let t76 = t58 - t66 + t74;
            let t77 = t76 * t76;
            let t78 = f64x8::splat(1.0) + t39;
            let t79 = t78 * t78;
            let t80 = f64x8::splat(1.0) / t79;
            let t82 = t77 * t80 + f64x8::splat(1.0);
            let t83 = ((t82).sqrt());
            let t84 = f64x8::splat(1.0) / t83;
            let t86 = t75 * t84 - t39;
            let t87 = param_a / f64x8::splat(40.0);
            let t88 = (t86).simd_le(t87);
            let t89 = f64x8::splat(39.0) / f64x8::splat(40.0) * param_a;
            let t90 = (t89).simd_le(t86);
            let t91 = param_a * param_b;
            let t92 = (t86).simd_lt(t87);
            let t93 = ((t92).select(t87, t86));
            let t94 = (t93).simd_lt(t89);
            let t95 = ((t94).select(t93, t89));
            let t96 = f64x8::splat(1.0) / t95;
            let t98 = (simd::exp(-t91 * t96));
            let t99 = param_a - t95;
            let t102 = (simd::exp(-param_a / t99));
            let t103 = f64x8::splat(1.0) + t102;
            let t104 = (simd::pow(t103, param_b));
            let t105 = t98 * t104;
            let t107 = (simd::exp(-param_a * t96));
            let t108 = t107 + t102;
            let t109 = (simd::pow(t108, param_b));
            let t110 = f64x8::splat(1.0) / t109;
            let t111 = t105 * t110;
            let t112 = ((t88).select(f64x8::splat(0.0), (t90).select(f64x8::splat(1.0), t111)));
            let t114 = t86 * t112 + t39;
            let t118 = ((t3).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t8 * t24 * t114));
            let tzk0 = f64x8::splat(2.0) * t118;
            acc_zk = tzk0;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        ip += 8;
    }
}
