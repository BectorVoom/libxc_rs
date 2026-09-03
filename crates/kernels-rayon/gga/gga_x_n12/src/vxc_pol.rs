//! GGA_X_N12 vxc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_n12.c`
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
pub fn gga_x_n12_vxc_pol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    param_CC_0_1: f64,
    param_CC_0_2: f64,
    param_CC_0_3: f64,
    param_CC_1_1: f64,
    param_CC_1_2: f64,
    param_CC_1_3: f64,
    param_CC_1_0: f64,
    param_CC_2_1: f64,
    param_CC_2_2: f64,
    param_CC_2_3: f64,
    param_CC_2_0: f64,
    param_CC_3_1: f64,
    param_CC_3_2: f64,
    param_CC_3_3: f64,
    param_CC_3_0: f64,
    param_CC_0_0: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_CC_0_1 = f64x8::splat(param_CC_0_1);
    let param_CC_0_2 = f64x8::splat(param_CC_0_2);
    let param_CC_0_3 = f64x8::splat(param_CC_0_3);
    let param_CC_1_1 = f64x8::splat(param_CC_1_1);
    let param_CC_1_2 = f64x8::splat(param_CC_1_2);
    let param_CC_1_3 = f64x8::splat(param_CC_1_3);
    let param_CC_1_0 = f64x8::splat(param_CC_1_0);
    let param_CC_2_1 = f64x8::splat(param_CC_2_1);
    let param_CC_2_2 = f64x8::splat(param_CC_2_2);
    let param_CC_2_3 = f64x8::splat(param_CC_2_3);
    let param_CC_2_0 = f64x8::splat(param_CC_2_0);
    let param_CC_3_1 = f64x8::splat(param_CC_3_1);
    let param_CC_3_2 = f64x8::splat(param_CC_3_2);
    let param_CC_3_3 = f64x8::splat(param_CC_3_3);
    let param_CC_3_0 = f64x8::splat(param_CC_3_0);
    let param_CC_0_0 = f64x8::splat(param_CC_0_0);
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
            let t17 = t16 * t7;
            let t18 = ((t10).select(t11, (t14).select(t15, t17)));
            let t19 = f64x8::splat(1.0) + t18;
            let t20 = (t19).simd_le(zeta_threshold);
            let t21 = (simd::cbrt(zeta_threshold));
            let t22 = t21 * zeta_threshold;
            let t23 = (simd::cbrt(t19));
            let t25 = ((t20).select(t22, t23 * t19));
            let t26 = (simd::cbrt(t6));
            let t27 = t25 * t26;
            let t28 = param_CC_0_0;
            let t29 = param_CC_0_1;
            let t30 = t29 * v_sigma0;
            let t31 = v_rho0 * v_rho0;
            let t32 = (simd::cbrt(v_rho0));
            let t33 = t32 * t32;
            let t35 = f64x8::splat(1.0) / t33 / t31;
            let t38 = f64x8::splat(1.0) + f64x8::splat(0.004) * v_sigma0 * t35;
            let t39 = f64x8::splat(1.0) / t38;
            let t40 = t35 * t39;
            let t43 = param_CC_0_2;
            let t44 = v_sigma0 * v_sigma0;
            let t45 = t43 * t44;
            let t46 = t31 * t31;
            let t47 = t46 * v_rho0;
            let t49 = f64x8::splat(1.0) / t32 / t47;
            let t50 = t38 * t38;
            let t51 = f64x8::splat(1.0) / t50;
            let t52 = t49 * t51;
            let t55 = param_CC_0_3;
            let t56 = t44 * v_sigma0;
            let t57 = t55 * t56;
            let t58 = t46 * t46;
            let t59 = f64x8::splat(1.0) / t58;
            let t60 = t50 * t38;
            let t61 = f64x8::splat(1.0) / t60;
            let t62 = t59 * t61;
            let t65 = param_CC_1_0;
            let t66 = param_CC_1_1;
            let t67 = t66 * v_sigma0;
            let t70 = param_CC_1_2;
            let t71 = t70 * t44;
            let t74 = param_CC_1_3;
            let t75 = t74 * t56;
            let t78 = t65 + f64x8::splat(0.004) * t67 * t40 + f64x8::splat(1.6e-05) * t71 * t52 + f64x8::splat(6.4e-08) * t75 * t62;
            let t80 = f64x8::splat(M_CBRT2);
            let t81 = f64x8::splat(1.0) / t26 * t80;
            let t83 = (f64x8::splat(1.0) + t17).simd_le(zeta_threshold);
            let t85 = (f64x8::splat(1.0) - t17).simd_le(zeta_threshold);
            let t86 = ((t83).select(t11, (t85).select(t15, t17)));
            let t87 = f64x8::splat(1.0) + t86;
            let t88 = (t87).simd_le(zeta_threshold);
            let t89 = f64x8::splat(1.0) / t21;
            let t90 = (simd::cbrt(t87));
            let t92 = ((t88).select(t89, f64x8::splat(1.0) / t90));
            let t95 = f64x8::splat(1.0) + f64x8::splat(0.4) * t81 * t92;
            let t96 = f64x8::splat(1.0) / t95;
            let t98 = param_CC_2_0;
            let t99 = param_CC_2_1;
            let t100 = t99 * v_sigma0;
            let t103 = param_CC_2_2;
            let t104 = t103 * t44;
            let t107 = param_CC_2_3;
            let t108 = t107 * t56;
            let t111 = t98 + f64x8::splat(0.004) * t100 * t40 + f64x8::splat(1.6e-05) * t104 * t52 + f64x8::splat(6.4e-08) * t108 * t62;
            let t112 = t95 * t95;
            let t113 = f64x8::splat(1.0) / t112;
            let t115 = param_CC_3_0;
            let t116 = param_CC_3_1;
            let t117 = t116 * v_sigma0;
            let t120 = param_CC_3_2;
            let t121 = t120 * t44;
            let t124 = param_CC_3_3;
            let t125 = t124 * t56;
            let t128 = t115 + f64x8::splat(0.004) * t117 * t40 + f64x8::splat(1.6e-05) * t121 * t52 + f64x8::splat(6.4e-08) * t125 * t62;
            let t129 = t112 * t95;
            let t130 = f64x8::splat(1.0) / t129;
            let t132 = t28 + f64x8::splat(0.004) * t30 * t40 + f64x8::splat(1.6e-05) * t45 * t52 + f64x8::splat(6.4e-08) * t57 * t62 + t78 * t96 + t111 * t113 + t128 * t130;
            let t136 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t27 * t132));
            let t137 = (v_rho1).simd_le(dens_threshold);
            let t138 = -t16;
            let t140 = ((t14).select(t11, (t10).select(t15, t138 * t7)));
            let t141 = f64x8::splat(1.0) + t140;
            let t142 = (t141).simd_le(zeta_threshold);
            let t143 = (simd::cbrt(t141));
            let t145 = ((t142).select(t22, t143 * t141));
            let t146 = t145 * t26;
            let t147 = t29 * v_sigma2;
            let t148 = v_rho1 * v_rho1;
            let t149 = (simd::cbrt(v_rho1));
            let t150 = t149 * t149;
            let t152 = f64x8::splat(1.0) / t150 / t148;
            let t155 = f64x8::splat(1.0) + f64x8::splat(0.004) * v_sigma2 * t152;
            let t156 = f64x8::splat(1.0) / t155;
            let t157 = t152 * t156;
            let t160 = v_sigma2 * v_sigma2;
            let t161 = t43 * t160;
            let t162 = t148 * t148;
            let t163 = t162 * v_rho1;
            let t165 = f64x8::splat(1.0) / t149 / t163;
            let t166 = t155 * t155;
            let t167 = f64x8::splat(1.0) / t166;
            let t168 = t165 * t167;
            let t171 = t160 * v_sigma2;
            let t172 = t55 * t171;
            let t173 = t162 * t162;
            let t174 = f64x8::splat(1.0) / t173;
            let t175 = t166 * t155;
            let t176 = f64x8::splat(1.0) / t175;
            let t177 = t174 * t176;
            let t180 = t66 * v_sigma2;
            let t183 = t70 * t160;
            let t186 = t74 * t171;
            let t189 = t65 + f64x8::splat(0.004) * t180 * t157 + f64x8::splat(1.6e-05) * t183 * t168 + f64x8::splat(6.4e-08) * t186 * t177;
            let t190 = ((t85).select(t11, (t83).select(t15, -t17)));
            let t191 = f64x8::splat(1.0) + t190;
            let t192 = (t191).simd_le(zeta_threshold);
            let t193 = (simd::cbrt(t191));
            let t195 = ((t192).select(t89, f64x8::splat(1.0) / t193));
            let t198 = f64x8::splat(1.0) + f64x8::splat(0.4) * t81 * t195;
            let t199 = f64x8::splat(1.0) / t198;
            let t201 = t99 * v_sigma2;
            let t204 = t103 * t160;
            let t207 = t107 * t171;
            let t210 = t98 + f64x8::splat(0.004) * t201 * t157 + f64x8::splat(1.6e-05) * t204 * t168 + f64x8::splat(6.4e-08) * t207 * t177;
            let t211 = t198 * t198;
            let t212 = f64x8::splat(1.0) / t211;
            let t214 = t116 * v_sigma2;
            let t217 = t120 * t160;
            let t220 = t124 * t171;
            let t223 = t115 + f64x8::splat(0.004) * t214 * t157 + f64x8::splat(1.6e-05) * t217 * t168 + f64x8::splat(6.4e-08) * t220 * t177;
            let t224 = t211 * t198;
            let t225 = f64x8::splat(1.0) / t224;
            let t227 = t28 + f64x8::splat(0.004) * t147 * t157 + f64x8::splat(1.6e-05) * t161 * t168 + f64x8::splat(6.4e-08) * t172 * t177 + t189 * t199 + t210 * t212 + t223 * t225;
            let t231 = ((t137).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t146 * t227));
            let tzk0 = t136 + t231;
            acc_zk = tzk0;
            let t232 = t6 * t6;
            let t233 = f64x8::splat(1.0) / t232;
            let t234 = t16 * t233;
            let t235 = t7 - t234;
            let t236 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), t235)));
            let t239 = ((t20).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t236));
            let t240 = t239 * t26;
            let t244 = t26 * t26;
            let t245 = f64x8::splat(1.0) / t244;
            let t246 = t25 * t245;
            let t249 = t5 * t246 * t132 / f64x8::splat(8.0);
            let t250 = t31 * v_rho0;
            let t252 = f64x8::splat(1.0) / t33 / t250;
            let t253 = t252 * t39;
            let t256 = t29 * t44;
            let t257 = t46 * t31;
            let t259 = f64x8::splat(1.0) / t32 / t257;
            let t260 = t259 * t51;
            let t265 = t43 * t56;
            let t266 = t58 * v_rho0;
            let t267 = f64x8::splat(1.0) / t266;
            let t268 = t267 * t61;
            let t273 = t44 * t44;
            let t274 = t55 * t273;
            let t275 = t58 * t250;
            let t277 = f64x8::splat(1.0) / t33 / t275;
            let t278 = t50 * t50;
            let t279 = f64x8::splat(1.0) / t278;
            let t280 = t277 * t279;
            let t285 = t66 * t44;
            let t290 = t70 * t56;
            let t295 = t74 * t273;
            let t298 = -f64x8::splat(0.010666666666666666) * t67 * t253 + f64x8::splat(4.266666666666667e-05) * t285 * t260 - f64x8::splat(8.533333333333334e-05) * t71 * t260 + f64x8::splat(3.413333333333333e-07) * t290 * t268 - f64x8::splat(5.12e-07) * t75 * t268 + f64x8::splat(2.048e-09) * t295 * t280;
            let t300 = t78 * t113;
            let t303 = f64x8::splat(1.0) / t26 / t6 * t80;
            let t305 = f64x8::splat(0.13333333333333333) * t303 * t92;
            let t307 = f64x8::splat(1.0) / t90 / t87;
            let t308 = ((t83).select(f64x8::splat(0.0), (t85).select(f64x8::splat(0.0), t235)));
            let t311 = ((t88).select(f64x8::splat(0.0), -t307 * t308 / f64x8::splat(3.0)));
            let t314 = -t305 + f64x8::splat(0.4) * t81 * t311;
            let t318 = t99 * t44;
            let t323 = t103 * t56;
            let t328 = t107 * t273;
            let t331 = -f64x8::splat(0.010666666666666666) * t100 * t253 + f64x8::splat(4.266666666666667e-05) * t318 * t260 - f64x8::splat(8.533333333333334e-05) * t104 * t260 + f64x8::splat(3.413333333333333e-07) * t323 * t268 - f64x8::splat(5.12e-07) * t108 * t268 + f64x8::splat(2.048e-09) * t328 * t280;
            let t333 = t111 * t130;
            let t338 = t116 * t44;
            let t343 = t120 * t56;
            let t348 = t124 * t273;
            let t351 = -f64x8::splat(0.010666666666666666) * t117 * t253 + f64x8::splat(4.266666666666667e-05) * t338 * t260 - f64x8::splat(8.533333333333334e-05) * t121 * t260 + f64x8::splat(3.413333333333333e-07) * t343 * t268 - f64x8::splat(5.12e-07) * t125 * t268 + f64x8::splat(2.048e-09) * t348 * t280;
            let t353 = t112 * t112;
            let t354 = f64x8::splat(1.0) / t353;
            let t355 = t128 * t354;
            let t358 = -f64x8::splat(0.010666666666666666) * t30 * t253 + f64x8::splat(4.266666666666667e-05) * t256 * t260 - f64x8::splat(8.533333333333334e-05) * t45 * t260 + f64x8::splat(3.413333333333333e-07) * t265 * t268 - f64x8::splat(5.12e-07) * t57 * t268 + f64x8::splat(2.048e-09) * t274 * t280 + t298 * t96 - t300 * t314 + t331 * t113 - f64x8::splat(2.0) * t333 * t314 + t351 * t130 - f64x8::splat(3.0) * t355 * t314;
            let t363 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t240 * t132 - t249 - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t27 * t358));
            let t364 = t138 * t233;
            let t366 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), -t7 - t364)));
            let t369 = ((t142).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t143 * t366));
            let t370 = t369 * t26;
            let t374 = t145 * t245;
            let t377 = t5 * t374 * t227 / f64x8::splat(8.0);
            let t378 = t189 * t212;
            let t380 = f64x8::splat(0.13333333333333333) * t303 * t195;
            let t382 = f64x8::splat(1.0) / t193 / t191;
            let t384 = ((t85).select(f64x8::splat(0.0), (t83).select(f64x8::splat(0.0), -t235)));
            let t387 = ((t192).select(f64x8::splat(0.0), -t382 * t384 / f64x8::splat(3.0)));
            let t390 = -t380 + f64x8::splat(0.4) * t81 * t387;
            let t392 = t210 * t225;
            let t395 = t211 * t211;
            let t396 = f64x8::splat(1.0) / t395;
            let t397 = t223 * t396;
            let t400 = -t378 * t390 - f64x8::splat(2.0) * t392 * t390 - f64x8::splat(3.0) * t397 * t390;
            let t405 = ((t137).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t370 * t227 - t377 - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t146 * t400));
            let tvrho0 = t136 + t231 + t6 * (t363 + t405);
            acc_vrho_0 = tvrho0;
            let t408 = -t7 - t234;
            let t409 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), t408)));
            let t412 = ((t20).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t409));
            let t413 = t412 * t26;
            let t417 = ((t83).select(f64x8::splat(0.0), (t85).select(f64x8::splat(0.0), t408)));
            let t420 = ((t88).select(f64x8::splat(0.0), -t307 * t417 / f64x8::splat(3.0)));
            let t423 = -t305 + f64x8::splat(0.4) * t81 * t420;
            let t429 = -t300 * t423 - f64x8::splat(2.0) * t333 * t423 - f64x8::splat(3.0) * t355 * t423;
            let t434 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t413 * t132 - t249 - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t27 * t429));
            let t436 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), t7 - t364)));
            let t439 = ((t142).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t143 * t436));
            let t440 = t439 * t26;
            let t444 = t148 * v_rho1;
            let t446 = f64x8::splat(1.0) / t150 / t444;
            let t447 = t446 * t156;
            let t450 = t29 * t160;
            let t451 = t162 * t148;
            let t453 = f64x8::splat(1.0) / t149 / t451;
            let t454 = t453 * t167;
            let t459 = t43 * t171;
            let t460 = t173 * v_rho1;
            let t461 = f64x8::splat(1.0) / t460;
            let t462 = t461 * t176;
            let t467 = t160 * t160;
            let t468 = t55 * t467;
            let t469 = t173 * t444;
            let t471 = f64x8::splat(1.0) / t150 / t469;
            let t472 = t166 * t166;
            let t473 = f64x8::splat(1.0) / t472;
            let t474 = t471 * t473;
            let t479 = t66 * t160;
            let t484 = t70 * t171;
            let t489 = t74 * t467;
            let t492 = -f64x8::splat(0.010666666666666666) * t180 * t447 + f64x8::splat(4.266666666666667e-05) * t479 * t454 - f64x8::splat(8.533333333333334e-05) * t183 * t454 + f64x8::splat(3.413333333333333e-07) * t484 * t462 - f64x8::splat(5.12e-07) * t186 * t462 + f64x8::splat(2.048e-09) * t489 * t474;
            let t495 = ((t85).select(f64x8::splat(0.0), (t83).select(f64x8::splat(0.0), -t408)));
            let t498 = ((t192).select(f64x8::splat(0.0), -t382 * t495 / f64x8::splat(3.0)));
            let t501 = -t380 + f64x8::splat(0.4) * t81 * t498;
            let t505 = t99 * t160;
            let t510 = t103 * t171;
            let t515 = t107 * t467;
            let t518 = -f64x8::splat(0.010666666666666666) * t201 * t447 + f64x8::splat(4.266666666666667e-05) * t505 * t454 - f64x8::splat(8.533333333333334e-05) * t204 * t454 + f64x8::splat(3.413333333333333e-07) * t510 * t462 - f64x8::splat(5.12e-07) * t207 * t462 + f64x8::splat(2.048e-09) * t515 * t474;
            let t524 = t116 * t160;
            let t529 = t120 * t171;
            let t534 = t124 * t467;
            let t537 = -f64x8::splat(0.010666666666666666) * t214 * t447 + f64x8::splat(4.266666666666667e-05) * t524 * t454 - f64x8::splat(8.533333333333334e-05) * t217 * t454 + f64x8::splat(3.413333333333333e-07) * t529 * t462 - f64x8::splat(5.12e-07) * t220 * t462 + f64x8::splat(2.048e-09) * t534 * t474;
            let t541 = -f64x8::splat(0.010666666666666666) * t147 * t447 + f64x8::splat(4.266666666666667e-05) * t450 * t454 - f64x8::splat(8.533333333333334e-05) * t161 * t454 + f64x8::splat(3.413333333333333e-07) * t459 * t462 - f64x8::splat(5.12e-07) * t172 * t462 + f64x8::splat(2.048e-09) * t468 * t474 + t492 * t199 - t378 * t501 + t518 * t212 - f64x8::splat(2.0) * t392 * t501 + t537 * t225 - f64x8::splat(3.0) * t397 * t501;
            let t546 = ((t137).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t440 * t227 - t377 - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t146 * t541));
            let tvrho1 = t136 + t231 + t6 * (t434 + t546);
            acc_vrho_1 = tvrho1;
            let t554 = t43 * v_sigma0;
            let t559 = t55 * t44;
            let t562 = t58 * t31;
            let t564 = f64x8::splat(1.0) / t33 / t562;
            let t565 = t564 * t279;
            let t573 = t70 * v_sigma0;
            let t578 = t74 * t44;
            let t583 = f64x8::splat(0.004) * t66 * t35 * t39 - f64x8::splat(1.6e-05) * t67 * t52 + f64x8::splat(3.2e-05) * t573 * t52 - f64x8::splat(1.28e-07) * t71 * t62 + f64x8::splat(1.92e-07) * t578 * t62 - f64x8::splat(7.68e-10) * t75 * t565;
            let t590 = t103 * v_sigma0;
            let t595 = t107 * t44;
            let t600 = f64x8::splat(0.004) * t99 * t35 * t39 - f64x8::splat(1.6e-05) * t100 * t52 + f64x8::splat(3.2e-05) * t590 * t52 - f64x8::splat(1.28e-07) * t104 * t62 + f64x8::splat(1.92e-07) * t595 * t62 - f64x8::splat(7.68e-10) * t108 * t565;
            let t607 = t120 * v_sigma0;
            let t612 = t124 * t44;
            let t617 = f64x8::splat(0.004) * t116 * t35 * t39 - f64x8::splat(1.6e-05) * t117 * t52 + f64x8::splat(3.2e-05) * t607 * t52 - f64x8::splat(1.28e-07) * t121 * t62 + f64x8::splat(1.92e-07) * t612 * t62 - f64x8::splat(7.68e-10) * t125 * t565;
            let t619 = f64x8::splat(0.004) * t29 * t35 * t39 - f64x8::splat(1.6e-05) * t30 * t52 + f64x8::splat(3.2e-05) * t554 * t52 - f64x8::splat(1.28e-07) * t45 * t62 + f64x8::splat(1.92e-07) * t559 * t62 - f64x8::splat(7.68e-10) * t57 * t565 + t583 * t96 + t600 * t113 + t617 * t130;
            let t623 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t27 * t619));
            let tvsigma0 = t6 * t623;
            acc_vsigma_0 = tvsigma0;
            let tvsigma1 = f64x8::splat(0.0);
            acc_vsigma_1 = tvsigma1;
            let t629 = t43 * v_sigma2;
            let t634 = t55 * t160;
            let t637 = t173 * t148;
            let t639 = f64x8::splat(1.0) / t150 / t637;
            let t640 = t639 * t473;
            let t648 = t70 * v_sigma2;
            let t653 = t74 * t160;
            let t658 = f64x8::splat(0.004) * t66 * t152 * t156 - f64x8::splat(1.6e-05) * t180 * t168 + f64x8::splat(3.2e-05) * t648 * t168 - f64x8::splat(1.28e-07) * t183 * t177 + f64x8::splat(1.92e-07) * t653 * t177 - f64x8::splat(7.68e-10) * t186 * t640;
            let t665 = t103 * v_sigma2;
            let t670 = t107 * t160;
            let t675 = f64x8::splat(0.004) * t99 * t152 * t156 - f64x8::splat(1.6e-05) * t201 * t168 + f64x8::splat(3.2e-05) * t665 * t168 - f64x8::splat(1.28e-07) * t204 * t177 + f64x8::splat(1.92e-07) * t670 * t177 - f64x8::splat(7.68e-10) * t207 * t640;
            let t682 = t120 * v_sigma2;
            let t687 = t124 * t160;
            let t692 = f64x8::splat(0.004) * t116 * t152 * t156 - f64x8::splat(1.6e-05) * t214 * t168 + f64x8::splat(3.2e-05) * t682 * t168 - f64x8::splat(1.28e-07) * t217 * t177 + f64x8::splat(1.92e-07) * t687 * t177 - f64x8::splat(7.68e-10) * t220 * t640;
            let t694 = f64x8::splat(0.004) * t29 * t152 * t156 - f64x8::splat(1.6e-05) * t147 * t168 + f64x8::splat(3.2e-05) * t629 * t168 - f64x8::splat(1.28e-07) * t161 * t177 + f64x8::splat(1.92e-07) * t634 * t177 - f64x8::splat(7.68e-10) * t172 * t640 + t658 * t199 + t675 * t212 + t692 * t225;
            let t698 = ((t137).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t146 * t694));
            let tvsigma2 = t6 * t698;
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
