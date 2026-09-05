//! GGA_X_PBE kxc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_pbe.c`
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
pub fn gga_x_pbe_kxc_pol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    v2rho2: &mut [f64],
    v2rhosigma: &mut [f64],
    v2sigma2: &mut [f64],
    v3rho3: &mut [f64],
    v3rho2sigma: &mut [f64],
    v3rhosigma2: &mut [f64],
    v3sigma3: &mut [f64],
    param_kappa: f64,
    param_mu: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
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
        let mut acc_zk = V_ZERO;
        let mut acc_vrho_0 = V_ZERO;
        let mut acc_vrho_1 = V_ZERO;
        let mut acc_vsigma_0 = V_ZERO;
        let mut acc_vsigma_1 = V_ZERO;
        let mut acc_vsigma_2 = V_ZERO;
        let mut acc_v2rho2_0 = V_ZERO;
        let mut acc_v2rho2_1 = V_ZERO;
        let mut acc_v2rho2_2 = V_ZERO;
        let mut acc_v2rhosigma_0 = V_ZERO;
        let mut acc_v2rhosigma_1 = V_ZERO;
        let mut acc_v2rhosigma_2 = V_ZERO;
        let mut acc_v2rhosigma_3 = V_ZERO;
        let mut acc_v2rhosigma_4 = V_ZERO;
        let mut acc_v2rhosigma_5 = V_ZERO;
        let mut acc_v2sigma2_0 = V_ZERO;
        let mut acc_v2sigma2_1 = V_ZERO;
        let mut acc_v2sigma2_2 = V_ZERO;
        let mut acc_v2sigma2_3 = V_ZERO;
        let mut acc_v2sigma2_4 = V_ZERO;
        let mut acc_v2sigma2_5 = V_ZERO;
        let mut acc_v3rho3_0 = V_ZERO;
        let mut acc_v3rho3_1 = V_ZERO;
        let mut acc_v3rho3_2 = V_ZERO;
        let mut acc_v3rho3_3 = V_ZERO;
        let mut acc_v3rho2sigma_0 = V_ZERO;
        let mut acc_v3rho2sigma_1 = V_ZERO;
        let mut acc_v3rho2sigma_2 = V_ZERO;
        let mut acc_v3rho2sigma_3 = V_ZERO;
        let mut acc_v3rho2sigma_4 = V_ZERO;
        let mut acc_v3rho2sigma_5 = V_ZERO;
        let mut acc_v3rho2sigma_6 = V_ZERO;
        let mut acc_v3rho2sigma_7 = V_ZERO;
        let mut acc_v3rho2sigma_8 = V_ZERO;
        let mut acc_v3rhosigma2_0 = V_ZERO;
        let mut acc_v3rhosigma2_1 = V_ZERO;
        let mut acc_v3rhosigma2_2 = V_ZERO;
        let mut acc_v3rhosigma2_3 = V_ZERO;
        let mut acc_v3rhosigma2_4 = V_ZERO;
        let mut acc_v3rhosigma2_5 = V_ZERO;
        let mut acc_v3rhosigma2_6 = V_ZERO;
        let mut acc_v3rhosigma2_7 = V_ZERO;
        let mut acc_v3rhosigma2_8 = V_ZERO;
        let mut acc_v3rhosigma2_9 = V_ZERO;
        let mut acc_v3rhosigma2_10 = V_ZERO;
        let mut acc_v3rhosigma2_11 = V_ZERO;
        let mut acc_v3sigma3_0 = V_ZERO;
        let mut acc_v3sigma3_1 = V_ZERO;
        let mut acc_v3sigma3_2 = V_ZERO;
        let mut acc_v3sigma3_3 = V_ZERO;
        let mut acc_v3sigma3_4 = V_ZERO;
        let mut acc_v3sigma3_5 = V_ZERO;
        let mut acc_v3sigma3_6 = V_ZERO;
        let mut acc_v3sigma3_7 = V_ZERO;
        let mut acc_v3sigma3_8 = V_ZERO;
        let mut acc_v3sigma3_9 = V_ZERO;
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
            let t43 = param_kappa + t29 * t34 * t39 / f64x8::splat(24.0);
            let t48 = f64x8::splat(1.0) + param_kappa * (f64x8::splat(1.0) - param_kappa / t43);
            let t52 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t27 * t48));
            let t53 = (v_rho1).simd_le(dens_threshold);
            let t54 = -t16;
            let t56 = ((t14).select(t11, (t10).select(t15, t54 * t7)));
            let t57 = f64x8::splat(1.0) + t56;
            let t58 = (t57).simd_le(zeta_threshold);
            let t59 = (simd::cbrt(t57));
            let t61 = ((t58).select(t22, t59 * t57));
            let t62 = t61 * t26;
            let t63 = t33 * v_sigma2;
            let t64 = v_rho1 * v_rho1;
            let t65 = (simd::cbrt(v_rho1));
            let t66 = t65 * t65;
            let t68 = f64x8::splat(1.0) / t66 / t64;
            let t72 = param_kappa + t29 * t63 * t68 / f64x8::splat(24.0);
            let t77 = f64x8::splat(1.0) + param_kappa * (f64x8::splat(1.0) - param_kappa / t72);
            let t81 = ((t53).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t62 * t77));
            let tzk0 = t52 + t81;
            acc_zk = tzk0;
            let t82 = t6 * t6;
            let t83 = f64x8::splat(1.0) / t82;
            let t84 = t16 * t83;
            let t86 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), t7 - t84)));
            let t89 = ((t20).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t86));
            let t90 = t89 * t26;
            let t94 = t26 * t26;
            let t95 = f64x8::splat(1.0) / t94;
            let t96 = t25 * t95;
            let t99 = t5 * t96 * t48 / f64x8::splat(8.0);
            let t100 = param_kappa * param_kappa;
            let t101 = t27 * t100;
            let t102 = t5 * t101;
            let t103 = t43 * t43;
            let t105 = f64x8::splat(1.0) / t103 * param_mu;
            let t106 = t105 * t28;
            let t107 = t35 * v_rho0;
            let t109 = f64x8::splat(1.0) / t37 / t107;
            let t111 = t106 * t34 * t109;
            let t115 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t90 * t48 - t99 + t102 * t111 / f64x8::splat(24.0)));
            let t116 = t54 * t83;
            let t118 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), -t7 - t116)));
            let t121 = ((t58).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t59 * t118));
            let t122 = t121 * t26;
            let t126 = t61 * t95;
            let t129 = t5 * t126 * t77 / f64x8::splat(8.0);
            let t131 = ((t53).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t122 * t77 - t129));
            let tvrho0 = t52 + t81 + t6 * (t115 + t131);
            acc_vrho_0 = tvrho0;
            let t135 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), -t7 - t84)));
            let t138 = ((t20).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t135));
            let t139 = t138 * t26;
            let t144 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t139 * t48 - t99));
            let t146 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), t7 - t116)));
            let t149 = ((t58).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t59 * t146));
            let t150 = t149 * t26;
            let t154 = t62 * t100;
            let t155 = t5 * t154;
            let t156 = t72 * t72;
            let t158 = f64x8::splat(1.0) / t156 * param_mu;
            let t159 = t158 * t28;
            let t160 = t64 * v_rho1;
            let t162 = f64x8::splat(1.0) / t66 / t160;
            let t164 = t159 * t63 * t162;
            let t168 = ((t53).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t150 * t77 - t129 + t155 * t164 / f64x8::splat(24.0)));
            let tvrho1 = t52 + t81 + t6 * (t144 + t168);
            acc_vrho_1 = tvrho1;
            let t171 = t28 * t33;
            let t173 = t105 * t171 * t39;
            let t176 = ((t1).select(f64x8::splat(0.0), -t102 * t173 / f64x8::splat(64.0)));
            let tvsigma0 = t6 * t176;
            acc_vsigma_0 = tvsigma0;
            let tvsigma1 = f64x8::splat(0.0);
            acc_vsigma_1 = tvsigma1;
            let t178 = t158 * t171 * t68;
            let t181 = ((t53).select(f64x8::splat(0.0), -t155 * t178 / f64x8::splat(64.0)));
            let tvsigma2 = t6 * t181;
            acc_vsigma_2 = tvsigma2;
            let t184 = t23 * t23;
            let t185 = f64x8::splat(1.0) / t184;
            let t186 = t86 * t86;
            let t189 = t82 * t6;
            let t190 = f64x8::splat(1.0) / t189;
            let t191 = t16 * t190;
            let t194 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), -f64x8::splat(2.0) * t83 + f64x8::splat(2.0) * t191)));
            let t198 = ((t20).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(9.0) * t185 * t186 + f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t194));
            let t199 = t198 * t26;
            let t203 = t89 * t95;
            let t205 = t5 * t203 * t48;
            let t208 = t5 * t90 * t100;
            let t212 = f64x8::splat(1.0) / t94 / t6;
            let t213 = t25 * t212;
            let t216 = t5 * t213 * t48 / f64x8::splat(12.0);
            let t218 = t5 * t96 * t100;
            let t219 = t218 * t111;
            let t223 = param_mu * param_mu;
            let t224 = f64x8::splat(1.0) / t103 / t43 * t223;
            let t225 = t28 * t28;
            let t226 = t224 * t225;
            let t228 = f64x8::splat(1.0) / t31 / t30;
            let t229 = v_sigma0 * v_sigma0;
            let t230 = t228 * t229;
            let t231 = t35 * t35;
            let t234 = f64x8::splat(1.0) / t36 / t231 / t107;
            let t236 = t226 * t230 * t234;
            let t240 = f64x8::splat(1.0) / t37 / t231;
            let t242 = t106 * t34 * t240;
            let t246 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t199 * t48 - t205 / f64x8::splat(4.0) + t208 * t111 / f64x8::splat(12.0) + t216 + t219 / f64x8::splat(36.0) + t102 * t236 / f64x8::splat(108.0) - f64x8::splat(11.0) / f64x8::splat(72.0) * t102 * t242));
            let t247 = t59 * t59;
            let t248 = f64x8::splat(1.0) / t247;
            let t249 = t118 * t118;
            let t252 = t54 * t190;
            let t255 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), f64x8::splat(2.0) * t83 + f64x8::splat(2.0) * t252)));
            let t259 = ((t58).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(9.0) * t248 * t249 + f64x8::splat(4.0) / f64x8::splat(3.0) * t59 * t255));
            let t260 = t259 * t26;
            let t264 = t121 * t95;
            let t266 = t5 * t264 * t77;
            let t268 = t61 * t212;
            let t271 = t5 * t268 * t77 / f64x8::splat(12.0);
            let t273 = ((t53).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t260 * t77 - t266 / f64x8::splat(4.0) + t271));
            let tv2rho20 = f64x8::splat(2.0) * t115 + f64x8::splat(2.0) * t131 + t6 * (t246 + t273);
            acc_v2rho2_0 = tv2rho20;
            let t276 = t185 * t135;
            let t280 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), f64x8::splat(2.0) * t191)));
            let t284 = ((t20).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(9.0) * t276 * t86 + f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t280));
            let t285 = t284 * t26;
            let t289 = t138 * t95;
            let t291 = t5 * t289 * t48;
            let t294 = t5 * t139 * t100;
            let t300 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t285 * t48 - t291 / f64x8::splat(8.0) + t294 * t111 / f64x8::splat(24.0) - t205 / f64x8::splat(8.0) + t216 + t219 / f64x8::splat(72.0)));
            let t301 = t248 * t146;
            let t305 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), f64x8::splat(2.0) * t252)));
            let t309 = ((t58).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(9.0) * t301 * t118 + f64x8::splat(4.0) / f64x8::splat(3.0) * t59 * t305));
            let t310 = t309 * t26;
            let t314 = t149 * t95;
            let t316 = t5 * t314 * t77;
            let t320 = t5 * t122 * t100;
            let t324 = t5 * t126 * t100;
            let t325 = t324 * t164;
            let t328 = ((t53).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t310 * t77 - t316 / f64x8::splat(8.0) - t266 / f64x8::splat(8.0) + t271 + t320 * t164 / f64x8::splat(24.0) + t325 / f64x8::splat(72.0)));
            let tv2rho21 = t115 + t131 + t144 + t168 + t6 * (t300 + t328);
            acc_v2rho2_1 = tv2rho21;
            let t333 = t135 * t135;
            let t338 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), f64x8::splat(2.0) * t83 + f64x8::splat(2.0) * t191)));
            let t342 = ((t20).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(9.0) * t185 * t333 + f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t338));
            let t343 = t342 * t26;
            let t349 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t343 * t48 - t291 / f64x8::splat(4.0) + t216));
            let t350 = t146 * t146;
            let t355 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), -f64x8::splat(2.0) * t83 + f64x8::splat(2.0) * t252)));
            let t359 = ((t58).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(9.0) * t248 * t350 + f64x8::splat(4.0) / f64x8::splat(3.0) * t59 * t355));
            let t360 = t359 * t26;
            let t366 = t5 * t150 * t100;
            let t372 = f64x8::splat(1.0) / t156 / t72 * t223;
            let t373 = t372 * t225;
            let t374 = v_sigma2 * v_sigma2;
            let t375 = t228 * t374;
            let t376 = t64 * t64;
            let t379 = f64x8::splat(1.0) / t65 / t376 / t160;
            let t381 = t373 * t375 * t379;
            let t385 = f64x8::splat(1.0) / t66 / t376;
            let t387 = t159 * t63 * t385;
            let t391 = ((t53).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t360 * t77 - t316 / f64x8::splat(4.0) + t366 * t164 / f64x8::splat(12.0) + t271 + t325 / f64x8::splat(36.0) + t155 * t381 / f64x8::splat(108.0) - f64x8::splat(11.0) / f64x8::splat(72.0) * t155 * t387));
            let tv2rho22 = f64x8::splat(2.0) * t144 + f64x8::splat(2.0) * t168 + t6 * (t349 + t391);
            acc_v2rho2_2 = tv2rho22;
            let t397 = t218 * t173 / f64x8::splat(192.0);
            let t398 = t231 * t35;
            let t400 = f64x8::splat(1.0) / t36 / t398;
            let t403 = t226 * t228 * t400 * v_sigma0;
            let t407 = t105 * t171 * t109;
            let t411 = ((t1).select(f64x8::splat(0.0), -t208 * t173 / f64x8::splat(64.0) - t397 - t102 * t403 / f64x8::splat(288.0) + t102 * t407 / f64x8::splat(24.0)));
            let tv2rhosigma0 = t6 * t411 + t176;
            acc_v2rhosigma_0 = tv2rhosigma0;
            let tv2rhosigma1 = f64x8::splat(0.0);
            acc_v2rhosigma_1 = tv2rhosigma1;
            let t416 = t324 * t178 / f64x8::splat(192.0);
            let t418 = ((t53).select(f64x8::splat(0.0), -t320 * t178 / f64x8::splat(64.0) - t416));
            let tv2rhosigma2 = t6 * t418 + t181;
            acc_v2rhosigma_2 = tv2rhosigma2;
            let t423 = ((t1).select(f64x8::splat(0.0), -t294 * t173 / f64x8::splat(64.0) - t397));
            let tv2rhosigma3 = t6 * t423 + t176;
            acc_v2rhosigma_3 = tv2rhosigma3;
            let tv2rhosigma4 = f64x8::splat(0.0);
            acc_v2rhosigma_4 = tv2rhosigma4;
            let t427 = t376 * t64;
            let t429 = f64x8::splat(1.0) / t65 / t427;
            let t432 = t373 * t228 * t429 * v_sigma2;
            let t436 = t158 * t171 * t162;
            let t440 = ((t53).select(f64x8::splat(0.0), -t366 * t178 / f64x8::splat(64.0) - t416 - t155 * t432 / f64x8::splat(288.0) + t155 * t436 / f64x8::splat(24.0)));
            let tv2rhosigma5 = t6 * t440 + t181;
            acc_v2rhosigma_5 = tv2rhosigma5;
            let t442 = t225 * t228;
            let t443 = t231 * v_rho0;
            let t447 = t224 * t442 / t36 / t443;
            let t450 = ((t1).select(f64x8::splat(0.0), t102 * t447 / f64x8::splat(768.0)));
            let tv2sigma20 = t6 * t450;
            acc_v2sigma2_0 = tv2sigma20;
            let tv2sigma21 = f64x8::splat(0.0);
            acc_v2sigma2_1 = tv2sigma21;
            let tv2sigma22 = f64x8::splat(0.0);
            acc_v2sigma2_2 = tv2sigma22;
            let tv2sigma23 = f64x8::splat(0.0);
            acc_v2sigma2_3 = tv2sigma23;
            let tv2sigma24 = f64x8::splat(0.0);
            acc_v2sigma2_4 = tv2sigma24;
            let t451 = t376 * v_rho1;
            let t455 = t372 * t442 / t65 / t451;
            let t458 = ((t53).select(f64x8::splat(0.0), t155 * t455 / f64x8::splat(768.0)));
            let tv2sigma25 = t6 * t458;
            acc_v2sigma2_5 = tv2sigma25;
            let t462 = f64x8::splat(1.0) / t37 / t443;
            let t464 = t106 * t34 * t462;
            let t467 = t218 * t242;
            let t469 = t231 * t231;
            let t471 = f64x8::splat(1.0) / t36 / t469;
            let t473 = t226 * t230 * t471;
            let t478 = t218 * t236;
            let t481 = t5 * t199 * t100;
            let t485 = t5 * t203 * t100;
            let t486 = t485 * t111;
            let t491 = t5 * t213 * t100;
            let t492 = t491 * t111;
            let t494 = t198 * t95;
            let t496 = t5 * t494 * t48;
            let t498 = t89 * t212;
            let t500 = t5 * t498 * t48;
            let t503 = f64x8::splat(1.0) / t94 / t82;
            let t504 = t25 * t503;
            let t507 = f64x8::splat(5.0) / f64x8::splat(36.0) * t5 * t504 * t48;
            let t509 = f64x8::splat(1.0) / t184 / t19;
            let t510 = t186 * t86;
            let t513 = t185 * t86;
            let t516 = t82 * t82;
            let t517 = f64x8::splat(1.0) / t516;
            let t518 = t16 * t517;
            let t521 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), f64x8::splat(6.0) * t190 - f64x8::splat(6.0) * t518)));
            let t525 = ((t20).select(f64x8::splat(0.0), -f64x8::splat(8.0) / f64x8::splat(27.0) * t509 * t510 + f64x8::splat(4.0) / f64x8::splat(3.0) * t513 * t194 + f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t521));
            let t526 = t525 * t26;
            let t530 = t30 * t30;
            let t533 = t2 / t3 / t530;
            let t534 = t533 * t27;
            let t535 = t103 * t103;
            let t537 = t100 / t535;
            let t538 = t223 * param_mu;
            let t539 = t229 * v_sigma0;
            let t540 = t538 * t539;
            let t541 = t469 * t107;
            let t542 = f64x8::splat(1.0) / t541;
            let t544 = t537 * t540 * t542;
            let t547 = f64x8::splat(77.0) / f64x8::splat(108.0) * t102 * t464 - f64x8::splat(11.0) / f64x8::splat(72.0) * t467 - f64x8::splat(11.0) / f64x8::splat(108.0) * t102 * t473 - f64x8::splat(11.0) / f64x8::splat(24.0) * t208 * t242 + t478 / f64x8::splat(108.0) + t481 * t111 / f64x8::splat(8.0) + t486 / f64x8::splat(12.0) + t208 * t236 / f64x8::splat(36.0) - t492 / f64x8::splat(36.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t496 + t500 / f64x8::splat(4.0) - t507 - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t526 * t48 + t534 * t544 / f64x8::splat(54.0);
            let t548 = ((t1).select(f64x8::splat(0.0), t547));
            let t550 = f64x8::splat(1.0) / t247 / t57;
            let t551 = t249 * t118;
            let t554 = t248 * t118;
            let t557 = t54 * t517;
            let t560 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), -f64x8::splat(6.0) * t190 - f64x8::splat(6.0) * t557)));
            let t564 = ((t58).select(f64x8::splat(0.0), -f64x8::splat(8.0) / f64x8::splat(27.0) * t550 * t551 + f64x8::splat(4.0) / f64x8::splat(3.0) * t554 * t255 + f64x8::splat(4.0) / f64x8::splat(3.0) * t59 * t560));
            let t565 = t564 * t26;
            let t569 = t259 * t95;
            let t571 = t5 * t569 * t77;
            let t573 = t121 * t212;
            let t575 = t5 * t573 * t77;
            let t577 = t61 * t503;
            let t580 = f64x8::splat(5.0) / f64x8::splat(36.0) * t5 * t577 * t77;
            let t582 = ((t53).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t565 * t77 - f64x8::splat(3.0) / f64x8::splat(8.0) * t571 + t575 / f64x8::splat(4.0) - t580));
            let tv3rho30 = f64x8::splat(3.0) * t246 + f64x8::splat(3.0) * t273 + t6 * (t548 + t582);
            acc_v3rho3_0 = tv3rho30;
            let t585 = f64x8::splat(2.0) * t300;
            let t586 = f64x8::splat(2.0) * t328;
            let t587 = t509 * t135;
            let t590 = t185 * t280;
            let t595 = f64x8::splat(2.0) * t190;
            let t596 = f64x8::splat(6.0) * t518;
            let t598 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), t595 - t596)));
            let t602 = ((t20).select(f64x8::splat(0.0), -f64x8::splat(8.0) / f64x8::splat(27.0) * t587 * t186 + f64x8::splat(8.0) / f64x8::splat(9.0) * t590 * t86 + f64x8::splat(4.0) / f64x8::splat(9.0) * t276 * t194 + f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t598));
            let t603 = t602 * t26;
            let t607 = t284 * t95;
            let t610 = t5 * t607 * t48 / f64x8::splat(4.0);
            let t612 = t5 * t285 * t100;
            let t615 = t138 * t212;
            let t617 = t5 * t615 * t48;
            let t620 = t5 * t289 * t100;
            let t622 = t620 * t111 / f64x8::splat(36.0);
            let t633 = -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t603 * t48 - t610 + t612 * t111 / f64x8::splat(12.0) + t617 / f64x8::splat(12.0) + t622 + t294 * t236 / f64x8::splat(108.0) - f64x8::splat(11.0) / f64x8::splat(72.0) * t294 * t242 - t496 / f64x8::splat(8.0) + t500 / f64x8::splat(6.0) + t486 / f64x8::splat(36.0) - t507 - t492 / f64x8::splat(54.0) + t478 / f64x8::splat(324.0) - f64x8::splat(11.0) / f64x8::splat(216.0) * t467;
            let t634 = ((t1).select(f64x8::splat(0.0), t633));
            let t635 = t550 * t146;
            let t638 = t248 * t305;
            let t643 = f64x8::splat(6.0) * t557;
            let t645 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), -t595 - t643)));
            let t649 = ((t58).select(f64x8::splat(0.0), -f64x8::splat(8.0) / f64x8::splat(27.0) * t635 * t249 + f64x8::splat(8.0) / f64x8::splat(9.0) * t638 * t118 + f64x8::splat(4.0) / f64x8::splat(9.0) * t301 * t255 + f64x8::splat(4.0) / f64x8::splat(3.0) * t59 * t645));
            let t650 = t649 * t26;
            let t654 = t309 * t95;
            let t657 = t5 * t654 * t77 / f64x8::splat(4.0);
            let t658 = t149 * t212;
            let t660 = t5 * t658 * t77;
            let t665 = t5 * t260 * t100;
            let t669 = t5 * t264 * t100;
            let t671 = t669 * t164 / f64x8::splat(36.0);
            let t673 = t5 * t268 * t100;
            let t674 = t673 * t164;
            let t677 = ((t53).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t650 * t77 - t657 + t660 / f64x8::splat(12.0) - t571 / f64x8::splat(8.0) + t575 / f64x8::splat(6.0) - t580 + t665 * t164 / f64x8::splat(24.0) + t671 - t674 / f64x8::splat(108.0)));
            let tv3rho31 = t246 + t273 + t585 + t586 + t6 * (t634 + t677);
            acc_v3rho3_1 = tv3rho31;
            let t680 = t509 * t333;
            let t685 = t185 * t338;
            let t689 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), -t595 - t596)));
            let t693 = ((t20).select(f64x8::splat(0.0), -f64x8::splat(8.0) / f64x8::splat(27.0) * t680 * t86 + f64x8::splat(8.0) / f64x8::splat(9.0) * t276 * t280 + f64x8::splat(4.0) / f64x8::splat(9.0) * t685 * t86 + f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t689));
            let t694 = t693 * t26;
            let t698 = t342 * t95;
            let t700 = t5 * t698 * t48;
            let t703 = t5 * t343 * t100;
            let t710 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t694 * t48 - t700 / f64x8::splat(8.0) + t703 * t111 / f64x8::splat(24.0) - t610 + t617 / f64x8::splat(6.0) + t622 + t500 / f64x8::splat(12.0) - t507 - t492 / f64x8::splat(108.0)));
            let t711 = t550 * t350;
            let t716 = t248 * t355;
            let t720 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), t595 - t643)));
            let t724 = ((t58).select(f64x8::splat(0.0), -f64x8::splat(8.0) / f64x8::splat(27.0) * t711 * t118 + f64x8::splat(8.0) / f64x8::splat(9.0) * t301 * t305 + f64x8::splat(4.0) / f64x8::splat(9.0) * t716 * t118 + f64x8::splat(4.0) / f64x8::splat(3.0) * t59 * t720));
            let t725 = t724 * t26;
            let t729 = t359 * t95;
            let t731 = t5 * t729 * t77;
            let t735 = t5 * t310 * t100;
            let t739 = t5 * t314 * t100;
            let t740 = t739 * t164;
            let t746 = t324 * t381;
            let t750 = t324 * t387;
            let t752 = -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t725 * t77 - t731 / f64x8::splat(8.0) - t657 + t660 / f64x8::splat(6.0) + t735 * t164 / f64x8::splat(12.0) + t740 / f64x8::splat(36.0) + t575 / f64x8::splat(12.0) - t580 + t671 - t674 / f64x8::splat(54.0) + t320 * t381 / f64x8::splat(108.0) + t746 / f64x8::splat(324.0) - f64x8::splat(11.0) / f64x8::splat(72.0) * t320 * t387 - f64x8::splat(11.0) / f64x8::splat(216.0) * t750;
            let t753 = ((t53).select(f64x8::splat(0.0), t752));
            let tv3rho32 = t585 + t586 + t349 + t391 + t6 * (t710 + t753);
            acc_v3rho3_2 = tv3rho32;
            let t758 = t333 * t135;
            let t765 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), -f64x8::splat(6.0) * t190 - f64x8::splat(6.0) * t518)));
            let t769 = ((t20).select(f64x8::splat(0.0), -f64x8::splat(8.0) / f64x8::splat(27.0) * t509 * t758 + f64x8::splat(4.0) / f64x8::splat(3.0) * t276 * t338 + f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t765));
            let t770 = t769 * t26;
            let t777 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t770 * t48 - f64x8::splat(3.0) / f64x8::splat(8.0) * t700 + t617 / f64x8::splat(4.0) - t507));
            let t778 = t350 * t146;
            let t785 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), f64x8::splat(6.0) * t190 - f64x8::splat(6.0) * t557)));
            let t789 = ((t58).select(f64x8::splat(0.0), -f64x8::splat(8.0) / f64x8::splat(27.0) * t550 * t778 + f64x8::splat(4.0) / f64x8::splat(3.0) * t301 * t355 + f64x8::splat(4.0) / f64x8::splat(3.0) * t59 * t785));
            let t790 = t789 * t26;
            let t794 = t376 * t376;
            let t796 = f64x8::splat(1.0) / t65 / t794;
            let t798 = t373 * t375 * t796;
            let t802 = f64x8::splat(1.0) / t66 / t451;
            let t804 = t159 * t63 * t802;
            let t816 = t5 * t360 * t100;
            let t821 = t533 * t62;
            let t822 = t156 * t156;
            let t824 = t100 / t822;
            let t825 = t374 * v_sigma2;
            let t826 = t538 * t825;
            let t827 = t794 * t160;
            let t828 = f64x8::splat(1.0) / t827;
            let t830 = t824 * t826 * t828;
            let t833 = -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t790 * t77 - f64x8::splat(11.0) / f64x8::splat(108.0) * t155 * t798 + f64x8::splat(77.0) / f64x8::splat(108.0) * t155 * t804 - f64x8::splat(11.0) / f64x8::splat(24.0) * t366 * t387 + t746 / f64x8::splat(108.0) - f64x8::splat(11.0) / f64x8::splat(72.0) * t750 - t674 / f64x8::splat(36.0) + t740 / f64x8::splat(12.0) + t366 * t381 / f64x8::splat(36.0) + t816 * t164 / f64x8::splat(8.0) + t660 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t731 - t580 + t821 * t830 / f64x8::splat(54.0);
            let t834 = ((t53).select(f64x8::splat(0.0), t833));
            let tv3rho33 = f64x8::splat(3.0) * t349 + f64x8::splat(3.0) * t391 + t6 * (t777 + t834);
            acc_v3rho3_3 = tv3rho33;
            let t840 = t485 * t173;
            let t847 = t491 * t173 / f64x8::splat(288.0);
            let t848 = t218 * t403;
            let t850 = t218 * t407;
            let t852 = t469 * t35;
            let t854 = t538 / t852;
            let t856 = t537 * t854 * t229;
            let t861 = t226 * t228 * t234 * v_sigma0;
            let t865 = t105 * t171 * t240;
            let t869 = ((t1).select(f64x8::splat(0.0), -t481 * t173 / f64x8::splat(64.0) - t840 / f64x8::splat(96.0) - t208 * t403 / f64x8::splat(144.0) + t208 * t407 / f64x8::splat(12.0) + t847 - t848 / f64x8::splat(432.0) + t850 / f64x8::splat(36.0) - t534 * t856 / f64x8::splat(144.0) + t102 * t861 / f64x8::splat(32.0) - f64x8::splat(11.0) / f64x8::splat(72.0) * t102 * t865));
            let tv3rho2sigma0 = t6 * t869 + f64x8::splat(2.0) * t411;
            acc_v3rho2sigma_0 = tv3rho2sigma0;
            let tv3rho2sigma1 = f64x8::splat(0.0);
            acc_v3rho2sigma_1 = tv3rho2sigma1;
            let t874 = t669 * t178;
            let t877 = t673 * t178 / f64x8::splat(288.0);
            let t879 = ((t53).select(f64x8::splat(0.0), -t665 * t178 / f64x8::splat(64.0) - t874 / f64x8::splat(96.0) + t877));
            let tv3rho2sigma2 = t6 * t879 + f64x8::splat(2.0) * t418;
            acc_v3rho2sigma_2 = tv3rho2sigma2;
            let t883 = t620 * t173;
            let t893 = ((t1).select(f64x8::splat(0.0), -t612 * t173 / f64x8::splat(64.0) - t883 / f64x8::splat(192.0) - t294 * t403 / f64x8::splat(288.0) + t294 * t407 / f64x8::splat(24.0) - t840 / f64x8::splat(192.0) + t847 - t848 / f64x8::splat(864.0) + t850 / f64x8::splat(72.0)));
            let tv3rho2sigma3 = t6 * t893 + t411 + t423;
            acc_v3rho2sigma_3 = tv3rho2sigma3;
            let tv3rho2sigma4 = f64x8::splat(0.0);
            acc_v3rho2sigma_4 = tv3rho2sigma4;
            let t897 = t739 * t178;
            let t902 = t324 * t432;
            let t906 = t324 * t436;
            let t909 = ((t53).select(f64x8::splat(0.0), -t735 * t178 / f64x8::splat(64.0) - t897 / f64x8::splat(192.0) - t874 / f64x8::splat(192.0) + t877 - t320 * t432 / f64x8::splat(288.0) - t902 / f64x8::splat(864.0) + t320 * t436 / f64x8::splat(24.0) + t906 / f64x8::splat(72.0)));
            let tv3rho2sigma5 = t6 * t909 + t418 + t440;
            acc_v3rho2sigma_5 = tv3rho2sigma5;
            let t916 = ((t1).select(f64x8::splat(0.0), -t703 * t173 / f64x8::splat(64.0) - t883 / f64x8::splat(96.0) + t847));
            let tv3rho2sigma6 = t6 * t916 + f64x8::splat(2.0) * t423;
            acc_v3rho2sigma_6 = tv3rho2sigma6;
            let tv3rho2sigma7 = f64x8::splat(0.0);
            acc_v3rho2sigma_7 = tv3rho2sigma7;
            let t928 = t794 * t64;
            let t930 = t538 / t928;
            let t932 = t824 * t930 * t374;
            let t937 = t373 * t228 * t379 * v_sigma2;
            let t941 = t158 * t171 * t385;
            let t945 = ((t53).select(f64x8::splat(0.0), -t816 * t178 / f64x8::splat(64.0) - t897 / f64x8::splat(96.0) - t366 * t432 / f64x8::splat(144.0) + t366 * t436 / f64x8::splat(12.0) + t877 - t902 / f64x8::splat(432.0) + t906 / f64x8::splat(36.0) - t821 * t932 / f64x8::splat(144.0) + t155 * t937 / f64x8::splat(32.0) - f64x8::splat(11.0) / f64x8::splat(72.0) * t155 * t941));
            let tv3rho2sigma8 = t6 * t945 + f64x8::splat(2.0) * t440;
            acc_v3rho2sigma_8 = tv3rho2sigma8;
            let t950 = t218 * t447 / f64x8::splat(2304.0);
            let t951 = t469 * v_rho0;
            let t953 = t538 / t951;
            let t955 = t537 * t953 * v_sigma0;
            let t959 = t224 * t442 * t400;
            let t963 = ((t1).select(f64x8::splat(0.0), t208 * t447 / f64x8::splat(768.0) + t950 + t534 * t955 / f64x8::splat(384.0) - t102 * t959 / f64x8::splat(144.0)));
            let tv3rhosigma20 = t6 * t963 + t450;
            acc_v3rhosigma2_0 = tv3rhosigma20;
            let tv3rhosigma21 = f64x8::splat(0.0);
            acc_v3rhosigma2_1 = tv3rhosigma21;
            let tv3rhosigma22 = f64x8::splat(0.0);
            acc_v3rhosigma2_2 = tv3rhosigma22;
            let tv3rhosigma23 = f64x8::splat(0.0);
            acc_v3rhosigma2_3 = tv3rhosigma23;
            let tv3rhosigma24 = f64x8::splat(0.0);
            acc_v3rhosigma2_4 = tv3rhosigma24;
            let t968 = t324 * t455 / f64x8::splat(2304.0);
            let t970 = ((t53).select(f64x8::splat(0.0), t320 * t455 / f64x8::splat(768.0) + t968));
            let tv3rhosigma25 = t6 * t970 + t458;
            acc_v3rhosigma2_5 = tv3rhosigma25;
            let t975 = ((t1).select(f64x8::splat(0.0), t294 * t447 / f64x8::splat(768.0) + t950));
            let tv3rhosigma26 = t6 * t975 + t450;
            acc_v3rhosigma2_6 = tv3rhosigma26;
            let tv3rhosigma27 = f64x8::splat(0.0);
            acc_v3rhosigma2_7 = tv3rhosigma27;
            let tv3rhosigma28 = f64x8::splat(0.0);
            acc_v3rhosigma2_8 = tv3rhosigma28;
            let tv3rhosigma29 = f64x8::splat(0.0);
            acc_v3rhosigma2_9 = tv3rhosigma29;
            let tv3rhosigma210 = f64x8::splat(0.0);
            acc_v3rhosigma2_10 = tv3rhosigma210;
            let t979 = t794 * v_rho1;
            let t981 = t538 / t979;
            let t983 = t824 * t981 * v_sigma2;
            let t987 = t372 * t442 * t429;
            let t991 = ((t53).select(f64x8::splat(0.0), t366 * t455 / f64x8::splat(768.0) + t968 + t821 * t983 / f64x8::splat(384.0) - t155 * t987 / f64x8::splat(144.0)));
            let tv3rhosigma211 = t6 * t991 + t458;
            acc_v3rhosigma2_11 = tv3rhosigma211;
            let t995 = t537 * t538 / t469;
            let t998 = ((t1).select(f64x8::splat(0.0), -t534 * t995 / f64x8::splat(1024.0)));
            let tv3sigma30 = t6 * t998;
            acc_v3sigma3_0 = tv3sigma30;
            let tv3sigma31 = f64x8::splat(0.0);
            acc_v3sigma3_1 = tv3sigma31;
            let tv3sigma32 = f64x8::splat(0.0);
            acc_v3sigma3_2 = tv3sigma32;
            let tv3sigma33 = f64x8::splat(0.0);
            acc_v3sigma3_3 = tv3sigma33;
            let tv3sigma34 = f64x8::splat(0.0);
            acc_v3sigma3_4 = tv3sigma34;
            let tv3sigma35 = f64x8::splat(0.0);
            acc_v3sigma3_5 = tv3sigma35;
            let tv3sigma36 = f64x8::splat(0.0);
            acc_v3sigma3_6 = tv3sigma36;
            let tv3sigma37 = f64x8::splat(0.0);
            acc_v3sigma3_7 = tv3sigma37;
            let tv3sigma38 = f64x8::splat(0.0);
            acc_v3sigma3_8 = tv3sigma38;
            let t1001 = t824 * t538 / t794;
            let t1004 = ((t53).select(f64x8::splat(0.0), -t821 * t1001 / f64x8::splat(1024.0)));
            let tv3sigma39 = t6 * t1004;
            acc_v3sigma3_9 = tv3sigma39;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        store_strided(vrho, ip, m, 2, 0, acc_vrho_0);
        store_strided(vrho, ip, m, 2, 1, acc_vrho_1);
        store_strided(vsigma, ip, m, 3, 0, acc_vsigma_0);
        store_strided(vsigma, ip, m, 3, 1, acc_vsigma_1);
        store_strided(vsigma, ip, m, 3, 2, acc_vsigma_2);
        store_strided(v2rho2, ip, m, 3, 0, acc_v2rho2_0);
        store_strided(v2rho2, ip, m, 3, 1, acc_v2rho2_1);
        store_strided(v2rho2, ip, m, 3, 2, acc_v2rho2_2);
        store_strided(v2rhosigma, ip, m, 6, 0, acc_v2rhosigma_0);
        store_strided(v2rhosigma, ip, m, 6, 1, acc_v2rhosigma_1);
        store_strided(v2rhosigma, ip, m, 6, 2, acc_v2rhosigma_2);
        store_strided(v2rhosigma, ip, m, 6, 3, acc_v2rhosigma_3);
        store_strided(v2rhosigma, ip, m, 6, 4, acc_v2rhosigma_4);
        store_strided(v2rhosigma, ip, m, 6, 5, acc_v2rhosigma_5);
        store_strided(v2sigma2, ip, m, 6, 0, acc_v2sigma2_0);
        store_strided(v2sigma2, ip, m, 6, 1, acc_v2sigma2_1);
        store_strided(v2sigma2, ip, m, 6, 2, acc_v2sigma2_2);
        store_strided(v2sigma2, ip, m, 6, 3, acc_v2sigma2_3);
        store_strided(v2sigma2, ip, m, 6, 4, acc_v2sigma2_4);
        store_strided(v2sigma2, ip, m, 6, 5, acc_v2sigma2_5);
        store_strided(v3rho3, ip, m, 4, 0, acc_v3rho3_0);
        store_strided(v3rho3, ip, m, 4, 1, acc_v3rho3_1);
        store_strided(v3rho3, ip, m, 4, 2, acc_v3rho3_2);
        store_strided(v3rho3, ip, m, 4, 3, acc_v3rho3_3);
        store_strided(v3rho2sigma, ip, m, 9, 0, acc_v3rho2sigma_0);
        store_strided(v3rho2sigma, ip, m, 9, 1, acc_v3rho2sigma_1);
        store_strided(v3rho2sigma, ip, m, 9, 2, acc_v3rho2sigma_2);
        store_strided(v3rho2sigma, ip, m, 9, 3, acc_v3rho2sigma_3);
        store_strided(v3rho2sigma, ip, m, 9, 4, acc_v3rho2sigma_4);
        store_strided(v3rho2sigma, ip, m, 9, 5, acc_v3rho2sigma_5);
        store_strided(v3rho2sigma, ip, m, 9, 6, acc_v3rho2sigma_6);
        store_strided(v3rho2sigma, ip, m, 9, 7, acc_v3rho2sigma_7);
        store_strided(v3rho2sigma, ip, m, 9, 8, acc_v3rho2sigma_8);
        store_strided(v3rhosigma2, ip, m, 12, 0, acc_v3rhosigma2_0);
        store_strided(v3rhosigma2, ip, m, 12, 1, acc_v3rhosigma2_1);
        store_strided(v3rhosigma2, ip, m, 12, 2, acc_v3rhosigma2_2);
        store_strided(v3rhosigma2, ip, m, 12, 3, acc_v3rhosigma2_3);
        store_strided(v3rhosigma2, ip, m, 12, 4, acc_v3rhosigma2_4);
        store_strided(v3rhosigma2, ip, m, 12, 5, acc_v3rhosigma2_5);
        store_strided(v3rhosigma2, ip, m, 12, 6, acc_v3rhosigma2_6);
        store_strided(v3rhosigma2, ip, m, 12, 7, acc_v3rhosigma2_7);
        store_strided(v3rhosigma2, ip, m, 12, 8, acc_v3rhosigma2_8);
        store_strided(v3rhosigma2, ip, m, 12, 9, acc_v3rhosigma2_9);
        store_strided(v3rhosigma2, ip, m, 12, 10, acc_v3rhosigma2_10);
        store_strided(v3rhosigma2, ip, m, 12, 11, acc_v3rhosigma2_11);
        store_strided(v3sigma3, ip, m, 10, 0, acc_v3sigma3_0);
        store_strided(v3sigma3, ip, m, 10, 1, acc_v3sigma3_1);
        store_strided(v3sigma3, ip, m, 10, 2, acc_v3sigma3_2);
        store_strided(v3sigma3, ip, m, 10, 3, acc_v3sigma3_3);
        store_strided(v3sigma3, ip, m, 10, 4, acc_v3sigma3_4);
        store_strided(v3sigma3, ip, m, 10, 5, acc_v3sigma3_5);
        store_strided(v3sigma3, ip, m, 10, 6, acc_v3sigma3_6);
        store_strided(v3sigma3, ip, m, 10, 7, acc_v3sigma3_7);
        store_strided(v3sigma3, ip, m, 10, 8, acc_v3sigma3_8);
        store_strided(v3sigma3, ip, m, 10, 9, acc_v3sigma3_9);
        ip += 8;
    }
}
