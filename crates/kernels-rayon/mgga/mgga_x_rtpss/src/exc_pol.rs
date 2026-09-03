//! MGGA_X_RTPSS exc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_rtpss.c`
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
pub fn mgga_x_rtpss_exc_pol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    param_b: f64,
    param_c: f64,
    param_e: f64,
    param_kappa: f64,
    param_mu: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_b = f64x8::splat(param_b);
    let param_c = f64x8::splat(param_c);
    let param_e = f64x8::splat(param_e);
    let param_kappa = f64x8::splat(param_kappa);
    let param_mu = f64x8::splat(param_mu);
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
        let v_lapl0 = load_strided(lapl, ip, np, 2, 0);
        let v_lapl1 = load_strided(lapl, ip, np, 2, 1);
        let v_tau0 = load_strided(tau, ip, np, 2, 0);
        let v_tau1 = load_strided(tau, ip, np, 2, 1);
        let mut acc_zk = V_ZERO;
        {
            let t2 = (v_rho0).simd_le(dens_threshold);
            let t3 = f64x8::splat(M_CBRT3);
            let t4 = f64x8::splat(M_CBRTPI);
            let t6 = t3 / t4;
            let t7 = v_rho0 + v_rho1;
            let t8 = f64x8::splat(1.0) / t7;
            let t11 = (f64x8::splat(2.0) * v_rho0 * t8).simd_le(zeta_threshold);
            let t12 = zeta_threshold - f64x8::splat(1.0);
            let t15 = (f64x8::splat(2.0) * v_rho1 * t8).simd_le(zeta_threshold);
            let t16 = -t12;
            let t17 = v_rho0 - v_rho1;
            let t19 = ((t11).select(t12, (t15).select(t16, t17 * t8)));
            let t20 = f64x8::splat(1.0) + t19;
            let t21 = (t20).simd_le(zeta_threshold);
            let t22 = (simd::cbrt(zeta_threshold));
            let t23 = t22 * zeta_threshold;
            let t24 = (simd::cbrt(t20));
            let t26 = ((t21).select(t23, t24 * t20));
            let t27 = (simd::cbrt(t7));
            let t28 = t26 * t27;
            let t29 = v_sigma0 * v_sigma0;
            let t30 = param_c * t29;
            let t31 = v_rho0 * v_rho0;
            let t32 = f64x8::splat(1.0) / t31;
            let t33 = v_tau0 * v_tau0;
            let t34 = f64x8::splat(1.0) / t33;
            let t35 = t32 * t34;
            let t36 = t29 * t32;
            let t37 = t36 * t34;
            let t39 = f64x8::splat(1.0) + t37 / f64x8::splat(64.0);
            let t40 = t39 * t39;
            let t41 = f64x8::splat(1.0) / t40;
            let t42 = t35 * t41;
            let t46 = f64x8::splat(M_CBRT6);
            let t47 = (f64x8::splat(10.0) / f64x8::splat(81.0) + t30 * t42 / f64x8::splat(64.0)) * t46;
            let t48 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t49 = (simd::cbrt(t48));
            let t50 = t49 * t49;
            let t51 = f64x8::splat(1.0) / t50;
            let t52 = t51 * v_sigma0;
            let t53 = (simd::cbrt(v_rho0));
            let t54 = t53 * t53;
            let t56 = f64x8::splat(1.0) / t54 / t31;
            let t57 = t52 * t56;
            let t61 = f64x8::splat(1.0) / t54 / v_rho0;
            let t63 = v_sigma0 * t56;
            let t65 = v_tau0 * t61 - t63 / f64x8::splat(8.0);
            let t69 = f64x8::splat(5.0) / f64x8::splat(9.0) * t65 * t46 * t51 - f64x8::splat(1.0);
            let t70 = param_b * t65;
            let t71 = t46 * t51;
            let t72 = t71 * t69;
            let t75 = f64x8::splat(5.0) * t70 * t72 + f64x8::splat(9.0);
            let t76 = ((t75).sqrt());
            let t77 = f64x8::splat(1.0) / t76;
            let t82 = f64x8::splat(27.0) / f64x8::splat(20.0) * t69 * t77 + t71 * t63 / f64x8::splat(36.0);
            let t83 = t82 * t82;
            let t86 = t46 * t46;
            let t88 = f64x8::splat(1.0) / t49 / t48;
            let t89 = t86 * t88;
            let t90 = t31 * t31;
            let t91 = t90 * v_rho0;
            let t93 = f64x8::splat(1.0) / t53 / t91;
            let t97 = f64x8::splat(50.0) * t89 * t29 * t93 + f64x8::splat(162.0) * t37;
            let t98 = ((t97).sqrt());
            let t101 = f64x8::splat(1.0) / param_kappa;
            let t102 = t101 * t86;
            let t103 = t88 * t29;
            let t107 = ((param_e).sqrt());
            let t108 = t107 * t29;
            let t111 = param_e * param_mu;
            let t112 = t48 * t48;
            let t113 = f64x8::splat(1.0) / t112;
            let t114 = t29 * v_sigma0;
            let t115 = t113 * t114;
            let t116 = t90 * t90;
            let t117 = f64x8::splat(1.0) / t116;
            let t121 = t47 * t57 / f64x8::splat(24.0) + f64x8::splat(146.0) / f64x8::splat(2025.0) * t83 - f64x8::splat(73.0) / f64x8::splat(97200.0) * t82 * t98 + f64x8::splat(25.0) / f64x8::splat(944784.0) * t102 * t103 * t93 + t108 * t35 / f64x8::splat(720.0) + t111 * t115 * t117 / f64x8::splat(2304.0);
            let t122 = t107 * t46;
            let t125 = f64x8::splat(1.0) + t122 * t57 / f64x8::splat(24.0);
            let t126 = t125 * t125;
            let t127 = f64x8::splat(1.0) / t126;
            let t130 = (simd::exp(-t121 * t127 * t101));
            let t133 = f64x8::splat(1.0) + param_kappa * (f64x8::splat(1.0) - t130);
            let t137 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t28 * t133));
            let t138 = (v_rho1).simd_le(dens_threshold);
            let t139 = -t17;
            let t141 = ((t15).select(t12, (t11).select(t16, t139 * t8)));
            let t142 = f64x8::splat(1.0) + t141;
            let t143 = (t142).simd_le(zeta_threshold);
            let t144 = (simd::cbrt(t142));
            let t146 = ((t143).select(t23, t144 * t142));
            let t147 = t146 * t27;
            let t148 = v_sigma2 * v_sigma2;
            let t149 = param_c * t148;
            let t150 = v_rho1 * v_rho1;
            let t151 = f64x8::splat(1.0) / t150;
            let t152 = v_tau1 * v_tau1;
            let t153 = f64x8::splat(1.0) / t152;
            let t154 = t151 * t153;
            let t155 = t148 * t151;
            let t156 = t155 * t153;
            let t158 = f64x8::splat(1.0) + t156 / f64x8::splat(64.0);
            let t159 = t158 * t158;
            let t160 = f64x8::splat(1.0) / t159;
            let t161 = t154 * t160;
            let t165 = (f64x8::splat(10.0) / f64x8::splat(81.0) + t149 * t161 / f64x8::splat(64.0)) * t46;
            let t166 = t51 * v_sigma2;
            let t167 = (simd::cbrt(v_rho1));
            let t168 = t167 * t167;
            let t170 = f64x8::splat(1.0) / t168 / t150;
            let t171 = t166 * t170;
            let t175 = f64x8::splat(1.0) / t168 / v_rho1;
            let t177 = v_sigma2 * t170;
            let t179 = v_tau1 * t175 - t177 / f64x8::splat(8.0);
            let t183 = f64x8::splat(5.0) / f64x8::splat(9.0) * t179 * t46 * t51 - f64x8::splat(1.0);
            let t184 = param_b * t179;
            let t185 = t71 * t183;
            let t188 = f64x8::splat(5.0) * t184 * t185 + f64x8::splat(9.0);
            let t189 = ((t188).sqrt());
            let t190 = f64x8::splat(1.0) / t189;
            let t195 = f64x8::splat(27.0) / f64x8::splat(20.0) * t183 * t190 + t71 * t177 / f64x8::splat(36.0);
            let t196 = t195 * t195;
            let t199 = t150 * t150;
            let t200 = t199 * v_rho1;
            let t202 = f64x8::splat(1.0) / t167 / t200;
            let t206 = f64x8::splat(50.0) * t89 * t148 * t202 + f64x8::splat(162.0) * t156;
            let t207 = ((t206).sqrt());
            let t210 = t88 * t148;
            let t214 = t107 * t148;
            let t217 = t148 * v_sigma2;
            let t218 = t113 * t217;
            let t219 = t199 * t199;
            let t220 = f64x8::splat(1.0) / t219;
            let t224 = t165 * t171 / f64x8::splat(24.0) + f64x8::splat(146.0) / f64x8::splat(2025.0) * t196 - f64x8::splat(73.0) / f64x8::splat(97200.0) * t195 * t207 + f64x8::splat(25.0) / f64x8::splat(944784.0) * t102 * t210 * t202 + t214 * t154 / f64x8::splat(720.0) + t111 * t218 * t220 / f64x8::splat(2304.0);
            let t227 = f64x8::splat(1.0) + t122 * t171 / f64x8::splat(24.0);
            let t228 = t227 * t227;
            let t229 = f64x8::splat(1.0) / t228;
            let t232 = (simd::exp(-t224 * t229 * t101));
            let t235 = f64x8::splat(1.0) + param_kappa * (f64x8::splat(1.0) - t232);
            let t239 = ((t138).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t147 * t235));
            let tzk0 = t137 + t239;
            acc_zk = tzk0;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        ip += 8;
    }
}
