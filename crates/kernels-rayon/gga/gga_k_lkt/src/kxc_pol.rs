//! GGA_K_LKT kxc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_k_lkt.c`
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
pub fn gga_k_lkt_kxc_pol(
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
    param_a: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_a = f64x8::splat(param_a);
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
            let t45 = (t44).simd_lt(f64x8::splat(200.0));
            let t46 = ((t45).select(t44, f64x8::splat(200.0)));
            let t47 = param_a * t46;
            let t48 = (simd::cosh(t47));
            let t49 = f64x8::splat(1.0) / t48;
            let t50 = t35 * t35;
            let t52 = t32 / t50;
            let t53 = v_rho0 * v_rho0;
            let t54 = t39 * t39;
            let t56 = f64x8::splat(1.0) / t54 / t53;
            let t60 = t49 + f64x8::splat(5.0) / f64x8::splat(72.0) * t52 * v_sigma0 * t56;
            let t64 = ((t1).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t31 * t60));
            let t65 = (v_rho1).simd_le(dens_threshold);
            let t66 = -t17;
            let t68 = ((t15).select(t12, (t11).select(t16, t66 * t8)));
            let t69 = f64x8::splat(1.0) + t68;
            let t70 = (t69).simd_le(zeta_threshold);
            let t71 = (simd::cbrt(t69));
            let t72 = t71 * t71;
            let t74 = ((t70).select(t24, t72 * t69));
            let t75 = t74 * t30;
            let t76 = ((v_sigma2).sqrt());
            let t77 = (simd::cbrt(v_rho1));
            let t79 = f64x8::splat(1.0) / t77 / v_rho1;
            let t82 = t37 * t76 * t79 / f64x8::splat(12.0);
            let t83 = (t82).simd_lt(f64x8::splat(200.0));
            let t84 = ((t83).select(t82, f64x8::splat(200.0)));
            let t85 = param_a * t84;
            let t86 = (simd::cosh(t85));
            let t87 = f64x8::splat(1.0) / t86;
            let t88 = v_rho1 * v_rho1;
            let t89 = t77 * t77;
            let t91 = f64x8::splat(1.0) / t89 / t88;
            let t95 = t87 + f64x8::splat(5.0) / f64x8::splat(72.0) * t52 * v_sigma2 * t91;
            let t99 = ((t65).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t75 * t95));
            let tzk0 = t64 + t99;
            acc_zk = tzk0;
            let t100 = t7 * t7;
            let t101 = f64x8::splat(1.0) / t100;
            let t102 = t17 * t101;
            let t104 = ((t11).select(f64x8::splat(0.0), (t15).select(f64x8::splat(0.0), t8 - t102)));
            let t107 = ((t21).select(f64x8::splat(0.0), f64x8::splat(5.0) / f64x8::splat(3.0) * t26 * t104));
            let t108 = t107 * t30;
            let t112 = f64x8::splat(1.0) / t29;
            let t113 = t28 * t112;
            let t116 = t6 * t113 * t60 / f64x8::splat(10.0);
            let t117 = t48 * t48;
            let t118 = f64x8::splat(1.0) / t117;
            let t119 = t118 * param_a;
            let t121 = f64x8::splat(1.0) / t39 / t53;
            let t125 = ((t45).select(-t37 * t38 * t121 / f64x8::splat(9.0), f64x8::splat(0.0)));
            let t126 = (simd::sinh(t47));
            let t127 = t125 * t126;
            let t129 = t53 * v_rho0;
            let t131 = f64x8::splat(1.0) / t54 / t129;
            let t135 = -t119 * t127 - f64x8::splat(5.0) / f64x8::splat(27.0) * t52 * v_sigma0 * t131;
            let t140 = ((t1).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t108 * t60 + t116 + f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t31 * t135));
            let t141 = t66 * t101;
            let t143 = ((t15).select(f64x8::splat(0.0), (t11).select(f64x8::splat(0.0), -t8 - t141)));
            let t146 = ((t70).select(f64x8::splat(0.0), f64x8::splat(5.0) / f64x8::splat(3.0) * t72 * t143));
            let t147 = t146 * t30;
            let t151 = t74 * t112;
            let t154 = t6 * t151 * t95 / f64x8::splat(10.0);
            let t156 = ((t65).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t147 * t95 + t154));
            let tvrho0 = t64 + t99 + t7 * (t140 + t156);
            acc_vrho_0 = tvrho0;
            let t160 = ((t11).select(f64x8::splat(0.0), (t15).select(f64x8::splat(0.0), -t8 - t102)));
            let t163 = ((t21).select(f64x8::splat(0.0), f64x8::splat(5.0) / f64x8::splat(3.0) * t26 * t160));
            let t164 = t163 * t30;
            let t169 = ((t1).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t164 * t60 + t116));
            let t171 = ((t15).select(f64x8::splat(0.0), (t11).select(f64x8::splat(0.0), t8 - t141)));
            let t174 = ((t70).select(f64x8::splat(0.0), f64x8::splat(5.0) / f64x8::splat(3.0) * t72 * t171));
            let t175 = t174 * t30;
            let t179 = t86 * t86;
            let t180 = f64x8::splat(1.0) / t179;
            let t181 = t180 * param_a;
            let t183 = f64x8::splat(1.0) / t77 / t88;
            let t187 = ((t83).select(-t37 * t76 * t183 / f64x8::splat(9.0), f64x8::splat(0.0)));
            let t188 = (simd::sinh(t85));
            let t189 = t187 * t188;
            let t191 = t88 * v_rho1;
            let t193 = f64x8::splat(1.0) / t89 / t191;
            let t197 = -t181 * t189 - f64x8::splat(5.0) / f64x8::splat(27.0) * t52 * v_sigma2 * t193;
            let t202 = ((t65).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t175 * t95 + t154 + f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t75 * t197));
            let tvrho1 = t64 + t99 + t7 * (t169 + t202);
            acc_vrho_1 = tvrho1;
            let t205 = f64x8::splat(1.0) / t38;
            let t209 = ((t45).select(t37 * t205 * t41 / f64x8::splat(24.0), f64x8::splat(0.0)));
            let t210 = t209 * t126;
            let t214 = -t119 * t210 + f64x8::splat(5.0) / f64x8::splat(72.0) * t52 * t56;
            let t218 = ((t1).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t31 * t214));
            let tvsigma0 = t7 * t218;
            acc_vsigma_0 = tvsigma0;
            let tvsigma1 = f64x8::splat(0.0);
            acc_vsigma_1 = tvsigma1;
            let t219 = f64x8::splat(1.0) / t76;
            let t223 = ((t83).select(t37 * t219 * t79 / f64x8::splat(24.0), f64x8::splat(0.0)));
            let t224 = t223 * t188;
            let t228 = -t181 * t224 + f64x8::splat(5.0) / f64x8::splat(72.0) * t52 * t91;
            let t232 = ((t65).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t75 * t228));
            let tvsigma2 = t7 * t232;
            acc_vsigma_2 = tvsigma2;
            let t235 = f64x8::splat(1.0) / t25;
            let t236 = t104 * t104;
            let t239 = t100 * t7;
            let t240 = f64x8::splat(1.0) / t239;
            let t241 = t17 * t240;
            let t244 = ((t11).select(f64x8::splat(0.0), (t15).select(f64x8::splat(0.0), -f64x8::splat(2.0) * t101 + f64x8::splat(2.0) * t241)));
            let t248 = ((t21).select(f64x8::splat(0.0), f64x8::splat(10.0) / f64x8::splat(9.0) * t235 * t236 + f64x8::splat(5.0) / f64x8::splat(3.0) * t26 * t244));
            let t249 = t248 * t30;
            let t253 = t107 * t112;
            let t255 = t6 * t253 * t60;
            let t261 = f64x8::splat(1.0) / t29 / t7;
            let t262 = t28 * t261;
            let t265 = t6 * t262 * t60 / f64x8::splat(30.0);
            let t267 = t6 * t113 * t135;
            let t270 = f64x8::splat(1.0) / t117 / t48;
            let t271 = param_a * param_a;
            let t272 = t270 * t271;
            let t273 = t125 * t125;
            let t274 = t126 * t126;
            let t279 = f64x8::splat(1.0) / t39 / t129;
            let t283 = ((t45).select(f64x8::splat(7.0) / f64x8::splat(27.0) * t37 * t38 * t279, f64x8::splat(0.0)));
            let t286 = t49 * t271;
            let t288 = t53 * t53;
            let t290 = f64x8::splat(1.0) / t54 / t288;
            let t294 = f64x8::splat(2.0) * t272 * t273 * t274 - t119 * t283 * t126 - t286 * t273 + f64x8::splat(55.0) / f64x8::splat(81.0) * t52 * v_sigma0 * t290;
            let t299 = ((t1).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t249 * t60 + t255 / f64x8::splat(5.0) + f64x8::splat(3.0) / f64x8::splat(10.0) * t6 * t108 * t135 - t265 + t267 / f64x8::splat(5.0) + f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t31 * t294));
            let t300 = f64x8::splat(1.0) / t71;
            let t301 = t143 * t143;
            let t304 = t66 * t240;
            let t307 = ((t15).select(f64x8::splat(0.0), (t11).select(f64x8::splat(0.0), f64x8::splat(2.0) * t101 + f64x8::splat(2.0) * t304)));
            let t311 = ((t70).select(f64x8::splat(0.0), f64x8::splat(10.0) / f64x8::splat(9.0) * t300 * t301 + f64x8::splat(5.0) / f64x8::splat(3.0) * t72 * t307));
            let t312 = t311 * t30;
            let t316 = t146 * t112;
            let t318 = t6 * t316 * t95;
            let t320 = t74 * t261;
            let t323 = t6 * t320 * t95 / f64x8::splat(30.0);
            let t325 = ((t65).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t312 * t95 + t318 / f64x8::splat(5.0) - t323));
            let tv2rho20 = f64x8::splat(2.0) * t140 + f64x8::splat(2.0) * t156 + t7 * (t299 + t325);
            acc_v2rho2_0 = tv2rho20;
            let t328 = t235 * t160;
            let t332 = ((t11).select(f64x8::splat(0.0), (t15).select(f64x8::splat(0.0), f64x8::splat(2.0) * t241)));
            let t336 = ((t21).select(f64x8::splat(0.0), f64x8::splat(10.0) / f64x8::splat(9.0) * t328 * t104 + f64x8::splat(5.0) / f64x8::splat(3.0) * t26 * t332));
            let t337 = t336 * t30;
            let t341 = t163 * t112;
            let t343 = t6 * t341 * t60;
            let t351 = ((t1).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t337 * t60 + t343 / f64x8::splat(10.0) + f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t164 * t135 + t255 / f64x8::splat(10.0) - t265 + t267 / f64x8::splat(10.0)));
            let t352 = t300 * t171;
            let t356 = ((t15).select(f64x8::splat(0.0), (t11).select(f64x8::splat(0.0), f64x8::splat(2.0) * t304)));
            let t360 = ((t70).select(f64x8::splat(0.0), f64x8::splat(10.0) / f64x8::splat(9.0) * t352 * t143 + f64x8::splat(5.0) / f64x8::splat(3.0) * t72 * t356));
            let t361 = t360 * t30;
            let t365 = t174 * t112;
            let t367 = t6 * t365 * t95;
            let t374 = t6 * t151 * t197;
            let t377 = ((t65).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t361 * t95 + t367 / f64x8::splat(10.0) + t318 / f64x8::splat(10.0) - t323 + f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t147 * t197 + t374 / f64x8::splat(10.0)));
            let tv2rho21 = t140 + t156 + t169 + t202 + t7 * (t351 + t377);
            acc_v2rho2_1 = tv2rho21;
            let t382 = t160 * t160;
            let t387 = ((t11).select(f64x8::splat(0.0), (t15).select(f64x8::splat(0.0), f64x8::splat(2.0) * t101 + f64x8::splat(2.0) * t241)));
            let t391 = ((t21).select(f64x8::splat(0.0), f64x8::splat(10.0) / f64x8::splat(9.0) * t235 * t382 + f64x8::splat(5.0) / f64x8::splat(3.0) * t26 * t387));
            let t392 = t391 * t30;
            let t398 = ((t1).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t392 * t60 + t343 / f64x8::splat(5.0) - t265));
            let t399 = t171 * t171;
            let t404 = ((t15).select(f64x8::splat(0.0), (t11).select(f64x8::splat(0.0), -f64x8::splat(2.0) * t101 + f64x8::splat(2.0) * t304)));
            let t408 = ((t70).select(f64x8::splat(0.0), f64x8::splat(10.0) / f64x8::splat(9.0) * t300 * t399 + f64x8::splat(5.0) / f64x8::splat(3.0) * t72 * t404));
            let t409 = t408 * t30;
            let t419 = f64x8::splat(1.0) / t179 / t86;
            let t420 = t419 * t271;
            let t421 = t187 * t187;
            let t422 = t188 * t188;
            let t427 = f64x8::splat(1.0) / t77 / t191;
            let t431 = ((t83).select(f64x8::splat(7.0) / f64x8::splat(27.0) * t37 * t76 * t427, f64x8::splat(0.0)));
            let t434 = t87 * t271;
            let t436 = t88 * t88;
            let t438 = f64x8::splat(1.0) / t89 / t436;
            let t442 = f64x8::splat(2.0) * t420 * t421 * t422 - t181 * t431 * t188 - t434 * t421 + f64x8::splat(55.0) / f64x8::splat(81.0) * t52 * v_sigma2 * t438;
            let t447 = ((t65).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t409 * t95 + t367 / f64x8::splat(5.0) + f64x8::splat(3.0) / f64x8::splat(10.0) * t6 * t175 * t197 - t323 + t374 / f64x8::splat(5.0) + f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t75 * t442));
            let tv2rho22 = f64x8::splat(2.0) * t169 + f64x8::splat(2.0) * t202 + t7 * (t398 + t447);
            acc_v2rho2_2 = tv2rho22;
            let t455 = t6 * t113 * t214 / f64x8::splat(10.0);
            let t456 = t209 * t274;
            let t463 = ((t45).select(-t37 * t205 * t121 / f64x8::splat(18.0), f64x8::splat(0.0)));
            let t464 = t463 * t126;
            let t470 = f64x8::splat(2.0) * t272 * t456 * t125 - t119 * t464 - t286 * t209 * t125 - f64x8::splat(5.0) / f64x8::splat(27.0) * t52 * t131;
            let t475 = ((t1).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t108 * t214 + t455 + f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t31 * t470));
            let tv2rhosigma0 = t7 * t475 + t218;
            acc_v2rhosigma_0 = tv2rhosigma0;
            let tv2rhosigma1 = f64x8::splat(0.0);
            acc_v2rhosigma_1 = tv2rhosigma1;
            let t482 = t6 * t151 * t228 / f64x8::splat(10.0);
            let t484 = ((t65).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t147 * t228 + t482));
            let tv2rhosigma2 = t7 * t484 + t232;
            acc_v2rhosigma_2 = tv2rhosigma2;
            let t490 = ((t1).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t164 * t214 + t455));
            let tv2rhosigma3 = t7 * t490 + t218;
            acc_v2rhosigma_3 = tv2rhosigma3;
            let tv2rhosigma4 = f64x8::splat(0.0);
            acc_v2rhosigma_4 = tv2rhosigma4;
            let t495 = t223 * t422;
            let t502 = ((t83).select(-t37 * t219 * t183 / f64x8::splat(18.0), f64x8::splat(0.0)));
            let t503 = t502 * t188;
            let t509 = f64x8::splat(2.0) * t420 * t495 * t187 - t181 * t503 - t434 * t223 * t187 - f64x8::splat(5.0) / f64x8::splat(27.0) * t52 * t193;
            let t514 = ((t65).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t175 * t228 + t482 + f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t75 * t509));
            let tv2rhosigma5 = t7 * t514 + t232;
            acc_v2rhosigma_5 = tv2rhosigma5;
            let t516 = t209 * t209;
            let t517 = t516 * t274;
            let t521 = f64x8::splat(1.0) / t38 / v_sigma0;
            let t525 = ((t45).select(-t37 * t521 * t41 / f64x8::splat(48.0), f64x8::splat(0.0)));
            let t526 = t525 * t126;
            let t529 = -t119 * t526 + f64x8::splat(2.0) * t272 * t517 - t286 * t516;
            let t533 = ((t1).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t31 * t529));
            let tv2sigma20 = t7 * t533;
            acc_v2sigma2_0 = tv2sigma20;
            let tv2sigma21 = f64x8::splat(0.0);
            acc_v2sigma2_1 = tv2sigma21;
            let tv2sigma22 = f64x8::splat(0.0);
            acc_v2sigma2_2 = tv2sigma22;
            let tv2sigma23 = f64x8::splat(0.0);
            acc_v2sigma2_3 = tv2sigma23;
            let tv2sigma24 = f64x8::splat(0.0);
            acc_v2sigma2_4 = tv2sigma24;
            let t534 = t223 * t223;
            let t535 = t534 * t422;
            let t539 = f64x8::splat(1.0) / t76 / v_sigma2;
            let t543 = ((t83).select(-t37 * t539 * t79 / f64x8::splat(48.0), f64x8::splat(0.0)));
            let t544 = t543 * t188;
            let t547 = -t181 * t544 + f64x8::splat(2.0) * t420 * t535 - t434 * t534;
            let t551 = ((t65).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t75 * t547));
            let tv2sigma25 = t7 * t551;
            acc_v2sigma2_5 = tv2sigma25;
            let t555 = f64x8::splat(1.0) / t25 / t20;
            let t556 = t236 * t104;
            let t559 = t235 * t104;
            let t562 = t100 * t100;
            let t563 = f64x8::splat(1.0) / t562;
            let t564 = t17 * t563;
            let t567 = ((t11).select(f64x8::splat(0.0), (t15).select(f64x8::splat(0.0), f64x8::splat(6.0) * t240 - f64x8::splat(6.0) * t564)));
            let t571 = ((t21).select(f64x8::splat(0.0), -f64x8::splat(10.0) / f64x8::splat(27.0) * t555 * t556 + f64x8::splat(10.0) / f64x8::splat(3.0) * t559 * t244 + f64x8::splat(5.0) / f64x8::splat(3.0) * t26 * t567));
            let t572 = t571 * t30;
            let t576 = t248 * t112;
            let t578 = t6 * t576 * t60;
            let t583 = t107 * t261;
            let t585 = t6 * t583 * t60;
            let t588 = t6 * t253 * t135;
            let t594 = f64x8::splat(1.0) / t29 / t100;
            let t595 = t28 * t594;
            let t598 = f64x8::splat(2.0) / f64x8::splat(45.0) * t6 * t595 * t60;
            let t600 = t6 * t262 * t135;
            let t603 = t6 * t113 * t294;
            let t605 = t117 * t117;
            let t607 = t271 * param_a;
            let t608 = f64x8::splat(1.0) / t605 * t607;
            let t609 = t273 * t125;
            let t610 = t274 * t126;
            let t614 = t125 * t274;
            let t618 = t118 * t607;
            let t623 = f64x8::splat(1.0) / t39 / t288;
            let t627 = ((t45).select(-f64x8::splat(70.0) / f64x8::splat(81.0) * t37 * t38 * t623, f64x8::splat(0.0)));
            let t633 = t288 * v_rho0;
            let t635 = f64x8::splat(1.0) / t54 / t633;
            let t639 = -f64x8::splat(6.0) * t608 * t609 * t610 + f64x8::splat(6.0) * t272 * t614 * t283 + f64x8::splat(5.0) * t618 * t609 * t126 - t119 * t627 * t126 - f64x8::splat(3.0) * t286 * t283 * t125 - f64x8::splat(770.0) / f64x8::splat(243.0) * t52 * v_sigma0 * t635;
            let t644 = ((t1).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t572 * t60 + f64x8::splat(3.0) / f64x8::splat(10.0) * t578 + f64x8::splat(9.0) / f64x8::splat(20.0) * t6 * t249 * t135 - t585 / f64x8::splat(10.0) + f64x8::splat(3.0) / f64x8::splat(5.0) * t588 + f64x8::splat(9.0) / f64x8::splat(20.0) * t6 * t108 * t294 + t598 - t600 / f64x8::splat(10.0) + f64x8::splat(3.0) / f64x8::splat(10.0) * t603 + f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t31 * t639));
            let t646 = f64x8::splat(1.0) / t71 / t69;
            let t647 = t301 * t143;
            let t650 = t300 * t143;
            let t653 = t66 * t563;
            let t656 = ((t15).select(f64x8::splat(0.0), (t11).select(f64x8::splat(0.0), -f64x8::splat(6.0) * t240 - f64x8::splat(6.0) * t653)));
            let t660 = ((t70).select(f64x8::splat(0.0), -f64x8::splat(10.0) / f64x8::splat(27.0) * t646 * t647 + f64x8::splat(10.0) / f64x8::splat(3.0) * t650 * t307 + f64x8::splat(5.0) / f64x8::splat(3.0) * t72 * t656));
            let t661 = t660 * t30;
            let t665 = t311 * t112;
            let t667 = t6 * t665 * t95;
            let t669 = t146 * t261;
            let t671 = t6 * t669 * t95;
            let t673 = t74 * t594;
            let t676 = f64x8::splat(2.0) / f64x8::splat(45.0) * t6 * t673 * t95;
            let t678 = ((t65).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t661 * t95 + f64x8::splat(3.0) / f64x8::splat(10.0) * t667 - t671 / f64x8::splat(10.0) + t676));
            let tv3rho30 = f64x8::splat(3.0) * t299 + f64x8::splat(3.0) * t325 + t7 * (t644 + t678);
            acc_v3rho3_0 = tv3rho30;
            let t681 = f64x8::splat(2.0) * t351;
            let t682 = f64x8::splat(2.0) * t377;
            let t683 = t555 * t160;
            let t686 = t235 * t332;
            let t691 = f64x8::splat(2.0) * t240;
            let t692 = f64x8::splat(6.0) * t564;
            let t694 = ((t11).select(f64x8::splat(0.0), (t15).select(f64x8::splat(0.0), t691 - t692)));
            let t698 = ((t21).select(f64x8::splat(0.0), -f64x8::splat(10.0) / f64x8::splat(27.0) * t683 * t236 + f64x8::splat(20.0) / f64x8::splat(9.0) * t686 * t104 + f64x8::splat(10.0) / f64x8::splat(9.0) * t328 * t244 + f64x8::splat(5.0) / f64x8::splat(3.0) * t26 * t694));
            let t699 = t698 * t30;
            let t703 = t336 * t112;
            let t706 = t6 * t703 * t60 / f64x8::splat(5.0);
            let t710 = t163 * t261;
            let t712 = t6 * t710 * t60;
            let t716 = t6 * t341 * t135 / f64x8::splat(5.0);
            let t725 = f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t699 * t60 + t706 + f64x8::splat(3.0) / f64x8::splat(10.0) * t6 * t337 * t135 - t712 / f64x8::splat(30.0) + t716 + f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t164 * t294 + t578 / f64x8::splat(10.0) - t585 / f64x8::splat(15.0) + t588 / f64x8::splat(5.0) + t598 - t600 / f64x8::splat(15.0) + t603 / f64x8::splat(10.0);
            let t726 = ((t1).select(f64x8::splat(0.0), t725));
            let t727 = t646 * t171;
            let t730 = t300 * t356;
            let t735 = f64x8::splat(6.0) * t653;
            let t737 = ((t15).select(f64x8::splat(0.0), (t11).select(f64x8::splat(0.0), -t691 - t735)));
            let t741 = ((t70).select(f64x8::splat(0.0), -f64x8::splat(10.0) / f64x8::splat(27.0) * t727 * t301 + f64x8::splat(20.0) / f64x8::splat(9.0) * t730 * t143 + f64x8::splat(10.0) / f64x8::splat(9.0) * t352 * t307 + f64x8::splat(5.0) / f64x8::splat(3.0) * t72 * t737));
            let t742 = t741 * t30;
            let t746 = t360 * t112;
            let t749 = t6 * t746 * t95 / f64x8::splat(5.0);
            let t750 = t174 * t261;
            let t752 = t6 * t750 * t95;
            let t761 = t6 * t316 * t197 / f64x8::splat(5.0);
            let t763 = t6 * t320 * t197;
            let t766 = ((t65).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t742 * t95 + t749 - t752 / f64x8::splat(30.0) + t667 / f64x8::splat(10.0) - t671 / f64x8::splat(15.0) + t676 + f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t312 * t197 + t761 - t763 / f64x8::splat(30.0)));
            let tv3rho31 = t299 + t325 + t681 + t682 + t7 * (t726 + t766);
            acc_v3rho3_1 = tv3rho31;
            let t769 = t555 * t382;
            let t774 = t235 * t387;
            let t778 = ((t11).select(f64x8::splat(0.0), (t15).select(f64x8::splat(0.0), -t691 - t692)));
            let t782 = ((t21).select(f64x8::splat(0.0), -f64x8::splat(10.0) / f64x8::splat(27.0) * t769 * t104 + f64x8::splat(20.0) / f64x8::splat(9.0) * t328 * t332 + f64x8::splat(10.0) / f64x8::splat(9.0) * t774 * t104 + f64x8::splat(5.0) / f64x8::splat(3.0) * t26 * t778));
            let t783 = t782 * t30;
            let t787 = t391 * t112;
            let t789 = t6 * t787 * t60;
            let t798 = ((t1).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t783 * t60 + t789 / f64x8::splat(10.0) + f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t392 * t135 + t706 - t712 / f64x8::splat(15.0) + t716 - t585 / f64x8::splat(30.0) + t598 - t600 / f64x8::splat(30.0)));
            let t799 = t646 * t399;
            let t804 = t300 * t404;
            let t808 = ((t15).select(f64x8::splat(0.0), (t11).select(f64x8::splat(0.0), t691 - t735)));
            let t812 = ((t70).select(f64x8::splat(0.0), -f64x8::splat(10.0) / f64x8::splat(27.0) * t799 * t143 + f64x8::splat(20.0) / f64x8::splat(9.0) * t352 * t356 + f64x8::splat(10.0) / f64x8::splat(9.0) * t804 * t143 + f64x8::splat(5.0) / f64x8::splat(3.0) * t72 * t808));
            let t813 = t812 * t30;
            let t817 = t408 * t112;
            let t819 = t6 * t817 * t95;
            let t826 = t6 * t365 * t197;
            let t834 = t6 * t151 * t442;
            let t836 = f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t813 * t95 + t819 / f64x8::splat(10.0) + t749 - t752 / f64x8::splat(15.0) + f64x8::splat(3.0) / f64x8::splat(10.0) * t6 * t361 * t197 + t826 / f64x8::splat(5.0) - t671 / f64x8::splat(30.0) + t676 + t761 - t763 / f64x8::splat(15.0) + f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t147 * t442 + t834 / f64x8::splat(10.0);
            let t837 = ((t65).select(f64x8::splat(0.0), t836));
            let tv3rho32 = t681 + t682 + t398 + t447 + t7 * (t798 + t837);
            acc_v3rho3_2 = tv3rho32;
            let t842 = t382 * t160;
            let t849 = ((t11).select(f64x8::splat(0.0), (t15).select(f64x8::splat(0.0), -f64x8::splat(6.0) * t240 - f64x8::splat(6.0) * t564)));
            let t853 = ((t21).select(f64x8::splat(0.0), -f64x8::splat(10.0) / f64x8::splat(27.0) * t555 * t842 + f64x8::splat(10.0) / f64x8::splat(3.0) * t328 * t387 + f64x8::splat(5.0) / f64x8::splat(3.0) * t26 * t849));
            let t854 = t853 * t30;
            let t861 = ((t1).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t854 * t60 + f64x8::splat(3.0) / f64x8::splat(10.0) * t789 - t712 / f64x8::splat(10.0) + t598));
            let t862 = t399 * t171;
            let t869 = ((t15).select(f64x8::splat(0.0), (t11).select(f64x8::splat(0.0), f64x8::splat(6.0) * t240 - f64x8::splat(6.0) * t653)));
            let t873 = ((t70).select(f64x8::splat(0.0), -f64x8::splat(10.0) / f64x8::splat(27.0) * t646 * t862 + f64x8::splat(10.0) / f64x8::splat(3.0) * t352 * t404 + f64x8::splat(5.0) / f64x8::splat(3.0) * t72 * t869));
            let t874 = t873 * t30;
            let t889 = t179 * t179;
            let t891 = f64x8::splat(1.0) / t889 * t607;
            let t892 = t421 * t187;
            let t893 = t422 * t188;
            let t897 = t187 * t422;
            let t901 = t180 * t607;
            let t906 = f64x8::splat(1.0) / t77 / t436;
            let t910 = ((t83).select(-f64x8::splat(70.0) / f64x8::splat(81.0) * t37 * t76 * t906, f64x8::splat(0.0)));
            let t916 = t436 * v_rho1;
            let t918 = f64x8::splat(1.0) / t89 / t916;
            let t922 = -f64x8::splat(6.0) * t891 * t892 * t893 + f64x8::splat(6.0) * t420 * t897 * t431 + f64x8::splat(5.0) * t901 * t892 * t188 - t181 * t910 * t188 - f64x8::splat(3.0) * t434 * t431 * t187 - f64x8::splat(770.0) / f64x8::splat(243.0) * t52 * v_sigma2 * t918;
            let t927 = ((t65).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t874 * t95 + f64x8::splat(3.0) / f64x8::splat(10.0) * t819 + f64x8::splat(9.0) / f64x8::splat(20.0) * t6 * t409 * t197 - t752 / f64x8::splat(10.0) + f64x8::splat(3.0) / f64x8::splat(5.0) * t826 + f64x8::splat(9.0) / f64x8::splat(20.0) * t6 * t175 * t442 + t676 - t763 / f64x8::splat(10.0) + f64x8::splat(3.0) / f64x8::splat(10.0) * t834 + f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t75 * t922));
            let tv3rho33 = f64x8::splat(3.0) * t398 + f64x8::splat(3.0) * t447 + t7 * (t861 + t927);
            acc_v3rho3_3 = tv3rho33;
            let t935 = t6 * t253 * t214;
            let t942 = t6 * t262 * t214 / f64x8::splat(30.0);
            let t944 = t6 * t113 * t470;
            let t950 = t463 * t274;
            let t963 = ((t45).select(f64x8::splat(7.0) / f64x8::splat(54.0) * t37 * t205 * t279, f64x8::splat(0.0)));
            let t973 = -f64x8::splat(6.0) * t608 * t209 * t610 * t273 + f64x8::splat(4.0) * t272 * t950 * t125 + f64x8::splat(5.0) * t618 * t210 * t273 + f64x8::splat(2.0) * t272 * t456 * t283 - t119 * t963 * t126 - f64x8::splat(2.0) * t286 * t463 * t125 - t286 * t209 * t283 + f64x8::splat(55.0) / f64x8::splat(81.0) * t52 * t290;
            let t978 = ((t1).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t249 * t214 + t935 / f64x8::splat(5.0) + f64x8::splat(3.0) / f64x8::splat(10.0) * t6 * t108 * t470 - t942 + t944 / f64x8::splat(5.0) + f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t31 * t973));
            let tv3rho2sigma0 = t7 * t978 + f64x8::splat(2.0) * t475;
            acc_v3rho2sigma_0 = tv3rho2sigma0;
            let tv3rho2sigma1 = f64x8::splat(0.0);
            acc_v3rho2sigma_1 = tv3rho2sigma1;
            let t985 = t6 * t316 * t228;
            let t989 = t6 * t320 * t228 / f64x8::splat(30.0);
            let t991 = ((t65).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t312 * t228 + t985 / f64x8::splat(5.0) - t989));
            let tv3rho2sigma2 = t7 * t991 + f64x8::splat(2.0) * t484;
            acc_v3rho2sigma_2 = tv3rho2sigma2;
            let t997 = t6 * t341 * t214;
            let t1005 = ((t1).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t337 * t214 + t997 / f64x8::splat(10.0) + f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t164 * t470 + t935 / f64x8::splat(10.0) - t942 + t944 / f64x8::splat(10.0)));
            let tv3rho2sigma3 = t7 * t1005 + t475 + t490;
            acc_v3rho2sigma_3 = tv3rho2sigma3;
            let tv3rho2sigma4 = f64x8::splat(0.0);
            acc_v3rho2sigma_4 = tv3rho2sigma4;
            let t1011 = t6 * t365 * t228;
            let t1018 = t6 * t151 * t509;
            let t1021 = ((t65).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t361 * t228 + t1011 / f64x8::splat(10.0) + t985 / f64x8::splat(10.0) - t989 + f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t147 * t509 + t1018 / f64x8::splat(10.0)));
            let tv3rho2sigma5 = t7 * t1021 + t484 + t514;
            acc_v3rho2sigma_5 = tv3rho2sigma5;
            let t1029 = ((t1).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t392 * t214 + t997 / f64x8::splat(5.0) - t942));
            let tv3rho2sigma6 = t7 * t1029 + f64x8::splat(2.0) * t490;
            acc_v3rho2sigma_6 = tv3rho2sigma6;
            let tv3rho2sigma7 = f64x8::splat(0.0);
            acc_v3rho2sigma_7 = tv3rho2sigma7;
            let t1044 = t502 * t422;
            let t1057 = ((t83).select(f64x8::splat(7.0) / f64x8::splat(54.0) * t37 * t219 * t427, f64x8::splat(0.0)));
            let t1067 = -f64x8::splat(6.0) * t891 * t223 * t893 * t421 + f64x8::splat(4.0) * t420 * t1044 * t187 + f64x8::splat(5.0) * t901 * t224 * t421 + f64x8::splat(2.0) * t420 * t495 * t431 - t181 * t1057 * t188 - f64x8::splat(2.0) * t434 * t502 * t187 - t434 * t223 * t431 + f64x8::splat(55.0) / f64x8::splat(81.0) * t52 * t438;
            let t1072 = ((t65).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t409 * t228 + t1011 / f64x8::splat(5.0) + f64x8::splat(3.0) / f64x8::splat(10.0) * t6 * t175 * t509 - t989 + t1018 / f64x8::splat(5.0) + f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t75 * t1067));
            let tv3rho2sigma8 = t7 * t1072 + f64x8::splat(2.0) * t514;
            acc_v3rho2sigma_8 = tv3rho2sigma8;
            let t1079 = t6 * t113 * t529 / f64x8::splat(10.0);
            let t1080 = t516 * t610;
            let t1087 = t516 * t126;
            let t1091 = t525 * t274;
            let t1098 = ((t45).select(t37 * t521 * t121 / f64x8::splat(36.0), f64x8::splat(0.0)));
            let t1106 = -f64x8::splat(6.0) * t608 * t1080 * t125 + f64x8::splat(5.0) * t618 * t1087 * t125 + f64x8::splat(2.0) * t272 * t1091 * t125 - t119 * t1098 * t126 - t286 * t525 * t125 - f64x8::splat(2.0) * t286 * t209 * t463 + f64x8::splat(4.0) * t272 * t456 * t463;
            let t1111 = ((t1).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t108 * t529 + t1079 + f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t31 * t1106));
            let tv3rhosigma20 = t7 * t1111 + t533;
            acc_v3rhosigma2_0 = tv3rhosigma20;
            let tv3rhosigma21 = f64x8::splat(0.0);
            acc_v3rhosigma2_1 = tv3rhosigma21;
            let tv3rhosigma22 = f64x8::splat(0.0);
            acc_v3rhosigma2_2 = tv3rhosigma22;
            let tv3rhosigma23 = f64x8::splat(0.0);
            acc_v3rhosigma2_3 = tv3rhosigma23;
            let tv3rhosigma24 = f64x8::splat(0.0);
            acc_v3rhosigma2_4 = tv3rhosigma24;
            let t1118 = t6 * t151 * t547 / f64x8::splat(10.0);
            let t1120 = ((t65).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t147 * t547 + t1118));
            let tv3rhosigma25 = t7 * t1120 + t551;
            acc_v3rhosigma2_5 = tv3rhosigma25;
            let t1126 = ((t1).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t164 * t529 + t1079));
            let tv3rhosigma26 = t7 * t1126 + t533;
            acc_v3rhosigma2_6 = tv3rhosigma26;
            let tv3rhosigma27 = f64x8::splat(0.0);
            acc_v3rhosigma2_7 = tv3rhosigma27;
            let tv3rhosigma28 = f64x8::splat(0.0);
            acc_v3rhosigma2_8 = tv3rhosigma28;
            let tv3rhosigma29 = f64x8::splat(0.0);
            acc_v3rhosigma2_9 = tv3rhosigma29;
            let tv3rhosigma210 = f64x8::splat(0.0);
            acc_v3rhosigma2_10 = tv3rhosigma210;
            let t1131 = t534 * t893;
            let t1138 = t534 * t188;
            let t1142 = t543 * t422;
            let t1149 = ((t83).select(t37 * t539 * t183 / f64x8::splat(36.0), f64x8::splat(0.0)));
            let t1157 = -f64x8::splat(6.0) * t891 * t1131 * t187 + f64x8::splat(5.0) * t901 * t1138 * t187 + f64x8::splat(2.0) * t420 * t1142 * t187 - t181 * t1149 * t188 - t434 * t543 * t187 - f64x8::splat(2.0) * t434 * t223 * t502 + f64x8::splat(4.0) * t420 * t495 * t502;
            let t1162 = ((t65).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t175 * t547 + t1118 + f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t75 * t1157));
            let tv3rhosigma211 = t7 * t1162 + t551;
            acc_v3rhosigma2_11 = tv3rhosigma211;
            let t1164 = t516 * t209;
            let t1174 = v_sigma0 * v_sigma0;
            let t1176 = f64x8::splat(1.0) / t38 / t1174;
            let t1180 = ((t45).select(t37 * t1176 * t41 / f64x8::splat(32.0), f64x8::splat(0.0)));
            let t1186 = f64x8::splat(5.0) * t618 * t1164 * t126 - f64x8::splat(6.0) * t608 * t1164 * t610 - t119 * t1180 * t126 - f64x8::splat(3.0) * t286 * t525 * t209 + f64x8::splat(6.0) * t272 * t456 * t525;
            let t1190 = ((t1).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t31 * t1186));
            let tv3sigma30 = t7 * t1190;
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
            let t1191 = t534 * t223;
            let t1201 = v_sigma2 * v_sigma2;
            let t1203 = f64x8::splat(1.0) / t76 / t1201;
            let t1207 = ((t83).select(t37 * t1203 * t79 / f64x8::splat(32.0), f64x8::splat(0.0)));
            let t1213 = f64x8::splat(5.0) * t901 * t1191 * t188 - f64x8::splat(6.0) * t891 * t1191 * t893 - t181 * t1207 * t188 - f64x8::splat(3.0) * t434 * t543 * t223 + f64x8::splat(6.0) * t420 * t495 * t543;
            let t1217 = ((t65).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t75 * t1213));
            let tv3sigma39 = t7 * t1217;
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
