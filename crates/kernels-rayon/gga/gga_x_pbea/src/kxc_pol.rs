//! GGA_X_PBEA kxc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_pbea.c`
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
pub fn gga_x_pbea_kxc_pol(
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
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
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
            let t28 = v_rho0 * v_rho0;
            let t29 = (simd::cbrt(v_rho0));
            let t30 = t29 * t29;
            let t32 = f64x8::splat(1.0) / t30 / t28;
            let t35 = f64x8::splat(1.0) + f64x8::splat(0.008639940809536326) * v_sigma0 * t32;
            let t36 = (simd::pow(t35, -f64x8::splat(0.52)));
            let t38 = f64x8::splat(1.804) - f64x8::splat(0.804) * t36;
            let t42 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t25 * t26 * t38));
            let t43 = (v_rho1).simd_le(dens_threshold);
            let t44 = -t16;
            let t46 = ((t14).select(t11, (t10).select(t15, t44 * t7)));
            let t47 = f64x8::splat(1.0) + t46;
            let t48 = (t47).simd_le(zeta_threshold);
            let t49 = (simd::cbrt(t47));
            let t51 = ((t48).select(t22, t49 * t47));
            let t53 = v_rho1 * v_rho1;
            let t54 = (simd::cbrt(v_rho1));
            let t55 = t54 * t54;
            let t57 = f64x8::splat(1.0) / t55 / t53;
            let t60 = f64x8::splat(1.0) + f64x8::splat(0.008639940809536326) * v_sigma2 * t57;
            let t61 = (simd::pow(t60, -f64x8::splat(0.52)));
            let t63 = f64x8::splat(1.804) - f64x8::splat(0.804) * t61;
            let t67 = ((t43).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t51 * t26 * t63));
            let tzk0 = t42 + t67;
            acc_zk = tzk0;
            let t68 = t6 * t6;
            let t69 = f64x8::splat(1.0) / t68;
            let t70 = t16 * t69;
            let t72 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), t7 - t70)));
            let t75 = ((t20).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t72));
            let t80 = t26 * t26;
            let t81 = f64x8::splat(1.0) / t80;
            let t85 = t5 * t25 * t81 * t38 / f64x8::splat(8.0);
            let t86 = t2 * t25;
            let t87 = t86 * t26;
            let t88 = (simd::pow(t35, -f64x8::splat(1.52)));
            let t89 = t88 * v_sigma0;
            let t90 = t28 * v_rho0;
            let t92 = f64x8::splat(1.0) / t30 / t90;
            let t93 = t89 * t92;
            let t97 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t75 * t26 * t38 - t85 + f64x8::splat(0.00246634334405953) * t87 * t93));
            let t98 = t44 * t69;
            let t100 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), -t7 - t98)));
            let t103 = ((t48).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t49 * t100));
            let t111 = t5 * t51 * t81 * t63 / f64x8::splat(8.0);
            let t113 = ((t43).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t103 * t26 * t63 - t111));
            let tvrho0 = t42 + t67 + t6 * (t97 + t113);
            acc_vrho_0 = tvrho0;
            let t117 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), -t7 - t70)));
            let t120 = ((t20).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t117));
            let t126 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t120 * t26 * t38 - t85));
            let t128 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), t7 - t98)));
            let t131 = ((t48).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t49 * t128));
            let t136 = t2 * t51;
            let t137 = t136 * t26;
            let t138 = (simd::pow(t60, -f64x8::splat(1.52)));
            let t139 = t138 * v_sigma2;
            let t140 = t53 * v_rho1;
            let t142 = f64x8::splat(1.0) / t55 / t140;
            let t143 = t139 * t142;
            let t147 = ((t43).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t131 * t26 * t63 - t111 + f64x8::splat(0.00246634334405953) * t137 * t143));
            let tvrho1 = t42 + t67 + t6 * (t126 + t147);
            acc_vrho_1 = tvrho1;
            let t150 = t26 * t88;
            let t151 = t150 * t32;
            let t154 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(0.0009248787540223239) * t86 * t151));
            let tvsigma0 = t6 * t154;
            acc_vsigma_0 = tvsigma0;
            let tvsigma1 = f64x8::splat(0.0);
            acc_vsigma_1 = tvsigma1;
            let t155 = t26 * t138;
            let t156 = t155 * t57;
            let t159 = ((t43).select(f64x8::splat(0.0), -f64x8::splat(0.0009248787540223239) * t136 * t156));
            let tvsigma2 = t6 * t159;
            acc_vsigma_2 = tvsigma2;
            let t162 = t23 * t23;
            let t163 = f64x8::splat(1.0) / t162;
            let t164 = t72 * t72;
            let t167 = t68 * t6;
            let t168 = f64x8::splat(1.0) / t167;
            let t169 = t16 * t168;
            let t172 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), -f64x8::splat(2.0) * t69 + f64x8::splat(2.0) * t169)));
            let t176 = ((t20).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(9.0) * t163 * t164 + f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t172));
            let t183 = t5 * t75 * t81 * t38;
            let t185 = t2 * t75;
            let t186 = t185 * t26;
            let t190 = f64x8::splat(1.0) / t80 / t6;
            let t194 = t5 * t25 * t190 * t38 / f64x8::splat(12.0);
            let t195 = t86 * t81;
            let t196 = t195 * t93;
            let t198 = (simd::pow(t35, -f64x8::splat(2.52)));
            let t199 = v_sigma0 * v_sigma0;
            let t200 = t198 * t199;
            let t201 = t28 * t28;
            let t204 = f64x8::splat(1.0) / t29 / t201 / t90;
            let t205 = t200 * t204;
            let t209 = f64x8::splat(1.0) / t30 / t201;
            let t210 = t89 * t209;
            let t214 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t176 * t26 * t38 - t183 / f64x8::splat(4.0) + f64x8::splat(0.00493268668811906) * t186 * t93 + t194 + f64x8::splat(0.0016442288960396869) * t196 + f64x8::splat(8.637272526180187e-05) * t87 * t205 - f64x8::splat(0.009043258928218278) * t87 * t210));
            let t215 = t49 * t49;
            let t216 = f64x8::splat(1.0) / t215;
            let t217 = t100 * t100;
            let t220 = t44 * t168;
            let t223 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), f64x8::splat(2.0) * t69 + f64x8::splat(2.0) * t220)));
            let t227 = ((t48).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(9.0) * t216 * t217 + f64x8::splat(4.0) / f64x8::splat(3.0) * t49 * t223));
            let t234 = t5 * t103 * t81 * t63;
            let t239 = t5 * t51 * t190 * t63 / f64x8::splat(12.0);
            let t241 = ((t43).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t227 * t26 * t63 - t234 / f64x8::splat(4.0) + t239));
            let tv2rho20 = f64x8::splat(2.0) * t97 + f64x8::splat(2.0) * t113 + t6 * (t214 + t241);
            acc_v2rho2_0 = tv2rho20;
            let t244 = t163 * t117;
            let t248 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), f64x8::splat(2.0) * t169)));
            let t252 = ((t20).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(9.0) * t244 * t72 + f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t248));
            let t259 = t5 * t120 * t81 * t38;
            let t261 = t2 * t120;
            let t262 = t261 * t26;
            let t268 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t252 * t26 * t38 - t259 / f64x8::splat(8.0) + f64x8::splat(0.00246634334405953) * t262 * t93 - t183 / f64x8::splat(8.0) + t194 + f64x8::splat(0.0008221144480198434) * t196));
            let t269 = t216 * t128;
            let t273 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), f64x8::splat(2.0) * t220)));
            let t277 = ((t48).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(9.0) * t269 * t100 + f64x8::splat(4.0) / f64x8::splat(3.0) * t49 * t273));
            let t284 = t5 * t131 * t81 * t63;
            let t287 = t2 * t103;
            let t288 = t287 * t26;
            let t291 = t136 * t81;
            let t292 = t291 * t143;
            let t295 = ((t43).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t277 * t26 * t63 - t284 / f64x8::splat(8.0) - t234 / f64x8::splat(8.0) + t239 + f64x8::splat(0.00246634334405953) * t288 * t143 + f64x8::splat(0.0008221144480198434) * t292));
            let tv2rho21 = t97 + t113 + t126 + t147 + t6 * (t268 + t295);
            acc_v2rho2_1 = tv2rho21;
            let t300 = t117 * t117;
            let t305 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), f64x8::splat(2.0) * t69 + f64x8::splat(2.0) * t169)));
            let t309 = ((t20).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(9.0) * t163 * t300 + f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t305));
            let t316 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t309 * t26 * t38 - t259 / f64x8::splat(4.0) + t194));
            let t317 = t128 * t128;
            let t322 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), -f64x8::splat(2.0) * t69 + f64x8::splat(2.0) * t220)));
            let t326 = ((t48).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(9.0) * t216 * t317 + f64x8::splat(4.0) / f64x8::splat(3.0) * t49 * t322));
            let t332 = t2 * t131;
            let t333 = t332 * t26;
            let t337 = (simd::pow(t60, -f64x8::splat(2.52)));
            let t338 = v_sigma2 * v_sigma2;
            let t339 = t337 * t338;
            let t340 = t53 * t53;
            let t343 = f64x8::splat(1.0) / t54 / t340 / t140;
            let t344 = t339 * t343;
            let t348 = f64x8::splat(1.0) / t55 / t340;
            let t349 = t139 * t348;
            let t353 = ((t43).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t326 * t26 * t63 - t284 / f64x8::splat(4.0) + f64x8::splat(0.00493268668811906) * t333 * t143 + t239 + f64x8::splat(0.0016442288960396869) * t292 + f64x8::splat(8.637272526180187e-05) * t137 * t344 - f64x8::splat(0.009043258928218278) * t137 * t349));
            let tv2rho22 = f64x8::splat(2.0) * t126 + f64x8::splat(2.0) * t147 + t6 * (t316 + t353);
            acc_v2rho2_2 = tv2rho22;
            let t358 = t81 * t88;
            let t359 = t358 * t32;
            let t361 = f64x8::splat(0.00030829291800744127) * t86 * t359;
            let t362 = t201 * t28;
            let t364 = f64x8::splat(1.0) / t29 / t362;
            let t366 = t198 * t364 * v_sigma0;
            let t369 = t150 * t92;
            let t373 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(0.0009248787540223239) * t185 * t151 - t361 - f64x8::splat(3.23897719731757e-05) * t87 * t366 + f64x8::splat(0.00246634334405953) * t86 * t369));
            let tv2rhosigma0 = t6 * t373 + t154;
            acc_v2rhosigma_0 = tv2rhosigma0;
            let tv2rhosigma1 = f64x8::splat(0.0);
            acc_v2rhosigma_1 = tv2rhosigma1;
            let t377 = t81 * t138;
            let t378 = t377 * t57;
            let t380 = f64x8::splat(0.00030829291800744127) * t136 * t378;
            let t382 = ((t43).select(f64x8::splat(0.0), -f64x8::splat(0.0009248787540223239) * t287 * t156 - t380));
            let tv2rhosigma2 = t6 * t382 + t159;
            acc_v2rhosigma_2 = tv2rhosigma2;
            let t387 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(0.0009248787540223239) * t261 * t151 - t361));
            let tv2rhosigma3 = t6 * t387 + t154;
            acc_v2rhosigma_3 = tv2rhosigma3;
            let tv2rhosigma4 = f64x8::splat(0.0);
            acc_v2rhosigma_4 = tv2rhosigma4;
            let t391 = t340 * t53;
            let t393 = f64x8::splat(1.0) / t54 / t391;
            let t395 = t337 * t393 * v_sigma2;
            let t398 = t155 * t142;
            let t402 = ((t43).select(f64x8::splat(0.0), -f64x8::splat(0.0009248787540223239) * t332 * t156 - t380 - f64x8::splat(3.23897719731757e-05) * t137 * t395 + f64x8::splat(0.00246634334405953) * t136 * t398));
            let tv2rhosigma5 = t6 * t402 + t159;
            acc_v2rhosigma_5 = tv2rhosigma5;
            let t404 = t26 * t198;
            let t405 = t201 * v_rho0;
            let t407 = f64x8::splat(1.0) / t29 / t405;
            let t408 = t404 * t407;
            let t411 = ((t1).select(f64x8::splat(0.0), f64x8::splat(1.214616448994089e-05) * t86 * t408));
            let tv2sigma20 = t6 * t411;
            acc_v2sigma2_0 = tv2sigma20;
            let tv2sigma21 = f64x8::splat(0.0);
            acc_v2sigma2_1 = tv2sigma21;
            let tv2sigma22 = f64x8::splat(0.0);
            acc_v2sigma2_2 = tv2sigma22;
            let tv2sigma23 = f64x8::splat(0.0);
            acc_v2sigma2_3 = tv2sigma23;
            let tv2sigma24 = f64x8::splat(0.0);
            acc_v2sigma2_4 = tv2sigma24;
            let t412 = t26 * t337;
            let t413 = t340 * v_rho1;
            let t415 = f64x8::splat(1.0) / t54 / t413;
            let t416 = t412 * t415;
            let t419 = ((t43).select(f64x8::splat(0.0), f64x8::splat(1.214616448994089e-05) * t136 * t416));
            let tv2sigma25 = t6 * t419;
            acc_v2sigma2_5 = tv2sigma25;
            let t423 = f64x8::splat(1.0) / t162 / t19;
            let t424 = t164 * t72;
            let t427 = t163 * t72;
            let t430 = t68 * t68;
            let t431 = f64x8::splat(1.0) / t430;
            let t432 = t16 * t431;
            let t435 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), f64x8::splat(6.0) * t168 - f64x8::splat(6.0) * t432)));
            let t439 = ((t20).select(f64x8::splat(0.0), -f64x8::splat(8.0) / f64x8::splat(27.0) * t423 * t424 + f64x8::splat(4.0) / f64x8::splat(3.0) * t427 * t172 + f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t435));
            let t444 = t195 * t210;
            let t446 = t201 * t201;
            let t448 = f64x8::splat(1.0) / t29 / t446;
            let t449 = t200 * t448;
            let t453 = f64x8::splat(1.0) / t30 / t405;
            let t454 = t89 * t453;
            let t459 = t185 * t81;
            let t460 = t459 * t93;
            let t465 = f64x8::splat(1.0) / t80 / t68;
            let t469 = f64x8::splat(5.0) / f64x8::splat(36.0) * t5 * t25 * t465 * t38;
            let t470 = t86 * t190;
            let t471 = t470 * t93;
            let t473 = t195 * t205;
            let t475 = (simd::pow(t35, -f64x8::splat(3.52)));
            let t476 = t199 * v_sigma0;
            let t477 = t475 * t476;
            let t478 = t446 * t90;
            let t479 = f64x8::splat(1.0) / t478;
            let t480 = t477 * t479;
            let t485 = t5 * t176 * t81 * t38;
            let t487 = t2 * t176;
            let t488 = t487 * t26;
            let t493 = t5 * t75 * t190 * t38;
            let t495 = -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t439 * t26 * t38 - f64x8::splat(0.009043258928218278) * t444 - f64x8::splat(0.0009500999778798207) * t87 * t449 + f64x8::splat(0.042201874998351964) * t87 * t454 - f64x8::splat(0.027129776784654835) * t186 * t210 + f64x8::splat(0.00493268668811906) * t460 + f64x8::splat(0.0002591181757854056) * t186 * t205 - t469 - f64x8::splat(0.0016442288960396869) * t471 + f64x8::splat(8.637272526180187e-05) * t473 + f64x8::splat(5.014835171272491e-06) * t87 * t480 - f64x8::splat(3.0) / f64x8::splat(8.0) * t485 + f64x8::splat(0.007399030032178591) * t488 * t93 + t493 / f64x8::splat(4.0);
            let t496 = ((t1).select(f64x8::splat(0.0), t495));
            let t498 = f64x8::splat(1.0) / t215 / t47;
            let t499 = t217 * t100;
            let t502 = t216 * t100;
            let t505 = t44 * t431;
            let t508 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), -f64x8::splat(6.0) * t168 - f64x8::splat(6.0) * t505)));
            let t512 = ((t48).select(f64x8::splat(0.0), -f64x8::splat(8.0) / f64x8::splat(27.0) * t498 * t499 + f64x8::splat(4.0) / f64x8::splat(3.0) * t502 * t223 + f64x8::splat(4.0) / f64x8::splat(3.0) * t49 * t508));
            let t519 = t5 * t227 * t81 * t63;
            let t523 = t5 * t103 * t190 * t63;
            let t528 = f64x8::splat(5.0) / f64x8::splat(36.0) * t5 * t51 * t465 * t63;
            let t530 = ((t43).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t512 * t26 * t63 - f64x8::splat(3.0) / f64x8::splat(8.0) * t519 + t523 / f64x8::splat(4.0) - t528));
            let tv3rho30 = f64x8::splat(3.0) * t214 + f64x8::splat(3.0) * t241 + t6 * (t496 + t530);
            acc_v3rho3_0 = tv3rho30;
            let t533 = f64x8::splat(2.0) * t268;
            let t534 = f64x8::splat(2.0) * t295;
            let t535 = t423 * t117;
            let t538 = t163 * t248;
            let t543 = f64x8::splat(2.0) * t168;
            let t544 = f64x8::splat(6.0) * t432;
            let t546 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), t543 - t544)));
            let t550 = ((t20).select(f64x8::splat(0.0), -f64x8::splat(8.0) / f64x8::splat(27.0) * t535 * t164 + f64x8::splat(8.0) / f64x8::splat(9.0) * t538 * t72 + f64x8::splat(4.0) / f64x8::splat(9.0) * t244 * t172 + f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t546));
            let t558 = t5 * t252 * t81 * t38 / f64x8::splat(4.0);
            let t559 = t2 * t252;
            let t560 = t559 * t26;
            let t565 = t5 * t120 * t190 * t38;
            let t567 = t261 * t81;
            let t569 = f64x8::splat(0.0016442288960396869) * t567 * t93;
            let t580 = -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t550 * t26 * t38 - t558 + f64x8::splat(0.00493268668811906) * t560 * t93 + t565 / f64x8::splat(12.0) + t569 + f64x8::splat(8.637272526180187e-05) * t262 * t205 - f64x8::splat(0.009043258928218278) * t262 * t210 - t485 / f64x8::splat(8.0) + t493 / f64x8::splat(6.0) + f64x8::splat(0.0016442288960396869) * t460 - t469 - f64x8::splat(0.0010961525973597912) * t471 + f64x8::splat(2.8790908420600628e-05) * t473 - f64x8::splat(0.003014419642739426) * t444;
            let t581 = ((t1).select(f64x8::splat(0.0), t580));
            let t582 = t498 * t128;
            let t585 = t216 * t273;
            let t590 = f64x8::splat(6.0) * t505;
            let t592 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), -t543 - t590)));
            let t596 = ((t48).select(f64x8::splat(0.0), -f64x8::splat(8.0) / f64x8::splat(27.0) * t582 * t217 + f64x8::splat(8.0) / f64x8::splat(9.0) * t585 * t100 + f64x8::splat(4.0) / f64x8::splat(9.0) * t269 * t223 + f64x8::splat(4.0) / f64x8::splat(3.0) * t49 * t592));
            let t604 = t5 * t277 * t81 * t63 / f64x8::splat(4.0);
            let t607 = t5 * t131 * t190 * t63;
            let t611 = t2 * t227;
            let t612 = t611 * t26;
            let t615 = t287 * t81;
            let t617 = f64x8::splat(0.0016442288960396869) * t615 * t143;
            let t618 = t136 * t190;
            let t619 = t618 * t143;
            let t622 = ((t43).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t596 * t26 * t63 - t604 + t607 / f64x8::splat(12.0) - t519 / f64x8::splat(8.0) + t523 / f64x8::splat(6.0) - t528 + f64x8::splat(0.00246634334405953) * t612 * t143 + t617 - f64x8::splat(0.0005480762986798956) * t619));
            let tv3rho31 = t214 + t241 + t533 + t534 + t6 * (t581 + t622);
            acc_v3rho3_1 = tv3rho31;
            let t625 = t423 * t300;
            let t630 = t163 * t305;
            let t634 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), -t543 - t544)));
            let t638 = ((t20).select(f64x8::splat(0.0), -f64x8::splat(8.0) / f64x8::splat(27.0) * t625 * t72 + f64x8::splat(8.0) / f64x8::splat(9.0) * t244 * t248 + f64x8::splat(4.0) / f64x8::splat(9.0) * t630 * t72 + f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t634));
            let t645 = t5 * t309 * t81 * t38;
            let t647 = t2 * t309;
            let t648 = t647 * t26;
            let t655 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t638 * t26 * t38 - t645 / f64x8::splat(8.0) + f64x8::splat(0.00246634334405953) * t648 * t93 - t558 + t565 / f64x8::splat(6.0) + t569 + t493 / f64x8::splat(12.0) - t469 - f64x8::splat(0.0005480762986798956) * t471));
            let t656 = t498 * t317;
            let t661 = t216 * t322;
            let t665 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), t543 - t590)));
            let t669 = ((t48).select(f64x8::splat(0.0), -f64x8::splat(8.0) / f64x8::splat(27.0) * t656 * t100 + f64x8::splat(8.0) / f64x8::splat(9.0) * t269 * t273 + f64x8::splat(4.0) / f64x8::splat(9.0) * t661 * t100 + f64x8::splat(4.0) / f64x8::splat(3.0) * t49 * t665));
            let t676 = t5 * t326 * t81 * t63;
            let t679 = t2 * t277;
            let t680 = t679 * t26;
            let t683 = t332 * t81;
            let t684 = t683 * t143;
            let t690 = t291 * t344;
            let t694 = t291 * t349;
            let t696 = -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t669 * t26 * t63 - t676 / f64x8::splat(8.0) - t604 + t607 / f64x8::splat(6.0) + f64x8::splat(0.00493268668811906) * t680 * t143 + f64x8::splat(0.0016442288960396869) * t684 + t523 / f64x8::splat(12.0) - t528 + t617 - f64x8::splat(0.0010961525973597912) * t619 + f64x8::splat(8.637272526180187e-05) * t288 * t344 + f64x8::splat(2.8790908420600628e-05) * t690 - f64x8::splat(0.009043258928218278) * t288 * t349 - f64x8::splat(0.003014419642739426) * t694;
            let t697 = ((t43).select(f64x8::splat(0.0), t696));
            let tv3rho32 = t533 + t534 + t316 + t353 + t6 * (t655 + t697);
            acc_v3rho3_2 = tv3rho32;
            let t702 = t300 * t117;
            let t709 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), -f64x8::splat(6.0) * t168 - f64x8::splat(6.0) * t432)));
            let t713 = ((t20).select(f64x8::splat(0.0), -f64x8::splat(8.0) / f64x8::splat(27.0) * t423 * t702 + f64x8::splat(4.0) / f64x8::splat(3.0) * t244 * t305 + f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t709));
            let t721 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t713 * t26 * t38 - f64x8::splat(3.0) / f64x8::splat(8.0) * t645 + t565 / f64x8::splat(4.0) - t469));
            let t722 = t2 * t326;
            let t723 = t722 * t26;
            let t728 = (simd::pow(t60, -f64x8::splat(3.52)));
            let t729 = t338 * v_sigma2;
            let t730 = t728 * t729;
            let t731 = t340 * t340;
            let t732 = t731 * t140;
            let t733 = f64x8::splat(1.0) / t732;
            let t734 = t730 * t733;
            let t739 = t317 * t128;
            let t746 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), f64x8::splat(6.0) * t168 - f64x8::splat(6.0) * t505)));
            let t750 = ((t48).select(f64x8::splat(0.0), -f64x8::splat(8.0) / f64x8::splat(27.0) * t498 * t739 + f64x8::splat(4.0) / f64x8::splat(3.0) * t269 * t322 + f64x8::splat(4.0) / f64x8::splat(3.0) * t49 * t746));
            let t758 = f64x8::splat(1.0) / t54 / t731;
            let t759 = t339 * t758;
            let t763 = f64x8::splat(1.0) / t55 / t413;
            let t764 = t139 * t763;
            let t771 = f64x8::splat(0.007399030032178591) * t723 * t143 + f64x8::splat(0.0002591181757854056) * t333 * t344 + f64x8::splat(5.014835171272491e-06) * t137 * t734 - f64x8::splat(3.0) / f64x8::splat(8.0) * t676 + t607 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t750 * t26 * t63 - f64x8::splat(0.027129776784654835) * t333 * t349 - f64x8::splat(0.0009500999778798207) * t137 * t759 + f64x8::splat(0.042201874998351964) * t137 * t764 + f64x8::splat(8.637272526180187e-05) * t690 - f64x8::splat(0.009043258928218278) * t694 + f64x8::splat(0.00493268668811906) * t684 - f64x8::splat(0.0016442288960396869) * t619 - t528;
            let t772 = ((t43).select(f64x8::splat(0.0), t771));
            let tv3rho33 = f64x8::splat(3.0) * t316 + f64x8::splat(3.0) * t353 + t6 * (t721 + t772);
            acc_v3rho3_3 = tv3rho33;
            let t778 = t185 * t359;
            let t784 = t190 * t88;
            let t785 = t784 * t32;
            let t787 = f64x8::splat(0.00020552861200496086) * t86 * t785;
            let t788 = t195 * t366;
            let t790 = t358 * t92;
            let t791 = t86 * t790;
            let t793 = t446 * t28;
            let t795 = t475 / t793;
            let t796 = t795 * t199;
            let t800 = t198 * t204 * v_sigma0;
            let t803 = t150 * t209;
            let t807 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(0.0009248787540223239) * t487 * t151 - f64x8::splat(0.0006165858360148825) * t778 - f64x8::splat(6.47795439463514e-05) * t186 * t366 + f64x8::splat(0.00493268668811906) * t185 * t369 + t787 - f64x8::splat(2.159318131545047e-05) * t788 + f64x8::splat(0.0016442288960396869) * t791 - f64x8::splat(1.8805631892271842e-06) * t87 * t796 + f64x8::splat(0.00029150794775858135) * t87 * t800 - f64x8::splat(0.009043258928218278) * t86 * t803));
            let tv3rho2sigma0 = t6 * t807 + f64x8::splat(2.0) * t373;
            acc_v3rho2sigma_0 = tv3rho2sigma0;
            let tv3rho2sigma1 = f64x8::splat(0.0);
            acc_v3rho2sigma_1 = tv3rho2sigma1;
            let t812 = t287 * t378;
            let t814 = t190 * t138;
            let t815 = t814 * t57;
            let t817 = f64x8::splat(0.00020552861200496086) * t136 * t815;
            let t819 = ((t43).select(f64x8::splat(0.0), -f64x8::splat(0.0009248787540223239) * t611 * t156 - f64x8::splat(0.0006165858360148825) * t812 + t817));
            let tv3rho2sigma2 = t6 * t819 + f64x8::splat(2.0) * t382;
            acc_v3rho2sigma_2 = tv3rho2sigma2;
            let t823 = t261 * t359;
            let t833 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(0.0009248787540223239) * t559 * t151 - f64x8::splat(0.00030829291800744127) * t823 - f64x8::splat(3.23897719731757e-05) * t262 * t366 + f64x8::splat(0.00246634334405953) * t261 * t369 - f64x8::splat(0.00030829291800744127) * t778 + t787 - f64x8::splat(1.0796590657725234e-05) * t788 + f64x8::splat(0.0008221144480198434) * t791));
            let tv3rho2sigma3 = t6 * t833 + t373 + t387;
            acc_v3rho2sigma_3 = tv3rho2sigma3;
            let tv3rho2sigma4 = f64x8::splat(0.0);
            acc_v3rho2sigma_4 = tv3rho2sigma4;
            let t837 = t332 * t378;
            let t842 = t291 * t395;
            let t846 = t377 * t142;
            let t847 = t136 * t846;
            let t850 = ((t43).select(f64x8::splat(0.0), -f64x8::splat(0.0009248787540223239) * t679 * t156 - f64x8::splat(0.00030829291800744127) * t837 - f64x8::splat(0.00030829291800744127) * t812 + t817 - f64x8::splat(3.23897719731757e-05) * t288 * t395 - f64x8::splat(1.0796590657725234e-05) * t842 + f64x8::splat(0.00246634334405953) * t287 * t398 + f64x8::splat(0.0008221144480198434) * t847));
            let tv3rho2sigma5 = t6 * t850 + t382 + t402;
            acc_v3rho2sigma_5 = tv3rho2sigma5;
            let t857 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(0.0009248787540223239) * t647 * t151 - f64x8::splat(0.0006165858360148825) * t823 + t787));
            let tv3rho2sigma6 = t6 * t857 + f64x8::splat(2.0) * t387;
            acc_v3rho2sigma_6 = tv3rho2sigma6;
            let tv3rho2sigma7 = f64x8::splat(0.0);
            acc_v3rho2sigma_7 = tv3rho2sigma7;
            let t869 = t731 * t53;
            let t871 = t728 / t869;
            let t872 = t871 * t338;
            let t876 = t337 * t343 * v_sigma2;
            let t879 = t155 * t348;
            let t883 = ((t43).select(f64x8::splat(0.0), -f64x8::splat(0.0009248787540223239) * t722 * t156 - f64x8::splat(0.0006165858360148825) * t837 - f64x8::splat(6.47795439463514e-05) * t333 * t395 + f64x8::splat(0.00493268668811906) * t332 * t398 + t817 - f64x8::splat(2.159318131545047e-05) * t842 + f64x8::splat(0.0016442288960396869) * t847 - f64x8::splat(1.8805631892271842e-06) * t137 * t872 + f64x8::splat(0.00029150794775858135) * t137 * t876 - f64x8::splat(0.009043258928218278) * t136 * t879));
            let tv3rho2sigma8 = t6 * t883 + f64x8::splat(2.0) * t402;
            acc_v3rho2sigma_8 = tv3rho2sigma8;
            let t887 = t81 * t198;
            let t888 = t887 * t407;
            let t890 = f64x8::splat(4.048721496646963e-06) * t86 * t888;
            let t891 = t446 * v_rho0;
            let t892 = f64x8::splat(1.0) / t891;
            let t894 = t475 * t892 * v_sigma0;
            let t897 = t404 * t364;
            let t901 = ((t1).select(f64x8::splat(0.0), f64x8::splat(1.214616448994089e-05) * t185 * t408 + t890 + f64x8::splat(7.05211195960194e-07) * t87 * t894 - f64x8::splat(6.47795439463514e-05) * t86 * t897));
            let tv3rhosigma20 = t6 * t901 + t411;
            acc_v3rhosigma2_0 = tv3rhosigma20;
            let tv3rhosigma21 = f64x8::splat(0.0);
            acc_v3rhosigma2_1 = tv3rhosigma21;
            let tv3rhosigma22 = f64x8::splat(0.0);
            acc_v3rhosigma2_2 = tv3rhosigma22;
            let tv3rhosigma23 = f64x8::splat(0.0);
            acc_v3rhosigma2_3 = tv3rhosigma23;
            let tv3rhosigma24 = f64x8::splat(0.0);
            acc_v3rhosigma2_4 = tv3rhosigma24;
            let t905 = t81 * t337;
            let t906 = t905 * t415;
            let t908 = f64x8::splat(4.048721496646963e-06) * t136 * t906;
            let t910 = ((t43).select(f64x8::splat(0.0), f64x8::splat(1.214616448994089e-05) * t287 * t416 + t908));
            let tv3rhosigma25 = t6 * t910 + t419;
            acc_v3rhosigma2_5 = tv3rhosigma25;
            let t915 = ((t1).select(f64x8::splat(0.0), f64x8::splat(1.214616448994089e-05) * t261 * t408 + t890));
            let tv3rhosigma26 = t6 * t915 + t411;
            acc_v3rhosigma2_6 = tv3rhosigma26;
            let tv3rhosigma27 = f64x8::splat(0.0);
            acc_v3rhosigma2_7 = tv3rhosigma27;
            let tv3rhosigma28 = f64x8::splat(0.0);
            acc_v3rhosigma2_8 = tv3rhosigma28;
            let tv3rhosigma29 = f64x8::splat(0.0);
            acc_v3rhosigma2_9 = tv3rhosigma29;
            let tv3rhosigma210 = f64x8::splat(0.0);
            acc_v3rhosigma2_10 = tv3rhosigma210;
            let t919 = t731 * v_rho1;
            let t920 = f64x8::splat(1.0) / t919;
            let t922 = t728 * t920 * v_sigma2;
            let t925 = t412 * t393;
            let t929 = ((t43).select(f64x8::splat(0.0), f64x8::splat(1.214616448994089e-05) * t332 * t416 + t908 + f64x8::splat(7.05211195960194e-07) * t137 * t922 - f64x8::splat(6.47795439463514e-05) * t136 * t925));
            let tv3rhosigma211 = t6 * t929 + t419;
            acc_v3rhosigma2_11 = tv3rhosigma211;
            let t931 = t26 * t475;
            let t932 = f64x8::splat(1.0) / t446;
            let t933 = t931 * t932;
            let t936 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(2.644541984850728e-07) * t86 * t933));
            let tv3sigma30 = t6 * t936;
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
            let t937 = t26 * t728;
            let t938 = f64x8::splat(1.0) / t731;
            let t939 = t937 * t938;
            let t942 = ((t43).select(f64x8::splat(0.0), -f64x8::splat(2.644541984850728e-07) * t136 * t939));
            let tv3sigma39 = t6 * t942;
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
