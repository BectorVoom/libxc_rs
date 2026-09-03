//! GGA_K_APBEINT vxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_k_apbeint.c`
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
pub fn gga_k_apbeint_vxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    param_muPBE: f64,
    param_muGE: f64,
    param_alpha: f64,
    param_kappa: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_muPBE = f64x8::splat(param_muPBE);
    let param_muGE = f64x8::splat(param_muGE);
    let param_alpha = f64x8::splat(param_alpha);
    let param_kappa = f64x8::splat(param_kappa);
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
            let t23 = t20 * t22;
            let t24 = param_muPBE - param_muGE;
            let t25 = t24 * param_alpha;
            let t26 = f64x8::splat(M_CBRT6);
            let t27 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t28 = (simd::cbrt(t27));
            let t29 = t28 * t28;
            let t30 = f64x8::splat(1.0) / t29;
            let t31 = t26 * t30;
            let t32 = t25 * t31;
            let t33 = f64x8::splat(M_CBRT2);
            let t34 = t33 * t33;
            let t35 = v_sigma * t34;
            let t36 = v_rho * v_rho;
            let t38 = f64x8::splat(1.0) / t22 / t36;
            let t41 = t35 * t38;
            let t44 = f64x8::splat(1.0) + param_alpha * t26 * t30 * t41 / f64x8::splat(24.0);
            let t45 = f64x8::splat(1.0) / t44;
            let t46 = t38 * t45;
            let t51 = (param_muGE + t32 * t35 * t46 / f64x8::splat(24.0)) * t26;
            let t52 = t51 * t30;
            let t55 = param_kappa + t52 * t41 / f64x8::splat(24.0);
            let t60 = f64x8::splat(1.0) + param_kappa * (f64x8::splat(1.0) - param_kappa / t55);
            let t64 = ((t2).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t7 * t23 * t60));
            let tzk0 = f64x8::splat(2.0) * t64;
            acc_zk = tzk0;
            let t65 = f64x8::splat(1.0) / t21;
            let t66 = t20 * t65;
            let t70 = t7 * t20;
            let t71 = param_kappa * param_kappa;
            let t72 = t22 * t71;
            let t73 = t55 * t55;
            let t74 = f64x8::splat(1.0) / t73;
            let t75 = t36 * v_rho;
            let t77 = f64x8::splat(1.0) / t22 / t75;
            let t78 = t77 * t45;
            let t82 = param_alpha * param_alpha;
            let t83 = t24 * t82;
            let t84 = t26 * t26;
            let t86 = f64x8::splat(1.0) / t28 / t27;
            let t87 = t84 * t86;
            let t88 = t83 * t87;
            let t89 = v_sigma * v_sigma;
            let t90 = t89 * t33;
            let t91 = t36 * t36;
            let t92 = t91 * t36;
            let t94 = f64x8::splat(1.0) / t21 / t92;
            let t95 = t44 * t44;
            let t96 = f64x8::splat(1.0) / t95;
            let t97 = t94 * t96;
            let t102 = (-t32 * t35 * t78 / f64x8::splat(9.0) + t88 * t90 * t97 / f64x8::splat(108.0)) * t26;
            let t103 = t102 * t30;
            let t106 = t35 * t77;
            let t109 = t103 * t41 / f64x8::splat(24.0) - t52 * t106 / f64x8::splat(9.0);
            let t110 = t74 * t109;
            let t115 = ((t2).select(f64x8::splat(0.0), t7 * t66 * t60 / f64x8::splat(10.0) + f64x8::splat(3.0) / f64x8::splat(20.0) * t70 * t72 * t110));
            let tvrho0 = f64x8::splat(2.0) * v_rho * t115 + f64x8::splat(2.0) * t64;
            acc_vrho = tvrho0;
            let t118 = t25 * t26;
            let t119 = t30 * t34;
            let t124 = t91 * v_rho;
            let t127 = f64x8::splat(1.0) / t21 / t124 * t96;
            let t132 = (t118 * t119 * t46 / f64x8::splat(24.0) - t88 * v_sigma * t33 * t127 / f64x8::splat(288.0)) * t26;
            let t133 = t132 * t30;
            let t135 = t119 * t38;
            let t138 = t133 * t41 / f64x8::splat(24.0) + t51 * t135 / f64x8::splat(24.0);
            let t139 = t74 * t138;
            let t143 = ((t2).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t70 * t72 * t139));
            let tvsigma0 = f64x8::splat(2.0) * v_rho * t143;
            acc_vsigma = tvsigma0;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_vrho.into(); vrho[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_vsigma.into(); vsigma[ip..ip + m].copy_from_slice(&a[..m]); }
        ip += 8;
    }
}
