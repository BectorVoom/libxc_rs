//! GGA_K_LC94 vxc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_k_lc94.c`
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
pub fn gga_k_lc94_vxc_pol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    param_a: f64,
    param_alpha: f64,
    param_b: f64,
    param_c: f64,
    param_d: f64,
    param_expo: f64,
    param_f: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_a = f64x8::splat(param_a);
    let param_alpha = f64x8::splat(param_alpha);
    let param_b = f64x8::splat(param_b);
    let param_c = f64x8::splat(param_c);
    let param_d = f64x8::splat(param_d);
    let param_expo = f64x8::splat(param_expo);
    let param_f = f64x8::splat(param_f);
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
            let t32 = f64x8::splat(M_CBRT6);
            let t33 = param_alpha * t32;
            let t34 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t35 = (simd::cbrt(t34));
            let t36 = t35 * t35;
            let t37 = f64x8::splat(1.0) / t36;
            let t38 = t37 * v_sigma0;
            let t39 = v_rho0 * v_rho0;
            let t40 = (simd::cbrt(v_rho0));
            let t41 = t40 * t40;
            let t43 = f64x8::splat(1.0) / t41 / t39;
            let t44 = t38 * t43;
            let t47 = (simd::exp(-t33 * t44 / f64x8::splat(24.0)));
            let t50 = (param_d * t47 + param_c) * t32;
            let t53 = t32 * t32;
            let t54 = f64x8::splat(1.0) / t35;
            let t55 = t53 * t54;
            let t56 = ((v_sigma0).sqrt());
            let t58 = f64x8::splat(1.0) / t40 / v_rho0;
            let t62 = (simd::pow(t55 * t56 * t58 / f64x8::splat(12.0), param_expo));
            let t63 = param_f * t62;
            let t64 = t50 * t44 / f64x8::splat(24.0) - t63;
            let t65 = t55 * t56;
            let t67 = param_b * t53;
            let t72 = (simd::ln(t67 * t54 * t56 * t58 / f64x8::splat(12.0) + ((((t67 * t54 * t56 * t58 / f64x8::splat(12.0)) * (t67 * t54 * t56 * t58 / f64x8::splat(12.0))) + f64x8::splat(1.0)).sqrt())));
            let t73 = t58 * param_a * t72;
            let t76 = f64x8::splat(1.0) + t65 * t73 / f64x8::splat(12.0) + t63;
            let t77 = f64x8::splat(1.0) / t76;
            let t79 = t64 * t77 + f64x8::splat(1.0);
            let t83 = ((t1).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t31 * t79));
            let t84 = (v_rho1).simd_le(dens_threshold);
            let t85 = -t17;
            let t87 = ((t15).select(t12, (t11).select(t16, t85 * t8)));
            let t88 = f64x8::splat(1.0) + t87;
            let t89 = (t88).simd_le(zeta_threshold);
            let t90 = (simd::cbrt(t88));
            let t91 = t90 * t90;
            let t93 = ((t89).select(t24, t91 * t88));
            let t94 = t93 * t30;
            let t95 = t37 * v_sigma2;
            let t96 = v_rho1 * v_rho1;
            let t97 = (simd::cbrt(v_rho1));
            let t98 = t97 * t97;
            let t100 = f64x8::splat(1.0) / t98 / t96;
            let t101 = t95 * t100;
            let t104 = (simd::exp(-t33 * t101 / f64x8::splat(24.0)));
            let t107 = (param_d * t104 + param_c) * t32;
            let t110 = ((v_sigma2).sqrt());
            let t112 = f64x8::splat(1.0) / t97 / v_rho1;
            let t116 = (simd::pow(t55 * t110 * t112 / f64x8::splat(12.0), param_expo));
            let t117 = param_f * t116;
            let t118 = t107 * t101 / f64x8::splat(24.0) - t117;
            let t119 = t55 * t110;
            let t125 = (simd::ln(t67 * t54 * t110 * t112 / f64x8::splat(12.0) + ((((t67 * t54 * t110 * t112 / f64x8::splat(12.0)) * (t67 * t54 * t110 * t112 / f64x8::splat(12.0))) + f64x8::splat(1.0)).sqrt())));
            let t126 = t112 * param_a * t125;
            let t129 = f64x8::splat(1.0) + t119 * t126 / f64x8::splat(12.0) + t117;
            let t130 = f64x8::splat(1.0) / t129;
            let t132 = t118 * t130 + f64x8::splat(1.0);
            let t136 = ((t84).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t94 * t132));
            let tzk0 = t83 + t136;
            acc_zk = tzk0;
            let t137 = t7 * t7;
            let t138 = f64x8::splat(1.0) / t137;
            let t139 = t17 * t138;
            let t141 = ((t11).select(f64x8::splat(0.0), (t15).select(f64x8::splat(0.0), t8 - t139)));
            let t144 = ((t21).select(f64x8::splat(0.0), f64x8::splat(5.0) / f64x8::splat(3.0) * t26 * t141));
            let t145 = t144 * t30;
            let t149 = f64x8::splat(1.0) / t29;
            let t150 = t28 * t149;
            let t153 = t6 * t150 * t79 / f64x8::splat(10.0);
            let t155 = param_d * param_alpha * t53;
            let t157 = f64x8::splat(1.0) / t35 / t34;
            let t158 = v_sigma0 * v_sigma0;
            let t159 = t157 * t158;
            let t160 = t39 * t39;
            let t161 = t160 * t39;
            let t163 = f64x8::splat(1.0) / t40 / t161;
            let t168 = t39 * v_rho0;
            let t170 = f64x8::splat(1.0) / t41 / t168;
            let t174 = f64x8::splat(1.0) / v_rho0;
            let t177 = f64x8::splat(4.0) / f64x8::splat(3.0) * t63 * param_expo * t174;
            let t178 = t155 * t159 * t163 * t47 / f64x8::splat(216.0) - t50 * t38 * t170 / f64x8::splat(9.0) + t177;
            let t180 = t76 * t76;
            let t181 = f64x8::splat(1.0) / t180;
            let t182 = t64 * t181;
            let t186 = f64x8::splat(1.0) / t40 / t39 * param_a * t72;
            let t189 = t32 * t37;
            let t190 = t189 * v_sigma0;
            let t192 = param_b * param_b;
            let t193 = t192 * t32;
            let t196 = f64x8::splat(6.0) * t193 * t44 + f64x8::splat(144.0);
            let t197 = ((t196).sqrt());
            let t198 = f64x8::splat(1.0) / t197;
            let t199 = param_b * t198;
            let t200 = t170 * param_a * t199;
            let t203 = -t65 * t186 / f64x8::splat(9.0) - f64x8::splat(2.0) / f64x8::splat(3.0) * t190 * t200 - t177;
            let t205 = t178 * t77 - t182 * t203;
            let t210 = ((t1).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t145 * t79 + t153 + f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t31 * t205));
            let t211 = t85 * t138;
            let t213 = ((t15).select(f64x8::splat(0.0), (t11).select(f64x8::splat(0.0), -t8 - t211)));
            let t216 = ((t89).select(f64x8::splat(0.0), f64x8::splat(5.0) / f64x8::splat(3.0) * t91 * t213));
            let t217 = t216 * t30;
            let t221 = t93 * t149;
            let t224 = t6 * t221 * t132 / f64x8::splat(10.0);
            let t226 = ((t84).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t217 * t132 + t224));
            let tvrho0 = t83 + t136 + t7 * (t210 + t226);
            acc_vrho_0 = tvrho0;
            let t230 = ((t11).select(f64x8::splat(0.0), (t15).select(f64x8::splat(0.0), -t8 - t139)));
            let t233 = ((t21).select(f64x8::splat(0.0), f64x8::splat(5.0) / f64x8::splat(3.0) * t26 * t230));
            let t234 = t233 * t30;
            let t239 = ((t1).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t234 * t79 + t153));
            let t241 = ((t15).select(f64x8::splat(0.0), (t11).select(f64x8::splat(0.0), t8 - t211)));
            let t244 = ((t89).select(f64x8::splat(0.0), f64x8::splat(5.0) / f64x8::splat(3.0) * t91 * t241));
            let t245 = t244 * t30;
            let t249 = v_sigma2 * v_sigma2;
            let t250 = t157 * t249;
            let t251 = t96 * t96;
            let t252 = t251 * t96;
            let t254 = f64x8::splat(1.0) / t97 / t252;
            let t259 = t96 * v_rho1;
            let t261 = f64x8::splat(1.0) / t98 / t259;
            let t265 = f64x8::splat(1.0) / v_rho1;
            let t268 = f64x8::splat(4.0) / f64x8::splat(3.0) * t117 * param_expo * t265;
            let t269 = t155 * t250 * t254 * t104 / f64x8::splat(216.0) - t107 * t95 * t261 / f64x8::splat(9.0) + t268;
            let t271 = t129 * t129;
            let t272 = f64x8::splat(1.0) / t271;
            let t273 = t118 * t272;
            let t277 = f64x8::splat(1.0) / t97 / t96 * param_a * t125;
            let t280 = t189 * v_sigma2;
            let t284 = f64x8::splat(6.0) * t193 * t101 + f64x8::splat(144.0);
            let t285 = ((t284).sqrt());
            let t286 = f64x8::splat(1.0) / t285;
            let t287 = param_b * t286;
            let t288 = t261 * param_a * t287;
            let t291 = -t119 * t277 / f64x8::splat(9.0) - f64x8::splat(2.0) / f64x8::splat(3.0) * t280 * t288 - t268;
            let t293 = t269 * t130 - t273 * t291;
            let t298 = ((t84).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t245 * t132 + t224 + f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t94 * t293));
            let tvrho1 = t83 + t136 + t7 * (t239 + t298);
            acc_vrho_1 = tvrho1;
            let t301 = t160 * v_rho0;
            let t303 = f64x8::splat(1.0) / t40 / t301;
            let t304 = t157 * t303;
            let t305 = t47 * v_sigma0;
            let t312 = f64x8::splat(1.0) / v_sigma0;
            let t315 = t63 * param_expo * t312 / f64x8::splat(2.0);
            let t316 = -t155 * t304 * t305 / f64x8::splat(576.0) + t50 * t37 * t43 / f64x8::splat(24.0) - t315;
            let t319 = t55 / t56;
            let t323 = param_a * param_b;
            let t324 = t323 * t198;
            let t327 = t319 * t73 / f64x8::splat(24.0) + t189 * t43 * t324 / f64x8::splat(4.0) + t315;
            let t329 = -t182 * t327 + t316 * t77;
            let t333 = ((t1).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t31 * t329));
            let tvsigma0 = t7 * t333;
            acc_vsigma_0 = tvsigma0;
            let tvsigma1 = f64x8::splat(0.0);
            acc_vsigma_1 = tvsigma1;
            let t334 = t251 * v_rho1;
            let t336 = f64x8::splat(1.0) / t97 / t334;
            let t337 = t157 * t336;
            let t338 = t104 * v_sigma2;
            let t345 = f64x8::splat(1.0) / v_sigma2;
            let t348 = t117 * param_expo * t345 / f64x8::splat(2.0);
            let t349 = -t155 * t337 * t338 / f64x8::splat(576.0) + t107 * t37 * t100 / f64x8::splat(24.0) - t348;
            let t352 = t55 / t110;
            let t356 = t323 * t286;
            let t359 = t352 * t126 / f64x8::splat(24.0) + t189 * t100 * t356 / f64x8::splat(4.0) + t348;
            let t361 = t349 * t130 - t273 * t359;
            let t365 = ((t84).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t94 * t361));
            let tvsigma2 = t7 * t365;
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
