//! GGA_K_VT84F exc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_k_vt84f.c`
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
pub fn gga_k_vt84f_exc_pol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    param_alpha: f64,
    param_mu: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_alpha = f64x8::splat(param_alpha);
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
        let mut acc_zk = V_ZERO;
        {
            let t1 = (v_rho0).simd_le(dens_threshold);
            let t2 = f64x8::splat(M_CBRT3);
            let t3 = t2 * t2;
            let t4 = f64x8::splat(M_CBRTPI);
            let t6 = t3 * t4 * f64x8::splat(M_PI);
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
            let t23 = t22 * t22;
            let t24 = t23 * zeta_threshold;
            let t25 = (simd::cbrt(t20));
            let t26 = t25 * t25;
            let t28 = ((t21).select(t24, t26 * t20));
            let t29 = (simd::cbrt(t7));
            let t30 = t29 * t29;
            let t31 = t28 * t30;
            let t32 = f64x8::splat(M_CBRT6);
            let t33 = t32 * t32;
            let t34 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t35 = (simd::cbrt(t34));
            let t37 = t33 / t35;
            let t38 = ((v_sigma0).sqrt());
            let t39 = (simd::cbrt(v_rho0));
            let t41 = f64x8::splat(1.0) / t39 / v_rho0;
            let t44 = t37 * t38 * t41 / f64x8::splat(12.0);
            let t45 = ((f64x8::splat(f64::EPSILON)).sqrt());
            let t46 = (t44).simd_le(t45);
            let t48 = (-param_mu + param_alpha + f64x8::splat(5.0) / f64x8::splat(3.0)) * t32;
            let t49 = t35 * t35;
            let t50 = f64x8::splat(1.0) / t49;
            let t51 = t50 * v_sigma0;
            let t52 = v_rho0 * v_rho0;
            let t53 = t39 * t39;
            let t55 = f64x8::splat(1.0) / t53 / t52;
            let t59 = param_mu * param_alpha;
            let t60 = param_mu * param_mu;
            let t62 = (t59 + t60 - param_alpha) * t33;
            let t64 = f64x8::splat(1.0) / t35 / t34;
            let t65 = v_sigma0 * v_sigma0;
            let t66 = t64 * t65;
            let t67 = t52 * t52;
            let t68 = t67 * v_rho0;
            let t70 = f64x8::splat(1.0) / t39 / t68;
            let t74 = param_alpha * param_alpha;
            let t76 = param_mu * t74 / f64x8::splat(2.0);
            let t79 = t74 / f64x8::splat(2.0);
            let t81 = t34 * t34;
            let t83 = (-t76 - (t59 + t60) * param_mu - t79) / t81;
            let t84 = t65 * v_sigma0;
            let t85 = t67 * t67;
            let t86 = f64x8::splat(1.0) / t85;
            let t90 = t74 * param_alpha;
            let t94 = t60 * param_mu;
            let t98 = (param_mu * t90 / f64x8::splat(6.0) - (-param_alpha * t60 - t76 - t94) * param_mu + t79) * t32;
            let t100 = f64x8::splat(1.0) / t49 / t81;
            let t101 = t65 * t65;
            let t102 = t100 * t101;
            let t103 = t85 * t52;
            let t105 = f64x8::splat(1.0) / t53 / t103;
            let t110 = (t45).simd_lt(t44);
            let t111 = ((t110).select(t44, t45));
            let t112 = t111 * t111;
            let t113 = param_mu * t112;
            let t114 = param_alpha * t112;
            let t115 = (simd::exp(-t114));
            let t116 = f64x8::splat(1.0) + t113;
            let t117 = f64x8::splat(1.0) / t116;
            let t118 = t115 * t117;
            let t120 = t112 * t112;
            let t122 = (simd::exp(-param_alpha * t120));
            let t123 = f64x8::splat(1.0) - t122;
            let t124 = f64x8::splat(1.0) / t112;
            let t125 = t124 - f64x8::splat(1.0);
            let t129 = ((t46).select(f64x8::splat(1.0) + t48 * t51 * t55 / f64x8::splat(24.0) + t62 * t66 * t70 / f64x8::splat(576.0) + t83 * t84 * t86 / f64x8::splat(2304.0) + t98 * t102 * t105 / f64x8::splat(55296.0), f64x8::splat(1.0) - t113 * t118 + t123 * t125 + f64x8::splat(5.0) / f64x8::splat(3.0) * t112));
            let t133 = ((t1).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t31 * t129));
            let t134 = (v_rho1).simd_le(dens_threshold);
            let t135 = -t17;
            let t137 = ((t15).select(t12, (t11).select(t16, t135 * t8)));
            let t138 = f64x8::splat(1.0) + t137;
            let t139 = (t138).simd_le(zeta_threshold);
            let t140 = (simd::cbrt(t138));
            let t141 = t140 * t140;
            let t143 = ((t139).select(t24, t141 * t138));
            let t144 = t143 * t30;
            let t145 = ((v_sigma2).sqrt());
            let t146 = (simd::cbrt(v_rho1));
            let t148 = f64x8::splat(1.0) / t146 / v_rho1;
            let t151 = t37 * t145 * t148 / f64x8::splat(12.0);
            let t152 = (t151).simd_le(t45);
            let t153 = t50 * v_sigma2;
            let t154 = v_rho1 * v_rho1;
            let t155 = t146 * t146;
            let t157 = f64x8::splat(1.0) / t155 / t154;
            let t161 = v_sigma2 * v_sigma2;
            let t162 = t64 * t161;
            let t163 = t154 * t154;
            let t164 = t163 * v_rho1;
            let t166 = f64x8::splat(1.0) / t146 / t164;
            let t170 = t161 * v_sigma2;
            let t171 = t163 * t163;
            let t172 = f64x8::splat(1.0) / t171;
            let t176 = t161 * t161;
            let t177 = t100 * t176;
            let t178 = t171 * t154;
            let t180 = f64x8::splat(1.0) / t155 / t178;
            let t185 = (t45).simd_lt(t151);
            let t186 = ((t185).select(t151, t45));
            let t187 = t186 * t186;
            let t188 = param_mu * t187;
            let t189 = param_alpha * t187;
            let t190 = (simd::exp(-t189));
            let t191 = f64x8::splat(1.0) + t188;
            let t192 = f64x8::splat(1.0) / t191;
            let t193 = t190 * t192;
            let t195 = t187 * t187;
            let t197 = (simd::exp(-param_alpha * t195));
            let t198 = f64x8::splat(1.0) - t197;
            let t199 = f64x8::splat(1.0) / t187;
            let t200 = t199 - f64x8::splat(1.0);
            let t204 = ((t152).select(f64x8::splat(1.0) + t48 * t153 * t157 / f64x8::splat(24.0) + t62 * t162 * t166 / f64x8::splat(576.0) + t83 * t170 * t172 / f64x8::splat(2304.0) + t98 * t177 * t180 / f64x8::splat(55296.0), f64x8::splat(1.0) - t188 * t193 + t198 * t200 + f64x8::splat(5.0) / f64x8::splat(3.0) * t187));
            let t208 = ((t134).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t144 * t204));
            let tzk0 = t133 + t208;
            acc_zk = tzk0;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        ip += 8;
    }
}
