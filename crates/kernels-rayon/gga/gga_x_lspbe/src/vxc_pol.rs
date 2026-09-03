//! GGA_X_LSPBE vxc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_lspbe.c`
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

/// Load 8 elements with a given stride and offset.
#[inline(always)]
fn load_strided(s: &[f64], ip: usize, np: usize, stride: usize, offset: usize) -> f64x8 {
    let mut b = [0.0f64; 8];
    if ip + 8 <= np {
        let base = ip * stride + offset;
        b[0] = s[base];
        b[1] = s[base + stride];
        b[2] = s[base + 2 * stride];
        b[3] = s[base + 3 * stride];
        b[4] = s[base + 4 * stride];
        b[5] = s[base + 5 * stride];
        b[6] = s[base + 6 * stride];
        b[7] = s[base + 7 * stride];
    } else {
        for k in 0..8 {
            let p = (ip + k).min(np - 1);
            b[k] = s[p * stride + offset];
        }
    }
    f64x8::new(b)
}

/// Store 8 elements with a given stride and offset.
#[inline(always)]
fn store_strided(s: &mut [f64], ip: usize, m: usize, stride: usize, offset: usize, acc: f64x8) {
    let a: [f64; 8] = acc.into();
    if m == 8 {
        let base = ip * stride + offset;
        s[base] = a[0];
        s[base + stride] = a[1];
        s[base + 2 * stride] = a[2];
        s[base + 3 * stride] = a[3];
        s[base + 4 * stride] = a[4];
        s[base + 5 * stride] = a[5];
        s[base + 6 * stride] = a[6];
        s[base + 7 * stride] = a[7];
    } else {
        for k in 0..m {
            s[(ip + k) * stride + offset] = a[k];
        }
    }
}

#[allow(unused_variables, non_snake_case)]
pub fn gga_x_lspbe_vxc_pol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    param_mu: f64,
    param_kappa: f64,
    param_alpha: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_mu = f64x8::splat(param_mu);
    let param_kappa = f64x8::splat(param_kappa);
    let param_alpha = f64x8::splat(param_alpha);
    let dens_threshold = f64x8::splat(dens_threshold);
    let zeta_threshold = f64x8::splat(zeta_threshold);
    let mut ip = 0usize;
    while ip < np {
        let m = (np - ip).min(8);
        let v_rho0 = load_strided(rho, ip, np, 2, 0);
        let v_rho1 = load_strided(rho, ip, np, 2, 1);
        let v_sigma0 = load_strided(sigma, ip, np, 3, 0);
        let v_sigma1 = load_strided(sigma, ip, np, 3, 1);
        let v_sigma2 = load_strided(sigma, ip, np, 3, 2);
        let mut acc_zk = V_ZERO;
        let mut acc_vrho_0 = V_ZERO;
        let mut acc_vrho_1 = V_ZERO;
        let mut acc_vsigma_0 = V_ZERO;
        let mut acc_vsigma_1 = V_ZERO;
        let mut acc_vsigma_2 = V_ZERO;
        {
            let t1 = (v_rho0).simd_le(dens_threshold);
            let t2 = f64x8::splat(M_CBRT3);
            let t3 = f64x8::splat(M_CBRTPI);
            let t5 = t2 / t3;
            let t6 = v_rho0 + v_rho1;
            let t7 = f64x8::splat(1.0) / t6;
            let t10 = (f64x8::splat(2.0) * v_rho0 * t7).simd_le(zeta_threshold);
            let t11 = zeta_threshold - f64x8::splat(1.0);
            let t14 = (f64x8::splat(2.0) * v_rho1 * t7).simd_le(zeta_threshold);
            let t15 = -t11;
            let t16 = v_rho0 - v_rho1;
            let t18 = ((t10).select(t11, (t14).select(t15, t16 * t7)));
            let t19 = f64x8::splat(1.0) + t18;
            let t20 = (t19).simd_le(zeta_threshold);
            let t21 = (simd::cbrt(zeta_threshold));
            let t22 = t21 * zeta_threshold;
            let t23 = (simd::cbrt(t19));
            let t25 = ((t20).select(t22, t23 * t19));
            let t26 = (simd::cbrt(t6));
            let t27 = t25 * t26;
            let t28 = f64x8::splat(M_CBRT6);
            let t29 = param_mu * t28;
            let t30 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t31 = (simd::cbrt(t30));
            let t32 = t31 * t31;
            let t33 = f64x8::splat(1.0) / t32;
            let t34 = t33 * v_sigma0;
            let t35 = v_rho0 * v_rho0;
            let t36 = (simd::cbrt(v_rho0));
            let t37 = t36 * t36;
            let t39 = f64x8::splat(1.0) / t37 / t35;
            let t40 = t34 * t39;
            let t43 = param_kappa + t29 * t40 / f64x8::splat(24.0);
            let t48 = param_kappa + f64x8::splat(1.0);
            let t49 = param_alpha * t28;
            let t52 = (simd::exp(-t49 * t40 / f64x8::splat(24.0)));
            let t55 = f64x8::splat(1.0) + param_kappa * (f64x8::splat(1.0) - param_kappa / t43) - t48 * (f64x8::splat(1.0) - t52);
            let t59 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t27 * t55));
            let t60 = (v_rho1).simd_le(dens_threshold);
            let t61 = -t16;
            let t63 = ((t14).select(t11, (t10).select(t15, t61 * t7)));
            let t64 = f64x8::splat(1.0) + t63;
            let t65 = (t64).simd_le(zeta_threshold);
            let t66 = (simd::cbrt(t64));
            let t68 = ((t65).select(t22, t66 * t64));
            let t69 = t68 * t26;
            let t70 = t33 * v_sigma2;
            let t71 = v_rho1 * v_rho1;
            let t72 = (simd::cbrt(v_rho1));
            let t73 = t72 * t72;
            let t75 = f64x8::splat(1.0) / t73 / t71;
            let t76 = t70 * t75;
            let t79 = param_kappa + t29 * t76 / f64x8::splat(24.0);
            let t86 = (simd::exp(-t49 * t76 / f64x8::splat(24.0)));
            let t89 = f64x8::splat(1.0) + param_kappa * (f64x8::splat(1.0) - param_kappa / t79) - t48 * (f64x8::splat(1.0) - t86);
            let t93 = ((t60).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t69 * t89));
            let tzk0 = t59 + t93;
            acc_zk = tzk0;
            let t94 = t6 * t6;
            let t95 = f64x8::splat(1.0) / t94;
            let t96 = t16 * t95;
            let t98 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), t7 - t96)));
            let t101 = ((t20).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t98));
            let t102 = t101 * t26;
            let t106 = t26 * t26;
            let t107 = f64x8::splat(1.0) / t106;
            let t108 = t25 * t107;
            let t111 = t5 * t108 * t55 / f64x8::splat(8.0);
            let t112 = param_kappa * param_kappa;
            let t113 = t43 * t43;
            let t116 = t112 / t113 * param_mu;
            let t117 = t28 * t33;
            let t118 = t35 * v_rho0;
            let t120 = f64x8::splat(1.0) / t37 / t118;
            let t125 = t48 * param_alpha * t28;
            let t130 = -t116 * t117 * v_sigma0 * t120 / f64x8::splat(9.0) + t125 * t34 * t120 * t52 / f64x8::splat(9.0);
            let t135 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t102 * t55 - t111 - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t27 * t130));
            let t136 = t61 * t95;
            let t138 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), -t7 - t136)));
            let t141 = ((t65).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t66 * t138));
            let t142 = t141 * t26;
            let t146 = t68 * t107;
            let t149 = t5 * t146 * t89 / f64x8::splat(8.0);
            let t151 = ((t60).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t142 * t89 - t149));
            let tvrho0 = t59 + t93 + t6 * (t135 + t151);
            acc_vrho_0 = tvrho0;
            let t155 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), -t7 - t96)));
            let t158 = ((t20).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t155));
            let t159 = t158 * t26;
            let t164 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t159 * t55 - t111));
            let t166 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), t7 - t136)));
            let t169 = ((t65).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t66 * t166));
            let t170 = t169 * t26;
            let t174 = t79 * t79;
            let t177 = t112 / t174 * param_mu;
            let t178 = t71 * v_rho1;
            let t180 = f64x8::splat(1.0) / t73 / t178;
            let t188 = -t177 * t117 * v_sigma2 * t180 / f64x8::splat(9.0) + t125 * t70 * t180 * t86 / f64x8::splat(9.0);
            let t193 = ((t60).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t170 * t89 - t149 - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t69 * t188));
            let tvrho1 = t59 + t93 + t6 * (t164 + t193);
            acc_vrho_1 = tvrho1;
            let t202 = -t125 * t33 * t39 * t52 / f64x8::splat(24.0) + t116 * t117 * t39 / f64x8::splat(24.0);
            let t206 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t27 * t202));
            let tvsigma0 = t6 * t206;
            acc_vsigma_0 = tvsigma0;
            let tvsigma1 = f64x8::splat(0.0);
            acc_vsigma_1 = tvsigma1;
            let t213 = -t125 * t33 * t75 * t86 / f64x8::splat(24.0) + t177 * t117 * t75 / f64x8::splat(24.0);
            let t217 = ((t60).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t69 * t213));
            let tvsigma2 = t6 * t217;
            acc_vsigma_2 = tvsigma2;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        store_strided(vrho, ip, m, 2, 0, acc_vrho_0);
        store_strided(vrho, ip, m, 2, 1, acc_vrho_1);
        store_strided(vsigma, ip, m, 3, 0, acc_vsigma_0);
        store_strided(vsigma, ip, m, 3, 1, acc_vsigma_1);
        store_strided(vsigma, ip, m, 3, 2, acc_vsigma_2);
        ip += 8;
    }
}
