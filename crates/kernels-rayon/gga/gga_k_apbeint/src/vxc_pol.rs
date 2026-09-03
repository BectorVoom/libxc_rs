//! GGA_K_APBEINT vxc pol kernel — explicit SIMD (bit-exact).
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
pub fn gga_k_apbeint_vxc_pol(
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
            let t32 = param_muPBE - param_muGE;
            let t34 = f64x8::splat(M_CBRT6);
            let t35 = t32 * param_alpha * t34;
            let t36 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t37 = (simd::cbrt(t36));
            let t38 = t37 * t37;
            let t39 = f64x8::splat(1.0) / t38;
            let t40 = t39 * v_sigma0;
            let t41 = v_rho0 * v_rho0;
            let t42 = (simd::cbrt(v_rho0));
            let t43 = t42 * t42;
            let t45 = f64x8::splat(1.0) / t43 / t41;
            let t46 = param_alpha * t34;
            let t47 = t40 * t45;
            let t50 = f64x8::splat(1.0) + t46 * t47 / f64x8::splat(24.0);
            let t51 = f64x8::splat(1.0) / t50;
            let t57 = (param_muGE + t35 * t40 * t45 * t51 / f64x8::splat(24.0)) * t34;
            let t60 = param_kappa + t57 * t47 / f64x8::splat(24.0);
            let t65 = f64x8::splat(1.0) + param_kappa * (f64x8::splat(1.0) - param_kappa / t60);
            let t69 = ((t1).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t31 * t65));
            let t70 = (v_rho1).simd_le(dens_threshold);
            let t71 = -t17;
            let t73 = ((t15).select(t12, (t11).select(t16, t71 * t8)));
            let t74 = f64x8::splat(1.0) + t73;
            let t75 = (t74).simd_le(zeta_threshold);
            let t76 = (simd::cbrt(t74));
            let t77 = t76 * t76;
            let t79 = ((t75).select(t24, t77 * t74));
            let t80 = t79 * t30;
            let t81 = t39 * v_sigma2;
            let t82 = v_rho1 * v_rho1;
            let t83 = (simd::cbrt(v_rho1));
            let t84 = t83 * t83;
            let t86 = f64x8::splat(1.0) / t84 / t82;
            let t87 = t81 * t86;
            let t90 = f64x8::splat(1.0) + t46 * t87 / f64x8::splat(24.0);
            let t91 = f64x8::splat(1.0) / t90;
            let t97 = (param_muGE + t35 * t81 * t86 * t91 / f64x8::splat(24.0)) * t34;
            let t100 = param_kappa + t97 * t87 / f64x8::splat(24.0);
            let t105 = f64x8::splat(1.0) + param_kappa * (f64x8::splat(1.0) - param_kappa / t100);
            let t109 = ((t70).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t80 * t105));
            let tzk0 = t69 + t109;
            acc_zk = tzk0;
            let t110 = t7 * t7;
            let t111 = f64x8::splat(1.0) / t110;
            let t112 = t17 * t111;
            let t114 = ((t11).select(f64x8::splat(0.0), (t15).select(f64x8::splat(0.0), t8 - t112)));
            let t117 = ((t21).select(f64x8::splat(0.0), f64x8::splat(5.0) / f64x8::splat(3.0) * t26 * t114));
            let t118 = t117 * t30;
            let t122 = f64x8::splat(1.0) / t29;
            let t123 = t28 * t122;
            let t126 = t6 * t123 * t65 / f64x8::splat(10.0);
            let t127 = t6 * t28;
            let t128 = param_kappa * param_kappa;
            let t129 = t30 * t128;
            let t130 = t60 * t60;
            let t131 = f64x8::splat(1.0) / t130;
            let t132 = t41 * v_rho0;
            let t134 = f64x8::splat(1.0) / t43 / t132;
            let t139 = param_alpha * param_alpha;
            let t141 = t34 * t34;
            let t142 = t32 * t139 * t141;
            let t144 = f64x8::splat(1.0) / t37 / t36;
            let t145 = v_sigma0 * v_sigma0;
            let t146 = t144 * t145;
            let t147 = t41 * t41;
            let t148 = t147 * t41;
            let t150 = f64x8::splat(1.0) / t42 / t148;
            let t151 = t50 * t50;
            let t152 = f64x8::splat(1.0) / t151;
            let t158 = (-t35 * t40 * t134 * t51 / f64x8::splat(9.0) + t142 * t146 * t150 * t152 / f64x8::splat(216.0)) * t34;
            let t161 = t40 * t134;
            let t164 = t158 * t47 / f64x8::splat(24.0) - t57 * t161 / f64x8::splat(9.0);
            let t165 = t131 * t164;
            let t166 = t129 * t165;
            let t170 = ((t1).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t118 * t65 + t126 + f64x8::splat(3.0) / f64x8::splat(20.0) * t127 * t166));
            let t171 = t71 * t111;
            let t173 = ((t15).select(f64x8::splat(0.0), (t11).select(f64x8::splat(0.0), -t8 - t171)));
            let t176 = ((t75).select(f64x8::splat(0.0), f64x8::splat(5.0) / f64x8::splat(3.0) * t77 * t173));
            let t177 = t176 * t30;
            let t181 = t79 * t122;
            let t184 = t6 * t181 * t105 / f64x8::splat(10.0);
            let t186 = ((t70).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t177 * t105 + t184));
            let tvrho0 = t69 + t109 + t7 * (t170 + t186);
            acc_vrho_0 = tvrho0;
            let t190 = ((t11).select(f64x8::splat(0.0), (t15).select(f64x8::splat(0.0), -t8 - t112)));
            let t193 = ((t21).select(f64x8::splat(0.0), f64x8::splat(5.0) / f64x8::splat(3.0) * t26 * t190));
            let t194 = t193 * t30;
            let t199 = ((t1).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t194 * t65 + t126));
            let t201 = ((t15).select(f64x8::splat(0.0), (t11).select(f64x8::splat(0.0), t8 - t171)));
            let t204 = ((t75).select(f64x8::splat(0.0), f64x8::splat(5.0) / f64x8::splat(3.0) * t77 * t201));
            let t205 = t204 * t30;
            let t209 = t6 * t79;
            let t210 = t100 * t100;
            let t211 = f64x8::splat(1.0) / t210;
            let t212 = t82 * v_rho1;
            let t214 = f64x8::splat(1.0) / t84 / t212;
            let t219 = v_sigma2 * v_sigma2;
            let t220 = t144 * t219;
            let t221 = t82 * t82;
            let t222 = t221 * t82;
            let t224 = f64x8::splat(1.0) / t83 / t222;
            let t225 = t90 * t90;
            let t226 = f64x8::splat(1.0) / t225;
            let t232 = (-t35 * t81 * t214 * t91 / f64x8::splat(9.0) + t142 * t220 * t224 * t226 / f64x8::splat(216.0)) * t34;
            let t235 = t81 * t214;
            let t238 = t232 * t87 / f64x8::splat(24.0) - t97 * t235 / f64x8::splat(9.0);
            let t239 = t211 * t238;
            let t240 = t129 * t239;
            let t244 = ((t70).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t205 * t105 + t184 + f64x8::splat(3.0) / f64x8::splat(20.0) * t209 * t240));
            let tvrho1 = t69 + t109 + t7 * (t199 + t244);
            acc_vrho_1 = tvrho1;
            let t247 = t39 * t45;
            let t252 = t147 * v_rho0;
            let t254 = f64x8::splat(1.0) / t42 / t252;
            let t260 = (t35 * t247 * t51 / f64x8::splat(24.0) - t142 * t144 * v_sigma0 * t254 * t152 / f64x8::splat(576.0)) * t34;
            let t264 = t57 * t247 / f64x8::splat(24.0) + t260 * t47 / f64x8::splat(24.0);
            let t265 = t131 * t264;
            let t266 = t129 * t265;
            let t269 = ((t1).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t127 * t266));
            let tvsigma0 = t7 * t269;
            acc_vsigma_0 = tvsigma0;
            let tvsigma1 = f64x8::splat(0.0);
            acc_vsigma_1 = tvsigma1;
            let t270 = t39 * t86;
            let t275 = t221 * v_rho1;
            let t277 = f64x8::splat(1.0) / t83 / t275;
            let t283 = (t35 * t270 * t91 / f64x8::splat(24.0) - t142 * t144 * v_sigma2 * t277 * t226 / f64x8::splat(576.0)) * t34;
            let t287 = t97 * t270 / f64x8::splat(24.0) + t283 * t87 / f64x8::splat(24.0);
            let t288 = t211 * t287;
            let t289 = t129 * t288;
            let t292 = ((t70).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t209 * t289));
            let tvsigma2 = t7 * t292;
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
