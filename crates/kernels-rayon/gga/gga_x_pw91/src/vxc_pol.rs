//! GGA_X_PW91 vxc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_pw91.c`
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
pub fn gga_x_pw91_vxc_pol(
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
            let t29 = param_alpha * t28;
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
            let t43 = (simd::exp(-t29 * t40 / f64x8::splat(24.0)));
            let t46 = (param_d * t43 + param_c) * t28;
            let t49 = t28 * t28;
            let t50 = f64x8::splat(1.0) / t31;
            let t51 = t49 * t50;
            let t52 = ((v_sigma0).sqrt());
            let t54 = f64x8::splat(1.0) / t36 / v_rho0;
            let t58 = (simd::pow(t51 * t52 * t54 / f64x8::splat(12.0), param_expo));
            let t59 = param_f * t58;
            let t60 = t46 * t40 / f64x8::splat(24.0) - t59;
            let t61 = t51 * t52;
            let t63 = param_b * t49;
            let t68 = (simd::ln(t63 * t50 * t52 * t54 / f64x8::splat(12.0) + ((((t63 * t50 * t52 * t54 / f64x8::splat(12.0)) * (t63 * t50 * t52 * t54 / f64x8::splat(12.0))) + f64x8::splat(1.0)).sqrt())));
            let t69 = t54 * param_a * t68;
            let t72 = f64x8::splat(1.0) + t61 * t69 / f64x8::splat(12.0) + t59;
            let t73 = f64x8::splat(1.0) / t72;
            let t75 = t60 * t73 + f64x8::splat(1.0);
            let t79 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t27 * t75));
            let t80 = (v_rho1).simd_le(dens_threshold);
            let t81 = -t16;
            let t83 = ((t14).select(t11, (t10).select(t15, t81 * t7)));
            let t84 = f64x8::splat(1.0) + t83;
            let t85 = (t84).simd_le(zeta_threshold);
            let t86 = (simd::cbrt(t84));
            let t88 = ((t85).select(t22, t86 * t84));
            let t89 = t88 * t26;
            let t90 = t33 * v_sigma2;
            let t91 = v_rho1 * v_rho1;
            let t92 = (simd::cbrt(v_rho1));
            let t93 = t92 * t92;
            let t95 = f64x8::splat(1.0) / t93 / t91;
            let t96 = t90 * t95;
            let t99 = (simd::exp(-t29 * t96 / f64x8::splat(24.0)));
            let t102 = (param_d * t99 + param_c) * t28;
            let t105 = ((v_sigma2).sqrt());
            let t107 = f64x8::splat(1.0) / t92 / v_rho1;
            let t111 = (simd::pow(t51 * t105 * t107 / f64x8::splat(12.0), param_expo));
            let t112 = param_f * t111;
            let t113 = t102 * t96 / f64x8::splat(24.0) - t112;
            let t114 = t51 * t105;
            let t120 = (simd::ln(t63 * t50 * t105 * t107 / f64x8::splat(12.0) + ((((t63 * t50 * t105 * t107 / f64x8::splat(12.0)) * (t63 * t50 * t105 * t107 / f64x8::splat(12.0))) + f64x8::splat(1.0)).sqrt())));
            let t121 = t107 * param_a * t120;
            let t124 = f64x8::splat(1.0) + t114 * t121 / f64x8::splat(12.0) + t112;
            let t125 = f64x8::splat(1.0) / t124;
            let t127 = t113 * t125 + f64x8::splat(1.0);
            let t131 = ((t80).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t89 * t127));
            let tzk0 = t79 + t131;
            acc_zk = tzk0;
            let t132 = t6 * t6;
            let t133 = f64x8::splat(1.0) / t132;
            let t134 = t16 * t133;
            let t136 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), t7 - t134)));
            let t139 = ((t20).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t136));
            let t140 = t139 * t26;
            let t144 = t26 * t26;
            let t145 = f64x8::splat(1.0) / t144;
            let t146 = t25 * t145;
            let t149 = t5 * t146 * t75 / f64x8::splat(8.0);
            let t151 = param_d * param_alpha * t49;
            let t153 = f64x8::splat(1.0) / t31 / t30;
            let t154 = v_sigma0 * v_sigma0;
            let t155 = t153 * t154;
            let t156 = t35 * t35;
            let t157 = t156 * t35;
            let t159 = f64x8::splat(1.0) / t36 / t157;
            let t164 = t35 * v_rho0;
            let t166 = f64x8::splat(1.0) / t37 / t164;
            let t170 = f64x8::splat(1.0) / v_rho0;
            let t173 = f64x8::splat(4.0) / f64x8::splat(3.0) * t59 * param_expo * t170;
            let t174 = t151 * t155 * t159 * t43 / f64x8::splat(216.0) - t46 * t34 * t166 / f64x8::splat(9.0) + t173;
            let t176 = t72 * t72;
            let t177 = f64x8::splat(1.0) / t176;
            let t178 = t60 * t177;
            let t182 = f64x8::splat(1.0) / t36 / t35 * param_a * t68;
            let t185 = t28 * t33;
            let t186 = t185 * v_sigma0;
            let t188 = param_b * param_b;
            let t189 = t188 * t28;
            let t192 = f64x8::splat(6.0) * t189 * t40 + f64x8::splat(144.0);
            let t193 = ((t192).sqrt());
            let t194 = f64x8::splat(1.0) / t193;
            let t195 = param_b * t194;
            let t196 = t166 * param_a * t195;
            let t199 = -t61 * t182 / f64x8::splat(9.0) - f64x8::splat(2.0) / f64x8::splat(3.0) * t186 * t196 - t173;
            let t201 = t174 * t73 - t178 * t199;
            let t206 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t140 * t75 - t149 - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t27 * t201));
            let t207 = t81 * t133;
            let t209 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), -t7 - t207)));
            let t212 = ((t85).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t86 * t209));
            let t213 = t212 * t26;
            let t217 = t88 * t145;
            let t220 = t5 * t217 * t127 / f64x8::splat(8.0);
            let t222 = ((t80).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t213 * t127 - t220));
            let tvrho0 = t79 + t131 + t6 * (t206 + t222);
            acc_vrho_0 = tvrho0;
            let t226 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), -t7 - t134)));
            let t229 = ((t20).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t226));
            let t230 = t229 * t26;
            let t235 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t230 * t75 - t149));
            let t237 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), t7 - t207)));
            let t240 = ((t85).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t86 * t237));
            let t241 = t240 * t26;
            let t245 = v_sigma2 * v_sigma2;
            let t246 = t153 * t245;
            let t247 = t91 * t91;
            let t248 = t247 * t91;
            let t250 = f64x8::splat(1.0) / t92 / t248;
            let t255 = t91 * v_rho1;
            let t257 = f64x8::splat(1.0) / t93 / t255;
            let t261 = f64x8::splat(1.0) / v_rho1;
            let t264 = f64x8::splat(4.0) / f64x8::splat(3.0) * t112 * param_expo * t261;
            let t265 = t151 * t246 * t250 * t99 / f64x8::splat(216.0) - t102 * t90 * t257 / f64x8::splat(9.0) + t264;
            let t267 = t124 * t124;
            let t268 = f64x8::splat(1.0) / t267;
            let t269 = t113 * t268;
            let t273 = f64x8::splat(1.0) / t92 / t91 * param_a * t120;
            let t276 = t185 * v_sigma2;
            let t280 = f64x8::splat(6.0) * t189 * t96 + f64x8::splat(144.0);
            let t281 = ((t280).sqrt());
            let t282 = f64x8::splat(1.0) / t281;
            let t283 = param_b * t282;
            let t284 = t257 * param_a * t283;
            let t287 = -t114 * t273 / f64x8::splat(9.0) - f64x8::splat(2.0) / f64x8::splat(3.0) * t276 * t284 - t264;
            let t289 = t265 * t125 - t269 * t287;
            let t294 = ((t80).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t241 * t127 - t220 - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t89 * t289));
            let tvrho1 = t79 + t131 + t6 * (t235 + t294);
            acc_vrho_1 = tvrho1;
            let t297 = t156 * v_rho0;
            let t299 = f64x8::splat(1.0) / t36 / t297;
            let t300 = t153 * t299;
            let t301 = t43 * v_sigma0;
            let t308 = f64x8::splat(1.0) / v_sigma0;
            let t311 = t59 * param_expo * t308 / f64x8::splat(2.0);
            let t312 = -t151 * t300 * t301 / f64x8::splat(576.0) + t46 * t33 * t39 / f64x8::splat(24.0) - t311;
            let t315 = t51 / t52;
            let t319 = param_a * param_b;
            let t320 = t319 * t194;
            let t323 = t315 * t69 / f64x8::splat(24.0) + t185 * t39 * t320 / f64x8::splat(4.0) + t311;
            let t325 = -t178 * t323 + t312 * t73;
            let t329 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t27 * t325));
            let tvsigma0 = t6 * t329;
            acc_vsigma_0 = tvsigma0;
            let tvsigma1 = f64x8::splat(0.0);
            acc_vsigma_1 = tvsigma1;
            let t330 = t247 * v_rho1;
            let t332 = f64x8::splat(1.0) / t92 / t330;
            let t333 = t153 * t332;
            let t334 = t99 * v_sigma2;
            let t341 = f64x8::splat(1.0) / v_sigma2;
            let t344 = t112 * param_expo * t341 / f64x8::splat(2.0);
            let t345 = -t151 * t333 * t334 / f64x8::splat(576.0) + t102 * t33 * t95 / f64x8::splat(24.0) - t344;
            let t348 = t51 / t105;
            let t352 = t319 * t282;
            let t355 = t348 * t121 / f64x8::splat(24.0) + t185 * t95 * t352 / f64x8::splat(4.0) + t344;
            let t357 = t345 * t125 - t269 * t355;
            let t361 = ((t80).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t89 * t357));
            let tvsigma2 = t6 * t361;
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
