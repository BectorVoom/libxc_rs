//! GGA_X_NCAP vxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_ncap.c`
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
pub fn gga_x_ncap_vxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    param_mu: f64,
    param_zeta: f64,
    param_alpha: f64,
    param_beta: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_mu = f64x8::splat(param_mu);
    let param_zeta = f64x8::splat(param_zeta);
    let param_alpha = f64x8::splat(param_alpha);
    let param_beta = f64x8::splat(param_beta);
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
            let t21 = t20 * t20;
            let t22 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t23 = (simd::cbrt(t22));
            let t24 = f64x8::splat(1.0) / t23;
            let t25 = t21 * t24;
            let t26 = ((v_sigma).sqrt());
            let t27 = f64x8::splat(M_CBRT2);
            let t28 = t26 * t27;
            let t30 = f64x8::splat(1.0) / t18 / v_rho;
            let t31 = t28 * t30;
            let t33 = t25 * t31 / f64x8::splat(12.0);
            let t34 = (simd::tanh(t33));
            let t35 = param_mu * t34;
            let t36 = (simd::ln(t33 + ((t33 * t33 + f64x8::splat(1.0)).sqrt())));
            let t37 = f64x8::splat(1.0) - param_zeta;
            let t39 = t37 * t21 * t24;
            let t40 = f64x8::splat(1.0) + t33;
            let t41 = (simd::ln(t40));
            let t42 = t30 * t41;
            let t46 = param_zeta * t21 * t24;
            let t51 = f64x8::splat(1.0) + param_alpha * (t39 * t28 * t42 / f64x8::splat(12.0) + t46 * t31 / f64x8::splat(12.0));
            let t52 = t36 * t51;
            let t53 = param_beta * t34;
            let t55 = t53 * t36 + f64x8::splat(1.0);
            let t56 = f64x8::splat(1.0) / t55;
            let t57 = t52 * t56;
            let t59 = t35 * t57 + f64x8::splat(1.0);
            let t63 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t59));
            let tzk0 = f64x8::splat(2.0) * t63;
            acc_zk = tzk0;
            let t64 = t18 * t18;
            let t66 = t17 / t64;
            let t70 = param_mu * t21;
            let t71 = t24 * t26;
            let t72 = t71 * t27;
            let t73 = t70 * t72;
            let t74 = v_rho * v_rho;
            let t76 = f64x8::splat(1.0) / t18 / t74;
            let t77 = t34 * t34;
            let t78 = f64x8::splat(1.0) - t77;
            let t79 = t76 * t78;
            let t80 = t79 * t57;
            let t84 = t35 * t25 * t26;
            let t85 = t27 * t76;
            let t86 = t23 * t23;
            let t87 = f64x8::splat(1.0) / t86;
            let t88 = t20 * t87;
            let t89 = t27 * t27;
            let t90 = v_sigma * t89;
            let t92 = f64x8::splat(1.0) / t64 / t74;
            let t96 = f64x8::splat(6.0) * t88 * t90 * t92 + f64x8::splat(144.0);
            let t97 = ((t96).sqrt());
            let t98 = f64x8::splat(1.0) / t97;
            let t100 = t98 * t51 * t56;
            let t101 = t85 * t100;
            let t104 = t35 * t36;
            let t105 = t76 * t41;
            let t110 = t37 * t20 * t87;
            let t111 = t74 * v_rho;
            let t113 = f64x8::splat(1.0) / t64 / t111;
            let t114 = f64x8::splat(1.0) / t40;
            let t115 = t113 * t114;
            let t119 = t28 * t76;
            let t122 = -t39 * t28 * t105 / f64x8::splat(9.0) - t110 * t90 * t115 / f64x8::splat(18.0) - t46 * t119 / f64x8::splat(9.0);
            let t123 = param_alpha * t122;
            let t124 = t123 * t56;
            let t126 = t55 * t55;
            let t127 = f64x8::splat(1.0) / t126;
            let t128 = t51 * t127;
            let t129 = param_beta * t21;
            let t130 = t129 * t71;
            let t131 = t78 * t36;
            let t132 = t85 * t131;
            let t135 = t53 * t25;
            let t136 = t76 * t98;
            let t140 = -t130 * t132 / f64x8::splat(9.0) - f64x8::splat(4.0) / f64x8::splat(3.0) * t135 * t28 * t136;
            let t141 = t128 * t140;
            let t143 = -t73 * t80 / f64x8::splat(9.0) - f64x8::splat(4.0) / f64x8::splat(3.0) * t84 * t101 + t104 * t124 - t104 * t141;
            let t148 = ((t2).select(f64x8::splat(0.0), -t6 * t66 * t59 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t143));
            let tvrho0 = f64x8::splat(2.0) * v_rho * t148 + f64x8::splat(2.0) * t63;
            acc_vrho = tvrho0;
            let t151 = f64x8::splat(1.0) / t26;
            let t152 = t24 * t151;
            let t153 = t152 * t27;
            let t154 = t70 * t153;
            let t155 = t30 * t78;
            let t156 = t155 * t57;
            let t160 = t35 * t25 * t151;
            let t161 = t27 * t30;
            let t162 = t161 * t100;
            let t165 = t151 * t27;
            let t169 = t89 * t92;
            let t173 = t165 * t30;
            let t176 = t39 * t165 * t42 / f64x8::splat(24.0) + t110 * t169 * t114 / f64x8::splat(48.0) + t46 * t173 / f64x8::splat(24.0);
            let t177 = param_alpha * t176;
            let t178 = t177 * t56;
            let t180 = t129 * t152;
            let t181 = t161 * t131;
            let t184 = t30 * t98;
            let t188 = t180 * t181 / f64x8::splat(24.0) + t135 * t165 * t184 / f64x8::splat(2.0);
            let t189 = t128 * t188;
            let t191 = t154 * t156 / f64x8::splat(24.0) + t160 * t162 / f64x8::splat(2.0) + t104 * t178 - t104 * t189;
            let t195 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t191));
            let tvsigma0 = f64x8::splat(2.0) * v_rho * t195;
            acc_vsigma = tvsigma0;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_vrho.into(); vrho[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_vsigma.into(); vsigma[ip..ip + m].copy_from_slice(&a[..m]); }
        ip += 8;
    }
}
