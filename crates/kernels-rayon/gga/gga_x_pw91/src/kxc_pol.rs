//! GGA_X_PW91 kxc pol kernel — explicit SIMD (bit-exact).
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
pub fn gga_x_pw91_kxc_pol(
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
            let t364 = t23 * t23;
            let t365 = f64x8::splat(1.0) / t364;
            let t366 = t136 * t136;
            let t369 = t132 * t6;
            let t370 = f64x8::splat(1.0) / t369;
            let t371 = t16 * t370;
            let t374 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), -f64x8::splat(2.0) * t133 + f64x8::splat(2.0) * t371)));
            let t378 = ((t20).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(9.0) * t365 * t366 + f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t374));
            let t379 = t378 * t26;
            let t383 = t139 * t145;
            let t385 = t5 * t383 * t75;
            let t391 = f64x8::splat(1.0) / t144 / t6;
            let t392 = t25 * t391;
            let t395 = t5 * t392 * t75 / f64x8::splat(12.0);
            let t397 = t5 * t146 * t201;
            let t399 = t156 * t164;
            let t401 = f64x8::splat(1.0) / t36 / t399;
            let t406 = param_alpha * param_alpha;
            let t407 = param_d * t406;
            let t408 = t30 * t30;
            let t409 = f64x8::splat(1.0) / t408;
            let t410 = t407 * t409;
            let t411 = t154 * v_sigma0;
            let t412 = t156 * t156;
            let t413 = t412 * t35;
            let t414 = f64x8::splat(1.0) / t413;
            let t420 = f64x8::splat(1.0) / t37 / t156;
            let t424 = param_expo * param_expo;
            let t425 = f64x8::splat(1.0) / t35;
            let t426 = t424 * t425;
            let t428 = f64x8::splat(16.0) / f64x8::splat(9.0) * t59 * t426;
            let t431 = f64x8::splat(4.0) / f64x8::splat(3.0) * t59 * param_expo * t425;
            let t432 = -t151 * t155 * t401 * t43 / f64x8::splat(24.0) + t410 * t411 * t414 * t43 / f64x8::splat(324.0) + f64x8::splat(11.0) / f64x8::splat(27.0) * t46 * t34 * t420 - t428 - t431;
            let t434 = t174 * t177;
            let t438 = f64x8::splat(1.0) / t176 / t72;
            let t439 = t60 * t438;
            let t440 = t199 * t199;
            let t446 = f64x8::splat(1.0) / t36 / t164 * param_a * t68;
            let t450 = t420 * param_a * t195;
            let t453 = t49 * t153;
            let t454 = t453 * t154;
            let t456 = t188 * param_b;
            let t458 = f64x8::splat(1.0) / t193 / t192;
            let t459 = t456 * t458;
            let t463 = f64x8::splat(7.0) / f64x8::splat(27.0) * t61 * t446 + f64x8::splat(10.0) / f64x8::splat(3.0) * t186 * t450 - f64x8::splat(16.0) / f64x8::splat(3.0) * t454 * t401 * param_a * t459 + t428 + t431;
            let t465 = -t178 * t463 - f64x8::splat(2.0) * t434 * t199 + t432 * t73 + f64x8::splat(2.0) * t439 * t440;
            let t470 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t379 * t75 - t385 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(4.0) * t5 * t140 * t201 + t395 - t397 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t27 * t465));
            let t471 = t86 * t86;
            let t472 = f64x8::splat(1.0) / t471;
            let t473 = t209 * t209;
            let t476 = t81 * t370;
            let t479 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), f64x8::splat(2.0) * t133 + f64x8::splat(2.0) * t476)));
            let t483 = ((t85).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(9.0) * t472 * t473 + f64x8::splat(4.0) / f64x8::splat(3.0) * t86 * t479));
            let t484 = t483 * t26;
            let t488 = t212 * t145;
            let t490 = t5 * t488 * t127;
            let t492 = t88 * t391;
            let t495 = t5 * t492 * t127 / f64x8::splat(12.0);
            let t497 = ((t80).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t484 * t127 - t490 / f64x8::splat(4.0) + t495));
            let tv2rho20 = f64x8::splat(2.0) * t206 + f64x8::splat(2.0) * t222 + t6 * (t470 + t497);
            acc_v2rho2_0 = tv2rho20;
            let t500 = t365 * t226;
            let t504 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), f64x8::splat(2.0) * t371)));
            let t508 = ((t20).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(9.0) * t500 * t136 + f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t504));
            let t509 = t508 * t26;
            let t513 = t229 * t145;
            let t515 = t5 * t513 * t75;
            let t523 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t509 * t75 - t515 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t230 * t201 - t385 / f64x8::splat(8.0) + t395 - t397 / f64x8::splat(8.0)));
            let t524 = t472 * t237;
            let t528 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), f64x8::splat(2.0) * t476)));
            let t532 = ((t85).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(9.0) * t524 * t209 + f64x8::splat(4.0) / f64x8::splat(3.0) * t86 * t528));
            let t533 = t532 * t26;
            let t537 = t240 * t145;
            let t539 = t5 * t537 * t127;
            let t546 = t5 * t217 * t289;
            let t549 = ((t80).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t533 * t127 - t539 / f64x8::splat(8.0) - t490 / f64x8::splat(8.0) + t495 - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t213 * t289 - t546 / f64x8::splat(8.0)));
            let tv2rho21 = t206 + t222 + t235 + t294 + t6 * (t523 + t549);
            acc_v2rho2_1 = tv2rho21;
            let t554 = t226 * t226;
            let t559 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), f64x8::splat(2.0) * t133 + f64x8::splat(2.0) * t371)));
            let t563 = ((t20).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(9.0) * t365 * t554 + f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t559));
            let t564 = t563 * t26;
            let t570 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t564 * t75 - t515 / f64x8::splat(4.0) + t395));
            let t571 = t237 * t237;
            let t576 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), -f64x8::splat(2.0) * t133 + f64x8::splat(2.0) * t476)));
            let t580 = ((t85).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(9.0) * t472 * t571 + f64x8::splat(4.0) / f64x8::splat(3.0) * t86 * t576));
            let t581 = t580 * t26;
            let t590 = t247 * t255;
            let t592 = f64x8::splat(1.0) / t92 / t590;
            let t597 = t245 * v_sigma2;
            let t598 = t247 * t247;
            let t599 = t598 * t91;
            let t600 = f64x8::splat(1.0) / t599;
            let t606 = f64x8::splat(1.0) / t93 / t247;
            let t610 = f64x8::splat(1.0) / t91;
            let t611 = t424 * t610;
            let t613 = f64x8::splat(16.0) / f64x8::splat(9.0) * t112 * t611;
            let t616 = f64x8::splat(4.0) / f64x8::splat(3.0) * t112 * param_expo * t610;
            let t617 = -t151 * t246 * t592 * t99 / f64x8::splat(24.0) + t410 * t597 * t600 * t99 / f64x8::splat(324.0) + f64x8::splat(11.0) / f64x8::splat(27.0) * t102 * t90 * t606 - t613 - t616;
            let t619 = t265 * t268;
            let t623 = f64x8::splat(1.0) / t267 / t124;
            let t624 = t113 * t623;
            let t625 = t287 * t287;
            let t631 = f64x8::splat(1.0) / t92 / t255 * param_a * t120;
            let t635 = t606 * param_a * t283;
            let t638 = t453 * t245;
            let t641 = f64x8::splat(1.0) / t281 / t280;
            let t642 = t456 * t641;
            let t646 = f64x8::splat(7.0) / f64x8::splat(27.0) * t114 * t631 + f64x8::splat(10.0) / f64x8::splat(3.0) * t276 * t635 - f64x8::splat(16.0) / f64x8::splat(3.0) * t638 * t592 * param_a * t642 + t613 + t616;
            let t648 = t617 * t125 - t269 * t646 - f64x8::splat(2.0) * t619 * t287 + f64x8::splat(2.0) * t624 * t625;
            let t653 = ((t80).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t581 * t127 - t539 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(4.0) * t5 * t241 * t289 + t495 - t546 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t89 * t648));
            let tv2rho22 = f64x8::splat(2.0) * t235 + f64x8::splat(2.0) * t294 + t6 * (t570 + t653);
            acc_v2rho2_2 = tv2rho22;
            let t661 = t5 * t146 * t325 / f64x8::splat(8.0);
            let t662 = t153 * t159;
            let t666 = t412 * v_rho0;
            let t667 = f64x8::splat(1.0) / t666;
            let t675 = t424 * t170;
            let t678 = f64x8::splat(2.0) / f64x8::splat(3.0) * t59 * t675 * t308;
            let t679 = t151 * t662 * t301 / f64x8::splat(72.0) - t410 * t667 * t154 * t43 / f64x8::splat(864.0) - t46 * t33 * t166 / f64x8::splat(9.0) + t678;
            let t681 = t312 * t177;
            let t684 = t323 * t199;
            let t691 = t453 * t159;
            let t692 = param_a * t456;
            let t694 = t692 * t458 * v_sigma0;
            let t697 = -t315 * t182 / f64x8::splat(18.0) - t185 * t166 * t320 + f64x8::splat(2.0) * t691 * t694 - t678;
            let t699 = -t178 * t697 - t681 * t199 - t434 * t323 + f64x8::splat(2.0) * t439 * t684 + t679 * t73;
            let t704 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t140 * t325 - t661 - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t27 * t699));
            let tv2rhosigma0 = t6 * t704 + t329;
            acc_v2rhosigma_0 = tv2rhosigma0;
            let tv2rhosigma1 = f64x8::splat(0.0);
            acc_v2rhosigma_1 = tv2rhosigma1;
            let t711 = t5 * t217 * t357 / f64x8::splat(8.0);
            let t713 = ((t80).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t213 * t357 - t711));
            let tv2rhosigma2 = t6 * t713 + t361;
            acc_v2rhosigma_2 = tv2rhosigma2;
            let t719 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t230 * t325 - t661));
            let tv2rhosigma3 = t6 * t719 + t329;
            acc_v2rhosigma_3 = tv2rhosigma3;
            let tv2rhosigma4 = f64x8::splat(0.0);
            acc_v2rhosigma_4 = tv2rhosigma4;
            let t724 = t153 * t250;
            let t728 = t598 * v_rho1;
            let t729 = f64x8::splat(1.0) / t728;
            let t737 = t424 * t261;
            let t740 = f64x8::splat(2.0) / f64x8::splat(3.0) * t112 * t737 * t341;
            let t741 = t151 * t724 * t334 / f64x8::splat(72.0) - t410 * t729 * t245 * t99 / f64x8::splat(864.0) - t102 * t33 * t257 / f64x8::splat(9.0) + t740;
            let t743 = t345 * t268;
            let t746 = t355 * t287;
            let t753 = t453 * t250;
            let t755 = t692 * t641 * v_sigma2;
            let t758 = -t348 * t273 / f64x8::splat(18.0) - t185 * t257 * t352 + f64x8::splat(2.0) * t753 * t755 - t740;
            let t760 = t741 * t125 - t269 * t758 - t743 * t287 - t619 * t355 + f64x8::splat(2.0) * t624 * t746;
            let t765 = ((t80).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t241 * t357 - t711 - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t89 * t760));
            let tv2rhosigma5 = t6 * t765 + t361;
            acc_v2rhosigma_5 = tv2rhosigma5;
            let t767 = f64x8::splat(1.0) / t412;
            let t775 = f64x8::splat(1.0) / t154;
            let t778 = t59 * t424 * t775 / f64x8::splat(4.0);
            let t781 = t59 * param_expo * t775 / f64x8::splat(2.0);
            let t782 = t410 * t767 * t43 * v_sigma0 / f64x8::splat(2304.0) - t151 * t300 * t43 / f64x8::splat(288.0) - t778 + t781;
            let t786 = t323 * t323;
            let t791 = t51 / t52 / v_sigma0;
            let t794 = t185 * t308;
            let t796 = t39 * param_a * t195;
            let t800 = t692 * t458;
            let t803 = -t791 * t69 / f64x8::splat(48.0) + t794 * t796 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(4.0) * t453 * t299 * t800 + t778 - t781;
            let t805 = -t178 * t803 - f64x8::splat(2.0) * t681 * t323 + f64x8::splat(2.0) * t439 * t786 + t782 * t73;
            let t809 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t27 * t805));
            let tv2sigma20 = t6 * t809;
            acc_v2sigma2_0 = tv2sigma20;
            let tv2sigma21 = f64x8::splat(0.0);
            acc_v2sigma2_1 = tv2sigma21;
            let tv2sigma22 = f64x8::splat(0.0);
            acc_v2sigma2_2 = tv2sigma22;
            let tv2sigma23 = f64x8::splat(0.0);
            acc_v2sigma2_3 = tv2sigma23;
            let tv2sigma24 = f64x8::splat(0.0);
            acc_v2sigma2_4 = tv2sigma24;
            let t810 = f64x8::splat(1.0) / t598;
            let t818 = f64x8::splat(1.0) / t245;
            let t821 = t112 * t424 * t818 / f64x8::splat(4.0);
            let t824 = t112 * param_expo * t818 / f64x8::splat(2.0);
            let t825 = t410 * t810 * t99 * v_sigma2 / f64x8::splat(2304.0) - t151 * t333 * t99 / f64x8::splat(288.0) - t821 + t824;
            let t829 = t355 * t355;
            let t834 = t51 / t105 / v_sigma2;
            let t837 = t185 * t341;
            let t839 = t95 * param_a * t283;
            let t843 = t692 * t641;
            let t846 = -t834 * t121 / f64x8::splat(48.0) + t837 * t839 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(4.0) * t453 * t332 * t843 + t821 - t824;
            let t848 = t825 * t125 - t269 * t846 - f64x8::splat(2.0) * t743 * t355 + f64x8::splat(2.0) * t624 * t829;
            let t852 = ((t80).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t89 * t848));
            let tv2sigma25 = t6 * t852;
            acc_v2sigma2_5 = tv2sigma25;
            let t856 = f64x8::splat(1.0) / t364 / t19;
            let t857 = t366 * t136;
            let t860 = t365 * t136;
            let t863 = t132 * t132;
            let t864 = f64x8::splat(1.0) / t863;
            let t865 = t16 * t864;
            let t868 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), f64x8::splat(6.0) * t370 - f64x8::splat(6.0) * t865)));
            let t872 = ((t20).select(f64x8::splat(0.0), -f64x8::splat(8.0) / f64x8::splat(27.0) * t856 * t857 + f64x8::splat(4.0) / f64x8::splat(3.0) * t860 * t374 + f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t868));
            let t873 = t872 * t26;
            let t877 = t378 * t145;
            let t879 = t5 * t877 * t75;
            let t884 = t139 * t391;
            let t886 = t5 * t884 * t75;
            let t889 = t5 * t383 * t201;
            let t895 = f64x8::splat(1.0) / t144 / t132;
            let t896 = t25 * t895;
            let t899 = f64x8::splat(5.0) / f64x8::splat(36.0) * t5 * t896 * t75;
            let t901 = t5 * t392 * t201;
            let t904 = t5 * t146 * t465;
            let t907 = f64x8::splat(1.0) / t36 / t412;
            let t912 = t412 * t164;
            let t913 = f64x8::splat(1.0) / t912;
            let t919 = param_d * t406 * param_alpha;
            let t920 = t154 * t154;
            let t921 = t409 * t920;
            let t922 = t919 * t921;
            let t923 = t412 * t297;
            let t925 = f64x8::splat(1.0) / t37 / t923;
            let t927 = t33 * t43;
            let t932 = f64x8::splat(1.0) / t37 / t297;
            let t936 = t424 * param_expo;
            let t937 = f64x8::splat(1.0) / t164;
            let t938 = t936 * t937;
            let t940 = f64x8::splat(64.0) / f64x8::splat(27.0) * t59 * t938;
            let t941 = t424 * t937;
            let t943 = f64x8::splat(16.0) / f64x8::splat(3.0) * t59 * t941;
            let t946 = f64x8::splat(8.0) / f64x8::splat(3.0) * t59 * param_expo * t937;
            let t947 = f64x8::splat(341.0) / f64x8::splat(972.0) * t151 * t155 * t907 * t43 - f64x8::splat(19.0) / f64x8::splat(324.0) * t410 * t411 * t913 * t43 + t922 * t925 * t28 * t927 / f64x8::splat(2916.0) - f64x8::splat(154.0) / f64x8::splat(81.0) * t46 * t34 * t932 + t940 + t943 + t946;
            let t949 = t432 * t177;
            let t952 = t174 * t438;
            let t957 = t176 * t176;
            let t958 = f64x8::splat(1.0) / t957;
            let t959 = t60 * t958;
            let t960 = t440 * t199;
            let t963 = t199 * t463;
            let t969 = f64x8::splat(1.0) / t36 / t156 * param_a * t68;
            let t980 = t409 * t411;
            let t982 = t188 * t188;
            let t983 = t982 * param_b;
            let t984 = param_a * t983;
            let t985 = t192 * t192;
            let t987 = f64x8::splat(1.0) / t193 / t985;
            let t988 = t984 * t987;
            let t991 = -f64x8::splat(70.0) / f64x8::splat(81.0) * t61 * t969 - f64x8::splat(476.0) / f64x8::splat(27.0) * t186 * t932 * param_a * t195 + f64x8::splat(592.0) / f64x8::splat(9.0) * t454 * t907 * param_a * t459 - f64x8::splat(768.0) * t980 * t913 * t988 - t940 - t943 - t946;
            let t993 = -t178 * t991 - f64x8::splat(3.0) * t949 * t199 - f64x8::splat(3.0) * t434 * t463 + f64x8::splat(6.0) * t439 * t963 + f64x8::splat(6.0) * t952 * t440 + t947 * t73 - f64x8::splat(6.0) * t959 * t960;
            let t998 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t873 * t75 - f64x8::splat(3.0) / f64x8::splat(8.0) * t879 - f64x8::splat(9.0) / f64x8::splat(8.0) * t5 * t379 * t201 + t886 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(4.0) * t889 - f64x8::splat(9.0) / f64x8::splat(8.0) * t5 * t140 * t465 - t899 + t901 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t904 - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t27 * t993));
            let t1000 = f64x8::splat(1.0) / t471 / t84;
            let t1001 = t473 * t209;
            let t1004 = t472 * t209;
            let t1007 = t81 * t864;
            let t1010 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), -f64x8::splat(6.0) * t370 - f64x8::splat(6.0) * t1007)));
            let t1014 = ((t85).select(f64x8::splat(0.0), -f64x8::splat(8.0) / f64x8::splat(27.0) * t1000 * t1001 + f64x8::splat(4.0) / f64x8::splat(3.0) * t1004 * t479 + f64x8::splat(4.0) / f64x8::splat(3.0) * t86 * t1010));
            let t1015 = t1014 * t26;
            let t1019 = t483 * t145;
            let t1021 = t5 * t1019 * t127;
            let t1023 = t212 * t391;
            let t1025 = t5 * t1023 * t127;
            let t1027 = t88 * t895;
            let t1030 = f64x8::splat(5.0) / f64x8::splat(36.0) * t5 * t1027 * t127;
            let t1032 = ((t80).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t1015 * t127 - f64x8::splat(3.0) / f64x8::splat(8.0) * t1021 + t1025 / f64x8::splat(4.0) - t1030));
            let tv3rho30 = f64x8::splat(3.0) * t470 + f64x8::splat(3.0) * t497 + t6 * (t998 + t1032);
            acc_v3rho3_0 = tv3rho30;
            let t1035 = f64x8::splat(2.0) * t523;
            let t1036 = f64x8::splat(2.0) * t549;
            let t1037 = t856 * t226;
            let t1040 = t365 * t504;
            let t1045 = f64x8::splat(2.0) * t370;
            let t1046 = f64x8::splat(6.0) * t865;
            let t1048 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), t1045 - t1046)));
            let t1052 = ((t20).select(f64x8::splat(0.0), -f64x8::splat(8.0) / f64x8::splat(27.0) * t1037 * t366 + f64x8::splat(8.0) / f64x8::splat(9.0) * t1040 * t136 + f64x8::splat(4.0) / f64x8::splat(9.0) * t500 * t374 + f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t1048));
            let t1053 = t1052 * t26;
            let t1057 = t508 * t145;
            let t1060 = t5 * t1057 * t75 / f64x8::splat(4.0);
            let t1064 = t229 * t391;
            let t1066 = t5 * t1064 * t75;
            let t1070 = t5 * t513 * t201 / f64x8::splat(4.0);
            let t1079 = -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t1053 * t75 - t1060 - f64x8::splat(3.0) / f64x8::splat(4.0) * t5 * t509 * t201 + t1066 / f64x8::splat(12.0) - t1070 - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t230 * t465 - t879 / f64x8::splat(8.0) + t886 / f64x8::splat(6.0) - t889 / f64x8::splat(4.0) - t899 + t901 / f64x8::splat(6.0) - t904 / f64x8::splat(8.0);
            let t1080 = ((t1).select(f64x8::splat(0.0), t1079));
            let t1081 = t1000 * t237;
            let t1084 = t472 * t528;
            let t1089 = f64x8::splat(6.0) * t1007;
            let t1091 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), -t1045 - t1089)));
            let t1095 = ((t85).select(f64x8::splat(0.0), -f64x8::splat(8.0) / f64x8::splat(27.0) * t1081 * t473 + f64x8::splat(8.0) / f64x8::splat(9.0) * t1084 * t209 + f64x8::splat(4.0) / f64x8::splat(9.0) * t524 * t479 + f64x8::splat(4.0) / f64x8::splat(3.0) * t86 * t1091));
            let t1096 = t1095 * t26;
            let t1100 = t532 * t145;
            let t1103 = t5 * t1100 * t127 / f64x8::splat(4.0);
            let t1104 = t240 * t391;
            let t1106 = t5 * t1104 * t127;
            let t1115 = t5 * t488 * t289 / f64x8::splat(4.0);
            let t1117 = t5 * t492 * t289;
            let t1120 = ((t80).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t1096 * t127 - t1103 + t1106 / f64x8::splat(12.0) - t1021 / f64x8::splat(8.0) + t1025 / f64x8::splat(6.0) - t1030 - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t484 * t289 - t1115 + t1117 / f64x8::splat(12.0)));
            let tv3rho31 = t470 + t497 + t1035 + t1036 + t6 * (t1080 + t1120);
            acc_v3rho3_1 = tv3rho31;
            let t1123 = t856 * t554;
            let t1128 = t365 * t559;
            let t1132 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), -t1045 - t1046)));
            let t1136 = ((t20).select(f64x8::splat(0.0), -f64x8::splat(8.0) / f64x8::splat(27.0) * t1123 * t136 + f64x8::splat(8.0) / f64x8::splat(9.0) * t500 * t504 + f64x8::splat(4.0) / f64x8::splat(9.0) * t1128 * t136 + f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t1132));
            let t1137 = t1136 * t26;
            let t1141 = t563 * t145;
            let t1143 = t5 * t1141 * t75;
            let t1152 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t1137 * t75 - t1143 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t564 * t201 - t1060 + t1066 / f64x8::splat(6.0) - t1070 + t886 / f64x8::splat(12.0) - t899 + t901 / f64x8::splat(12.0)));
            let t1153 = t1000 * t571;
            let t1158 = t472 * t576;
            let t1162 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), t1045 - t1089)));
            let t1166 = ((t85).select(f64x8::splat(0.0), -f64x8::splat(8.0) / f64x8::splat(27.0) * t1153 * t209 + f64x8::splat(8.0) / f64x8::splat(9.0) * t524 * t528 + f64x8::splat(4.0) / f64x8::splat(9.0) * t1158 * t209 + f64x8::splat(4.0) / f64x8::splat(3.0) * t86 * t1162));
            let t1167 = t1166 * t26;
            let t1171 = t580 * t145;
            let t1173 = t5 * t1171 * t127;
            let t1180 = t5 * t537 * t289;
            let t1188 = t5 * t217 * t648;
            let t1190 = -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t1167 * t127 - t1173 / f64x8::splat(8.0) - t1103 + t1106 / f64x8::splat(6.0) - f64x8::splat(3.0) / f64x8::splat(4.0) * t5 * t533 * t289 - t1180 / f64x8::splat(4.0) + t1025 / f64x8::splat(12.0) - t1030 - t1115 + t1117 / f64x8::splat(6.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t213 * t648 - t1188 / f64x8::splat(8.0);
            let t1191 = ((t80).select(f64x8::splat(0.0), t1190));
            let tv3rho32 = t1035 + t1036 + t570 + t653 + t6 * (t1152 + t1191);
            acc_v3rho3_2 = tv3rho32;
            let t1196 = t554 * t226;
            let t1203 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), -f64x8::splat(6.0) * t370 - f64x8::splat(6.0) * t865)));
            let t1207 = ((t20).select(f64x8::splat(0.0), -f64x8::splat(8.0) / f64x8::splat(27.0) * t856 * t1196 + f64x8::splat(4.0) / f64x8::splat(3.0) * t500 * t559 + f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t1203));
            let t1208 = t1207 * t26;
            let t1215 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t1208 * t75 - f64x8::splat(3.0) / f64x8::splat(8.0) * t1143 + t1066 / f64x8::splat(4.0) - t899));
            let t1216 = t571 * t237;
            let t1223 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), f64x8::splat(6.0) * t370 - f64x8::splat(6.0) * t1007)));
            let t1227 = ((t85).select(f64x8::splat(0.0), -f64x8::splat(8.0) / f64x8::splat(27.0) * t1000 * t1216 + f64x8::splat(4.0) / f64x8::splat(3.0) * t524 * t576 + f64x8::splat(4.0) / f64x8::splat(3.0) * t86 * t1223));
            let t1228 = t1227 * t26;
            let t1244 = f64x8::splat(1.0) / t92 / t598;
            let t1249 = t598 * t255;
            let t1250 = f64x8::splat(1.0) / t1249;
            let t1255 = t245 * t245;
            let t1256 = t409 * t1255;
            let t1257 = t919 * t1256;
            let t1258 = t598 * t330;
            let t1260 = f64x8::splat(1.0) / t93 / t1258;
            let t1262 = t33 * t99;
            let t1267 = f64x8::splat(1.0) / t93 / t330;
            let t1271 = f64x8::splat(1.0) / t255;
            let t1272 = t936 * t1271;
            let t1274 = f64x8::splat(64.0) / f64x8::splat(27.0) * t112 * t1272;
            let t1275 = t424 * t1271;
            let t1277 = f64x8::splat(16.0) / f64x8::splat(3.0) * t112 * t1275;
            let t1280 = f64x8::splat(8.0) / f64x8::splat(3.0) * t112 * param_expo * t1271;
            let t1281 = f64x8::splat(341.0) / f64x8::splat(972.0) * t151 * t246 * t1244 * t99 - f64x8::splat(19.0) / f64x8::splat(324.0) * t410 * t597 * t1250 * t99 + t1257 * t1260 * t28 * t1262 / f64x8::splat(2916.0) - f64x8::splat(154.0) / f64x8::splat(81.0) * t102 * t90 * t1267 + t1274 + t1277 + t1280;
            let t1283 = t617 * t268;
            let t1286 = t265 * t623;
            let t1291 = t267 * t267;
            let t1292 = f64x8::splat(1.0) / t1291;
            let t1293 = t113 * t1292;
            let t1294 = t625 * t287;
            let t1297 = t287 * t646;
            let t1303 = f64x8::splat(1.0) / t92 / t247 * param_a * t120;
            let t1314 = t409 * t597;
            let t1316 = t280 * t280;
            let t1318 = f64x8::splat(1.0) / t281 / t1316;
            let t1319 = t984 * t1318;
            let t1322 = -f64x8::splat(70.0) / f64x8::splat(81.0) * t114 * t1303 - f64x8::splat(476.0) / f64x8::splat(27.0) * t276 * t1267 * param_a * t283 + f64x8::splat(592.0) / f64x8::splat(9.0) * t638 * t1244 * param_a * t642 - f64x8::splat(768.0) * t1314 * t1250 * t1319 - t1274 - t1277 - t1280;
            let t1324 = t1281 * t125 - f64x8::splat(3.0) * t1283 * t287 + f64x8::splat(6.0) * t1286 * t625 - f64x8::splat(6.0) * t1293 * t1294 + f64x8::splat(6.0) * t624 * t1297 - t269 * t1322 - f64x8::splat(3.0) * t619 * t646;
            let t1329 = ((t80).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t1228 * t127 - f64x8::splat(3.0) / f64x8::splat(8.0) * t1173 - f64x8::splat(9.0) / f64x8::splat(8.0) * t5 * t581 * t289 + t1106 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(4.0) * t1180 - f64x8::splat(9.0) / f64x8::splat(8.0) * t5 * t241 * t648 - t1030 + t1117 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t1188 - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t89 * t1324));
            let tv3rho33 = f64x8::splat(3.0) * t570 + f64x8::splat(3.0) * t653 + t6 * (t1215 + t1329);
            acc_v3rho3_3 = tv3rho33;
            let t1337 = t5 * t383 * t325;
            let t1344 = t5 * t392 * t325 / f64x8::splat(12.0);
            let t1346 = t5 * t146 * t699;
            let t1348 = t153 * t401;
            let t1356 = t412 * t156;
            let t1359 = t409 / t37 / t1356;
            let t1360 = t919 * t1359;
            let t1362 = t411 * t28 * t927;
            let t1368 = t936 * t425;
            let t1371 = f64x8::splat(8.0) / f64x8::splat(9.0) * t59 * t1368 * t308;
            let t1374 = f64x8::splat(2.0) / f64x8::splat(3.0) * t59 * t426 * t308;
            let t1375 = -f64x8::splat(65.0) / f64x8::splat(648.0) * t151 * t1348 * t301 + f64x8::splat(17.0) / f64x8::splat(864.0) * t410 * t414 * t154 * t43 - t1360 * t1362 / f64x8::splat(7776.0) + f64x8::splat(11.0) / f64x8::splat(27.0) * t46 * t33 * t420 - t1371 - t1374;
            let t1377 = t679 * t177;
            let t1380 = t312 * t438;
            let t1389 = t323 * t440;
            let t1392 = t697 * t199;
            let t1395 = t323 * t463;
            let t1403 = t453 * t401;
            let t1407 = t409 * t414 * param_a;
            let t1408 = t983 * t987;
            let t1409 = t1408 * t154;
            let t1412 = f64x8::splat(7.0) / f64x8::splat(54.0) * t315 * t446 + f64x8::splat(37.0) / f64x8::splat(9.0) * t185 * t420 * t320 - f64x8::splat(62.0) / f64x8::splat(3.0) * t1403 * t694 + f64x8::splat(288.0) * t1407 * t1409 + t1371 + t1374;
            let t1414 = t1375 * t73 - f64x8::splat(2.0) * t1377 * t199 + f64x8::splat(2.0) * t1380 * t440 - f64x8::splat(6.0) * t959 * t1389 + f64x8::splat(4.0) * t439 * t1392 + f64x8::splat(2.0) * t439 * t1395 - t178 * t1412 - t949 * t323 - f64x8::splat(2.0) * t434 * t697 - t681 * t463 + f64x8::splat(4.0) * t952 * t684;
            let t1419 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t379 * t325 - t1337 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(4.0) * t5 * t140 * t699 + t1344 - t1346 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t27 * t1414));
            let tv3rho2sigma0 = t6 * t1419 + f64x8::splat(2.0) * t704;
            acc_v3rho2sigma_0 = tv3rho2sigma0;
            let tv3rho2sigma1 = f64x8::splat(0.0);
            acc_v3rho2sigma_1 = tv3rho2sigma1;
            let t1426 = t5 * t488 * t357;
            let t1430 = t5 * t492 * t357 / f64x8::splat(12.0);
            let t1432 = ((t80).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t484 * t357 - t1426 / f64x8::splat(4.0) + t1430));
            let tv3rho2sigma2 = t6 * t1432 + f64x8::splat(2.0) * t713;
            acc_v3rho2sigma_2 = tv3rho2sigma2;
            let t1438 = t5 * t513 * t325;
            let t1446 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t509 * t325 - t1438 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t230 * t699 - t1337 / f64x8::splat(8.0) + t1344 - t1346 / f64x8::splat(8.0)));
            let tv3rho2sigma3 = t6 * t1446 + t704 + t719;
            acc_v3rho2sigma_3 = tv3rho2sigma3;
            let tv3rho2sigma4 = f64x8::splat(0.0);
            acc_v3rho2sigma_4 = tv3rho2sigma4;
            let t1452 = t5 * t537 * t357;
            let t1459 = t5 * t217 * t760;
            let t1462 = ((t80).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t533 * t357 - t1452 / f64x8::splat(8.0) - t1426 / f64x8::splat(8.0) + t1430 - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t213 * t760 - t1459 / f64x8::splat(8.0)));
            let tv3rho2sigma5 = t6 * t1462 + t713 + t765;
            acc_v3rho2sigma_5 = tv3rho2sigma5;
            let t1470 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t564 * t325 - t1438 / f64x8::splat(4.0) + t1344));
            let tv3rho2sigma6 = t6 * t1470 + f64x8::splat(2.0) * t719;
            acc_v3rho2sigma_6 = tv3rho2sigma6;
            let tv3rho2sigma7 = f64x8::splat(0.0);
            acc_v3rho2sigma_7 = tv3rho2sigma7;
            let t1481 = t153 * t592;
            let t1489 = t598 * t247;
            let t1492 = t409 / t93 / t1489;
            let t1493 = t919 * t1492;
            let t1495 = t597 * t28 * t1262;
            let t1501 = t936 * t610;
            let t1504 = f64x8::splat(8.0) / f64x8::splat(9.0) * t112 * t1501 * t341;
            let t1507 = f64x8::splat(2.0) / f64x8::splat(3.0) * t112 * t611 * t341;
            let t1508 = -f64x8::splat(65.0) / f64x8::splat(648.0) * t151 * t1481 * t334 + f64x8::splat(17.0) / f64x8::splat(864.0) * t410 * t600 * t245 * t99 - t1493 * t1495 / f64x8::splat(7776.0) + f64x8::splat(11.0) / f64x8::splat(27.0) * t102 * t33 * t606 - t1504 - t1507;
            let t1510 = t741 * t268;
            let t1513 = t345 * t623;
            let t1522 = t355 * t625;
            let t1525 = t758 * t287;
            let t1528 = t355 * t646;
            let t1536 = t453 * t592;
            let t1540 = t409 * t600 * param_a;
            let t1541 = t983 * t1318;
            let t1542 = t1541 * t245;
            let t1545 = f64x8::splat(7.0) / f64x8::splat(54.0) * t348 * t631 + f64x8::splat(37.0) / f64x8::splat(9.0) * t185 * t606 * t352 - f64x8::splat(62.0) / f64x8::splat(3.0) * t1536 * t755 + f64x8::splat(288.0) * t1540 * t1542 + t1504 + t1507;
            let t1547 = t1508 * t125 - t1283 * t355 + f64x8::splat(4.0) * t1286 * t746 - f64x8::splat(6.0) * t1293 * t1522 - f64x8::splat(2.0) * t1510 * t287 + f64x8::splat(2.0) * t1513 * t625 + f64x8::splat(4.0) * t624 * t1525 + f64x8::splat(2.0) * t624 * t1528 - t269 * t1545 - f64x8::splat(2.0) * t619 * t758 - t743 * t646;
            let t1552 = ((t80).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t581 * t357 - t1452 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(4.0) * t5 * t241 * t760 + t1430 - t1459 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t89 * t1547));
            let tv3rho2sigma8 = t6 * t1552 + f64x8::splat(2.0) * t765;
            acc_v3rho2sigma_8 = tv3rho2sigma8;
            let t1559 = t5 * t146 * t805 / f64x8::splat(8.0);
            let t1566 = t409 / t37 / t912;
            let t1567 = t919 * t1566;
            let t1568 = t154 * t43;
            let t1569 = t185 * t1568;
            let t1575 = t936 * t170;
            let t1578 = t59 * t1575 * t775 / f64x8::splat(3.0);
            let t1581 = f64x8::splat(2.0) / f64x8::splat(3.0) * t59 * t675 * t775;
            let t1582 = -f64x8::splat(5.0) / f64x8::splat(864.0) * t410 * t667 * t43 * v_sigma0 + t1567 * t1569 / f64x8::splat(20736.0) + t151 * t662 * t43 / f64x8::splat(54.0) + t1578 - t1581;
            let t1584 = t782 * t177;
            let t1594 = t786 * t199;
            let t1597 = t323 * t697;
            let t1601 = t803 * t199;
            let t1610 = t409 * t667;
            let t1612 = t1408 * v_sigma0;
            let t1615 = t791 * t182 / f64x8::splat(36.0) - t794 * t196 / f64x8::splat(6.0) + f64x8::splat(5.0) * t691 * t800 - f64x8::splat(108.0) * t1610 * param_a * t1612 - t1578 + t1581;
            let t1617 = -f64x8::splat(2.0) * t1377 * t323 + f64x8::splat(4.0) * t1380 * t684 + t1582 * t73 - t1584 * t199 - f64x8::splat(6.0) * t959 * t1594 + f64x8::splat(4.0) * t439 * t1597 + f64x8::splat(2.0) * t439 * t1601 - t178 * t1615 - t434 * t803 - f64x8::splat(2.0) * t681 * t697 + f64x8::splat(2.0) * t952 * t786;
            let t1622 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t140 * t805 - t1559 - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t27 * t1617));
            let tv3rhosigma20 = t6 * t1622 + t809;
            acc_v3rhosigma2_0 = tv3rhosigma20;
            let tv3rhosigma21 = f64x8::splat(0.0);
            acc_v3rhosigma2_1 = tv3rhosigma21;
            let tv3rhosigma22 = f64x8::splat(0.0);
            acc_v3rhosigma2_2 = tv3rhosigma22;
            let tv3rhosigma23 = f64x8::splat(0.0);
            acc_v3rhosigma2_3 = tv3rhosigma23;
            let tv3rhosigma24 = f64x8::splat(0.0);
            acc_v3rhosigma2_4 = tv3rhosigma24;
            let t1629 = t5 * t217 * t848 / f64x8::splat(8.0);
            let t1631 = ((t80).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t213 * t848 - t1629));
            let tv3rhosigma25 = t6 * t1631 + t852;
            acc_v3rhosigma2_5 = tv3rhosigma25;
            let t1637 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t230 * t805 - t1559));
            let tv3rhosigma26 = t6 * t1637 + t809;
            acc_v3rhosigma2_6 = tv3rhosigma26;
            let tv3rhosigma27 = f64x8::splat(0.0);
            acc_v3rhosigma2_7 = tv3rhosigma27;
            let tv3rhosigma28 = f64x8::splat(0.0);
            acc_v3rhosigma2_8 = tv3rhosigma28;
            let tv3rhosigma29 = f64x8::splat(0.0);
            acc_v3rhosigma2_9 = tv3rhosigma29;
            let tv3rhosigma210 = f64x8::splat(0.0);
            acc_v3rhosigma2_10 = tv3rhosigma210;
            let t1648 = t409 / t93 / t1249;
            let t1649 = t919 * t1648;
            let t1650 = t245 * t99;
            let t1651 = t185 * t1650;
            let t1657 = t936 * t261;
            let t1660 = t112 * t1657 * t818 / f64x8::splat(3.0);
            let t1663 = f64x8::splat(2.0) / f64x8::splat(3.0) * t112 * t737 * t818;
            let t1664 = -f64x8::splat(5.0) / f64x8::splat(864.0) * t410 * t729 * t99 * v_sigma2 + t1649 * t1651 / f64x8::splat(20736.0) + t151 * t724 * t99 / f64x8::splat(54.0) + t1660 - t1663;
            let t1666 = t825 * t268;
            let t1676 = t829 * t287;
            let t1679 = t355 * t758;
            let t1683 = t846 * t287;
            let t1692 = t409 * t729;
            let t1694 = t1541 * v_sigma2;
            let t1697 = t834 * t273 / f64x8::splat(36.0) - t837 * t284 / f64x8::splat(6.0) + f64x8::splat(5.0) * t753 * t843 - f64x8::splat(108.0) * t1692 * param_a * t1694 - t1660 + t1663;
            let t1699 = t1664 * t125 + f64x8::splat(2.0) * t1286 * t829 - f64x8::splat(6.0) * t1293 * t1676 - f64x8::splat(2.0) * t1510 * t355 + f64x8::splat(4.0) * t1513 * t746 - t1666 * t287 + f64x8::splat(4.0) * t624 * t1679 + f64x8::splat(2.0) * t624 * t1683 - t269 * t1697 - t619 * t846 - f64x8::splat(2.0) * t743 * t758;
            let t1704 = ((t80).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t241 * t848 - t1629 - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t89 * t1699));
            let tv3rhosigma211 = t6 * t1704 + t852;
            acc_v3rhosigma2_11 = tv3rhosigma211;
            let t1707 = f64x8::splat(1.0) / t37 / t413;
            let t1708 = t409 * t1707;
            let t1710 = t185 * t301;
            let t1713 = t409 * t767;
            let t1717 = f64x8::splat(1.0) / t411;
            let t1720 = t59 * t936 * t1717 / f64x8::splat(8.0);
            let t1723 = f64x8::splat(3.0) / f64x8::splat(4.0) * t59 * t424 * t1717;
            let t1725 = t59 * param_expo * t1717;
            let t1726 = -t919 * t1708 * t1710 / f64x8::splat(55296.0) + t407 * t1713 * t43 / f64x8::splat(768.0) - t1720 + t1723 - t1725;
            let t1734 = t786 * t323;
            let t1737 = t323 * t803;
            let t1742 = t51 / t52 / t154;
            let t1745 = t185 * t775;
            let t1748 = t453 * t308;
            let t1749 = t299 * param_a;
            let t1750 = t1749 * t459;
            let t1755 = t1742 * t69 / f64x8::splat(32.0) - f64x8::splat(3.0) / f64x8::splat(16.0) * t1745 * t796 - f64x8::splat(3.0) / f64x8::splat(8.0) * t1748 * t1750 + f64x8::splat(81.0) / f64x8::splat(2.0) * t1713 * t988 + t1720 - t1723 + t1725;
            let t1757 = f64x8::splat(6.0) * t1380 * t786 - f64x8::splat(3.0) * t1584 * t323 + t1726 * t73 - f64x8::splat(6.0) * t959 * t1734 + f64x8::splat(6.0) * t439 * t1737 - t178 * t1755 - f64x8::splat(3.0) * t681 * t803;
            let t1761 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t27 * t1757));
            let tv3sigma30 = t6 * t1761;
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
            let t1763 = f64x8::splat(1.0) / t93 / t599;
            let t1764 = t409 * t1763;
            let t1766 = t185 * t334;
            let t1769 = t409 * t810;
            let t1773 = f64x8::splat(1.0) / t597;
            let t1776 = t112 * t936 * t1773 / f64x8::splat(8.0);
            let t1779 = f64x8::splat(3.0) / f64x8::splat(4.0) * t112 * t424 * t1773;
            let t1781 = t112 * param_expo * t1773;
            let t1782 = -t919 * t1764 * t1766 / f64x8::splat(55296.0) + t407 * t1769 * t99 / f64x8::splat(768.0) - t1776 + t1779 - t1781;
            let t1790 = t829 * t355;
            let t1793 = t355 * t846;
            let t1798 = t51 / t105 / t245;
            let t1801 = t185 * t818;
            let t1804 = t453 * t341;
            let t1805 = t332 * param_a;
            let t1806 = t1805 * t642;
            let t1811 = t1798 * t121 / f64x8::splat(32.0) - f64x8::splat(3.0) / f64x8::splat(16.0) * t1801 * t839 - f64x8::splat(3.0) / f64x8::splat(8.0) * t1804 * t1806 + f64x8::splat(81.0) / f64x8::splat(2.0) * t1769 * t1319 + t1776 - t1779 + t1781;
            let t1813 = t1782 * t125 - f64x8::splat(6.0) * t1293 * t1790 + f64x8::splat(6.0) * t1513 * t829 - f64x8::splat(3.0) * t1666 * t355 + f64x8::splat(6.0) * t624 * t1793 - t269 * t1811 - f64x8::splat(3.0) * t743 * t846;
            let t1817 = ((t80).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t89 * t1813));
            let tv3sigma39 = t6 * t1817;
            acc_v3sigma3_9 = tv3sigma39;
        }
        store_add(zk, ip, m, acc_zk);
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
