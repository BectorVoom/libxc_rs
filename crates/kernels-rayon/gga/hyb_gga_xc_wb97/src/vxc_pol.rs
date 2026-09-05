//! HYB_GGA_XC_WB97 vxc pol kernel — explicit SIMD (bit-exact).
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
pub fn hyb_gga_xc_wb97_vxc_pol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
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
        let mut acc_vrho_0 = V_ZERO;
        let mut acc_vrho_1 = V_ZERO;
        let mut acc_vsigma_0 = V_ZERO;
        let mut acc_vsigma_1 = V_ZERO;
        let mut acc_vsigma_2 = V_ZERO;
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
            let t546 = f64x8::splat(1.0) / t471;
            let t547 = t2 * t546;
            let t548 = t4 - t547;
            let t550 = t548 * t9 * t17;
            let t553 = t23 * t278;
            let t554 = t553 * t142;
            let t556 = t18 * t554 / f64x8::splat(64.0);
            let t558 = t26 * t9 * t17;
            let t560 = f64x8::splat(1.0) / t27 / t26;
            let t561 = t25 * t560;
            let t562 = t91 * t141;
            let t563 = t562 * t548;
            let t567 = t46 * t45;
            let t568 = f64x8::splat(1.0) / t567;
            let t570 = f64x8::splat(1.0) / t24 / t3;
            let t571 = t570 * t16;
            let t573 = t34 * t571 * t39;
            let t575 = t32 * t33 * t35;
            let t576 = t27 * t27;
            let t577 = f64x8::splat(1.0) / t576;
            let t578 = t16 * t577;
            let t579 = t6 * t6;
            let t580 = f64x8::splat(1.0) / t579;
            let t581 = t38 * t580;
            let t583 = t578 * t581 * t548;
            let t586 = -t575 * t583 / f64x8::splat(54.0) - t573 / f64x8::splat(54.0);
            let t587 = ((t44).select(t586, f64x8::splat(0.0)));
            let t590 = t49 * t45;
            let t591 = f64x8::splat(1.0) / t590;
            let t594 = t49 * t567;
            let t595 = f64x8::splat(1.0) / t594;
            let t599 = f64x8::splat(1.0) / t55 / t45;
            let t603 = f64x8::splat(1.0) / t55 / t567;
            let t607 = f64x8::splat(1.0) / t55 / t590;
            let t611 = f64x8::splat(1.0) / t55 / t594;
            let t615 = f64x8::splat(1.0) / t67 / t45;
            let t619 = ((t44).select(f64x8::splat(0.0), t586));
            let t621 = t80 * t78;
            let t625 = t77 * t71;
            let t626 = f64x8::splat(1.0) / t625;
            let t630 = t71 * t81;
            let t635 = t626 * t619 * t80 / f64x8::splat(2.0) - f64x8::splat(4.0) * t630 * t619 - t73 * t619 * t80;
            let t638 = -t619 * t621 + f64x8::splat(2.0) * t619 * t84 + f64x8::splat(2.0) * t635 * t71;
            let t642 = ((t43).select(-t568 * t587 / f64x8::splat(18.0) + t591 * t587 / f64x8::splat(240.0) - t595 * t587 / f64x8::splat(4480.0) + t599 * t587 / f64x8::splat(103680.0) - t603 * t587 / f64x8::splat(2838528.0) + t607 * t587 / f64x8::splat(89456640.0) - t611 * t587 / f64x8::splat(3185049600.0) + t615 * t587 / f64x8::splat(126340300800.0), -f64x8::splat(8.0) / f64x8::splat(3.0) * t619 * t87 - f64x8::splat(8.0) / f64x8::splat(3.0) * t71 * t638));
            let t643 = t28 * t642;
            let t644 = t643 * t141;
            let t645 = t25 * t644;
            let t648 = t96 * v_rho0;
            let t650 = f64x8::splat(1.0) / t98 / t648;
            let t654 = t94 * t109;
            let t655 = t111 * t96;
            let t657 = f64x8::splat(1.0) / t97 / t655;
            let t658 = t657 * t116;
            let t663 = t108 * t121;
            let t664 = t123 * v_rho0;
            let t665 = f64x8::splat(1.0) / t664;
            let t666 = t665 * t126;
            let t671 = t120 * t131;
            let t672 = t123 * t648;
            let t674 = f64x8::splat(1.0) / t98 / t672;
            let t675 = t674 * t137;
            let t680 = t131 * v_sigma0;
            let t681 = t130 * t680;
            let t682 = t123 * t655;
            let t684 = f64x8::splat(1.0) / t97 / t682;
            let t686 = f64x8::splat(1.0) / t136 / t103;
            let t687 = t684 * t686;
            let t690 = -f64x8::splat(0.010666666666666666) * t95 * t650 * t104 + f64x8::splat(4.266666666666667e-05) * t654 * t658 - f64x8::splat(8.533333333333334e-05) * t110 * t658 + f64x8::splat(3.413333333333333e-07) * t663 * t666 - f64x8::splat(5.12e-07) * t122 * t666 + f64x8::splat(2.048e-09) * t671 * t675 - f64x8::splat(2.7306666666666667e-09) * t132 * t675 + f64x8::splat(1.0922666666666666e-11) * t681 * t687;
            let t691 = t92 * t690;
            let t692 = t25 * t691;
            let t696 = ((t8).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(64.0) * t550 * t143 - t556 - t558 * t561 * t563 / f64x8::splat(64.0) - f64x8::splat(3.0) / f64x8::splat(64.0) * t18 * t645 - f64x8::splat(3.0) / f64x8::splat(64.0) * t18 * t692));
            let t697 = -t548;
            let t699 = t697 * t9 * t17;
            let t702 = t553 * t253;
            let t704 = t152 * t702 / f64x8::splat(64.0);
            let t706 = t153 * t9 * t17;
            let t708 = f64x8::splat(1.0) / t154 / t153;
            let t709 = t25 * t708;
            let t710 = t207 * t252;
            let t711 = t710 * t697;
            let t715 = t163 * t162;
            let t716 = f64x8::splat(1.0) / t715;
            let t718 = t34 * t571 * t156;
            let t719 = t154 * t154;
            let t720 = f64x8::splat(1.0) / t719;
            let t721 = t16 * t720;
            let t722 = t148 * t148;
            let t723 = f64x8::splat(1.0) / t722;
            let t724 = t38 * t723;
            let t726 = t721 * t724 * t697;
            let t729 = -t575 * t726 / f64x8::splat(54.0) - t718 / f64x8::splat(54.0);
            let t730 = ((t161).select(t729, f64x8::splat(0.0)));
            let t733 = t166 * t162;
            let t734 = f64x8::splat(1.0) / t733;
            let t737 = t166 * t715;
            let t738 = f64x8::splat(1.0) / t737;
            let t742 = f64x8::splat(1.0) / t172 / t162;
            let t746 = f64x8::splat(1.0) / t172 / t715;
            let t750 = f64x8::splat(1.0) / t172 / t733;
            let t754 = f64x8::splat(1.0) / t172 / t737;
            let t758 = f64x8::splat(1.0) / t184 / t162;
            let t762 = ((t161).select(f64x8::splat(0.0), t729));
            let t764 = t196 * t194;
            let t768 = t193 * t188;
            let t769 = f64x8::splat(1.0) / t768;
            let t773 = t188 * t197;
            let t778 = t769 * t762 * t196 / f64x8::splat(2.0) - f64x8::splat(4.0) * t773 * t762 - t189 * t762 * t196;
            let t781 = f64x8::splat(2.0) * t188 * t778 + f64x8::splat(2.0) * t200 * t762 - t762 * t764;
            let t785 = ((t160).select(-t716 * t730 / f64x8::splat(18.0) + t734 * t730 / f64x8::splat(240.0) - t738 * t730 / f64x8::splat(4480.0) + t742 * t730 / f64x8::splat(103680.0) - t746 * t730 / f64x8::splat(2838528.0) + t750 * t730 / f64x8::splat(89456640.0) - t754 * t730 / f64x8::splat(3185049600.0) + t758 * t730 / f64x8::splat(126340300800.0), -f64x8::splat(8.0) / f64x8::splat(3.0) * t188 * t781 - f64x8::splat(8.0) / f64x8::splat(3.0) * t762 * t203));
            let t786 = t155 * t785;
            let t787 = t786 * t252;
            let t788 = t25 * t787;
            let t792 = ((t150).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(64.0) * t699 * t254 - t704 - t706 * t709 * t711 / f64x8::splat(64.0) - f64x8::splat(3.0) / f64x8::splat(64.0) * t152 * t788));
            let t793 = ((t7).select(f64x8::splat(0.0), t548));
            let t796 = t260 * t571 * t264;
            let t797 = f64x8::splat(0.017808333333333332) * t796;
            let t798 = f64x8::splat(1.0) / t475;
            let t801 = ((t7).select(f64x8::splat(0.0), -t798 * t548 / f64x8::splat(3.0)));
            let t803 = t260 * t36 * t801;
            let t805 = -t797 + f64x8::splat(0.053425) * t803;
            let t807 = f64x8::splat(0.062182) * t805 * t289;
            let t808 = t285 * t285;
            let t809 = f64x8::splat(1.0) / t808;
            let t810 = t268 * t809;
            let t811 = f64x8::splat(1.0) / t269;
            let t812 = t796 / f64x8::splat(3.0);
            let t813 = -t812 + t803;
            let t814 = t811 * t813;
            let t816 = f64x8::splat(0.29896666666666666) * t796;
            let t818 = ((t266).sqrt());
            let t819 = t818 * t813;
            let t822 = f64x8::splat(1.0) / t277 / t3;
            let t823 = t822 * t279;
            let t825 = t276 * t823 * t281;
            let t826 = f64x8::splat(0.08215666666666667) * t825;
            let t827 = t264 * t801;
            let t829 = t276 * t280 * t827;
            let t831 = f64x8::splat(1.898925) * t814 - t816 + f64x8::splat(0.8969) * t803 + f64x8::splat(0.3071625) * t819 - t826 + f64x8::splat(0.24647) * t829;
            let t832 = f64x8::splat(1.0) / t288;
            let t833 = t831 * t832;
            let t835 = f64x8::splat(1.0) * t810 * t833;
            let t836 = f64x8::splat(0.017123333333333334) * t796;
            let t838 = -t836 + f64x8::splat(0.05137) * t803;
            let t841 = t305 * t305;
            let t842 = f64x8::splat(1.0) / t841;
            let t843 = t300 * t842;
            let t845 = f64x8::splat(0.516475) * t796;
            let t848 = f64x8::splat(0.104195) * t825;
            let t850 = f64x8::splat(3.529725) * t814 - t845 + f64x8::splat(1.549425) * t803 + f64x8::splat(0.6311625) * t819 - t848 + f64x8::splat(0.312585) * t829;
            let t851 = f64x8::splat(1.0) / t308;
            let t852 = t850 * t851;
            let t855 = f64x8::splat(0.009270833333333334) * t796;
            let t857 = -t855 + f64x8::splat(0.0278125) * t803;
            let t858 = t857 * t322;
            let t860 = t318 * t318;
            let t861 = f64x8::splat(1.0) / t860;
            let t862 = t313 * t861;
            let t864 = f64x8::splat(0.301925) * t796;
            let t867 = f64x8::splat(0.082785) * t825;
            let t869 = f64x8::splat(2.58925) * t814 - t864 + f64x8::splat(0.905775) * t803 + f64x8::splat(0.16504875) * t819 - t867 + f64x8::splat(0.248355) * t829;
            let t870 = f64x8::splat(1.0) / t321;
            let t871 = t869 * t870;
            let t878 = t298 * t313;
            let t880 = t861 * t869 * t870;
            let t883 = -t807 + t835 + t298 * (-f64x8::splat(0.03109) * t838 * t309 + f64x8::splat(1.0) * t843 * t852 + t807 - t835 - f64x8::splat(0.019751789702565206) * t858 + f64x8::splat(0.5848223397455204) * t862 * t871) + f64x8::splat(0.019751789702565206) * t298 * t858 - f64x8::splat(0.5848223397455204) * t878 * t880;
            let t887 = ((t8).select(f64x8::splat(0.0), t258 * t883 / f64x8::splat(2.0) + t793 * t329 / f64x8::splat(2.0)));
            let t888 = t887 * t363;
            let t892 = t334 * t109;
            let t893 = t657 * t345;
            let t898 = t342 * t121;
            let t899 = t665 * t352;
            let t904 = t349 * t131;
            let t905 = t674 * t359;
            let t910 = t356 * t680;
            let t912 = f64x8::splat(1.0) / t358 / t337;
            let t913 = t684 * t912;
            let t916 = -f64x8::splat(0.5333333333333333) * t335 * t650 * t338 + f64x8::splat(0.10666666666666667) * t892 * t893 - f64x8::splat(0.21333333333333335) * t343 * t893 + f64x8::splat(0.042666666666666665) * t898 * t899 - f64x8::splat(0.064) * t350 * t899 + f64x8::splat(0.0128) * t904 * t905 - f64x8::splat(0.017066666666666667) * t357 * t905 + f64x8::splat(0.0034133333333333333) * t910 * t913;
            let t917 = t332 * t916;
            let t918 = ((t149).select(f64x8::splat(0.0), t697));
            let t921 = t260 * t571 * t368;
            let t922 = f64x8::splat(0.017808333333333332) * t921;
            let t923 = f64x8::splat(1.0) / t477;
            let t926 = ((t149).select(f64x8::splat(0.0), -t923 * t697 / f64x8::splat(3.0)));
            let t928 = t260 * t36 * t926;
            let t930 = -t922 + f64x8::splat(0.053425) * t928;
            let t932 = f64x8::splat(0.062182) * t930 * t386;
            let t933 = t382 * t382;
            let t934 = f64x8::splat(1.0) / t933;
            let t935 = t372 * t934;
            let t936 = f64x8::splat(1.0) / t373;
            let t937 = t921 / f64x8::splat(3.0);
            let t938 = -t937 + t928;
            let t939 = t936 * t938;
            let t941 = f64x8::splat(0.29896666666666666) * t921;
            let t943 = ((t370).sqrt());
            let t944 = t943 * t938;
            let t947 = t276 * t823 * t378;
            let t948 = f64x8::splat(0.08215666666666667) * t947;
            let t949 = t368 * t926;
            let t951 = t276 * t280 * t949;
            let t953 = f64x8::splat(1.898925) * t939 - t941 + f64x8::splat(0.8969) * t928 + f64x8::splat(0.3071625) * t944 - t948 + f64x8::splat(0.24647) * t951;
            let t954 = f64x8::splat(1.0) / t385;
            let t955 = t953 * t954;
            let t957 = f64x8::splat(1.0) * t935 * t955;
            let t958 = f64x8::splat(0.017123333333333334) * t921;
            let t960 = -t958 + f64x8::splat(0.05137) * t928;
            let t963 = t395 * t395;
            let t964 = f64x8::splat(1.0) / t963;
            let t965 = t390 * t964;
            let t967 = f64x8::splat(0.516475) * t921;
            let t970 = f64x8::splat(0.104195) * t947;
            let t972 = f64x8::splat(3.529725) * t939 - t967 + f64x8::splat(1.549425) * t928 + f64x8::splat(0.6311625) * t944 - t970 + f64x8::splat(0.312585) * t951;
            let t973 = f64x8::splat(1.0) / t398;
            let t974 = t972 * t973;
            let t977 = f64x8::splat(0.009270833333333334) * t921;
            let t979 = -t977 + f64x8::splat(0.0278125) * t928;
            let t980 = t979 * t412;
            let t982 = t408 * t408;
            let t983 = f64x8::splat(1.0) / t982;
            let t984 = t403 * t983;
            let t986 = f64x8::splat(0.301925) * t921;
            let t989 = f64x8::splat(0.082785) * t947;
            let t991 = f64x8::splat(2.58925) * t939 - t986 + f64x8::splat(0.905775) * t928 + f64x8::splat(0.16504875) * t944 - t989 + f64x8::splat(0.248355) * t951;
            let t992 = f64x8::splat(1.0) / t411;
            let t993 = t991 * t992;
            let t1000 = t298 * t403;
            let t1002 = t983 * t991 * t992;
            let t1005 = -t932 + t957 + t298 * (-f64x8::splat(0.03109) * t960 * t399 + f64x8::splat(1.0) * t965 * t974 + t932 - t957 - f64x8::splat(0.019751789702565206) * t980 + f64x8::splat(0.5848223397455204) * t984 * t993) + f64x8::splat(0.019751789702565206) * t298 * t980 - f64x8::splat(0.5848223397455204) * t1000 * t1002;
            let t1009 = ((t150).select(f64x8::splat(0.0), t365 * t1005 / f64x8::splat(2.0) + t918 * t419 / f64x8::splat(2.0)));
            let t1010 = t1009 * t448;
            let t1011 = t14 * t570;
            let t1014 = f64x8::splat(0.0011073577833333333) * t259 * t1011 * t466;
            let t1015 = t462 * t462;
            let t1016 = f64x8::splat(1.0) / t1015;
            let t1017 = t453 * t1016;
            let t1019 = f64x8::splat(1.0) / t454 * t9;
            let t1020 = t15 * t570;
            let t1021 = t1019 * t1020;
            let t1023 = t259 * t1011;
            let t1025 = ((t451).sqrt());
            let t1026 = t1025 * t9;
            let t1027 = t1026 * t1020;
            let t1030 = t275 * t13 * t822;
            let t1032 = -f64x8::splat(0.632975) * t1021 - f64x8::splat(0.29896666666666666) * t1023 - f64x8::splat(0.1023875) * t1027 - f64x8::splat(0.08215666666666667) * t1030;
            let t1033 = f64x8::splat(1.0) / t465;
            let t1034 = t1032 * t1033;
            let t1036 = f64x8::splat(1.0) * t1017 * t1034;
            let t1037 = t469 * t2;
            let t1038 = t1037 * t473;
            let t1040 = f64x8::splat(4.0) * t1038 * t508;
            let t1041 = t472 * t3;
            let t1042 = f64x8::splat(1.0) / t1041;
            let t1043 = t470 * t1042;
            let t1045 = f64x8::splat(4.0) * t1043 * t508;
            let t1048 = ((t7).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t262 * t548));
            let t1051 = ((t149).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t366 * t697));
            let t1053 = (t1048 + t1051) * t297;
            let t1054 = t1053 * t507;
            let t1059 = t487 * t487;
            let t1060 = f64x8::splat(1.0) / t1059;
            let t1061 = t482 * t1060;
            let t1066 = -f64x8::splat(1.176575) * t1021 - f64x8::splat(0.516475) * t1023 - f64x8::splat(0.2103875) * t1027 - f64x8::splat(0.104195) * t1030;
            let t1067 = f64x8::splat(1.0) / t490;
            let t1068 = t1066 * t1067;
            let t1074 = t500 * t500;
            let t1075 = f64x8::splat(1.0) / t1074;
            let t1076 = t495 * t1075;
            let t1081 = -f64x8::splat(0.8630833333333333) * t1021 - f64x8::splat(0.301925) * t1023 - f64x8::splat(0.05501625) * t1027 - f64x8::splat(0.082785) * t1030;
            let t1082 = f64x8::splat(1.0) / t503;
            let t1083 = t1081 * t1082;
            let t1086 = f64x8::splat(0.0005323644333333333) * t259 * t1011 * t491 + f64x8::splat(1.0) * t1061 * t1068 - t1014 - t1036 + f64x8::splat(0.0001831155503675316) * t259 * t1011 * t504 + f64x8::splat(0.5848223397455204) * t1076 * t1083;
            let t1087 = t480 * t1086;
            let t1088 = t474 * t1087;
            let t1091 = t480 * t9;
            let t1093 = t15 * t570 * t504;
            let t1095 = f64x8::splat(0.0001831155503675316) * t1091 * t1093;
            let t1096 = t480 * t495;
            let t1098 = t1075 * t1081 * t1082;
            let t1100 = f64x8::splat(0.5848223397455204) * t1096 * t1098;
            let t1101 = t1014 + t1036 + t1040 - t1045 + t474 * t1054 + t1088 + f64x8::splat(0.019751789702565206) * t1053 * t505 - t1095 - t1100 - t887 - t1009;
            let t1102 = t1101 * t544;
            let t1103 = t514 * v_sigma0;
            let t1107 = t527 * v_sigma0;
            let t1108 = t1107 * t650;
            let t1111 = t523 * t515;
            let t1114 = t534 * v_sigma0;
            let t1115 = t1114 * t650;
            let t1118 = t530 * t524;
            let t1121 = t541 * v_sigma0;
            let t1122 = t1121 * t650;
            let t1125 = t537 * t531;
            let t1129 = f64x8::splat(1.0) / t540 / t519;
            let t1130 = t1129 * v_sigma0;
            let t1134 = -f64x8::splat(0.008) * t1103 * t650 * t520 + f64x8::splat(2.4e-05) * t516 * t1108 - f64x8::splat(4.8e-05) * t1111 * t1108 + f64x8::splat(1.44e-07) * t525 * t1115 - f64x8::splat(2.16e-07) * t1118 * t1115 + f64x8::splat(6.48e-10) * t532 * t1122 - f64x8::splat(8.64e-10) * t1125 * t1122 + f64x8::splat(2.592e-12) * t539 * t1130 * t650;
            let t1135 = t512 * t1134;
            let tvrho0 = t146 + t257 + t364 + t449 + t545 + t3 * (t696 + t792 + t888 + t917 + t1010 + t1102 + t1135);
            acc_vrho_0 = tvrho0;
            let t1138 = -t4 - t547;
            let t1140 = t1138 * t9 * t17;
            let t1143 = t562 * t1138;
            let t1148 = t578 * t581 * t1138;
            let t1151 = -t575 * t1148 / f64x8::splat(54.0) - t573 / f64x8::splat(54.0);
            let t1152 = ((t44).select(t1151, f64x8::splat(0.0)));
            let t1155 = t591 * t1152;
            let t1157 = t595 * t1152;
            let t1159 = t599 * t1152;
            let t1161 = t603 * t1152;
            let t1163 = t607 * t1152;
            let t1165 = t611 * t1152;
            let t1167 = t615 * t1152;
            let t1170 = ((t44).select(f64x8::splat(0.0), t1151));
            let t1182 = t626 * t1170 * t80 / f64x8::splat(2.0) - f64x8::splat(4.0) * t630 * t1170 - t73 * t1170 * t80;
            let t1185 = -t1170 * t621 + f64x8::splat(2.0) * t1170 * t84 + f64x8::splat(2.0) * t1182 * t71;
            let t1189 = ((t43).select(-t568 * t1152 / f64x8::splat(18.0) + t1155 / f64x8::splat(240.0) - t1157 / f64x8::splat(4480.0) + t1159 / f64x8::splat(103680.0) - t1161 / f64x8::splat(2838528.0) + t1163 / f64x8::splat(89456640.0) - t1165 / f64x8::splat(3185049600.0) + t1167 / f64x8::splat(126340300800.0), -f64x8::splat(8.0) / f64x8::splat(3.0) * t1170 * t87 - f64x8::splat(8.0) / f64x8::splat(3.0) * t71 * t1185));
            let t1190 = t28 * t1189;
            let t1191 = t1190 * t141;
            let t1192 = t25 * t1191;
            let t1196 = ((t8).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(64.0) * t1140 * t143 - t556 - t558 * t561 * t1143 / f64x8::splat(64.0) - f64x8::splat(3.0) / f64x8::splat(64.0) * t18 * t1192));
            let t1197 = -t1138;
            let t1199 = t1197 * t9 * t17;
            let t1202 = t710 * t1197;
            let t1207 = t721 * t724 * t1197;
            let t1210 = -t575 * t1207 / f64x8::splat(54.0) - t718 / f64x8::splat(54.0);
            let t1211 = ((t161).select(t1210, f64x8::splat(0.0)));
            let t1214 = t734 * t1211;
            let t1216 = t738 * t1211;
            let t1218 = t742 * t1211;
            let t1220 = t746 * t1211;
            let t1222 = t750 * t1211;
            let t1224 = t754 * t1211;
            let t1226 = t758 * t1211;
            let t1229 = ((t161).select(f64x8::splat(0.0), t1210));
            let t1241 = t769 * t1229 * t196 / f64x8::splat(2.0) - f64x8::splat(4.0) * t773 * t1229 - t189 * t1229 * t196;
            let t1244 = f64x8::splat(2.0) * t1229 * t200 - t1229 * t764 + f64x8::splat(2.0) * t1241 * t188;
            let t1248 = ((t160).select(-t716 * t1211 / f64x8::splat(18.0) + t1214 / f64x8::splat(240.0) - t1216 / f64x8::splat(4480.0) + t1218 / f64x8::splat(103680.0) - t1220 / f64x8::splat(2838528.0) + t1222 / f64x8::splat(89456640.0) - t1224 / f64x8::splat(3185049600.0) + t1226 / f64x8::splat(126340300800.0), -f64x8::splat(8.0) / f64x8::splat(3.0) * t1229 * t203 - f64x8::splat(8.0) / f64x8::splat(3.0) * t188 * t1244));
            let t1249 = t155 * t1248;
            let t1250 = t1249 * t252;
            let t1251 = t25 * t1250;
            let t1254 = t210 * v_rho1;
            let t1256 = f64x8::splat(1.0) / t212 / t1254;
            let t1260 = t94 * t222;
            let t1261 = t224 * t210;
            let t1263 = f64x8::splat(1.0) / t211 / t1261;
            let t1264 = t1263 * t229;
            let t1269 = t108 * t233;
            let t1270 = t235 * v_rho1;
            let t1271 = f64x8::splat(1.0) / t1270;
            let t1272 = t1271 * t238;
            let t1277 = t120 * t242;
            let t1278 = t235 * t1254;
            let t1280 = f64x8::splat(1.0) / t212 / t1278;
            let t1281 = t1280 * t248;
            let t1286 = t242 * v_sigma2;
            let t1287 = t130 * t1286;
            let t1288 = t235 * t1261;
            let t1290 = f64x8::splat(1.0) / t211 / t1288;
            let t1292 = f64x8::splat(1.0) / t247 / t217;
            let t1293 = t1290 * t1292;
            let t1296 = -f64x8::splat(0.010666666666666666) * t209 * t1256 * t218 + f64x8::splat(4.266666666666667e-05) * t1260 * t1264 - f64x8::splat(8.533333333333334e-05) * t223 * t1264 + f64x8::splat(3.413333333333333e-07) * t1269 * t1272 - f64x8::splat(5.12e-07) * t234 * t1272 + f64x8::splat(2.048e-09) * t1277 * t1281 - f64x8::splat(2.7306666666666667e-09) * t243 * t1281 + f64x8::splat(1.0922666666666666e-11) * t1287 * t1293;
            let t1297 = t208 * t1296;
            let t1298 = t25 * t1297;
            let t1302 = ((t150).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(64.0) * t1199 * t254 - t704 - t706 * t709 * t1202 / f64x8::splat(64.0) - f64x8::splat(3.0) / f64x8::splat(64.0) * t152 * t1251 - f64x8::splat(3.0) / f64x8::splat(64.0) * t152 * t1298));
            let t1303 = ((t7).select(f64x8::splat(0.0), t1138));
            let t1307 = ((t7).select(f64x8::splat(0.0), -t798 * t1138 / f64x8::splat(3.0)));
            let t1309 = t260 * t36 * t1307;
            let t1311 = -t797 + f64x8::splat(0.053425) * t1309;
            let t1313 = f64x8::splat(0.062182) * t1311 * t289;
            let t1314 = -t812 + t1309;
            let t1315 = t811 * t1314;
            let t1318 = t818 * t1314;
            let t1320 = t264 * t1307;
            let t1322 = t276 * t280 * t1320;
            let t1324 = f64x8::splat(1.898925) * t1315 - t816 + f64x8::splat(0.8969) * t1309 + f64x8::splat(0.3071625) * t1318 - t826 + f64x8::splat(0.24647) * t1322;
            let t1325 = t1324 * t832;
            let t1327 = f64x8::splat(1.0) * t810 * t1325;
            let t1329 = -t836 + f64x8::splat(0.05137) * t1309;
            let t1336 = f64x8::splat(3.529725) * t1315 - t845 + f64x8::splat(1.549425) * t1309 + f64x8::splat(0.6311625) * t1318 - t848 + f64x8::splat(0.312585) * t1322;
            let t1337 = t1336 * t851;
            let t1341 = -t855 + f64x8::splat(0.0278125) * t1309;
            let t1342 = t1341 * t322;
            let t1348 = f64x8::splat(2.58925) * t1315 - t864 + f64x8::splat(0.905775) * t1309 + f64x8::splat(0.16504875) * t1318 - t867 + f64x8::splat(0.248355) * t1322;
            let t1349 = t1348 * t870;
            let t1357 = t861 * t1348 * t870;
            let t1360 = -t1313 + t1327 + t298 * (-f64x8::splat(0.03109) * t1329 * t309 + f64x8::splat(1.0) * t843 * t1337 + t1313 - t1327 - f64x8::splat(0.019751789702565206) * t1342 + f64x8::splat(0.5848223397455204) * t862 * t1349) + f64x8::splat(0.019751789702565206) * t298 * t1342 - f64x8::splat(0.5848223397455204) * t878 * t1357;
            let t1364 = ((t8).select(f64x8::splat(0.0), t1303 * t329 / f64x8::splat(2.0) + t258 * t1360 / f64x8::splat(2.0)));
            let t1365 = t1364 * t363;
            let t1366 = ((t149).select(f64x8::splat(0.0), t1197));
            let t1370 = ((t149).select(f64x8::splat(0.0), -t923 * t1197 / f64x8::splat(3.0)));
            let t1372 = t260 * t36 * t1370;
            let t1374 = -t922 + f64x8::splat(0.053425) * t1372;
            let t1376 = f64x8::splat(0.062182) * t1374 * t386;
            let t1377 = -t937 + t1372;
            let t1378 = t936 * t1377;
            let t1381 = t943 * t1377;
            let t1383 = t368 * t1370;
            let t1385 = t276 * t280 * t1383;
            let t1387 = f64x8::splat(1.898925) * t1378 - t941 + f64x8::splat(0.8969) * t1372 + f64x8::splat(0.3071625) * t1381 - t948 + f64x8::splat(0.24647) * t1385;
            let t1388 = t1387 * t954;
            let t1390 = f64x8::splat(1.0) * t935 * t1388;
            let t1392 = -t958 + f64x8::splat(0.05137) * t1372;
            let t1399 = f64x8::splat(3.529725) * t1378 - t967 + f64x8::splat(1.549425) * t1372 + f64x8::splat(0.6311625) * t1381 - t970 + f64x8::splat(0.312585) * t1385;
            let t1400 = t1399 * t973;
            let t1404 = -t977 + f64x8::splat(0.0278125) * t1372;
            let t1405 = t1404 * t412;
            let t1411 = f64x8::splat(2.58925) * t1378 - t986 + f64x8::splat(0.905775) * t1372 + f64x8::splat(0.16504875) * t1381 - t989 + f64x8::splat(0.248355) * t1385;
            let t1412 = t1411 * t992;
            let t1420 = t983 * t1411 * t992;
            let t1423 = -t1376 + t1390 + t298 * (-f64x8::splat(0.03109) * t1392 * t399 + f64x8::splat(1.0) * t965 * t1400 + t1376 - t1390 - f64x8::splat(0.019751789702565206) * t1405 + f64x8::splat(0.5848223397455204) * t984 * t1412) + f64x8::splat(0.019751789702565206) * t298 * t1405 - f64x8::splat(0.5848223397455204) * t1000 * t1420;
            let t1427 = ((t150).select(f64x8::splat(0.0), t1366 * t419 / f64x8::splat(2.0) + t365 * t1423 / f64x8::splat(2.0)));
            let t1428 = t1427 * t448;
            let t1432 = t334 * t222;
            let t1433 = t1263 * t432;
            let t1438 = t342 * t233;
            let t1439 = t1271 * t438;
            let t1444 = t349 * t242;
            let t1445 = t1280 * t444;
            let t1450 = t356 * t1286;
            let t1452 = f64x8::splat(1.0) / t443 / t425;
            let t1453 = t1290 * t1452;
            let t1456 = -f64x8::splat(0.5333333333333333) * t423 * t1256 * t426 + f64x8::splat(0.10666666666666667) * t1432 * t1433 - f64x8::splat(0.21333333333333335) * t430 * t1433 + f64x8::splat(0.042666666666666665) * t1438 * t1439 - f64x8::splat(0.064) * t436 * t1439 + f64x8::splat(0.0128) * t1444 * t1445 - f64x8::splat(0.017066666666666667) * t442 * t1445 + f64x8::splat(0.0034133333333333333) * t1450 * t1453;
            let t1457 = t422 * t1456;
            let t1460 = ((t7).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t262 * t1138));
            let t1463 = ((t149).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t366 * t1197));
            let t1465 = (t1460 + t1463) * t297;
            let t1466 = t1465 * t507;
            let t1470 = t1014 + t1036 - t1040 - t1045 + t474 * t1466 + t1088 + f64x8::splat(0.019751789702565206) * t1465 * t505 - t1095 - t1100 - t1364 - t1427;
            let t1471 = t1470 * t544;
            let t1472 = t514 * v_sigma2;
            let t1476 = t527 * v_sigma2;
            let t1477 = t1476 * t1256;
            let t1482 = t534 * v_sigma2;
            let t1483 = t1482 * t1256;
            let t1488 = t541 * v_sigma2;
            let t1489 = t1488 * t1256;
            let t1494 = t1129 * v_sigma2;
            let t1498 = -f64x8::splat(0.008) * t1472 * t1256 * t520 + f64x8::splat(2.4e-05) * t516 * t1477 - f64x8::splat(4.8e-05) * t1111 * t1477 + f64x8::splat(1.44e-07) * t525 * t1483 - f64x8::splat(2.16e-07) * t1118 * t1483 + f64x8::splat(6.48e-10) * t532 * t1489 - f64x8::splat(8.64e-10) * t1125 * t1489 + f64x8::splat(2.592e-12) * t539 * t1494 * t1256;
            let t1499 = t512 * t1498;
            let tvrho1 = t146 + t257 + t364 + t449 + t545 + t3 * (t1196 + t1302 + t1365 + t1428 + t1457 + t1471 + t1499);
            acc_vrho_1 = tvrho1;
            let t1507 = t108 * v_sigma0;
            let t1512 = t120 * t109;
            let t1517 = t130 * t121;
            let t1520 = t123 * t112;
            let t1522 = f64x8::splat(1.0) / t97 / t1520;
            let t1523 = t1522 * t686;
            let t1526 = f64x8::splat(0.004) * t94 * t100 * t104 - f64x8::splat(1.6e-05) * t95 * t117 + f64x8::splat(3.2e-05) * t1507 * t117 - f64x8::splat(1.28e-07) * t110 * t127 + f64x8::splat(1.92e-07) * t1512 * t127 - f64x8::splat(7.68e-10) * t122 * t138 + f64x8::splat(1.024e-09) * t1517 * t138 - f64x8::splat(4.096e-12) * t132 * t1523;
            let t1527 = t92 * t1526;
            let t1528 = t25 * t1527;
            let t1531 = ((t8).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(64.0) * t18 * t1528));
            let t1537 = t342 * v_sigma0;
            let t1542 = t349 * t109;
            let t1547 = t356 * t121;
            let t1550 = t1522 * t912;
            let t1553 = f64x8::splat(0.2) * t334 * t100 * t338 - f64x8::splat(0.04) * t335 * t346 + f64x8::splat(0.08) * t1537 * t346 - f64x8::splat(0.016) * t343 * t353 + f64x8::splat(0.024) * t1542 * t353 - f64x8::splat(0.0048) * t350 * t360 + f64x8::splat(0.0064) * t1547 * t360 - f64x8::splat(0.00128) * t357 * t1550;
            let t1554 = t332 * t1553;
            let t1555 = t514 * t100;
            let t1558 = t527 * t100;
            let t1563 = t534 * t100;
            let t1568 = t541 * t100;
            let t1576 = f64x8::splat(0.003) * t1555 * t520 - f64x8::splat(9e-06) * t516 * t1558 + f64x8::splat(1.8e-05) * t1111 * t1558 - f64x8::splat(5.4e-08) * t525 * t1563 + f64x8::splat(8.1e-08) * t1118 * t1563 - f64x8::splat(2.43e-10) * t532 * t1568 + f64x8::splat(3.24e-10) * t1125 * t1568 - f64x8::splat(9.72e-13) * t539 * t1129 * t100;
            let t1577 = t512 * t1576;
            let tvsigma0 = t3 * (t1531 + t1554 + t1577);
            acc_vsigma_0 = tvsigma0;
            let tvsigma1 = f64x8::splat(0.0);
            acc_vsigma_1 = tvsigma1;
            let t1584 = t108 * v_sigma2;
            let t1589 = t120 * t222;
            let t1594 = t130 * t233;
            let t1597 = t235 * t225;
            let t1599 = f64x8::splat(1.0) / t211 / t1597;
            let t1600 = t1599 * t1292;
            let t1603 = f64x8::splat(0.004) * t94 * t214 * t218 - f64x8::splat(1.6e-05) * t209 * t230 + f64x8::splat(3.2e-05) * t1584 * t230 - f64x8::splat(1.28e-07) * t223 * t239 + f64x8::splat(1.92e-07) * t1589 * t239 - f64x8::splat(7.68e-10) * t234 * t249 + f64x8::splat(1.024e-09) * t1594 * t249 - f64x8::splat(4.096e-12) * t243 * t1600;
            let t1604 = t208 * t1603;
            let t1605 = t25 * t1604;
            let t1608 = ((t150).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(64.0) * t152 * t1605));
            let t1614 = t342 * v_sigma2;
            let t1619 = t349 * t222;
            let t1624 = t356 * t233;
            let t1627 = t1599 * t1452;
            let t1630 = f64x8::splat(0.2) * t334 * t214 * t426 - f64x8::splat(0.04) * t423 * t433 + f64x8::splat(0.08) * t1614 * t433 - f64x8::splat(0.016) * t430 * t439 + f64x8::splat(0.024) * t1619 * t439 - f64x8::splat(0.0048) * t436 * t445 + f64x8::splat(0.0064) * t1624 * t445 - f64x8::splat(0.00128) * t442 * t1627;
            let t1631 = t422 * t1630;
            let t1632 = t514 * t214;
            let t1635 = t527 * t214;
            let t1640 = t534 * t214;
            let t1645 = t541 * t214;
            let t1650 = t1129 * t214;
            let t1653 = f64x8::splat(0.003) * t1632 * t520 - f64x8::splat(9e-06) * t516 * t1635 + f64x8::splat(1.8e-05) * t1111 * t1635 - f64x8::splat(5.4e-08) * t525 * t1640 + f64x8::splat(8.1e-08) * t1118 * t1640 - f64x8::splat(2.43e-10) * t532 * t1645 + f64x8::splat(3.24e-10) * t1125 * t1645 - f64x8::splat(9.72e-13) * t539 * t1650;
            let t1654 = t512 * t1653;
            let tvsigma2 = t3 * (t1608 + t1631 + t1654);
            acc_vsigma_2 = tvsigma2;
        }
        store_add(zk, ip, m, acc_zk);
        store_strided(vrho, ip, m, 2, 0, acc_vrho_0);
        store_strided(vrho, ip, m, 2, 1, acc_vrho_1);
        store_strided(vsigma, ip, m, 3, 0, acc_vsigma_0);
        store_strided(vsigma, ip, m, 3, 1, acc_vsigma_1);
        store_strided(vsigma, ip, m, 3, 2, acc_vsigma_2);
        ip += 8;
    }
}
