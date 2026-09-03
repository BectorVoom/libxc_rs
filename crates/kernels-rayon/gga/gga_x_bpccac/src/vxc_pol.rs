//! GGA_X_BPCCAC vxc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_bpccac.c`
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
pub fn gga_x_bpccac_vxc_pol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
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
            let t28 = ((v_sigma0).sqrt());
            let t29 = (simd::cbrt(v_rho0));
            let t31 = f64x8::splat(1.0) / t29 / v_rho0;
            let t32 = t28 * t31;
            let t34 = (simd::exp(-t32 + f64x8::splat(19.0)));
            let t35 = f64x8::splat(1.0) + t34;
            let t36 = f64x8::splat(1.0) / t35;
            let t37 = f64x8::splat(1.0) - t36;
            let t38 = f64x8::splat(M_CBRT6);
            let t39 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t40 = (simd::cbrt(t39));
            let t41 = t40 * t40;
            let t42 = f64x8::splat(1.0) / t41;
            let t43 = t38 * t42;
            let t44 = v_rho0 * v_rho0;
            let t45 = t29 * t29;
            let t47 = f64x8::splat(1.0) / t45 / t44;
            let t49 = t43 * v_sigma0 * t47;
            let t51 = f64x8::splat(1.227) + f64x8::splat(0.009146457198521547) * t49;
            let t54 = f64x8::splat(2.227) - f64x8::splat(1.505529) / t51;
            let t57 = (simd::exp(-f64x8::splat(25.0) / f64x8::splat(6.0) * t49));
            let t60 = (f64x8::splat(0.2743) - f64x8::splat(0.1508) * t57) * t38;
            let t61 = t42 * v_sigma0;
            let t65 = t38 * t38;
            let t67 = f64x8::splat(1.0) / t40 / t39;
            let t68 = t65 * t67;
            let t69 = v_sigma0 * v_sigma0;
            let t70 = t44 * t44;
            let t71 = t70 * v_rho0;
            let t73 = f64x8::splat(1.0) / t29 / t71;
            let t76 = f64x8::splat(6.944444444444445e-06) * t68 * t69 * t73;
            let t77 = t60 * t61 * t47 / f64x8::splat(24.0) - t76;
            let t79 = t65 / t40;
            let t82 = (simd::ln(f64x8::splat(0.6496333333333333) * t79 * t32 + ((((f64x8::splat(0.6496333333333333) * t79 * t32) * (f64x8::splat(0.6496333333333333) * t79 * t32)) + f64x8::splat(1.0)).sqrt())));
            let t86 = f64x8::splat(1.0) + f64x8::splat(0.016370833333333334) * t79 * t32 * t82 + t76;
            let t87 = f64x8::splat(1.0) / t86;
            let t89 = t77 * t87 + f64x8::splat(1.0);
            let t91 = t36 * t89 + t37 * t54;
            let t95 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t27 * t91));
            let t96 = (v_rho1).simd_le(dens_threshold);
            let t97 = -t16;
            let t99 = ((t14).select(t11, (t10).select(t15, t97 * t7)));
            let t100 = f64x8::splat(1.0) + t99;
            let t101 = (t100).simd_le(zeta_threshold);
            let t102 = (simd::cbrt(t100));
            let t104 = ((t101).select(t22, t102 * t100));
            let t105 = t104 * t26;
            let t106 = ((v_sigma2).sqrt());
            let t107 = (simd::cbrt(v_rho1));
            let t109 = f64x8::splat(1.0) / t107 / v_rho1;
            let t110 = t106 * t109;
            let t112 = (simd::exp(-t110 + f64x8::splat(19.0)));
            let t113 = f64x8::splat(1.0) + t112;
            let t114 = f64x8::splat(1.0) / t113;
            let t115 = f64x8::splat(1.0) - t114;
            let t116 = v_rho1 * v_rho1;
            let t117 = t107 * t107;
            let t119 = f64x8::splat(1.0) / t117 / t116;
            let t121 = t43 * v_sigma2 * t119;
            let t123 = f64x8::splat(1.227) + f64x8::splat(0.009146457198521547) * t121;
            let t126 = f64x8::splat(2.227) - f64x8::splat(1.505529) / t123;
            let t129 = (simd::exp(-f64x8::splat(25.0) / f64x8::splat(6.0) * t121));
            let t132 = (f64x8::splat(0.2743) - f64x8::splat(0.1508) * t129) * t38;
            let t133 = t42 * v_sigma2;
            let t137 = v_sigma2 * v_sigma2;
            let t138 = t116 * t116;
            let t139 = t138 * v_rho1;
            let t141 = f64x8::splat(1.0) / t107 / t139;
            let t144 = f64x8::splat(6.944444444444445e-06) * t68 * t137 * t141;
            let t145 = t132 * t133 * t119 / f64x8::splat(24.0) - t144;
            let t148 = (simd::ln(f64x8::splat(0.6496333333333333) * t79 * t110 + ((((f64x8::splat(0.6496333333333333) * t79 * t110) * (f64x8::splat(0.6496333333333333) * t79 * t110)) + f64x8::splat(1.0)).sqrt())));
            let t152 = f64x8::splat(1.0) + f64x8::splat(0.016370833333333334) * t79 * t110 * t148 + t144;
            let t153 = f64x8::splat(1.0) / t152;
            let t155 = t145 * t153 + f64x8::splat(1.0);
            let t157 = t114 * t155 + t115 * t126;
            let t161 = ((t96).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t105 * t157));
            let tzk0 = t95 + t161;
            acc_zk = tzk0;
            let t162 = t6 * t6;
            let t163 = f64x8::splat(1.0) / t162;
            let t164 = t16 * t163;
            let t166 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), t7 - t164)));
            let t169 = ((t20).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t166));
            let t170 = t169 * t26;
            let t174 = t26 * t26;
            let t175 = f64x8::splat(1.0) / t174;
            let t176 = t25 * t175;
            let t179 = t5 * t176 * t91 / f64x8::splat(8.0);
            let t180 = t35 * t35;
            let t181 = f64x8::splat(1.0) / t180;
            let t182 = t181 * t28;
            let t184 = f64x8::splat(1.0) / t29 / t44;
            let t186 = t184 * t34 * t54;
            let t189 = t51 * t51;
            let t190 = f64x8::splat(1.0) / t189;
            let t191 = t37 * t190;
            let t192 = t191 * t38;
            let t193 = t44 * v_rho0;
            let t195 = f64x8::splat(1.0) / t45 / t193;
            let t196 = t61 * t195;
            let t199 = t181 * t89;
            let t200 = t28 * t184;
            let t201 = t200 * t34;
            let t204 = t70 * t44;
            let t206 = f64x8::splat(1.0) / t29 / t204;
            let t207 = t69 * t206;
            let t214 = f64x8::splat(3.7037037037037037e-05) * t68 * t207;
            let t215 = -f64x8::splat(0.06981481481481482) * t68 * t207 * t57 - t60 * t196 / f64x8::splat(9.0) + t214;
            let t217 = t86 * t86;
            let t218 = f64x8::splat(1.0) / t217;
            let t219 = t77 * t218;
            let t225 = f64x8::splat(2.532140806666667) * t49 + f64x8::splat(1.0);
            let t226 = ((t225).sqrt());
            let t227 = f64x8::splat(1.0) / t226;
            let t231 = -f64x8::splat(0.02182777777777778) * t79 * t200 * t82 - f64x8::splat(0.08508031222222222) * t43 * v_sigma0 * t195 * t227 - t214;
            let t233 = t215 * t87 - t219 * t231;
            let t235 = f64x8::splat(4.0) / f64x8::splat(3.0) * t182 * t186 - f64x8::splat(0.03672068415902118) * t192 * t196 - f64x8::splat(4.0) / f64x8::splat(3.0) * t199 * t201 + t36 * t233;
            let t240 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t170 * t91 - t179 - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t27 * t235));
            let t241 = t97 * t163;
            let t243 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), -t7 - t241)));
            let t246 = ((t101).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t102 * t243));
            let t247 = t246 * t26;
            let t251 = t104 * t175;
            let t254 = t5 * t251 * t157 / f64x8::splat(8.0);
            let t256 = ((t96).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t247 * t157 - t254));
            let tvrho0 = t95 + t161 + t6 * (t240 + t256);
            acc_vrho_0 = tvrho0;
            let t260 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), -t7 - t164)));
            let t263 = ((t20).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t260));
            let t264 = t263 * t26;
            let t269 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t264 * t91 - t179));
            let t271 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), t7 - t241)));
            let t274 = ((t101).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t102 * t271));
            let t275 = t274 * t26;
            let t279 = t113 * t113;
            let t280 = f64x8::splat(1.0) / t279;
            let t281 = t280 * t106;
            let t283 = f64x8::splat(1.0) / t107 / t116;
            let t285 = t283 * t112 * t126;
            let t288 = t123 * t123;
            let t289 = f64x8::splat(1.0) / t288;
            let t290 = t115 * t289;
            let t291 = t290 * t38;
            let t292 = t116 * v_rho1;
            let t294 = f64x8::splat(1.0) / t117 / t292;
            let t295 = t133 * t294;
            let t298 = t280 * t155;
            let t299 = t106 * t283;
            let t300 = t299 * t112;
            let t303 = t138 * t116;
            let t305 = f64x8::splat(1.0) / t107 / t303;
            let t306 = t137 * t305;
            let t313 = f64x8::splat(3.7037037037037037e-05) * t68 * t306;
            let t314 = -f64x8::splat(0.06981481481481482) * t68 * t306 * t129 - t132 * t295 / f64x8::splat(9.0) + t313;
            let t316 = t152 * t152;
            let t317 = f64x8::splat(1.0) / t316;
            let t318 = t145 * t317;
            let t324 = f64x8::splat(2.532140806666667) * t121 + f64x8::splat(1.0);
            let t325 = ((t324).sqrt());
            let t326 = f64x8::splat(1.0) / t325;
            let t330 = -f64x8::splat(0.02182777777777778) * t79 * t299 * t148 - f64x8::splat(0.08508031222222222) * t43 * v_sigma2 * t294 * t326 - t313;
            let t332 = t314 * t153 - t318 * t330;
            let t334 = f64x8::splat(4.0) / f64x8::splat(3.0) * t281 * t285 - f64x8::splat(0.03672068415902118) * t291 * t295 - f64x8::splat(4.0) / f64x8::splat(3.0) * t298 * t300 + t114 * t332;
            let t339 = ((t96).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t275 * t157 - t254 - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t105 * t334));
            let tvrho1 = t95 + t161 + t6 * (t269 + t339);
            acc_vrho_1 = tvrho1;
            let t342 = f64x8::splat(1.0) / t28;
            let t343 = t181 * t342;
            let t345 = t31 * t34 * t54;
            let t351 = t342 * t31;
            let t352 = t351 * t34;
            let t355 = t73 * t57;
            let t364 = f64x8::splat(1.388888888888889e-05) * t68 * v_sigma0 * t73;
            let t365 = f64x8::splat(0.026180555555555554) * t68 * t355 * v_sigma0 + t60 * t42 * t47 / f64x8::splat(24.0) - t364;
            let t373 = f64x8::splat(0.008185416666666667) * t79 * t351 * t82 + f64x8::splat(0.03190511708333333) * t43 * t47 * t227 + t364;
            let t375 = -t219 * t373 + t365 * t87;
            let t377 = -t343 * t345 / f64x8::splat(2.0) + f64x8::splat(0.013770256559632944) * t191 * t43 * t47 + t199 * t352 / f64x8::splat(2.0) + t36 * t375;
            let t381 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t27 * t377));
            let tvsigma0 = t6 * t381;
            acc_vsigma_0 = tvsigma0;
            let tvsigma1 = f64x8::splat(0.0);
            acc_vsigma_1 = tvsigma1;
            let t382 = f64x8::splat(1.0) / t106;
            let t383 = t280 * t382;
            let t385 = t109 * t112 * t126;
            let t391 = t382 * t109;
            let t392 = t391 * t112;
            let t395 = t141 * t129;
            let t404 = f64x8::splat(1.388888888888889e-05) * t68 * v_sigma2 * t141;
            let t405 = f64x8::splat(0.026180555555555554) * t68 * t395 * v_sigma2 + t132 * t42 * t119 / f64x8::splat(24.0) - t404;
            let t413 = f64x8::splat(0.008185416666666667) * t79 * t391 * t148 + f64x8::splat(0.03190511708333333) * t43 * t119 * t326 + t404;
            let t415 = t405 * t153 - t318 * t413;
            let t417 = -t383 * t385 / f64x8::splat(2.0) + f64x8::splat(0.013770256559632944) * t290 * t43 * t119 + t298 * t392 / f64x8::splat(2.0) + t114 * t415;
            let t421 = ((t96).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t105 * t417));
            let tvsigma2 = t6 * t421;
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
