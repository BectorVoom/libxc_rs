//! GGA_K_MEYER vxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_k_meyer.c`
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
pub fn gga_k_meyer_vxc_unpol(
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
            let t23 = t20 * t22;
            let t24 = f64x8::splat(M_CBRT6);
            let t25 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t26 = (simd::cbrt(t25));
            let t27 = t26 * t26;
            let t28 = f64x8::splat(1.0) / t27;
            let t29 = t24 * t28;
            let t30 = f64x8::splat(M_CBRT2);
            let t31 = t30 * t30;
            let t32 = v_sigma * t31;
            let t33 = v_rho * v_rho;
            let t35 = f64x8::splat(1.0) / t22 / t33;
            let t39 = f64x8::splat(1.0) - t29 * t32 * t35 / f64x8::splat(864.0);
            let t40 = t24 * t24;
            let t41 = f64x8::splat(1.0) / t26;
            let t42 = t40 * t41;
            let t43 = ((v_sigma).sqrt());
            let t44 = t43 * t30;
            let t45 = t21 * v_rho;
            let t46 = f64x8::splat(1.0) / t45;
            let t49 = t42 * t44 * t46 / f64x8::splat(72.0);
            let t50 = f64x8::splat(1.0) + t49;
            let t51 = f64x8::splat(1.0) - t49;
            let t52 = ((t51).abs());
            let t53 = f64x8::splat(1.0) / t52;
            let t55 = (simd::ln(t50 * t53));
            let t57 = t39 * t55 * t24;
            let t58 = f64x8::splat(1.0) / t43;
            let t59 = t26 * t58;
            let t60 = t31 * t45;
            let t63 = f64x8::splat(3.0) / f64x8::splat(2.0) * t57 * t59 * t60;
            let t64 = f64x8::splat(1.0) / f64x8::splat(2.0) - t63;
            let t65 = f64x8::splat(1.0) / f64x8::splat(2.0) + t63;
            let t66 = f64x8::splat(1.0) / t65;
            let t69 = f64x8::splat(20.0) * t64 * t66 + f64x8::splat(1.0);
            let t73 = ((t2).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t7 * t23 * t69));
            let tzk0 = f64x8::splat(2.0) * t73;
            acc_zk = tzk0;
            let t75 = t20 / t21;
            let t79 = t42 * t43;
            let t81 = f64x8::splat(1.0) / t21 / t33;
            let t82 = t30 * t81;
            let t83 = t82 * t55;
            let t86 = t82 * t53;
            let t88 = t52 * t52;
            let t89 = f64x8::splat(1.0) / t88;
            let t90 = t50 * t89;
            let t91 = t90 * t42;
            let t92 = ((t51).abs()) / t51;
            let t93 = t81 * t92;
            let t97 = -t91 * t44 * t93 / f64x8::splat(54.0) - t79 * t86 / f64x8::splat(54.0);
            let t98 = t39 * t97;
            let t99 = f64x8::splat(1.0) / t50;
            let t100 = t99 * t52;
            let t101 = t98 * t100;
            let t102 = t24 * t26;
            let t103 = t58 * t31;
            let t105 = t102 * t103 * t45;
            let t108 = t31 * t21;
            let t112 = -t79 * t83 / f64x8::splat(108.0) - f64x8::splat(3.0) / f64x8::splat(2.0) * t101 * t105 - f64x8::splat(2.0) * t57 * t59 * t108;
            let t114 = t65 * t65;
            let t115 = f64x8::splat(1.0) / t114;
            let t116 = t64 * t115;
            let t117 = -t112;
            let t120 = f64x8::splat(20.0) * t112 * t66 - f64x8::splat(20.0) * t116 * t117;
            let t125 = ((t2).select(f64x8::splat(0.0), t7 * t75 * t69 / f64x8::splat(10.0) + f64x8::splat(3.0) / f64x8::splat(20.0) * t7 * t23 * t120));
            let tvrho0 = f64x8::splat(2.0) * v_rho * t125 + f64x8::splat(2.0) * t73;
            acc_vrho = tvrho0;
            let t128 = t42 * t30;
            let t129 = t46 * t55;
            let t133 = t42 * t58;
            let t134 = t30 * t46;
            let t135 = t134 * t53;
            let t137 = t58 * t30;
            let t138 = t46 * t92;
            let t142 = t91 * t137 * t138 / f64x8::splat(144.0) + t133 * t135 / f64x8::splat(144.0);
            let t143 = t39 * t142;
            let t144 = t143 * t100;
            let t147 = t43 * v_sigma;
            let t148 = f64x8::splat(1.0) / t147;
            let t149 = t26 * t148;
            let t153 = t128 * t129 * t58 / f64x8::splat(288.0) - f64x8::splat(3.0) / f64x8::splat(2.0) * t144 * t105 + f64x8::splat(3.0) / f64x8::splat(4.0) * t57 * t149 * t60;
            let t155 = -t153;
            let t158 = -f64x8::splat(20.0) * t116 * t155 + f64x8::splat(20.0) * t153 * t66;
            let t162 = ((t2).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t7 * t23 * t158));
            let tvsigma0 = f64x8::splat(2.0) * v_rho * t162;
            acc_vsigma = tvsigma0;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_vrho.into(); vrho[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_vsigma.into(); vsigma[ip..ip + m].copy_from_slice(&a[..m]); }
        ip += 8;
    }
}
