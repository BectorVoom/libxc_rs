//! GGA_X_SFAT_PBE vxc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_sfat_pbe.c`
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
pub fn gga_x_sfat_pbe_vxc_pol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    param_hyb_omega_0: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_hyb_omega_0 = f64x8::splat(param_hyb_omega_0);
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
            let t5 = f64x8::splat(1.0) / t3 * t2;
            let t6 = v_rho0 + v_rho1;
            let t7 = f64x8::splat(1.0) / t6;
            let t10 = (f64x8::splat(2.0) * t7 * v_rho0).simd_le(zeta_threshold);
            let t11 = zeta_threshold - f64x8::splat(1.0);
            let t14 = (f64x8::splat(2.0) * t7 * v_rho1).simd_le(zeta_threshold);
            let t15 = -t11;
            let t16 = v_rho0 - v_rho1;
            let t18 = ((t10).select(t11, (t14).select(t15, t7 * t16)));
            let t19 = f64x8::splat(1.0) + t18;
            let t20 = (t19).simd_le(zeta_threshold);
            let t21 = (simd::cbrt(zeta_threshold));
            let t22 = t21 * zeta_threshold;
            let t23 = (simd::cbrt(t19));
            let t25 = ((t20).select(t22, t23 * t19));
            let t26 = t25 * t5;
            let t27 = (simd::cbrt(t6));
            let t28 = t2 * t2;
            let t29 = t28 * f64x8::splat(M_PI);
            let t30 = f64x8::splat(1.0) / f64x8::splat(M_PI);
            let t31 = (simd::cbrt(t30));
            let t32 = f64x8::splat(1.0) / t31;
            let t33 = f64x8::splat(M_CBRT4);
            let t34 = t33 * t32;
            let t35 = f64x8::splat(M_CBRT6);
            let t36 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t37 = (simd::cbrt(t36));
            let t38 = t37 * t37;
            let t39 = f64x8::splat(1.0) / t38;
            let t40 = t39 * t35;
            let t41 = v_rho0 * v_rho0;
            let t42 = (simd::cbrt(v_rho0));
            let t43 = t42 * t42;
            let t45 = f64x8::splat(1.0) / t43 / t41;
            let t49 = f64x8::splat(0.804) + f64x8::splat(0.009146457198521547) * t45 * v_sigma0 * t40;
            let t52 = f64x8::splat(1.804) - f64x8::splat(0.646416) / t49;
            let t55 = f64x8::splat(1.0) / t52 * t34 * t29;
            let t56 = ((t55).sqrt());
            let t58 = f64x8::splat(1.0) / t56 * param_hyb_omega_0;
            let t59 = f64x8::splat(M_CBRT2);
            let t60 = t6 * t19;
            let t61 = (simd::cbrt(t60));
            let t62 = f64x8::splat(1.0) / t61;
            let t63 = t62 * t59;
            let t65 = t63 * t58 / f64x8::splat(2.0);
            let t66 = (f64x8::splat(1.92)).simd_le(t65);
            let t67 = (f64x8::splat(1.92)).simd_lt(t65);
            let t68 = ((t67).select(t65, f64x8::splat(1.92)));
            let t69 = t68 * t68;
            let t70 = t69 * t69;
            let t71 = f64x8::splat(1.0) / t70;
            let t73 = t70 * t69;
            let t74 = f64x8::splat(1.0) / t73;
            let t76 = t70 * t70;
            let t77 = f64x8::splat(1.0) / t76;
            let t79 = t76 * t69;
            let t80 = f64x8::splat(1.0) / t79;
            let t82 = t76 * t70;
            let t83 = f64x8::splat(1.0) / t82;
            let t85 = t76 * t73;
            let t86 = f64x8::splat(1.0) / t85;
            let t88 = t76 * t76;
            let t89 = f64x8::splat(1.0) / t88;
            let t92 = f64x8::splat(1.0) / t88 / t69;
            let t95 = f64x8::splat(1.0) / t88 / t70;
            let t98 = f64x8::splat(1.0) / t88 / t73;
            let t101 = f64x8::splat(1.0) / t88 / t76;
            let t104 = f64x8::splat(1.0) / t88 / t79;
            let t107 = f64x8::splat(1.0) / t88 / t82;
            let t110 = f64x8::splat(1.0) / t88 / t85;
            let t112 = t88 * t88;
            let t113 = f64x8::splat(1.0) / t112;
            let t116 = f64x8::splat(1.0) / t112 / t69;
            let t119 = f64x8::splat(1.0) / t112 / t70;
            let t123 = -t71 / f64x8::splat(30.0) + t74 / f64x8::splat(70.0) - t77 / f64x8::splat(135.0) + t80 / f64x8::splat(231.0) - t83 / f64x8::splat(364.0) + t86 / f64x8::splat(540.0) - t89 / f64x8::splat(765.0) + t92 / f64x8::splat(1045.0) - t95 / f64x8::splat(1386.0) + t98 / f64x8::splat(1794.0) - t101 / f64x8::splat(2275.0) + t104 / f64x8::splat(2835.0) - t107 / f64x8::splat(3480.0) + t110 / f64x8::splat(4216.0) - t113 / f64x8::splat(5049.0) + t116 / f64x8::splat(5985.0) - t119 / f64x8::splat(7030.0) + f64x8::splat(1.0) / t69 / f64x8::splat(9.0);
            let t124 = ((t67).select(f64x8::splat(1.92), t65));
            let t125 = (simd::atan2(f64x8::splat(1.0), t124));
            let t126 = t124 * t124;
            let t127 = t126 + f64x8::splat(3.0);
            let t128 = f64x8::splat(1.0) / t126;
            let t129 = f64x8::splat(1.0) + t128;
            let t130 = (simd::ln(t129));
            let t132 = -t130 * t127 + f64x8::splat(1.0);
            let t135 = t125 + t132 * t124 / f64x8::splat(4.0);
            let t139 = ((t66).select(t123, f64x8::splat(1.0) - f64x8::splat(8.0) / f64x8::splat(3.0) * t135 * t124));
            let t140 = t139 * t27;
            let t141 = t52 * t140;
            let t144 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t141 * t26));
            let t145 = (v_rho1).simd_le(dens_threshold);
            let t146 = -t16;
            let t148 = ((t14).select(t11, (t10).select(t15, t7 * t146)));
            let t149 = f64x8::splat(1.0) + t148;
            let t150 = (t149).simd_le(zeta_threshold);
            let t151 = (simd::cbrt(t149));
            let t153 = ((t150).select(t22, t151 * t149));
            let t154 = t153 * t5;
            let t155 = v_rho1 * v_rho1;
            let t156 = (simd::cbrt(v_rho1));
            let t157 = t156 * t156;
            let t159 = f64x8::splat(1.0) / t157 / t155;
            let t163 = f64x8::splat(0.804) + f64x8::splat(0.009146457198521547) * t159 * v_sigma2 * t40;
            let t166 = f64x8::splat(1.804) - f64x8::splat(0.646416) / t163;
            let t169 = f64x8::splat(1.0) / t166 * t34 * t29;
            let t170 = ((t169).sqrt());
            let t172 = f64x8::splat(1.0) / t170 * param_hyb_omega_0;
            let t173 = t6 * t149;
            let t174 = (simd::cbrt(t173));
            let t175 = f64x8::splat(1.0) / t174;
            let t176 = t175 * t59;
            let t178 = t176 * t172 / f64x8::splat(2.0);
            let t179 = (f64x8::splat(1.92)).simd_le(t178);
            let t180 = (f64x8::splat(1.92)).simd_lt(t178);
            let t181 = ((t180).select(t178, f64x8::splat(1.92)));
            let t182 = t181 * t181;
            let t183 = t182 * t182;
            let t184 = f64x8::splat(1.0) / t183;
            let t186 = t183 * t182;
            let t187 = f64x8::splat(1.0) / t186;
            let t189 = t183 * t183;
            let t190 = f64x8::splat(1.0) / t189;
            let t192 = t189 * t182;
            let t193 = f64x8::splat(1.0) / t192;
            let t195 = t189 * t183;
            let t196 = f64x8::splat(1.0) / t195;
            let t198 = t189 * t186;
            let t199 = f64x8::splat(1.0) / t198;
            let t201 = t189 * t189;
            let t202 = f64x8::splat(1.0) / t201;
            let t205 = f64x8::splat(1.0) / t201 / t182;
            let t208 = f64x8::splat(1.0) / t201 / t183;
            let t211 = f64x8::splat(1.0) / t201 / t186;
            let t214 = f64x8::splat(1.0) / t201 / t189;
            let t217 = f64x8::splat(1.0) / t201 / t192;
            let t220 = f64x8::splat(1.0) / t201 / t195;
            let t223 = f64x8::splat(1.0) / t201 / t198;
            let t225 = t201 * t201;
            let t226 = f64x8::splat(1.0) / t225;
            let t229 = f64x8::splat(1.0) / t225 / t182;
            let t232 = f64x8::splat(1.0) / t225 / t183;
            let t236 = -t184 / f64x8::splat(30.0) + t187 / f64x8::splat(70.0) - t190 / f64x8::splat(135.0) + t193 / f64x8::splat(231.0) - t196 / f64x8::splat(364.0) + t199 / f64x8::splat(540.0) - t202 / f64x8::splat(765.0) + t205 / f64x8::splat(1045.0) - t208 / f64x8::splat(1386.0) + t211 / f64x8::splat(1794.0) - t214 / f64x8::splat(2275.0) + t217 / f64x8::splat(2835.0) - t220 / f64x8::splat(3480.0) + t223 / f64x8::splat(4216.0) - t226 / f64x8::splat(5049.0) + t229 / f64x8::splat(5985.0) - t232 / f64x8::splat(7030.0) + f64x8::splat(1.0) / t182 / f64x8::splat(9.0);
            let t237 = ((t180).select(f64x8::splat(1.92), t178));
            let t238 = (simd::atan2(f64x8::splat(1.0), t237));
            let t239 = t237 * t237;
            let t240 = t239 + f64x8::splat(3.0);
            let t241 = f64x8::splat(1.0) / t239;
            let t242 = f64x8::splat(1.0) + t241;
            let t243 = (simd::ln(t242));
            let t245 = -t243 * t240 + f64x8::splat(1.0);
            let t248 = t238 + t245 * t237 / f64x8::splat(4.0);
            let t252 = ((t179).select(t236, f64x8::splat(1.0) - f64x8::splat(8.0) / f64x8::splat(3.0) * t248 * t237));
            let t253 = t252 * t27;
            let t254 = t166 * t253;
            let t257 = ((t145).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t254 * t154));
            let tzk0 = t144 + t257;
            acc_zk = tzk0;
            let t258 = t6 * t6;
            let t259 = f64x8::splat(1.0) / t258;
            let t260 = t259 * t16;
            let t262 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), t7 - t260)));
            let t265 = ((t20).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t262 * t23));
            let t266 = t265 * t5;
            let t269 = t27 * t27;
            let t270 = f64x8::splat(1.0) / t269;
            let t271 = t139 * t270;
            let t272 = t52 * t271;
            let t274 = t272 * t26 / f64x8::splat(8.0);
            let t275 = t70 * t68;
            let t276 = f64x8::splat(1.0) / t275;
            let t279 = f64x8::splat(1.0) / t56 / t55 * param_hyb_omega_0;
            let t280 = t59 * t279;
            let t283 = t32 * t28 * t62 * t280;
            let t284 = t52 * t52;
            let t285 = f64x8::splat(1.0) / t284;
            let t286 = t285 * t33;
            let t287 = t49 * t49;
            let t288 = f64x8::splat(1.0) / t287;
            let t289 = t288 * t286;
            let t290 = t41 * v_rho0;
            let t292 = f64x8::splat(1.0) / t43 / t290;
            let t293 = t292 * v_sigma0;
            let t294 = t293 * t40;
            let t295 = t294 * t289;
            let t299 = f64x8::splat(1.0) / t61 / t60;
            let t300 = t299 * t59;
            let t302 = t6 * t262 + t18 + f64x8::splat(1.0);
            let t306 = -f64x8::splat(0.01238293569268471) * t295 * t283 - t302 * t300 * t58 / f64x8::splat(6.0);
            let t307 = ((t67).select(t306, f64x8::splat(0.0)));
            let t310 = t69 * t68;
            let t311 = t70 * t310;
            let t312 = f64x8::splat(1.0) / t311;
            let t315 = t76 * t68;
            let t316 = f64x8::splat(1.0) / t315;
            let t319 = t76 * t310;
            let t320 = f64x8::splat(1.0) / t319;
            let t323 = t76 * t275;
            let t324 = f64x8::splat(1.0) / t323;
            let t327 = t76 * t311;
            let t328 = f64x8::splat(1.0) / t327;
            let t332 = f64x8::splat(1.0) / t88 / t68;
            let t336 = f64x8::splat(1.0) / t88 / t310;
            let t340 = f64x8::splat(1.0) / t88 / t275;
            let t344 = f64x8::splat(1.0) / t88 / t311;
            let t348 = f64x8::splat(1.0) / t88 / t315;
            let t352 = f64x8::splat(1.0) / t88 / t319;
            let t356 = f64x8::splat(1.0) / t88 / t323;
            let t360 = f64x8::splat(1.0) / t88 / t327;
            let t364 = f64x8::splat(1.0) / t112 / t68;
            let t368 = f64x8::splat(1.0) / t112 / t310;
            let t372 = f64x8::splat(1.0) / t112 / t275;
            let t375 = f64x8::splat(1.0) / t310;
            let t378 = f64x8::splat(2.0) / f64x8::splat(15.0) * t307 * t276 - f64x8::splat(3.0) / f64x8::splat(35.0) * t307 * t312 + f64x8::splat(8.0) / f64x8::splat(135.0) * t307 * t316 - f64x8::splat(10.0) / f64x8::splat(231.0) * t307 * t320 + f64x8::splat(3.0) / f64x8::splat(91.0) * t307 * t324 - f64x8::splat(7.0) / f64x8::splat(270.0) * t307 * t328 + f64x8::splat(16.0) / f64x8::splat(765.0) * t307 * t332 - f64x8::splat(18.0) / f64x8::splat(1045.0) * t307 * t336 + f64x8::splat(10.0) / f64x8::splat(693.0) * t307 * t340 - f64x8::splat(11.0) / f64x8::splat(897.0) * t307 * t344 + f64x8::splat(24.0) / f64x8::splat(2275.0) * t307 * t348 - f64x8::splat(26.0) / f64x8::splat(2835.0) * t307 * t352 + f64x8::splat(7.0) / f64x8::splat(870.0) * t307 * t356 - f64x8::splat(15.0) / f64x8::splat(2108.0) * t307 * t360 + f64x8::splat(32.0) / f64x8::splat(5049.0) * t307 * t364 - f64x8::splat(34.0) / f64x8::splat(5985.0) * t307 * t368 + f64x8::splat(18.0) / f64x8::splat(3515.0) * t307 * t372 - f64x8::splat(2.0) / f64x8::splat(9.0) * t307 * t375;
            let t379 = ((t67).select(f64x8::splat(0.0), t306));
            let t382 = f64x8::splat(1.0) / t129;
            let t388 = t126 * t124;
            let t389 = f64x8::splat(1.0) / t388;
            let t390 = t389 * t127;
            let t391 = t382 * t379;
            let t394 = -f64x8::splat(2.0) * t130 * t379 * t124 + f64x8::splat(2.0) * t391 * t390;
            let t397 = -t382 * t128 * t379 + t132 * t379 / f64x8::splat(4.0) + t394 * t124 / f64x8::splat(4.0);
            let t401 = ((t66).select(t378, -f64x8::splat(8.0) / f64x8::splat(3.0) * t397 * t124 - f64x8::splat(8.0) / f64x8::splat(3.0) * t135 * t379));
            let t402 = t401 * t27;
            let t403 = t52 * t402;
            let t406 = t25 * t2;
            let t407 = t140 * t406;
            let t408 = t35 * t288;
            let t409 = v_sigma0 * t39;
            let t411 = t292 * t409 * t408;
            let t415 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t141 * t266 - t274 - f64x8::splat(3.0) / f64x8::splat(8.0) * t403 * t26 + f64x8::splat(0.0040369036088841095) * t411 * t407));
            let t416 = t259 * t146;
            let t418 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), -t7 - t416)));
            let t421 = ((t150).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t418 * t151));
            let t422 = t421 * t5;
            let t425 = t252 * t270;
            let t426 = t166 * t425;
            let t428 = t426 * t154 / f64x8::splat(8.0);
            let t429 = t183 * t181;
            let t430 = f64x8::splat(1.0) / t429;
            let t432 = f64x8::splat(1.0) / t174 / t173;
            let t433 = t432 * t59;
            let t435 = t6 * t418 + t148 + f64x8::splat(1.0);
            let t438 = t435 * t433 * t172 / f64x8::splat(6.0);
            let t439 = ((t180).select(-t438, f64x8::splat(0.0)));
            let t442 = t182 * t181;
            let t443 = t183 * t442;
            let t444 = f64x8::splat(1.0) / t443;
            let t447 = t189 * t181;
            let t448 = f64x8::splat(1.0) / t447;
            let t451 = t189 * t442;
            let t452 = f64x8::splat(1.0) / t451;
            let t455 = t189 * t429;
            let t456 = f64x8::splat(1.0) / t455;
            let t459 = t189 * t443;
            let t460 = f64x8::splat(1.0) / t459;
            let t464 = f64x8::splat(1.0) / t201 / t181;
            let t468 = f64x8::splat(1.0) / t201 / t442;
            let t472 = f64x8::splat(1.0) / t201 / t429;
            let t476 = f64x8::splat(1.0) / t201 / t443;
            let t480 = f64x8::splat(1.0) / t201 / t447;
            let t484 = f64x8::splat(1.0) / t201 / t451;
            let t488 = f64x8::splat(1.0) / t201 / t455;
            let t492 = f64x8::splat(1.0) / t201 / t459;
            let t496 = f64x8::splat(1.0) / t225 / t181;
            let t500 = f64x8::splat(1.0) / t225 / t442;
            let t504 = f64x8::splat(1.0) / t225 / t429;
            let t507 = f64x8::splat(1.0) / t442;
            let t510 = f64x8::splat(2.0) / f64x8::splat(15.0) * t439 * t430 - f64x8::splat(3.0) / f64x8::splat(35.0) * t439 * t444 + f64x8::splat(8.0) / f64x8::splat(135.0) * t439 * t448 - f64x8::splat(10.0) / f64x8::splat(231.0) * t439 * t452 + f64x8::splat(3.0) / f64x8::splat(91.0) * t439 * t456 - f64x8::splat(7.0) / f64x8::splat(270.0) * t439 * t460 + f64x8::splat(16.0) / f64x8::splat(765.0) * t439 * t464 - f64x8::splat(18.0) / f64x8::splat(1045.0) * t439 * t468 + f64x8::splat(10.0) / f64x8::splat(693.0) * t439 * t472 - f64x8::splat(11.0) / f64x8::splat(897.0) * t439 * t476 + f64x8::splat(24.0) / f64x8::splat(2275.0) * t439 * t480 - f64x8::splat(26.0) / f64x8::splat(2835.0) * t439 * t484 + f64x8::splat(7.0) / f64x8::splat(870.0) * t439 * t488 - f64x8::splat(15.0) / f64x8::splat(2108.0) * t439 * t492 + f64x8::splat(32.0) / f64x8::splat(5049.0) * t439 * t496 - f64x8::splat(34.0) / f64x8::splat(5985.0) * t439 * t500 + f64x8::splat(18.0) / f64x8::splat(3515.0) * t439 * t504 - f64x8::splat(2.0) / f64x8::splat(9.0) * t439 * t507;
            let t511 = ((t180).select(f64x8::splat(0.0), -t438));
            let t514 = f64x8::splat(1.0) / t242;
            let t520 = t239 * t237;
            let t521 = f64x8::splat(1.0) / t520;
            let t522 = t521 * t240;
            let t523 = t514 * t511;
            let t526 = -f64x8::splat(2.0) * t243 * t511 * t237 + f64x8::splat(2.0) * t523 * t522;
            let t529 = -t514 * t241 * t511 + t245 * t511 / f64x8::splat(4.0) + t526 * t237 / f64x8::splat(4.0);
            let t533 = ((t179).select(t510, -f64x8::splat(8.0) / f64x8::splat(3.0) * t529 * t237 - f64x8::splat(8.0) / f64x8::splat(3.0) * t248 * t511));
            let t534 = t533 * t27;
            let t535 = t166 * t534;
            let t539 = ((t145).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t254 * t422 - t428 - f64x8::splat(3.0) / f64x8::splat(8.0) * t535 * t154));
            let tvrho0 = t144 + t257 + (t415 + t539) * t6;
            acc_vrho_0 = tvrho0;
            let t543 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), -t7 - t260)));
            let t546 = ((t20).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t543 * t23));
            let t547 = t546 * t5;
            let t551 = t6 * t543 + t18 + f64x8::splat(1.0);
            let t554 = t551 * t300 * t58 / f64x8::splat(6.0);
            let t555 = ((t67).select(-t554, f64x8::splat(0.0)));
            let t556 = t555 * t276;
            let t558 = t555 * t312;
            let t560 = t555 * t316;
            let t562 = t555 * t320;
            let t564 = t555 * t324;
            let t566 = t555 * t328;
            let t568 = t555 * t332;
            let t570 = t555 * t336;
            let t572 = t555 * t340;
            let t574 = t555 * t344;
            let t576 = t555 * t348;
            let t578 = t555 * t352;
            let t580 = t555 * t356;
            let t582 = t555 * t360;
            let t584 = t555 * t364;
            let t586 = t555 * t368;
            let t588 = t555 * t372;
            let t592 = f64x8::splat(2.0) / f64x8::splat(15.0) * t556 - f64x8::splat(3.0) / f64x8::splat(35.0) * t558 + f64x8::splat(8.0) / f64x8::splat(135.0) * t560 - f64x8::splat(10.0) / f64x8::splat(231.0) * t562 + f64x8::splat(3.0) / f64x8::splat(91.0) * t564 - f64x8::splat(7.0) / f64x8::splat(270.0) * t566 + f64x8::splat(16.0) / f64x8::splat(765.0) * t568 - f64x8::splat(18.0) / f64x8::splat(1045.0) * t570 + f64x8::splat(10.0) / f64x8::splat(693.0) * t572 - f64x8::splat(11.0) / f64x8::splat(897.0) * t574 + f64x8::splat(24.0) / f64x8::splat(2275.0) * t576 - f64x8::splat(26.0) / f64x8::splat(2835.0) * t578 + f64x8::splat(7.0) / f64x8::splat(870.0) * t580 - f64x8::splat(15.0) / f64x8::splat(2108.0) * t582 + f64x8::splat(32.0) / f64x8::splat(5049.0) * t584 - f64x8::splat(34.0) / f64x8::splat(5985.0) * t586 + f64x8::splat(18.0) / f64x8::splat(3515.0) * t588 - f64x8::splat(2.0) / f64x8::splat(9.0) * t555 * t375;
            let t593 = ((t67).select(f64x8::splat(0.0), -t554));
            let t595 = t128 * t593;
            let t601 = t382 * t593;
            let t604 = -f64x8::splat(2.0) * t130 * t593 * t124 + f64x8::splat(2.0) * t601 * t390;
            let t607 = -t382 * t595 + t132 * t593 / f64x8::splat(4.0) + t604 * t124 / f64x8::splat(4.0);
            let t611 = ((t66).select(t592, -f64x8::splat(8.0) / f64x8::splat(3.0) * t607 * t124 - f64x8::splat(8.0) / f64x8::splat(3.0) * t135 * t593));
            let t612 = t611 * t27;
            let t613 = t52 * t612;
            let t617 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t141 * t547 - t274 - f64x8::splat(3.0) / f64x8::splat(8.0) * t613 * t26));
            let t619 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), t7 - t416)));
            let t622 = ((t150).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t619 * t151));
            let t623 = t622 * t5;
            let t628 = f64x8::splat(1.0) / t170 / t169 * param_hyb_omega_0;
            let t629 = t59 * t628;
            let t632 = t32 * t28 * t175 * t629;
            let t633 = t166 * t166;
            let t634 = f64x8::splat(1.0) / t633;
            let t635 = t634 * t33;
            let t636 = t163 * t163;
            let t637 = f64x8::splat(1.0) / t636;
            let t638 = t637 * t635;
            let t639 = t155 * v_rho1;
            let t641 = f64x8::splat(1.0) / t157 / t639;
            let t642 = t641 * v_sigma2;
            let t644 = t642 * t40 * t638;
            let t648 = t6 * t619 + t148 + f64x8::splat(1.0);
            let t652 = -f64x8::splat(0.01238293569268471) * t644 * t632 - t648 * t433 * t172 / f64x8::splat(6.0);
            let t653 = ((t180).select(t652, f64x8::splat(0.0)));
            let t654 = t653 * t430;
            let t656 = t653 * t444;
            let t658 = t653 * t448;
            let t660 = t653 * t452;
            let t662 = t653 * t456;
            let t664 = t653 * t460;
            let t666 = t653 * t464;
            let t668 = t653 * t468;
            let t670 = t653 * t472;
            let t672 = t653 * t476;
            let t674 = t653 * t480;
            let t676 = t653 * t484;
            let t678 = t653 * t488;
            let t680 = t653 * t492;
            let t682 = t653 * t496;
            let t684 = t653 * t500;
            let t686 = t653 * t504;
            let t690 = f64x8::splat(2.0) / f64x8::splat(15.0) * t654 - f64x8::splat(3.0) / f64x8::splat(35.0) * t656 + f64x8::splat(8.0) / f64x8::splat(135.0) * t658 - f64x8::splat(10.0) / f64x8::splat(231.0) * t660 + f64x8::splat(3.0) / f64x8::splat(91.0) * t662 - f64x8::splat(7.0) / f64x8::splat(270.0) * t664 + f64x8::splat(16.0) / f64x8::splat(765.0) * t666 - f64x8::splat(18.0) / f64x8::splat(1045.0) * t668 + f64x8::splat(10.0) / f64x8::splat(693.0) * t670 - f64x8::splat(11.0) / f64x8::splat(897.0) * t672 + f64x8::splat(24.0) / f64x8::splat(2275.0) * t674 - f64x8::splat(26.0) / f64x8::splat(2835.0) * t676 + f64x8::splat(7.0) / f64x8::splat(870.0) * t678 - f64x8::splat(15.0) / f64x8::splat(2108.0) * t680 + f64x8::splat(32.0) / f64x8::splat(5049.0) * t682 - f64x8::splat(34.0) / f64x8::splat(5985.0) * t684 + f64x8::splat(18.0) / f64x8::splat(3515.0) * t686 - f64x8::splat(2.0) / f64x8::splat(9.0) * t653 * t507;
            let t691 = ((t180).select(f64x8::splat(0.0), t652));
            let t693 = t241 * t691;
            let t699 = t514 * t691;
            let t702 = -f64x8::splat(2.0) * t243 * t691 * t237 + f64x8::splat(2.0) * t699 * t522;
            let t705 = -t514 * t693 + t245 * t691 / f64x8::splat(4.0) + t702 * t237 / f64x8::splat(4.0);
            let t709 = ((t179).select(t690, -f64x8::splat(8.0) / f64x8::splat(3.0) * t705 * t237 - f64x8::splat(8.0) / f64x8::splat(3.0) * t248 * t691));
            let t710 = t709 * t27;
            let t711 = t166 * t710;
            let t714 = t153 * t2;
            let t715 = t253 * t714;
            let t716 = t35 * t637;
            let t717 = v_sigma2 * t39;
            let t719 = t641 * t717 * t716;
            let t723 = ((t145).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t254 * t623 - t428 - f64x8::splat(3.0) / f64x8::splat(8.0) * t711 * t154 + f64x8::splat(0.0040369036088841095) * t719 * t715));
            let tvrho1 = t144 + t257 + (t617 + t723) * t6;
            acc_vrho_1 = tvrho1;
            let t729 = f64x8::splat(0.0046436008847567664) * t45 * t40 * t289 * t283;
            let t730 = ((t67).select(t729, f64x8::splat(0.0)));
            let t731 = t730 * t276;
            let t733 = t730 * t312;
            let t735 = t730 * t316;
            let t737 = t730 * t320;
            let t739 = t730 * t324;
            let t741 = t730 * t328;
            let t743 = t730 * t332;
            let t745 = t730 * t336;
            let t747 = t730 * t340;
            let t749 = t730 * t344;
            let t751 = t730 * t348;
            let t753 = t730 * t352;
            let t755 = t730 * t356;
            let t757 = t730 * t360;
            let t759 = t730 * t364;
            let t761 = t730 * t368;
            let t763 = t730 * t372;
            let t767 = f64x8::splat(2.0) / f64x8::splat(15.0) * t731 - f64x8::splat(3.0) / f64x8::splat(35.0) * t733 + f64x8::splat(8.0) / f64x8::splat(135.0) * t735 - f64x8::splat(10.0) / f64x8::splat(231.0) * t737 + f64x8::splat(3.0) / f64x8::splat(91.0) * t739 - f64x8::splat(7.0) / f64x8::splat(270.0) * t741 + f64x8::splat(16.0) / f64x8::splat(765.0) * t743 - f64x8::splat(18.0) / f64x8::splat(1045.0) * t745 + f64x8::splat(10.0) / f64x8::splat(693.0) * t747 - f64x8::splat(11.0) / f64x8::splat(897.0) * t749 + f64x8::splat(24.0) / f64x8::splat(2275.0) * t751 - f64x8::splat(26.0) / f64x8::splat(2835.0) * t753 + f64x8::splat(7.0) / f64x8::splat(870.0) * t755 - f64x8::splat(15.0) / f64x8::splat(2108.0) * t757 + f64x8::splat(32.0) / f64x8::splat(5049.0) * t759 - f64x8::splat(34.0) / f64x8::splat(5985.0) * t761 + f64x8::splat(18.0) / f64x8::splat(3515.0) * t763 - f64x8::splat(2.0) / f64x8::splat(9.0) * t730 * t375;
            let t768 = ((t67).select(f64x8::splat(0.0), t729));
            let t770 = t128 * t768;
            let t776 = t382 * t768;
            let t779 = -f64x8::splat(2.0) * t130 * t768 * t124 + f64x8::splat(2.0) * t776 * t390;
            let t782 = -t382 * t770 + t132 * t768 / f64x8::splat(4.0) + t779 * t124 / f64x8::splat(4.0);
            let t786 = ((t66).select(t767, -f64x8::splat(8.0) / f64x8::splat(3.0) * t782 * t124 - f64x8::splat(8.0) / f64x8::splat(3.0) * t135 * t768));
            let t787 = t786 * t27;
            let t788 = t52 * t787;
            let t791 = t45 * t39;
            let t792 = t791 * t408;
            let t796 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t788 * t26 - f64x8::splat(0.0015138388533315413) * t792 * t407));
            let tvsigma0 = t796 * t6;
            acc_vsigma_0 = tvsigma0;
            let tvsigma1 = f64x8::splat(0.0);
            acc_vsigma_1 = tvsigma1;
            let t800 = f64x8::splat(0.0046436008847567664) * t159 * t40 * t638 * t632;
            let t801 = ((t180).select(t800, f64x8::splat(0.0)));
            let t802 = t801 * t430;
            let t804 = t801 * t444;
            let t806 = t801 * t448;
            let t808 = t801 * t452;
            let t810 = t801 * t456;
            let t812 = t801 * t460;
            let t814 = t801 * t464;
            let t816 = t801 * t468;
            let t818 = t801 * t472;
            let t820 = t801 * t476;
            let t822 = t801 * t480;
            let t824 = t801 * t484;
            let t826 = t801 * t488;
            let t828 = t801 * t492;
            let t830 = t801 * t496;
            let t832 = t801 * t500;
            let t834 = t801 * t504;
            let t838 = f64x8::splat(2.0) / f64x8::splat(15.0) * t802 - f64x8::splat(3.0) / f64x8::splat(35.0) * t804 + f64x8::splat(8.0) / f64x8::splat(135.0) * t806 - f64x8::splat(10.0) / f64x8::splat(231.0) * t808 + f64x8::splat(3.0) / f64x8::splat(91.0) * t810 - f64x8::splat(7.0) / f64x8::splat(270.0) * t812 + f64x8::splat(16.0) / f64x8::splat(765.0) * t814 - f64x8::splat(18.0) / f64x8::splat(1045.0) * t816 + f64x8::splat(10.0) / f64x8::splat(693.0) * t818 - f64x8::splat(11.0) / f64x8::splat(897.0) * t820 + f64x8::splat(24.0) / f64x8::splat(2275.0) * t822 - f64x8::splat(26.0) / f64x8::splat(2835.0) * t824 + f64x8::splat(7.0) / f64x8::splat(870.0) * t826 - f64x8::splat(15.0) / f64x8::splat(2108.0) * t828 + f64x8::splat(32.0) / f64x8::splat(5049.0) * t830 - f64x8::splat(34.0) / f64x8::splat(5985.0) * t832 + f64x8::splat(18.0) / f64x8::splat(3515.0) * t834 - f64x8::splat(2.0) / f64x8::splat(9.0) * t801 * t507;
            let t839 = ((t180).select(f64x8::splat(0.0), t800));
            let t841 = t241 * t839;
            let t847 = t514 * t839;
            let t850 = -f64x8::splat(2.0) * t243 * t839 * t237 + f64x8::splat(2.0) * t847 * t522;
            let t853 = -t514 * t841 + t245 * t839 / f64x8::splat(4.0) + t850 * t237 / f64x8::splat(4.0);
            let t857 = ((t179).select(t838, -f64x8::splat(8.0) / f64x8::splat(3.0) * t853 * t237 - f64x8::splat(8.0) / f64x8::splat(3.0) * t248 * t839));
            let t858 = t857 * t27;
            let t859 = t166 * t858;
            let t862 = t159 * t39;
            let t863 = t862 * t716;
            let t867 = ((t145).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t859 * t154 - f64x8::splat(0.0015138388533315413) * t863 * t715));
            let tvsigma2 = t867 * t6;
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
