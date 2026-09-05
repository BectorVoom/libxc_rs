//! HYB_GGA_XC_WB97 exc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/hyb_gga_xc_wb97.c`
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

/// Accumulate 8 consecutive grid points into an output array.
///
/// `+=`, not `=`. The scalar kernel writes `out[ip] += v`; a plain store is a
/// different operation in two ways. It keeps the sign of a negative zero where
/// `0.0 + -0.0` gives `+0.0` -- a bit difference the fingerprint gate reports
/// as a rejection even though no value changed (`gga_x_pbepow fxc` was
/// rejected on exactly this, 273 of 200,000 `v2sigma2` elements) -- and it
/// would discard whatever a caller had already put in the buffer.
#[inline(always)]
fn store_add(s: &mut [f64], ip: usize, m: usize, acc: f64x8) {
    let a: [f64; 8] = acc.into();
    if m == 8 {
        let mut b = [0.0f64; 8];
        b.copy_from_slice(&s[ip..ip + 8]);
        let r: [f64; 8] = (f64x8::new(b) + acc).into();
        s[ip..ip + 8].copy_from_slice(&r);
    } else {
        for k in 0..m {
            s[ip + k] += a[k];
        }
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

/// Accumulate 8 elements with a given stride and offset.
///
/// `+=`, not `=`: the scalar kernel this was translated from writes
/// `out[ip * stride + offset] += v`, and a plain store is not the same
/// operation. It differs on the sign of zero -- `0.0 + -0.0` is `+0.0`
/// while a store of `-0.0` keeps the sign -- which is a bit difference
/// the fingerprint gate sees, and it would silently drop a caller's
/// existing contribution if one were ever there.
///
/// The read is not free on this path: a polarized `kxc`/`lxc` kernel
/// writes many strided outputs per point, and `lda_c_pw_erf kxc pol`
/// measured 84 -> 114 ns/pt (1.36x). It is charged anyway, because the
/// scalar kernel this is compared against does the same read. Gathering
/// into a vector, adding once and scattering back was tried and is no
/// faster (117 ns/pt), so the cost is the load itself, not scheduling.
#[inline(always)]
fn store_strided(s: &mut [f64], ip: usize, m: usize, stride: usize, offset: usize, acc: f64x8) {
    let a: [f64; 8] = acc.into();
    if m == 8 {
        let base = ip * stride + offset;
        s[base] += a[0];
        s[base + stride] += a[1];
        s[base + 2 * stride] += a[2];
        s[base + 3 * stride] += a[3];
        s[base + 4 * stride] += a[4];
        s[base + 5 * stride] += a[5];
        s[base + 6 * stride] += a[6];
        s[base + 7 * stride] += a[7];
    } else {
        for k in 0..m {
            s[(ip + k) * stride + offset] += a[k];
        }
    }
}

#[allow(unused_variables, non_snake_case)]
pub fn hyb_gga_xc_wb97_exc_pol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    param_c_x_1: f64,
    param_c_x_2: f64,
    param_c_x_3: f64,
    param_c_x_4: f64,
    param_c_x_0: f64,
    param_c_ss_1: f64,
    param_c_ss_2: f64,
    param_c_ss_3: f64,
    param_c_ss_4: f64,
    param_c_ss_0: f64,
    param_c_ab_1: f64,
    param_c_ab_2: f64,
    param_c_ab_3: f64,
    param_c_ab_4: f64,
    param_c_ab_0: f64,
    param_hyb_omega_0: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_c_x_1 = f64x8::splat(param_c_x_1);
    let param_c_x_2 = f64x8::splat(param_c_x_2);
    let param_c_x_3 = f64x8::splat(param_c_x_3);
    let param_c_x_4 = f64x8::splat(param_c_x_4);
    let param_c_x_0 = f64x8::splat(param_c_x_0);
    let param_c_ss_1 = f64x8::splat(param_c_ss_1);
    let param_c_ss_2 = f64x8::splat(param_c_ss_2);
    let param_c_ss_3 = f64x8::splat(param_c_ss_3);
    let param_c_ss_4 = f64x8::splat(param_c_ss_4);
    let param_c_ss_0 = f64x8::splat(param_c_ss_0);
    let param_c_ab_1 = f64x8::splat(param_c_ab_1);
    let param_c_ab_2 = f64x8::splat(param_c_ab_2);
    let param_c_ab_3 = f64x8::splat(param_c_ab_3);
    let param_c_ab_4 = f64x8::splat(param_c_ab_4);
    let param_c_ab_0 = f64x8::splat(param_c_ab_0);
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
        {
            let t2 = v_rho0 - v_rho1;
            let t3 = v_rho0 + v_rho1;
            let t4 = f64x8::splat(1.0) / t3;
            let t5 = t2 * t4;
            let t6 = f64x8::splat(1.0) + t5;
            let t7 = (t6).simd_le(zeta_threshold);
            let t8 = ((v_rho0).simd_le(dens_threshold)) | (t7);
            let t9 = f64x8::splat(M_CBRT3);
            let t11 = f64x8::splat(1.0) / f64x8::splat(M_PI);
            let t12 = (simd::cbrt(t11));
            let t13 = f64x8::splat(M_CBRT4);
            let t14 = t13 * t13;
            let t15 = t12 * t14;
            let t16 = f64x8::splat(M_CBRT2);
            let t17 = t15 * t16;
            let t18 = t6 * t9 * t17;
            let t19 = (f64x8::splat(2.0)).simd_le(zeta_threshold);
            let t20 = (simd::cbrt(zeta_threshold));
            let t21 = t20 * zeta_threshold;
            let t23 = ((t19).select(t21, f64x8::splat(2.0) * t16));
            let t24 = (simd::cbrt(t3));
            let t25 = t23 * t24;
            let t26 = f64x8::splat(1.0) / t6;
            let t27 = (simd::cbrt(t26));
            let t28 = f64x8::splat(1.0) / t27;
            let t29 = (simd::cbrt(f64x8::splat(9.0)));
            let t30 = t29 * t29;
            let t31 = t12 * t12;
            let t32 = t30 * t31;
            let t33 = param_hyb_omega_0 * t9;
            let t34 = t32 * t33;
            let t35 = f64x8::splat(1.0) / t24;
            let t36 = t35 * t16;
            let t37 = ((t19).select(t20, t16));
            let t38 = f64x8::splat(1.0) / t37;
            let t39 = t27 * t38;
            let t42 = t34 * t36 * t39 / f64x8::splat(18.0);
            let t43 = (f64x8::splat(1.35)).simd_le(t42);
            let t44 = (f64x8::splat(1.35)).simd_lt(t42);
            let t45 = ((t44).select(t42, f64x8::splat(1.35)));
            let t46 = t45 * t45;
            let t49 = t46 * t46;
            let t50 = f64x8::splat(1.0) / t49;
            let t52 = t49 * t46;
            let t53 = f64x8::splat(1.0) / t52;
            let t55 = t49 * t49;
            let t56 = f64x8::splat(1.0) / t55;
            let t59 = f64x8::splat(1.0) / t55 / t46;
            let t62 = f64x8::splat(1.0) / t55 / t49;
            let t65 = f64x8::splat(1.0) / t55 / t52;
            let t67 = t55 * t55;
            let t68 = f64x8::splat(1.0) / t67;
            let t71 = ((t44).select(f64x8::splat(1.35), t42));
            let t72 = ((f64x8::splat(M_PI)).sqrt());
            let t73 = f64x8::splat(1.0) / t71;
            let t75 = (simd::erf(t73 / f64x8::splat(2.0)));
            let t77 = t71 * t71;
            let t78 = f64x8::splat(1.0) / t77;
            let t80 = (simd::exp(-t78 / f64x8::splat(4.0)));
            let t81 = t80 - f64x8::splat(1.0);
            let t84 = t80 - f64x8::splat(3.0) / f64x8::splat(2.0) - f64x8::splat(2.0) * t77 * t81;
            let t87 = f64x8::splat(2.0) * t71 * t84 + t72 * t75;
            let t91 = ((t43).select(f64x8::splat(1.0) / t46 / f64x8::splat(36.0) - t50 / f64x8::splat(960.0) + t53 / f64x8::splat(26880.0) - t56 / f64x8::splat(829440.0) + t59 / f64x8::splat(28385280.0) - t62 / f64x8::splat(1073479680.0) + t65 / f64x8::splat(44590694400.0) - t68 / f64x8::splat(2021444812800.0), f64x8::splat(1.0) - f64x8::splat(8.0) / f64x8::splat(3.0) * t71 * t87));
            let t92 = t28 * t91;
            let t93 = param_c_x_0;
            let t94 = param_c_x_1;
            let t95 = t94 * v_sigma0;
            let t96 = v_rho0 * v_rho0;
            let t97 = (simd::cbrt(v_rho0));
            let t98 = t97 * t97;
            let t100 = f64x8::splat(1.0) / t98 / t96;
            let t101 = v_sigma0 * t100;
            let t103 = f64x8::splat(1.0) + f64x8::splat(0.004) * t101;
            let t104 = f64x8::splat(1.0) / t103;
            let t108 = param_c_x_2;
            let t109 = v_sigma0 * v_sigma0;
            let t110 = t108 * t109;
            let t111 = t96 * t96;
            let t112 = t111 * v_rho0;
            let t114 = f64x8::splat(1.0) / t97 / t112;
            let t115 = t103 * t103;
            let t116 = f64x8::splat(1.0) / t115;
            let t117 = t114 * t116;
            let t120 = param_c_x_3;
            let t121 = t109 * v_sigma0;
            let t122 = t120 * t121;
            let t123 = t111 * t111;
            let t124 = f64x8::splat(1.0) / t123;
            let t125 = t115 * t103;
            let t126 = f64x8::splat(1.0) / t125;
            let t127 = t124 * t126;
            let t130 = param_c_x_4;
            let t131 = t109 * t109;
            let t132 = t130 * t131;
            let t133 = t123 * t96;
            let t135 = f64x8::splat(1.0) / t98 / t133;
            let t136 = t115 * t115;
            let t137 = f64x8::splat(1.0) / t136;
            let t138 = t135 * t137;
            let t141 = t93 + f64x8::splat(0.004) * t95 * t100 * t104 + f64x8::splat(1.6e-05) * t110 * t117 + f64x8::splat(6.4e-08) * t122 * t127 + f64x8::splat(2.56e-10) * t132 * t138;
            let t142 = t92 * t141;
            let t143 = t25 * t142;
            let t146 = ((t8).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(64.0) * t18 * t143));
            let t148 = f64x8::splat(1.0) - t5;
            let t149 = (t148).simd_le(zeta_threshold);
            let t150 = ((v_rho1).simd_le(dens_threshold)) | (t149);
            let t152 = t148 * t9 * t17;
            let t153 = f64x8::splat(1.0) / t148;
            let t154 = (simd::cbrt(t153));
            let t155 = f64x8::splat(1.0) / t154;
            let t156 = t154 * t38;
            let t159 = t34 * t36 * t156 / f64x8::splat(18.0);
            let t160 = (f64x8::splat(1.35)).simd_le(t159);
            let t161 = (f64x8::splat(1.35)).simd_lt(t159);
            let t162 = ((t161).select(t159, f64x8::splat(1.35)));
            let t163 = t162 * t162;
            let t166 = t163 * t163;
            let t167 = f64x8::splat(1.0) / t166;
            let t169 = t166 * t163;
            let t170 = f64x8::splat(1.0) / t169;
            let t172 = t166 * t166;
            let t173 = f64x8::splat(1.0) / t172;
            let t176 = f64x8::splat(1.0) / t172 / t163;
            let t179 = f64x8::splat(1.0) / t172 / t166;
            let t182 = f64x8::splat(1.0) / t172 / t169;
            let t184 = t172 * t172;
            let t185 = f64x8::splat(1.0) / t184;
            let t188 = ((t161).select(f64x8::splat(1.35), t159));
            let t189 = f64x8::splat(1.0) / t188;
            let t191 = (simd::erf(t189 / f64x8::splat(2.0)));
            let t193 = t188 * t188;
            let t194 = f64x8::splat(1.0) / t193;
            let t196 = (simd::exp(-t194 / f64x8::splat(4.0)));
            let t197 = t196 - f64x8::splat(1.0);
            let t200 = t196 - f64x8::splat(3.0) / f64x8::splat(2.0) - f64x8::splat(2.0) * t193 * t197;
            let t203 = f64x8::splat(2.0) * t188 * t200 + t191 * t72;
            let t207 = ((t160).select(f64x8::splat(1.0) / t163 / f64x8::splat(36.0) - t167 / f64x8::splat(960.0) + t170 / f64x8::splat(26880.0) - t173 / f64x8::splat(829440.0) + t176 / f64x8::splat(28385280.0) - t179 / f64x8::splat(1073479680.0) + t182 / f64x8::splat(44590694400.0) - t185 / f64x8::splat(2021444812800.0), f64x8::splat(1.0) - f64x8::splat(8.0) / f64x8::splat(3.0) * t188 * t203));
            let t208 = t155 * t207;
            let t209 = t94 * v_sigma2;
            let t210 = v_rho1 * v_rho1;
            let t211 = (simd::cbrt(v_rho1));
            let t212 = t211 * t211;
            let t214 = f64x8::splat(1.0) / t212 / t210;
            let t215 = v_sigma2 * t214;
            let t217 = f64x8::splat(1.0) + f64x8::splat(0.004) * t215;
            let t218 = f64x8::splat(1.0) / t217;
            let t222 = v_sigma2 * v_sigma2;
            let t223 = t108 * t222;
            let t224 = t210 * t210;
            let t225 = t224 * v_rho1;
            let t227 = f64x8::splat(1.0) / t211 / t225;
            let t228 = t217 * t217;
            let t229 = f64x8::splat(1.0) / t228;
            let t230 = t227 * t229;
            let t233 = t222 * v_sigma2;
            let t234 = t120 * t233;
            let t235 = t224 * t224;
            let t236 = f64x8::splat(1.0) / t235;
            let t237 = t228 * t217;
            let t238 = f64x8::splat(1.0) / t237;
            let t239 = t236 * t238;
            let t242 = t222 * t222;
            let t243 = t130 * t242;
            let t244 = t235 * t210;
            let t246 = f64x8::splat(1.0) / t212 / t244;
            let t247 = t228 * t228;
            let t248 = f64x8::splat(1.0) / t247;
            let t249 = t246 * t248;
            let t252 = t93 + f64x8::splat(0.004) * t209 * t214 * t218 + f64x8::splat(1.6e-05) * t223 * t230 + f64x8::splat(6.4e-08) * t234 * t239 + f64x8::splat(2.56e-10) * t243 * t249;
            let t253 = t208 * t252;
            let t254 = t25 * t253;
            let t257 = ((t150).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(64.0) * t152 * t254));
            let t258 = ((t7).select(zeta_threshold, t6));
            let t259 = t9 * t12;
            let t260 = t259 * t14;
            let t261 = f64x8::splat(1.0) / t20;
            let t262 = (simd::cbrt(t6));
            let t264 = ((t7).select(t261, f64x8::splat(1.0) / t262));
            let t266 = t260 * t36 * t264;
            let t268 = f64x8::splat(1.0) + f64x8::splat(0.053425) * t266;
            let t269 = ((t266).sqrt());
            let t272 = ((t266) * (t266).sqrt());
            let t274 = t9 * t9;
            let t275 = t274 * t31;
            let t276 = t275 * t13;
            let t277 = t24 * t24;
            let t278 = f64x8::splat(1.0) / t277;
            let t279 = t16 * t16;
            let t280 = t278 * t279;
            let t281 = t264 * t264;
            let t283 = t276 * t280 * t281;
            let t285 = f64x8::splat(3.79785) * t269 + f64x8::splat(0.8969) * t266 + f64x8::splat(0.204775) * t272 + f64x8::splat(0.123235) * t283;
            let t288 = f64x8::splat(1.0) + f64x8::splat(16.081824322151103) / t285;
            let t289 = (simd::ln(t288));
            let t291 = f64x8::splat(0.062182) * t268 * t289;
            let t293 = (((f64x8::splat(0.0)).simd_le(zeta_threshold)).select(t21, f64x8::splat(0.0)));
            let t297 = f64x8::splat(1.0) / (f64x8::splat(2.0) * t16 - f64x8::splat(2.0));
            let t298 = (t23 + t293 - f64x8::splat(2.0)) * t297;
            let t300 = f64x8::splat(1.0) + f64x8::splat(0.05137) * t266;
            let t305 = f64x8::splat(7.05945) * t269 + f64x8::splat(1.549425) * t266 + f64x8::splat(0.420775) * t272 + f64x8::splat(0.1562925) * t283;
            let t308 = f64x8::splat(1.0) + f64x8::splat(32.1646831778707) / t305;
            let t309 = (simd::ln(t308));
            let t313 = f64x8::splat(1.0) + f64x8::splat(0.0278125) * t266;
            let t318 = f64x8::splat(5.1785) * t269 + f64x8::splat(0.905775) * t266 + f64x8::splat(0.1100325) * t272 + f64x8::splat(0.1241775) * t283;
            let t321 = f64x8::splat(1.0) + f64x8::splat(29.608574643216677) / t318;
            let t322 = (simd::ln(t321));
            let t323 = t313 * t322;
            let t329 = -t291 + t298 * (-f64x8::splat(0.03109) * t300 * t309 + t291 - f64x8::splat(0.019751789702565206) * t323) + f64x8::splat(0.019751789702565206) * t298 * t323;
            let t332 = ((t8).select(f64x8::splat(0.0), t258 * t329 / f64x8::splat(2.0)));
            let t333 = param_c_ss_0;
            let t334 = param_c_ss_1;
            let t335 = t334 * v_sigma0;
            let t337 = f64x8::splat(1.0) + f64x8::splat(0.2) * t101;
            let t338 = f64x8::splat(1.0) / t337;
            let t342 = param_c_ss_2;
            let t343 = t342 * t109;
            let t344 = t337 * t337;
            let t345 = f64x8::splat(1.0) / t344;
            let t346 = t114 * t345;
            let t349 = param_c_ss_3;
            let t350 = t349 * t121;
            let t351 = t344 * t337;
            let t352 = f64x8::splat(1.0) / t351;
            let t353 = t124 * t352;
            let t356 = param_c_ss_4;
            let t357 = t356 * t131;
            let t358 = t344 * t344;
            let t359 = f64x8::splat(1.0) / t358;
            let t360 = t135 * t359;
            let t363 = t333 + f64x8::splat(0.2) * t335 * t100 * t338 + f64x8::splat(0.04) * t343 * t346 + f64x8::splat(0.008) * t350 * t353 + f64x8::splat(0.0016) * t357 * t360;
            let t364 = t332 * t363;
            let t365 = ((t149).select(zeta_threshold, t148));
            let t366 = (simd::cbrt(t148));
            let t368 = ((t149).select(t261, f64x8::splat(1.0) / t366));
            let t370 = t260 * t36 * t368;
            let t372 = f64x8::splat(1.0) + f64x8::splat(0.053425) * t370;
            let t373 = ((t370).sqrt());
            let t376 = ((t370) * (t370).sqrt());
            let t378 = t368 * t368;
            let t380 = t276 * t280 * t378;
            let t382 = f64x8::splat(3.79785) * t373 + f64x8::splat(0.8969) * t370 + f64x8::splat(0.204775) * t376 + f64x8::splat(0.123235) * t380;
            let t385 = f64x8::splat(1.0) + f64x8::splat(16.081824322151103) / t382;
            let t386 = (simd::ln(t385));
            let t388 = f64x8::splat(0.062182) * t372 * t386;
            let t390 = f64x8::splat(1.0) + f64x8::splat(0.05137) * t370;
            let t395 = f64x8::splat(7.05945) * t373 + f64x8::splat(1.549425) * t370 + f64x8::splat(0.420775) * t376 + f64x8::splat(0.1562925) * t380;
            let t398 = f64x8::splat(1.0) + f64x8::splat(32.1646831778707) / t395;
            let t399 = (simd::ln(t398));
            let t403 = f64x8::splat(1.0) + f64x8::splat(0.0278125) * t370;
            let t408 = f64x8::splat(5.1785) * t373 + f64x8::splat(0.905775) * t370 + f64x8::splat(0.1100325) * t376 + f64x8::splat(0.1241775) * t380;
            let t411 = f64x8::splat(1.0) + f64x8::splat(29.608574643216677) / t408;
            let t412 = (simd::ln(t411));
            let t413 = t403 * t412;
            let t419 = -t388 + t298 * (-f64x8::splat(0.03109) * t390 * t399 + t388 - f64x8::splat(0.019751789702565206) * t413) + f64x8::splat(0.019751789702565206) * t298 * t413;
            let t422 = ((t150).select(f64x8::splat(0.0), t365 * t419 / f64x8::splat(2.0)));
            let t423 = t334 * v_sigma2;
            let t425 = f64x8::splat(1.0) + f64x8::splat(0.2) * t215;
            let t426 = f64x8::splat(1.0) / t425;
            let t430 = t342 * t222;
            let t431 = t425 * t425;
            let t432 = f64x8::splat(1.0) / t431;
            let t433 = t227 * t432;
            let t436 = t349 * t233;
            let t437 = t431 * t425;
            let t438 = f64x8::splat(1.0) / t437;
            let t439 = t236 * t438;
            let t442 = t356 * t242;
            let t443 = t431 * t431;
            let t444 = f64x8::splat(1.0) / t443;
            let t445 = t246 * t444;
            let t448 = t333 + f64x8::splat(0.2) * t423 * t214 * t426 + f64x8::splat(0.04) * t430 * t433 + f64x8::splat(0.008) * t436 * t439 + f64x8::splat(0.0016) * t442 * t445;
            let t449 = t422 * t448;
            let t451 = t259 * t14 * t35;
            let t453 = f64x8::splat(1.0) + f64x8::splat(0.053425) * t451;
            let t454 = ((t451).sqrt());
            let t457 = ((t451) * (t451).sqrt());
            let t460 = t275 * t13 * t278;
            let t462 = f64x8::splat(3.79785) * t454 + f64x8::splat(0.8969) * t451 + f64x8::splat(0.204775) * t457 + f64x8::splat(0.123235) * t460;
            let t465 = f64x8::splat(1.0) + f64x8::splat(16.081824322151103) / t462;
            let t466 = (simd::ln(t465));
            let t468 = f64x8::splat(0.062182) * t453 * t466;
            let t469 = t2 * t2;
            let t470 = t469 * t469;
            let t471 = t3 * t3;
            let t472 = t471 * t471;
            let t473 = f64x8::splat(1.0) / t472;
            let t474 = t470 * t473;
            let t475 = t262 * t6;
            let t476 = ((t7).select(t21, t475));
            let t477 = t366 * t148;
            let t478 = ((t149).select(t21, t477));
            let t479 = t476 + t478 - f64x8::splat(2.0);
            let t480 = t479 * t297;
            let t482 = f64x8::splat(1.0) + f64x8::splat(0.05137) * t451;
            let t487 = f64x8::splat(7.05945) * t454 + f64x8::splat(1.549425) * t451 + f64x8::splat(0.420775) * t457 + f64x8::splat(0.1562925) * t460;
            let t490 = f64x8::splat(1.0) + f64x8::splat(32.1646831778707) / t487;
            let t491 = (simd::ln(t490));
            let t495 = f64x8::splat(1.0) + f64x8::splat(0.0278125) * t451;
            let t500 = f64x8::splat(5.1785) * t454 + f64x8::splat(0.905775) * t451 + f64x8::splat(0.1100325) * t457 + f64x8::splat(0.1241775) * t460;
            let t503 = f64x8::splat(1.0) + f64x8::splat(29.608574643216677) / t500;
            let t504 = (simd::ln(t503));
            let t505 = t495 * t504;
            let t507 = -f64x8::splat(0.03109) * t482 * t491 + t468 - f64x8::splat(0.019751789702565206) * t505;
            let t508 = t480 * t507;
            let t512 = -t468 + t474 * t508 + f64x8::splat(0.019751789702565206) * t480 * t505 - t332 - t422;
            let t514 = param_c_ab_1;
            let t515 = t101 + t215;
            let t516 = t514 * t515;
            let t519 = f64x8::splat(1.0) + f64x8::splat(0.003) * t101 + f64x8::splat(0.003) * t215;
            let t520 = f64x8::splat(1.0) / t519;
            let t523 = param_c_ab_2;
            let t524 = t515 * t515;
            let t525 = t523 * t524;
            let t526 = t519 * t519;
            let t527 = f64x8::splat(1.0) / t526;
            let t530 = param_c_ab_3;
            let t531 = t524 * t515;
            let t532 = t530 * t531;
            let t533 = t526 * t519;
            let t534 = f64x8::splat(1.0) / t533;
            let t537 = param_c_ab_4;
            let t538 = t524 * t524;
            let t539 = t537 * t538;
            let t540 = t526 * t526;
            let t541 = f64x8::splat(1.0) / t540;
            let t544 = param_c_ab_0 + f64x8::splat(0.003) * t516 * t520 + f64x8::splat(9e-06) * t525 * t527 + f64x8::splat(2.7e-08) * t532 * t534 + f64x8::splat(8.1e-11) * t539 * t541;
            let t545 = t512 * t544;
            let tzk0 = t146 + t257 + t364 + t449 + t545;
            acc_zk = tzk0;
        }
        store_add(zk, ip, m, acc_zk);
        ip += 8;
    }
}
