//! MGGA_X_RPPSCAN exc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_rppscan.c`
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
pub fn mgga_x_rppscan_exc_pol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    param_c2: f64,
    param_d: f64,
    param_eta: f64,
    param_k1: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_c2 = f64x8::splat(param_c2);
    let param_d = f64x8::splat(param_d);
    let param_eta = f64x8::splat(param_eta);
    let param_k1 = f64x8::splat(param_k1);
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
            let t27 = t6 * t26;
            let t28 = (simd::cbrt(t7));
            let t29 = f64x8::splat(M_CBRT6);
            let t30 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t31 = (simd::cbrt(t30));
            let t32 = t31 * t31;
            let t33 = f64x8::splat(1.0) / t32;
            let t34 = t29 * t33;
            let t35 = v_rho0 * v_rho0;
            let t36 = (simd::cbrt(v_rho0));
            let t37 = t36 * t36;
            let t38 = t37 * t35;
            let t39 = f64x8::splat(1.0) / t38;
            let t40 = v_sigma0 * t39;
            let t41 = t34 * t40;
            let t45 = f64x8::splat(100.0) / f64x8::splat(6561.0) / param_k1 - f64x8::splat(73.0) / f64x8::splat(648.0);
            let t46 = t29 * t29;
            let t47 = t45 * t46;
            let t48 = t31 * t30;
            let t49 = f64x8::splat(1.0) / t48;
            let t50 = t47 * t49;
            let t51 = v_sigma0 * v_sigma0;
            let t52 = t35 * t35;
            let t53 = t52 * v_rho0;
            let t55 = f64x8::splat(1.0) / t36 / t53;
            let t56 = t51 * t55;
            let t57 = t45 * t29;
            let t58 = t33 * v_sigma0;
            let t59 = t58 * t39;
            let t62 = (simd::exp(-f64x8::splat(27.0) / f64x8::splat(80.0) * t57 * t59));
            let t66 = ((f64x8::splat(146.0)).sqrt());
            let t67 = t66 * t29;
            let t70 = t37 * v_rho0;
            let t71 = f64x8::splat(1.0) / t70;
            let t74 = v_tau0 * t71 - t40 / f64x8::splat(8.0);
            let t76 = f64x8::splat(3.0) / f64x8::splat(10.0) * t46 * t32;
            let t77 = param_eta * v_sigma0;
            let t80 = t76 + t77 * t39 / f64x8::splat(8.0);
            let t81 = f64x8::splat(1.0) / t80;
            let t82 = t74 * t81;
            let t83 = f64x8::splat(1.0) - t82;
            let t85 = t83 * t83;
            let t87 = (simd::exp(-t85 / f64x8::splat(2.0)));
            let t90 = f64x8::splat(7.0) / f64x8::splat(12960.0) * t67 * t59 + t66 * t83 * t87 / f64x8::splat(100.0);
            let t91 = t90 * t90;
            let t92 = param_k1 + f64x8::splat(5.0) / f64x8::splat(972.0) * t41 + t50 * t56 * t62 / f64x8::splat(576.0) + t91;
            let t97 = f64x8::splat(1.0) + param_k1 * (f64x8::splat(1.0) - param_k1 / t92);
            let t98 = (t82).simd_le(f64x8::splat(2.5));
            let t99 = (f64x8::splat(2.5)).simd_lt(t82);
            let t100 = ((t99).select(f64x8::splat(2.5), t82));
            let t102 = t100 * t100;
            let t104 = t102 * t100;
            let t106 = t102 * t102;
            let t108 = t106 * t100;
            let t110 = t106 * t102;
            let t115 = ((t99).select(t82, f64x8::splat(2.5)));
            let t116 = f64x8::splat(1.0) - t115;
            let t119 = (simd::exp(param_c2 / t116));
            let t121 = ((t98).select(f64x8::splat(1.0) - f64x8::splat(0.667) * t100 - f64x8::splat(0.4445555) * t102 - f64x8::splat(0.663086601049) * t104 + f64x8::splat(1.45129704449) * t106 - f64x8::splat(0.887998041597) * t108 + f64x8::splat(0.234528941479) * t110 - f64x8::splat(0.023185843322) * t106 * t104, -param_d * t119));
            let t122 = f64x8::splat(1.0) - t121;
            let t125 = t97 * t122 + f64x8::splat(1.174) * t121;
            let t126 = t28 * t125;
            let t127 = ((f64x8::splat(3.0)).sqrt());
            let t128 = f64x8::splat(1.0) / t31;
            let t129 = t46 * t128;
            let t130 = ((v_sigma0).sqrt());
            let t131 = t36 * v_rho0;
            let t132 = f64x8::splat(1.0) / t131;
            let t134 = t129 * t130 * t132;
            let t135 = ((t134).sqrt());
            let t139 = (simd::exp(-f64x8::splat(9.8958) * t127 / t135));
            let t140 = f64x8::splat(1.0) - t139;
            let t141 = t126 * t140;
            let t144 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t27 * t141));
            let t145 = (v_rho1).simd_le(dens_threshold);
            let t146 = -t17;
            let t148 = ((t15).select(t12, (t11).select(t16, t146 * t8)));
            let t149 = f64x8::splat(1.0) + t148;
            let t150 = (t149).simd_le(zeta_threshold);
            let t151 = (simd::cbrt(t149));
            let t153 = ((t150).select(t23, t151 * t149));
            let t154 = t6 * t153;
            let t155 = v_rho1 * v_rho1;
            let t156 = (simd::cbrt(v_rho1));
            let t157 = t156 * t156;
            let t158 = t157 * t155;
            let t159 = f64x8::splat(1.0) / t158;
            let t160 = v_sigma2 * t159;
            let t161 = t34 * t160;
            let t163 = v_sigma2 * v_sigma2;
            let t164 = t155 * t155;
            let t165 = t164 * v_rho1;
            let t167 = f64x8::splat(1.0) / t156 / t165;
            let t168 = t163 * t167;
            let t169 = t33 * v_sigma2;
            let t170 = t169 * t159;
            let t173 = (simd::exp(-f64x8::splat(27.0) / f64x8::splat(80.0) * t57 * t170));
            let t179 = t157 * v_rho1;
            let t180 = f64x8::splat(1.0) / t179;
            let t183 = v_tau1 * t180 - t160 / f64x8::splat(8.0);
            let t184 = param_eta * v_sigma2;
            let t187 = t76 + t184 * t159 / f64x8::splat(8.0);
            let t188 = f64x8::splat(1.0) / t187;
            let t189 = t183 * t188;
            let t190 = f64x8::splat(1.0) - t189;
            let t192 = t190 * t190;
            let t194 = (simd::exp(-t192 / f64x8::splat(2.0)));
            let t197 = f64x8::splat(7.0) / f64x8::splat(12960.0) * t67 * t170 + t66 * t190 * t194 / f64x8::splat(100.0);
            let t198 = t197 * t197;
            let t199 = param_k1 + f64x8::splat(5.0) / f64x8::splat(972.0) * t161 + t50 * t168 * t173 / f64x8::splat(576.0) + t198;
            let t204 = f64x8::splat(1.0) + param_k1 * (f64x8::splat(1.0) - param_k1 / t199);
            let t205 = (t189).simd_le(f64x8::splat(2.5));
            let t206 = (f64x8::splat(2.5)).simd_lt(t189);
            let t207 = ((t206).select(f64x8::splat(2.5), t189));
            let t209 = t207 * t207;
            let t211 = t209 * t207;
            let t213 = t209 * t209;
            let t215 = t213 * t207;
            let t217 = t213 * t209;
            let t222 = ((t206).select(t189, f64x8::splat(2.5)));
            let t223 = f64x8::splat(1.0) - t222;
            let t226 = (simd::exp(param_c2 / t223));
            let t228 = ((t205).select(f64x8::splat(1.0) - f64x8::splat(0.667) * t207 - f64x8::splat(0.4445555) * t209 - f64x8::splat(0.663086601049) * t211 + f64x8::splat(1.45129704449) * t213 - f64x8::splat(0.887998041597) * t215 + f64x8::splat(0.234528941479) * t217 - f64x8::splat(0.023185843322) * t213 * t211, -param_d * t226));
            let t229 = f64x8::splat(1.0) - t228;
            let t232 = t204 * t229 + f64x8::splat(1.174) * t228;
            let t233 = t28 * t232;
            let t234 = ((v_sigma2).sqrt());
            let t235 = t156 * v_rho1;
            let t236 = f64x8::splat(1.0) / t235;
            let t238 = t129 * t234 * t236;
            let t239 = ((t238).sqrt());
            let t243 = (simd::exp(-f64x8::splat(9.8958) * t127 / t239));
            let t244 = f64x8::splat(1.0) - t243;
            let t245 = t233 * t244;
            let t248 = ((t145).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t154 * t245));
            let tzk0 = t144 + t248;
            acc_zk = tzk0;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        ip += 8;
    }
}
