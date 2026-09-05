//! GGA_X_SOGGA11 lxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_sogga11.c`
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

#[allow(unused_variables, non_snake_case)]
pub fn gga_x_sogga11_lxc_unpol(
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
    v4rho4: &mut [f64],
    v4rho3sigma: &mut [f64],
    v4rho2sigma2: &mut [f64],
    v4rhosigma3: &mut [f64],
    v4sigma4: &mut [f64],
    param_a_1: f64,
    param_mu: f64,
    param_kappa: f64,
    param_a_2: f64,
    param_a_3: f64,
    param_a_4: f64,
    param_a_5: f64,
    param_b_1: f64,
    param_b_2: f64,
    param_b_3: f64,
    param_b_4: f64,
    param_b_5: f64,
    param_a_0: f64,
    param_b_0: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_a_1 = f64x8::splat(param_a_1);
    let param_mu = f64x8::splat(param_mu);
    let param_kappa = f64x8::splat(param_kappa);
    let param_a_2 = f64x8::splat(param_a_2);
    let param_a_3 = f64x8::splat(param_a_3);
    let param_a_4 = f64x8::splat(param_a_4);
    let param_a_5 = f64x8::splat(param_a_5);
    let param_b_1 = f64x8::splat(param_b_1);
    let param_b_2 = f64x8::splat(param_b_2);
    let param_b_3 = f64x8::splat(param_b_3);
    let param_b_4 = f64x8::splat(param_b_4);
    let param_b_5 = f64x8::splat(param_b_5);
    let param_a_0 = f64x8::splat(param_a_0);
    let param_b_0 = f64x8::splat(param_b_0);
    let dens_threshold = f64x8::splat(dens_threshold);
    let zeta_threshold = f64x8::splat(zeta_threshold);
    let mut ip = 0usize;
    while ip < np {
        let m = (np - ip).min(8);
        let v_rho = load(rho, ip, np);
        let v_sigma = load(sigma, ip, np);
        let mut acc_zk = V_ZERO;
        let mut acc_vrho = V_ZERO;
        let mut acc_vsigma = V_ZERO;
        let mut acc_v2rho2 = V_ZERO;
        let mut acc_v2rhosigma = V_ZERO;
        let mut acc_v2sigma2 = V_ZERO;
        let mut acc_v3rho3 = V_ZERO;
        let mut acc_v3rho2sigma = V_ZERO;
        let mut acc_v3rhosigma2 = V_ZERO;
        let mut acc_v3sigma3 = V_ZERO;
        let mut acc_v4rho4 = V_ZERO;
        let mut acc_v4rho3sigma = V_ZERO;
        let mut acc_v4rho2sigma2 = V_ZERO;
        let mut acc_v4rhosigma3 = V_ZERO;
        let mut acc_v4sigma4 = V_ZERO;
        {
            let t2 = (v_rho / f64x8::splat(2.0)).simd_le(dens_threshold);
            let t3 = f64x8::splat(M_CBRT3);
            let t4 = f64x8::splat(M_CBRTPI);
            let t6 = t3 / t4;
            let t7 = (f64x8::splat(1.0)).simd_le(zeta_threshold);
            let t8 = zeta_threshold - f64x8::splat(1.0);
            let t10 = ((t7).select(t8, (t7).select(-t8, f64x8::splat(0.0))));
            let t11 = f64x8::splat(1.0) + t10;
            let t13 = (simd::cbrt(zeta_threshold));
            let t15 = (simd::cbrt(t11));
            let t17 = (((t11).simd_le(zeta_threshold)).select(t13 * zeta_threshold, t15 * t11));
            let t18 = (simd::cbrt(v_rho));
            let t19 = t17 * t18;
            let t21 = param_a_1;
            let t22 = f64x8::splat(M_CBRT6);
            let t23 = param_mu * t22;
            let t24 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t25 = (simd::cbrt(t24));
            let t26 = t25 * t25;
            let t27 = f64x8::splat(1.0) / t26;
            let t28 = t23 * t27;
            let t29 = f64x8::splat(1.0) / param_kappa;
            let t30 = t29 * v_sigma;
            let t31 = f64x8::splat(M_CBRT2);
            let t32 = t31 * t31;
            let t33 = v_rho * v_rho;
            let t34 = t18 * t18;
            let t36 = f64x8::splat(1.0) / t34 / t33;
            let t37 = t32 * t36;
            let t40 = t28 * t30 * t37 / f64x8::splat(24.0);
            let t41 = f64x8::splat(1.0) + t40;
            let t43 = f64x8::splat(1.0) - f64x8::splat(1.0) / t41;
            let t45 = param_a_2;
            let t46 = t43 * t43;
            let t48 = param_a_3;
            let t49 = t46 * t43;
            let t51 = param_a_4;
            let t52 = t46 * t46;
            let t54 = param_a_5;
            let t58 = param_b_1;
            let t59 = (simd::exp(-t40));
            let t60 = f64x8::splat(1.0) - t59;
            let t62 = param_b_2;
            let t63 = t60 * t60;
            let t65 = param_b_3;
            let t66 = t63 * t60;
            let t68 = param_b_4;
            let t69 = t63 * t63;
            let t71 = param_b_5;
            let t74 = t54 * t52 * t43 + t71 * t69 * t60 + t21 * t43 + t45 * t46 + t48 * t49 + t51 * t52 + t58 * t60 + t62 * t63 + t65 * t66 + t68 * t69 + param_a_0 + param_b_0;
            let t78 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t74));
            let tzk0 = f64x8::splat(2.0) * t78;
            acc_zk = tzk0;
            let t80 = t17 / t34;
            let t84 = t41 * t41;
            let t85 = f64x8::splat(1.0) / t84;
            let t87 = t21 * t85 * t23;
            let t88 = t27 * t29;
            let t89 = v_sigma * t32;
            let t90 = t33 * v_rho;
            let t92 = f64x8::splat(1.0) / t34 / t90;
            let t94 = t88 * t89 * t92;
            let t97 = t45 * t43;
            let t98 = t85 * param_mu;
            let t99 = t98 * t22;
            let t100 = t97 * t99;
            let t103 = t48 * t46;
            let t104 = t103 * t99;
            let t107 = t51 * t49;
            let t108 = t107 * t99;
            let t111 = t54 * t52;
            let t112 = t111 * t99;
            let t116 = t22 * t27;
            let t117 = t58 * param_mu * t116;
            let t118 = t32 * t92;
            let t119 = t118 * t59;
            let t120 = t30 * t119;
            let t123 = t62 * t60;
            let t124 = t123 * t28;
            let t127 = t65 * t63;
            let t128 = t127 * t28;
            let t131 = t68 * t66;
            let t132 = t131 * t28;
            let t135 = t71 * t69;
            let t136 = t135 * t28;
            let t139 = -t87 * t94 / f64x8::splat(9.0) - f64x8::splat(2.0) / f64x8::splat(9.0) * t100 * t94 - t104 * t94 / f64x8::splat(3.0) - f64x8::splat(4.0) / f64x8::splat(9.0) * t108 * t94 - f64x8::splat(5.0) / f64x8::splat(9.0) * t112 * t94 - t117 * t120 / f64x8::splat(9.0) - f64x8::splat(2.0) / f64x8::splat(9.0) * t124 * t120 - t128 * t120 / f64x8::splat(3.0) - f64x8::splat(4.0) / f64x8::splat(9.0) * t132 * t120 - f64x8::splat(5.0) / f64x8::splat(9.0) * t136 * t120;
            let t144 = ((t2).select(f64x8::splat(0.0), -t6 * t80 * t74 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t139));
            let tvrho0 = f64x8::splat(2.0) * v_rho * t144 + f64x8::splat(2.0) * t78;
            acc_vrho = tvrho0;
            let t150 = t97 * t98;
            let t151 = t29 * t32;
            let t153 = t116 * t151 * t36;
            let t156 = t103 * t98;
            let t159 = t107 * t98;
            let t162 = t111 * t98;
            let t169 = t123 * t23;
            let t171 = t88 * t37 * t59;
            let t174 = t127 * t23;
            let t177 = t131 * t23;
            let t180 = t135 * t23;
            let t183 = t87 * t88 * t37 / f64x8::splat(24.0) + t150 * t153 / f64x8::splat(12.0) + t156 * t153 / f64x8::splat(8.0) + t159 * t153 / f64x8::splat(6.0) + f64x8::splat(5.0) / f64x8::splat(24.0) * t162 * t153 + t117 * t151 * t36 * t59 / f64x8::splat(24.0) + t169 * t171 / f64x8::splat(12.0) + t174 * t171 / f64x8::splat(8.0) + t177 * t171 / f64x8::splat(6.0) + f64x8::splat(5.0) / f64x8::splat(24.0) * t180 * t171;
            let t187 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t183));
            let tvsigma0 = f64x8::splat(2.0) * v_rho * t187;
            acc_vsigma = tvsigma0;
            let t192 = t17 / t34 / v_rho;
            let t199 = t84 * t41;
            let t200 = f64x8::splat(1.0) / t199;
            let t201 = param_mu * param_mu;
            let t202 = t200 * t201;
            let t203 = t22 * t22;
            let t204 = t202 * t203;
            let t205 = t97 * t204;
            let t207 = f64x8::splat(1.0) / t25 / t24;
            let t208 = param_kappa * param_kappa;
            let t209 = f64x8::splat(1.0) / t208;
            let t210 = t207 * t209;
            let t211 = v_sigma * v_sigma;
            let t212 = t211 * t31;
            let t213 = t33 * t33;
            let t216 = f64x8::splat(1.0) / t18 / t213 / t90;
            let t218 = t210 * t212 * t216;
            let t221 = t48 * t43;
            let t222 = t84 * t84;
            let t223 = f64x8::splat(1.0) / t222;
            let t224 = t223 * t201;
            let t225 = t224 * t203;
            let t226 = t221 * t225;
            let t229 = t103 * t204;
            let t232 = t51 * t46;
            let t233 = t232 * t225;
            let t237 = f64x8::splat(1.0) / t34 / t213;
            let t238 = t32 * t237;
            let t239 = t238 * t59;
            let t240 = t30 * t239;
            let t248 = t88 * t89 * t237;
            let t259 = t65 * t60;
            let t260 = t201 * t203;
            let t261 = t260 * t207;
            let t262 = t259 * t261;
            let t263 = t209 * t211;
            let t264 = t31 * t216;
            let t265 = t59 * t59;
            let t266 = t264 * t265;
            let t267 = t263 * t266;
            let t270 = t127 * t261;
            let t271 = t264 * t59;
            let t272 = t263 * t271;
            let t275 = -f64x8::splat(8.0) / f64x8::splat(81.0) * t205 * t218 + f64x8::splat(4.0) / f64x8::splat(27.0) * t226 * t218 - f64x8::splat(4.0) / f64x8::splat(27.0) * t229 * t218 + f64x8::splat(8.0) / f64x8::splat(27.0) * t233 * t218 + f64x8::splat(11.0) / f64x8::splat(9.0) * t128 * t240 + f64x8::splat(44.0) / f64x8::splat(27.0) * t132 * t240 + f64x8::splat(55.0) / f64x8::splat(27.0) * t136 * t240 + f64x8::splat(55.0) / f64x8::splat(27.0) * t112 * t248 + f64x8::splat(22.0) / f64x8::splat(27.0) * t124 * t240 + f64x8::splat(22.0) / f64x8::splat(27.0) * t100 * t248 + f64x8::splat(11.0) / f64x8::splat(9.0) * t104 * t248 + f64x8::splat(44.0) / f64x8::splat(27.0) * t108 * t248 + f64x8::splat(4.0) / f64x8::splat(27.0) * t262 * t267 - f64x8::splat(2.0) / f64x8::splat(27.0) * t270 * t272;
            let t276 = t68 * t63;
            let t277 = t276 * t261;
            let t280 = t131 * t261;
            let t283 = t71 * t66;
            let t284 = t283 * t261;
            let t287 = t135 * t261;
            let t290 = t107 * t204;
            let t293 = t54 * t49;
            let t294 = t293 * t225;
            let t297 = t111 * t204;
            let t300 = t123 * t261;
            let t308 = t21 * t200 * t260;
            let t312 = t45 * t223 * t260;
            let t316 = t203 * t207;
            let t317 = t58 * t201 * t316;
            let t321 = t62 * t201 * t316;
            let t324 = f64x8::splat(8.0) / f64x8::splat(27.0) * t277 * t267 - f64x8::splat(8.0) / f64x8::splat(81.0) * t280 * t272 + f64x8::splat(40.0) / f64x8::splat(81.0) * t284 * t267 - f64x8::splat(10.0) / f64x8::splat(81.0) * t287 * t272 - f64x8::splat(16.0) / f64x8::splat(81.0) * t290 * t218 + f64x8::splat(40.0) / f64x8::splat(81.0) * t294 * t218 - f64x8::splat(20.0) / f64x8::splat(81.0) * t297 * t218 - f64x8::splat(4.0) / f64x8::splat(81.0) * t300 * t272 + f64x8::splat(11.0) / f64x8::splat(27.0) * t117 * t240 + f64x8::splat(11.0) / f64x8::splat(27.0) * t87 * t248 - f64x8::splat(4.0) / f64x8::splat(81.0) * t308 * t218 + f64x8::splat(4.0) / f64x8::splat(81.0) * t312 * t218 - f64x8::splat(2.0) / f64x8::splat(81.0) * t317 * t272 + f64x8::splat(4.0) / f64x8::splat(81.0) * t321 * t267;
            let t325 = t275 + t324;
            let t330 = ((t2).select(f64x8::splat(0.0), t6 * t192 * t74 / f64x8::splat(12.0) - t6 * t80 * t139 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t325));
            let tv2rho20 = f64x8::splat(2.0) * v_rho * t330 + f64x8::splat(4.0) * t144;
            acc_v2rho2 = tv2rho20;
            let t343 = t209 * t31;
            let t344 = t213 * t33;
            let t346 = f64x8::splat(1.0) / t18 / t344;
            let t347 = t346 * t265;
            let t349 = t343 * t347 * v_sigma;
            let t354 = t343 * t346 * v_sigma * t59;
            let t365 = t31 * t346;
            let t367 = t210 * t365 * v_sigma;
            let t380 = -t87 * t88 * t118 / f64x8::splat(9.0) - t117 * t151 * t92 * t59 / f64x8::splat(9.0) - f64x8::splat(5.0) / f64x8::splat(27.0) * t284 * t349 + f64x8::splat(5.0) / f64x8::splat(108.0) * t287 * t354 - t262 * t349 / f64x8::splat(18.0) + t270 * t354 / f64x8::splat(36.0) - t277 * t349 / f64x8::splat(9.0) + t280 * t354 / f64x8::splat(27.0) - t233 * t367 / f64x8::splat(9.0) + f64x8::splat(2.0) / f64x8::splat(27.0) * t290 * t367 - f64x8::splat(5.0) / f64x8::splat(27.0) * t294 * t367 + f64x8::splat(5.0) / f64x8::splat(54.0) * t297 * t367 + t300 * t354 / f64x8::splat(54.0) + t205 * t367 / f64x8::splat(27.0);
            let t385 = t88 * t119;
            let t391 = t116 * t151 * t92;
            let t412 = -t226 * t367 / f64x8::splat(18.0) + t229 * t367 / f64x8::splat(18.0) - f64x8::splat(4.0) / f64x8::splat(9.0) * t177 * t385 - f64x8::splat(5.0) / f64x8::splat(9.0) * t180 * t385 - f64x8::splat(4.0) / f64x8::splat(9.0) * t159 * t391 - f64x8::splat(5.0) / f64x8::splat(9.0) * t162 * t391 - f64x8::splat(2.0) / f64x8::splat(9.0) * t169 * t385 - f64x8::splat(2.0) / f64x8::splat(9.0) * t150 * t391 - t156 * t391 / f64x8::splat(3.0) + t308 * t367 / f64x8::splat(54.0) - t312 * t367 / f64x8::splat(54.0) + t317 * t354 / f64x8::splat(108.0) - t321 * t349 / f64x8::splat(54.0) - t174 * t385 / f64x8::splat(3.0);
            let t413 = t380 + t412;
            let t418 = ((t2).select(f64x8::splat(0.0), -t6 * t80 * t183 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t413));
            let tv2rhosigma0 = f64x8::splat(2.0) * v_rho * t418 + f64x8::splat(2.0) * t187;
            acc_v2rhosigma = tv2rhosigma0;
            let t421 = t213 * v_rho;
            let t423 = f64x8::splat(1.0) / t18 / t421;
            let t424 = t31 * t423;
            let t425 = t210 * t424;
            let t430 = t97 * t202;
            let t432 = t316 * t343 * t423;
            let t435 = t221 * t224;
            let t438 = t103 * t202;
            let t441 = t232 * t224;
            let t444 = t107 * t202;
            let t447 = t293 * t224;
            let t450 = t111 * t202;
            let t461 = t123 * t260;
            let t463 = t210 * t424 * t59;
            let t466 = t259 * t260;
            let t468 = t210 * t424 * t265;
            let t471 = t127 * t260;
            let t474 = t276 * t260;
            let t477 = t131 * t260;
            let t480 = t283 * t260;
            let t483 = t135 * t260;
            let t486 = -t308 * t425 / f64x8::splat(144.0) + t312 * t425 / f64x8::splat(144.0) - t430 * t432 / f64x8::splat(72.0) + t435 * t432 / f64x8::splat(48.0) - t438 * t432 / f64x8::splat(48.0) + t441 * t432 / f64x8::splat(24.0) - t444 * t432 / f64x8::splat(36.0) + f64x8::splat(5.0) / f64x8::splat(72.0) * t447 * t432 - f64x8::splat(5.0) / f64x8::splat(144.0) * t450 * t432 - t317 * t343 * t423 * t59 / f64x8::splat(288.0) + t321 * t343 * t423 * t265 / f64x8::splat(144.0) - t461 * t463 / f64x8::splat(144.0) + t466 * t468 / f64x8::splat(48.0) - t471 * t463 / f64x8::splat(96.0) + t474 * t468 / f64x8::splat(24.0) - t477 * t463 / f64x8::splat(72.0) + f64x8::splat(5.0) / f64x8::splat(72.0) * t480 * t468 - f64x8::splat(5.0) / f64x8::splat(288.0) * t483 * t463;
            let t490 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t486));
            let tv2sigma20 = f64x8::splat(2.0) * v_rho * t490;
            acc_v2sigma2 = tv2sigma20;
            let t493 = t17 * t36;
            let t503 = t54 * t46;
            let t505 = f64x8::splat(1.0) / t222 / t84;
            let t506 = t201 * param_mu;
            let t507 = t505 * t506;
            let t508 = t503 * t507;
            let t509 = t24 * t24;
            let t510 = f64x8::splat(1.0) / t509;
            let t512 = f64x8::splat(1.0) / t208 / param_kappa;
            let t513 = t510 * t512;
            let t514 = t211 * v_sigma;
            let t515 = t213 * t213;
            let t516 = t515 * t90;
            let t517 = f64x8::splat(1.0) / t516;
            let t519 = t513 * t514 * t517;
            let t523 = f64x8::splat(1.0) / t222 / t41;
            let t524 = t523 * t506;
            let t525 = t293 * t524;
            let t528 = t223 * t506;
            let t529 = t111 * t528;
            let t532 = t506 * t510;
            let t533 = t123 * t532;
            let t534 = t512 * t514;
            let t536 = t534 * t517 * t59;
            let t539 = t97 * t528;
            let t542 = t221 * t524;
            let t545 = t103 * t528;
            let t548 = t51 * t43;
            let t549 = t548 * t507;
            let t553 = f64x8::splat(1.0) / t34 / t421;
            let t555 = t88 * t89 * t553;
            let t558 = t32 * t553;
            let t559 = t558 * t59;
            let t560 = t30 * t559;
            let t564 = f64x8::splat(1.0) / t18 / t515;
            let t565 = t31 * t564;
            let t567 = t263 * t565 * t59;
            let t571 = t210 * t212 * t564;
            let t574 = t565 * t265;
            let t575 = t263 * t574;
            let t578 = -f64x8::splat(160.0) / f64x8::splat(81.0) * t508 * t519 + f64x8::splat(320.0) / f64x8::splat(81.0) * t525 * t519 - f64x8::splat(80.0) / f64x8::splat(81.0) * t529 * t519 - f64x8::splat(16.0) / f64x8::splat(243.0) * t533 * t536 - f64x8::splat(32.0) / f64x8::splat(81.0) * t539 * t519 + f64x8::splat(32.0) / f64x8::splat(27.0) * t542 * t519 - f64x8::splat(16.0) / f64x8::splat(27.0) * t545 * t519 - f64x8::splat(64.0) / f64x8::splat(81.0) * t549 * t519 - f64x8::splat(154.0) / f64x8::splat(81.0) * t87 * t555 - f64x8::splat(154.0) / f64x8::splat(81.0) * t117 * t560 + f64x8::splat(22.0) / f64x8::splat(81.0) * t317 * t567 + f64x8::splat(44.0) / f64x8::splat(81.0) * t308 * t571 - f64x8::splat(44.0) / f64x8::splat(81.0) * t321 * t575;
            let t581 = t259 * t532;
            let t583 = t534 * t517 * t265;
            let t586 = t127 * t532;
            let t589 = t68 * t60;
            let t590 = t589 * t532;
            let t591 = t265 * t59;
            let t593 = t534 * t517 * t591;
            let t596 = t276 * t532;
            let t599 = t131 * t532;
            let t602 = t71 * t63;
            let t603 = t602 * t532;
            let t606 = t283 * t532;
            let t609 = t135 * t532;
            let t612 = t232 * t524;
            let t615 = t107 * t528;
            let t619 = t65 * t506 * t510;
            let t623 = t62 * t506 * t510;
            let t626 = -f64x8::splat(44.0) / f64x8::splat(81.0) * t312 * t571 + f64x8::splat(16.0) / f64x8::splat(27.0) * t581 * t583 - f64x8::splat(8.0) / f64x8::splat(81.0) * t586 * t536 - f64x8::splat(64.0) / f64x8::splat(81.0) * t590 * t593 + f64x8::splat(32.0) / f64x8::splat(27.0) * t596 * t583 - f64x8::splat(32.0) / f64x8::splat(243.0) * t599 * t536 - f64x8::splat(160.0) / f64x8::splat(81.0) * t603 * t593 + f64x8::splat(160.0) / f64x8::splat(81.0) * t606 * t583 - f64x8::splat(40.0) / f64x8::splat(243.0) * t609 * t536 + f64x8::splat(64.0) / f64x8::splat(27.0) * t612 * t519 - f64x8::splat(64.0) / f64x8::splat(81.0) * t615 * t519 - f64x8::splat(16.0) / f64x8::splat(81.0) * t619 * t593 + f64x8::splat(16.0) / f64x8::splat(81.0) * t623 * t583;
            let t629 = t45 * t523 * t506;
            let t633 = t48 * t505 * t506;
            let t637 = t21 * t223 * t506;
            let t641 = t58 * t506 * t510;
            let t662 = f64x8::splat(32.0) / f64x8::splat(81.0) * t629 * t519 - f64x8::splat(16.0) / f64x8::splat(81.0) * t633 * t519 - f64x8::splat(16.0) / f64x8::splat(81.0) * t637 * t519 - f64x8::splat(8.0) / f64x8::splat(243.0) * t641 * t536 - f64x8::splat(440.0) / f64x8::splat(81.0) * t294 * t571 + f64x8::splat(220.0) / f64x8::splat(81.0) * t297 * t571 - f64x8::splat(440.0) / f64x8::splat(81.0) * t284 * t575 + f64x8::splat(110.0) / f64x8::splat(81.0) * t287 * t567 - f64x8::splat(88.0) / f64x8::splat(27.0) * t277 * t575 + f64x8::splat(88.0) / f64x8::splat(81.0) * t280 * t567 - f64x8::splat(44.0) / f64x8::splat(27.0) * t262 * t575 + f64x8::splat(22.0) / f64x8::splat(27.0) * t270 * t567 - f64x8::splat(308.0) / f64x8::splat(81.0) * t100 * t555;
            let t689 = -f64x8::splat(154.0) / f64x8::splat(27.0) * t104 * t555 - f64x8::splat(616.0) / f64x8::splat(81.0) * t108 * t555 - f64x8::splat(770.0) / f64x8::splat(81.0) * t136 * t560 - f64x8::splat(770.0) / f64x8::splat(81.0) * t112 * t555 - f64x8::splat(308.0) / f64x8::splat(81.0) * t124 * t560 - f64x8::splat(88.0) / f64x8::splat(27.0) * t233 * t571 - f64x8::splat(154.0) / f64x8::splat(27.0) * t128 * t560 - f64x8::splat(616.0) / f64x8::splat(81.0) * t132 * t560 - f64x8::splat(44.0) / f64x8::splat(27.0) * t226 * t571 + f64x8::splat(44.0) / f64x8::splat(27.0) * t229 * t571 + f64x8::splat(44.0) / f64x8::splat(81.0) * t300 * t567 + f64x8::splat(88.0) / f64x8::splat(81.0) * t205 * t571 + f64x8::splat(176.0) / f64x8::splat(81.0) * t290 * t571;
            let t691 = t578 + t626 + t662 + t689;
            let t696 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(5.0) / f64x8::splat(36.0) * t6 * t493 * t74 + t6 * t192 * t139 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t80 * t325 - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t691));
            let tv3rho30 = f64x8::splat(2.0) * v_rho * t696 + f64x8::splat(6.0) * t330;
            acc_v3rho3 = tv3rho30;
            let t713 = t515 * t33;
            let t714 = f64x8::splat(1.0) / t713;
            let t715 = t512 * t714;
            let t716 = t265 * t211;
            let t717 = t715 * t716;
            let t720 = t211 * t59;
            let t721 = t715 * t720;
            let t728 = t591 * t211;
            let t729 = t715 * t728;
            let t735 = t513 * t714 * t211;
            let t746 = f64x8::splat(11.0) / f64x8::splat(27.0) * t87 * t88 * t238 + f64x8::splat(11.0) / f64x8::splat(27.0) * t117 * t151 * t237 * t59 - f64x8::splat(20.0) / f64x8::splat(27.0) * t606 * t717 + f64x8::splat(5.0) / f64x8::splat(81.0) * t609 * t721 - f64x8::splat(2.0) / f64x8::splat(9.0) * t581 * t717 + t586 * t721 / f64x8::splat(27.0) + f64x8::splat(8.0) / f64x8::splat(27.0) * t590 * t729 + f64x8::splat(4.0) / f64x8::splat(81.0) * t599 * t721 + f64x8::splat(8.0) / f64x8::splat(27.0) * t549 * t735 - f64x8::splat(8.0) / f64x8::splat(9.0) * t612 * t735 + f64x8::splat(8.0) / f64x8::splat(27.0) * t615 * t735 + f64x8::splat(20.0) / f64x8::splat(27.0) * t508 * t735 - f64x8::splat(40.0) / f64x8::splat(27.0) * t525 * t735;
            let t761 = t88 * t239;
            let t765 = t116 * t151 * t237;
            let t776 = f64x8::splat(10.0) / f64x8::splat(27.0) * t529 * t735 + f64x8::splat(2.0) / f64x8::splat(81.0) * t533 * t721 + f64x8::splat(4.0) / f64x8::splat(27.0) * t539 * t735 - f64x8::splat(4.0) / f64x8::splat(9.0) * t542 * t735 + f64x8::splat(2.0) / f64x8::splat(9.0) * t545 * t735 + f64x8::splat(20.0) / f64x8::splat(27.0) * t603 * t729 - f64x8::splat(4.0) / f64x8::splat(9.0) * t596 * t717 + f64x8::splat(22.0) / f64x8::splat(27.0) * t169 * t761 + f64x8::splat(22.0) / f64x8::splat(27.0) * t150 * t765 + f64x8::splat(55.0) / f64x8::splat(27.0) * t162 * t765 + f64x8::splat(55.0) / f64x8::splat(27.0) * t180 * t761 + f64x8::splat(44.0) / f64x8::splat(27.0) * t159 * t765 + f64x8::splat(11.0) / f64x8::splat(9.0) * t174 * t761;
            let t780 = t209 * v_sigma;
            let t781 = t780 * t266;
            let t786 = t210 * v_sigma * t31 * t216;
            let t793 = t343 * t216 * v_sigma * t59;
            let t812 = f64x8::splat(44.0) / f64x8::splat(27.0) * t177 * t761 + t321 * t781 / f64x8::splat(6.0) + t312 * t786 / f64x8::splat(6.0) - t308 * t786 / f64x8::splat(6.0) - t317 * t793 / f64x8::splat(12.0) + f64x8::splat(11.0) / f64x8::splat(9.0) * t156 * t765 + f64x8::splat(2.0) / f64x8::splat(27.0) * t633 * t735 + f64x8::splat(2.0) / f64x8::splat(27.0) * t637 * t735 + t641 * t721 / f64x8::splat(81.0) + f64x8::splat(2.0) / f64x8::splat(27.0) * t619 * t729 - f64x8::splat(2.0) / f64x8::splat(27.0) * t623 * t717 - f64x8::splat(4.0) / f64x8::splat(27.0) * t629 * t735 + t226 * t786 / f64x8::splat(2.0);
            let t837 = -t229 * t786 / f64x8::splat(2.0) - t300 * t793 / f64x8::splat(6.0) - t205 * t786 / f64x8::splat(3.0) - f64x8::splat(2.0) / f64x8::splat(3.0) * t290 * t786 + f64x8::splat(5.0) / f64x8::splat(3.0) * t294 * t786 - f64x8::splat(5.0) / f64x8::splat(6.0) * t297 * t786 + t233 * t786 + t262 * t781 / f64x8::splat(2.0) - t270 * t793 / f64x8::splat(4.0) + t277 * t781 - t280 * t793 / f64x8::splat(3.0) - f64x8::splat(5.0) / f64x8::splat(12.0) * t287 * t793 + f64x8::splat(5.0) / f64x8::splat(3.0) * t284 * t781;
            let t839 = t746 + t776 + t812 + t837;
            let t844 = ((t2).select(f64x8::splat(0.0), t6 * t192 * t183 / f64x8::splat(12.0) - t6 * t80 * t413 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t839));
            let tv3rho2sigma0 = f64x8::splat(2.0) * v_rho * t844 + f64x8::splat(4.0) * t418;
            acc_v3rho2sigma = tv3rho2sigma0;
            let t850 = t515 * v_rho;
            let t851 = f64x8::splat(1.0) / t850;
            let t853 = t513 * t851 * v_sigma;
            let t864 = t210 * t365;
            let t879 = t512 * t851;
            let t880 = v_sigma * t59;
            let t881 = t879 * t880;
            let t884 = t265 * v_sigma;
            let t885 = t879 * t884;
            let t890 = t591 * v_sigma;
            let t891 = t879 * t890;
            let t908 = -t533 * t881 / f64x8::splat(108.0) + t581 * t885 / f64x8::splat(12.0) - t586 * t881 / f64x8::splat(72.0) - t590 * t891 / f64x8::splat(9.0) + t596 * t885 / f64x8::splat(6.0) - t599 * t881 / f64x8::splat(54.0) - f64x8::splat(5.0) / f64x8::splat(18.0) * t603 * t891 + f64x8::splat(5.0) / f64x8::splat(18.0) * t606 * t885 - f64x8::splat(5.0) / f64x8::splat(216.0) * t609 * t881 - t539 * t853 / f64x8::splat(18.0) + t542 * t853 / f64x8::splat(6.0);
            let t913 = t210 * t365 * t59;
            let t917 = t210 * t365 * t265;
            let t929 = t316 * t343 * t346;
            let t959 = f64x8::splat(4.0) / f64x8::splat(27.0) * t444 * t929 - f64x8::splat(10.0) / f64x8::splat(27.0) * t447 * t929 + f64x8::splat(2.0) / f64x8::splat(27.0) * t430 * t929 - t435 * t929 / f64x8::splat(9.0) + t438 * t929 / f64x8::splat(9.0) - t633 * t853 / f64x8::splat(36.0) - t641 * t881 / f64x8::splat(216.0) + t623 * t885 / f64x8::splat(36.0) - t619 * t891 / f64x8::splat(36.0) - t637 * t853 / f64x8::splat(36.0) + t629 * t853 / f64x8::splat(18.0);
            let t961 = -t549 * t853 / f64x8::splat(9.0) + t612 * t853 / f64x8::splat(3.0) - t615 * t853 / f64x8::splat(9.0) - f64x8::splat(5.0) / f64x8::splat(18.0) * t508 * t853 + f64x8::splat(5.0) / f64x8::splat(9.0) * t525 * t853 + t308 * t864 / f64x8::splat(27.0) - t312 * t864 / f64x8::splat(27.0) + t317 * t343 * t346 * t59 / f64x8::splat(54.0) - t321 * t343 * t347 / f64x8::splat(27.0) - f64x8::splat(5.0) / f64x8::splat(36.0) * t529 * t853 + t908 - t545 * t853 / f64x8::splat(12.0) + f64x8::splat(2.0) / f64x8::splat(27.0) * t477 * t913 - f64x8::splat(10.0) / f64x8::splat(27.0) * t480 * t917 + f64x8::splat(5.0) / f64x8::splat(54.0) * t483 * t913 - t466 * t917 / f64x8::splat(9.0) + t471 * t913 / f64x8::splat(18.0) - f64x8::splat(2.0) / f64x8::splat(9.0) * t474 * t917 + f64x8::splat(5.0) / f64x8::splat(27.0) * t450 * t929 + t461 * t913 / f64x8::splat(27.0) - f64x8::splat(2.0) / f64x8::splat(9.0) * t441 * t929 + t959;
            let t966 = ((t2).select(f64x8::splat(0.0), -t6 * t80 * t486 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t961));
            let tv3rhosigma20 = f64x8::splat(2.0) * v_rho * t966 + f64x8::splat(2.0) * t490;
            acc_v3rhosigma2 = tv3rhosigma20;
            let t969 = t97 * t223;
            let t970 = f64x8::splat(1.0) / t515;
            let t971 = t512 * t970;
            let t972 = t532 * t971;
            let t975 = t221 * t523;
            let t978 = t103 * t223;
            let t981 = t548 * t505;
            let t984 = t232 * t523;
            let t987 = t107 * t223;
            let t990 = t503 * t505;
            let t993 = t293 * t523;
            let t996 = t111 * t223;
            let t999 = t123 * t506;
            let t1001 = t513 * t970 * t59;
            let t1004 = t259 * t506;
            let t1006 = t513 * t970 * t265;
            let t1009 = t127 * t506;
            let t1012 = t969 * t972 / f64x8::splat(48.0) - t975 * t972 / f64x8::splat(16.0) + t978 * t972 / f64x8::splat(32.0) + t981 * t972 / f64x8::splat(24.0) - t984 * t972 / f64x8::splat(8.0) + t987 * t972 / f64x8::splat(24.0) + f64x8::splat(5.0) / f64x8::splat(48.0) * t990 * t972 - f64x8::splat(5.0) / f64x8::splat(24.0) * t993 * t972 + f64x8::splat(5.0) / f64x8::splat(96.0) * t996 * t972 + t999 * t1001 / f64x8::splat(288.0) - t1004 * t1006 / f64x8::splat(32.0) + t1009 * t1001 / f64x8::splat(192.0);
            let t1013 = t589 * t506;
            let t1015 = t513 * t970 * t591;
            let t1018 = t276 * t506;
            let t1021 = t131 * t506;
            let t1024 = t602 * t506;
            let t1027 = t283 * t506;
            let t1030 = t135 * t506;
            let t1033 = t513 * t970;
            let t1049 = t1013 * t1015 / f64x8::splat(24.0) - t1018 * t1006 / f64x8::splat(16.0) + t1021 * t1001 / f64x8::splat(144.0) + f64x8::splat(5.0) / f64x8::splat(48.0) * t1024 * t1015 - f64x8::splat(5.0) / f64x8::splat(48.0) * t1027 * t1006 + f64x8::splat(5.0) / f64x8::splat(576.0) * t1030 * t1001 + t637 * t1033 / f64x8::splat(96.0) - t629 * t1033 / f64x8::splat(48.0) + t633 * t1033 / f64x8::splat(96.0) + t641 * t971 * t59 / f64x8::splat(576.0) - t623 * t971 * t265 / f64x8::splat(96.0) + t619 * t971 * t591 / f64x8::splat(96.0);
            let t1050 = t1012 + t1049;
            let t1054 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t1050));
            let tv3sigma30 = f64x8::splat(2.0) * v_rho * t1054;
            acc_v3sigma3 = tv3sigma30;
            let t1070 = t515 * t213;
            let t1071 = f64x8::splat(1.0) / t1070;
            let t1073 = t534 * t1071 * t591;
            let t1077 = t534 * t1071 * t265;
            let t1081 = t513 * t514 * t1071;
            let t1089 = t534 * t1071 * t59;
            let t1092 = t201 * t201;
            let t1094 = t505 * t1092 * t510;
            let t1095 = t293 * t1094;
            let t1096 = t208 * t208;
            let t1097 = f64x8::splat(1.0) / t1096;
            let t1098 = t211 * t211;
            let t1099 = t1097 * t1098;
            let t1102 = f64x8::splat(1.0) / t34 / t515 / t344;
            let t1104 = t116 * t32;
            let t1105 = t1099 * t1102 * t1104;
            let t1109 = t523 * t1092 * t510;
            let t1110 = t111 * t1109;
            let t1113 = t1092 * t510;
            let t1114 = t1113 * t1097;
            let t1115 = t123 * t1114;
            let t1116 = t1098 * t1102;
            let t1119 = t27 * t32 * t59;
            let t1120 = t1116 * t22 * t1119;
            let t1123 = t97 * t1109;
            let t1126 = t221 * t1094;
            let t1129 = t103 * t1109;
            let t1132 = t259 * t1114;
            let t1134 = t1116 * t265 * t1104;
            let t1137 = t127 * t1114;
            let t1140 = t589 * t1114;
            let t1142 = t1116 * t591 * t1104;
            let t1145 = t276 * t1114;
            let t1148 = t131 * t1114;
            let t1152 = t71 * t60 * t1114;
            let t1153 = t265 * t265;
            let t1158 = t602 * t1114;
            let t1161 = t283 * t1114;
            let t1164 = f64x8::splat(352.0) / f64x8::splat(81.0) * t619 * t1073 - f64x8::splat(352.0) / f64x8::splat(81.0) * t623 * t1077 - f64x8::splat(704.0) / f64x8::splat(81.0) * t629 * t1081 + f64x8::splat(352.0) / f64x8::splat(81.0) * t633 * t1081 + f64x8::splat(352.0) / f64x8::splat(81.0) * t637 * t1081 + f64x8::splat(176.0) / f64x8::splat(243.0) * t641 * t1089 + f64x8::splat(640.0) / f64x8::splat(243.0) * t1095 * t1105 - f64x8::splat(320.0) / f64x8::splat(729.0) * t1110 * t1105 - f64x8::splat(16.0) / f64x8::splat(2187.0) * t1115 * t1120 - f64x8::splat(128.0) / f64x8::splat(729.0) * t1123 * t1105 + f64x8::splat(64.0) / f64x8::splat(81.0) * t1126 * t1105 - f64x8::splat(64.0) / f64x8::splat(243.0) * t1129 * t1105 + f64x8::splat(112.0) / f64x8::splat(729.0) * t1132 * t1134 - f64x8::splat(8.0) / f64x8::splat(729.0) * t1137 * t1120 - f64x8::splat(128.0) / f64x8::splat(243.0) * t1140 * t1142 + f64x8::splat(224.0) / f64x8::splat(729.0) * t1145 * t1134 - f64x8::splat(32.0) / f64x8::splat(2187.0) * t1148 * t1120 + f64x8::splat(320.0) / f64x8::splat(729.0) * t1152 * t1116 * t1153 * t1104 - f64x8::splat(320.0) / f64x8::splat(243.0) * t1158 * t1142 + f64x8::splat(1120.0) / f64x8::splat(2187.0) * t1161 * t1134;
            let t1165 = t135 * t1114;
            let t1169 = f64x8::splat(1.0) / t222 / t199;
            let t1171 = t1169 * t1092 * t510;
            let t1172 = t548 * t1171;
            let t1175 = t232 * t1094;
            let t1178 = t107 * t1109;
            let t1182 = t222 * t222;
            let t1183 = f64x8::splat(1.0) / t1182;
            let t1186 = t54 * t43 * t1183 * t1092 * t510;
            let t1189 = t503 * t1171;
            let t1220 = -f64x8::splat(40.0) / f64x8::splat(2187.0) * t1165 * t1120 - f64x8::splat(256.0) / f64x8::splat(243.0) * t1172 * t1105 + f64x8::splat(128.0) / f64x8::splat(81.0) * t1175 * t1105 - f64x8::splat(256.0) / f64x8::splat(729.0) * t1178 * t1105 + f64x8::splat(320.0) / f64x8::splat(729.0) * t1186 * t1105 - f64x8::splat(640.0) / f64x8::splat(243.0) * t1189 * t1105 + f64x8::splat(3520.0) / f64x8::splat(81.0) * t508 * t1081 - f64x8::splat(7040.0) / f64x8::splat(81.0) * t525 * t1081 + f64x8::splat(3520.0) / f64x8::splat(81.0) * t603 * t1073 - f64x8::splat(3520.0) / f64x8::splat(81.0) * t606 * t1077 + f64x8::splat(880.0) / f64x8::splat(243.0) * t609 * t1089 - f64x8::splat(1408.0) / f64x8::splat(27.0) * t612 * t1081 + f64x8::splat(176.0) / f64x8::splat(81.0) * t586 * t1089 + f64x8::splat(1408.0) / f64x8::splat(81.0) * t590 * t1073 - f64x8::splat(704.0) / f64x8::splat(27.0) * t596 * t1077 + f64x8::splat(704.0) / f64x8::splat(243.0) * t599 * t1089 - f64x8::splat(352.0) / f64x8::splat(27.0) * t581 * t1077 + f64x8::splat(352.0) / f64x8::splat(27.0) * t545 * t1081 + f64x8::splat(1408.0) / f64x8::splat(81.0) * t549 * t1081 + f64x8::splat(1760.0) / f64x8::splat(81.0) * t529 * t1081;
            let t1231 = f64x8::splat(1.0) / t18 / t850;
            let t1232 = t31 * t1231;
            let t1234 = t263 * t1232 * t59;
            let t1238 = t210 * t212 * t1231;
            let t1242 = t263 * t1232 * t265;
            let t1246 = f64x8::splat(1.0) / t34 / t344;
            let t1248 = t88 * t89 * t1246;
            let t1253 = t30 * t32 * t1246 * t59;
            let t1258 = t65 * t1092;
            let t1259 = t510 * t1097;
            let t1260 = t1259 * t1098;
            let t1266 = t62 * t1092;
            let t1272 = t45 * t505;
            let t1274 = t1116 * t1104;
            let t1277 = t48 * t1169;
            let t1293 = f64x8::splat(352.0) / f64x8::splat(243.0) * t533 * t1089 + f64x8::splat(704.0) / f64x8::splat(81.0) * t539 * t1081 - f64x8::splat(704.0) / f64x8::splat(27.0) * t542 * t1081 + f64x8::splat(1408.0) / f64x8::splat(81.0) * t615 * t1081 - f64x8::splat(1958.0) / f64x8::splat(729.0) * t317 * t1234 + f64x8::splat(3916.0) / f64x8::splat(729.0) * t312 * t1238 + f64x8::splat(3916.0) / f64x8::splat(729.0) * t321 * t1242 + f64x8::splat(2618.0) / f64x8::splat(243.0) * t87 * t1248 + f64x8::splat(2618.0) / f64x8::splat(243.0) * t117 * t1253 - f64x8::splat(3916.0) / f64x8::splat(729.0) * t308 * t1238 - f64x8::splat(32.0) / f64x8::splat(243.0) * t1258 * t1260 * t1102 * t591 * t1104 + f64x8::splat(112.0) / f64x8::splat(2187.0) * t1266 * t1260 * t1102 * t265 * t1104 + f64x8::splat(64.0) / f64x8::splat(243.0) * t1272 * t1114 * t1274 - f64x8::splat(64.0) / f64x8::splat(243.0) * t1277 * t1114 * t1274 + f64x8::splat(10472.0) / f64x8::splat(243.0) * t132 * t1253 + f64x8::splat(5236.0) / f64x8::splat(243.0) * t124 * t1253 + f64x8::splat(2618.0) / f64x8::splat(81.0) * t128 * t1253 + f64x8::splat(10472.0) / f64x8::splat(243.0) * t108 * t1248 + f64x8::splat(13090.0) / f64x8::splat(243.0) * t136 * t1253 + f64x8::splat(13090.0) / f64x8::splat(243.0) * t112 * t1248;
            let t1326 = t51 * t1183;
            let t1327 = t1092 * t22;
            let t1328 = t1327 * t27;
            let t1329 = t1326 * t1328;
            let t1335 = t21 * t523;
            let t1336 = t1335 * t1114;
            let t1339 = t58 * t1092;
            let t1345 = t68 * t1092;
            let t1346 = t116 * t1097;
            let t1347 = t1345 * t1346;
            let t1354 = f64x8::splat(5236.0) / f64x8::splat(243.0) * t100 * t1248 + f64x8::splat(2618.0) / f64x8::splat(81.0) * t104 * t1248 + f64x8::splat(7832.0) / f64x8::splat(243.0) * t277 * t1242 - f64x8::splat(7832.0) / f64x8::splat(729.0) * t280 * t1234 + f64x8::splat(39160.0) / f64x8::splat(729.0) * t294 * t1238 - f64x8::splat(19580.0) / f64x8::splat(729.0) * t297 * t1238 - f64x8::splat(3916.0) / f64x8::splat(729.0) * t300 * t1234 + f64x8::splat(3916.0) / f64x8::splat(243.0) * t262 * t1242 - f64x8::splat(1958.0) / f64x8::splat(243.0) * t270 * t1234 - f64x8::splat(7832.0) / f64x8::splat(729.0) * t205 * t1238 + f64x8::splat(3916.0) / f64x8::splat(243.0) * t226 * t1238 - f64x8::splat(3916.0) / f64x8::splat(243.0) * t229 * t1238 + f64x8::splat(7832.0) / f64x8::splat(243.0) * t233 * t1238 - f64x8::splat(15664.0) / f64x8::splat(729.0) * t290 * t1238 + f64x8::splat(39160.0) / f64x8::splat(729.0) * t284 * t1242 - f64x8::splat(9790.0) / f64x8::splat(729.0) * t287 * t1234 + f64x8::splat(64.0) / f64x8::splat(729.0) * t1329 * t1099 * t32 * t1102 * t510 - f64x8::splat(64.0) / f64x8::splat(729.0) * t1336 * t1274 - f64x8::splat(8.0) / f64x8::splat(2187.0) * t1339 * t1260 * t1102 * t22 * t1119 + f64x8::splat(64.0) / f64x8::splat(729.0) * t1347 * t1098 * t32 * t1102 * t1153 * t510;
            let t1361 = ((t2).select(f64x8::splat(0.0), f64x8::splat(10.0) / f64x8::splat(27.0) * t6 * t17 * t92 * t74 - f64x8::splat(5.0) / f64x8::splat(9.0) * t6 * t493 * t139 + t6 * t192 * t325 / f64x8::splat(2.0) - t6 * t80 * t691 / f64x8::splat(2.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * (t1164 + t1220 + t1293 + t1354)));
            let tv4rho40 = f64x8::splat(2.0) * v_rho * t1361 + f64x8::splat(8.0) * t696;
            acc_v4rho4 = tv4rho40;
            let t1377 = f64x8::splat(1.0) / t34 / t515 / t421;
            let t1380 = t1097 * t514 * t32 * t1377 * t510;
            let t1383 = t1266 * t1346;
            let t1384 = t514 * t32;
            let t1385 = t1377 * t265;
            let t1390 = t1272 * t1328;
            let t1393 = t1277 * t1328;
            let t1397 = t210 * t565 * v_sigma;
            let t1403 = t513 * t517 * t211;
            let t1408 = t512 * t517;
            let t1409 = t1408 * t716;
            let t1412 = t1408 * t720;
            let t1434 = t1408 * t728;
            let t1439 = -f64x8::splat(8.0) / f64x8::splat(243.0) * t1329 * t1380 - f64x8::splat(14.0) / f64x8::splat(729.0) * t1383 * t1384 * t1385 * t510 - f64x8::splat(8.0) / f64x8::splat(81.0) * t1390 * t1380 + f64x8::splat(8.0) / f64x8::splat(81.0) * t1393 * t1380 - f64x8::splat(341.0) / f64x8::splat(81.0) * t226 * t1397 + f64x8::splat(341.0) / f64x8::splat(81.0) * t229 * t1397 - f64x8::splat(380.0) / f64x8::splat(27.0) * t508 * t1403 + f64x8::splat(760.0) / f64x8::splat(27.0) * t525 * t1403 + f64x8::splat(76.0) / f64x8::splat(9.0) * t596 * t1409 - f64x8::splat(76.0) / f64x8::splat(81.0) * t599 * t1412 - f64x8::splat(152.0) / f64x8::splat(27.0) * t549 * t1403 + f64x8::splat(76.0) / f64x8::splat(9.0) * t542 * t1403 - f64x8::splat(38.0) / f64x8::splat(9.0) * t545 * t1403 - f64x8::splat(154.0) / f64x8::splat(81.0) * t117 * t151 * t553 * t59 - f64x8::splat(154.0) / f64x8::splat(81.0) * t87 * t88 * t558 - f64x8::splat(95.0) / f64x8::splat(81.0) * t609 * t1412 + f64x8::splat(38.0) / f64x8::splat(9.0) * t581 * t1409 - f64x8::splat(19.0) / f64x8::splat(27.0) * t586 * t1412 - f64x8::splat(152.0) / f64x8::splat(27.0) * t590 * t1434 - f64x8::splat(380.0) / f64x8::splat(27.0) * t603 * t1434;
            let t1465 = t1385 * t514 * t1104;
            let t1468 = t1377 * t514;
            let t1470 = t1468 * t22 * t1119;
            let t1477 = t1377 * t591;
            let t1479 = t1477 * t514 * t1104;
            let t1488 = t1097 * t1377 * t514 * t1104;
            let t1491 = f64x8::splat(380.0) / f64x8::splat(27.0) * t606 * t1409 - f64x8::splat(190.0) / f64x8::splat(27.0) * t529 * t1403 - f64x8::splat(38.0) / f64x8::splat(81.0) * t533 * t1412 - f64x8::splat(76.0) / f64x8::splat(27.0) * t539 * t1403 + f64x8::splat(152.0) / f64x8::splat(9.0) * t612 * t1403 - f64x8::splat(152.0) / f64x8::splat(27.0) * t615 * t1403 - f64x8::splat(38.0) / f64x8::splat(27.0) * t637 * t1403 - f64x8::splat(19.0) / f64x8::splat(81.0) * t641 * t1412 - f64x8::splat(38.0) / f64x8::splat(27.0) * t619 * t1434 + f64x8::splat(38.0) / f64x8::splat(27.0) * t623 * t1409 + f64x8::splat(76.0) / f64x8::splat(27.0) * t629 * t1403 - f64x8::splat(38.0) / f64x8::splat(27.0) * t633 * t1403 - f64x8::splat(140.0) / f64x8::splat(729.0) * t1161 * t1465 + f64x8::splat(5.0) / f64x8::splat(729.0) * t1165 * t1470 - f64x8::splat(14.0) / f64x8::splat(243.0) * t1132 * t1465 + t1137 * t1470 / f64x8::splat(243.0) + f64x8::splat(16.0) / f64x8::splat(81.0) * t1140 * t1479 - f64x8::splat(28.0) / f64x8::splat(243.0) * t1145 * t1465 + f64x8::splat(4.0) / f64x8::splat(729.0) * t1148 * t1470 + f64x8::splat(32.0) / f64x8::splat(81.0) * t1172 * t1488;
            let t1513 = t1377 * t1153;
            let t1521 = t116 * t151 * t553;
            let t1524 = t88 * t559;
            let t1539 = t343 * t564 * v_sigma * t59;
            let t1542 = -f64x8::splat(16.0) / f64x8::splat(27.0) * t1175 * t1488 + f64x8::splat(32.0) / f64x8::splat(243.0) * t1178 * t1488 - f64x8::splat(40.0) / f64x8::splat(243.0) * t1186 * t1488 + f64x8::splat(80.0) / f64x8::splat(81.0) * t1189 * t1488 - f64x8::splat(80.0) / f64x8::splat(81.0) * t1095 * t1488 + f64x8::splat(40.0) / f64x8::splat(243.0) * t1110 * t1488 + f64x8::splat(2.0) / f64x8::splat(729.0) * t1115 * t1470 + f64x8::splat(16.0) / f64x8::splat(243.0) * t1123 * t1488 - f64x8::splat(8.0) / f64x8::splat(27.0) * t1126 * t1488 + f64x8::splat(8.0) / f64x8::splat(81.0) * t1129 * t1488 - f64x8::splat(40.0) / f64x8::splat(243.0) * t1152 * t1513 * t514 * t1104 + f64x8::splat(40.0) / f64x8::splat(81.0) * t1158 * t1479 - f64x8::splat(616.0) / f64x8::splat(81.0) * t159 * t1521 - f64x8::splat(154.0) / f64x8::splat(27.0) * t174 * t1524 - f64x8::splat(770.0) / f64x8::splat(81.0) * t162 * t1521 - f64x8::splat(770.0) / f64x8::splat(81.0) * t180 * t1524 - f64x8::splat(308.0) / f64x8::splat(81.0) * t150 * t1521 - f64x8::splat(308.0) / f64x8::splat(81.0) * t169 * t1524 - f64x8::splat(154.0) / f64x8::splat(27.0) * t156 * t1521 + f64x8::splat(341.0) / f64x8::splat(486.0) * t317 * t1539;
            let t1545 = t780 * t574;
            let t1574 = t1258 * t1346;
            let t1594 = f64x8::splat(341.0) / f64x8::splat(243.0) * t308 * t1397 - f64x8::splat(341.0) / f64x8::splat(243.0) * t321 * t1545 - f64x8::splat(341.0) / f64x8::splat(243.0) * t312 * t1397 - f64x8::splat(616.0) / f64x8::splat(81.0) * t177 * t1524 + f64x8::splat(1364.0) / f64x8::splat(243.0) * t290 * t1397 - f64x8::splat(682.0) / f64x8::splat(81.0) * t277 * t1545 + f64x8::splat(682.0) / f64x8::splat(243.0) * t280 * t1539 - f64x8::splat(682.0) / f64x8::splat(81.0) * t233 * t1397 - f64x8::splat(341.0) / f64x8::splat(81.0) * t262 * t1545 + f64x8::splat(341.0) / f64x8::splat(162.0) * t270 * t1539 - f64x8::splat(3410.0) / f64x8::splat(243.0) * t294 * t1397 + f64x8::splat(1705.0) / f64x8::splat(243.0) * t297 * t1397 - f64x8::splat(3410.0) / f64x8::splat(243.0) * t284 * t1545 + f64x8::splat(1705.0) / f64x8::splat(486.0) * t287 * t1539 + f64x8::splat(682.0) / f64x8::splat(243.0) * t205 * t1397 + f64x8::splat(4.0) / f64x8::splat(81.0) * t1574 * t1384 * t1477 * t510 - f64x8::splat(8.0) / f64x8::splat(243.0) * t1347 * t1384 * t1513 * t510 + f64x8::splat(8.0) / f64x8::splat(243.0) * t1336 * t1468 * t1104 + t1339 * t1259 * t1377 * t514 * t22 * t1119 / f64x8::splat(729.0) + f64x8::splat(341.0) / f64x8::splat(243.0) * t300 * t1539;
            let t1601 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(5.0) / f64x8::splat(36.0) * t6 * t493 * t183 + t6 * t192 * t413 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t80 * t839 - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * (t1439 + t1491 + t1542 + t1594)));
            let tv4rho3sigma0 = f64x8::splat(2.0) * v_rho * t1601 + f64x8::splat(6.0) * t844;
            acc_v4rho3sigma = tv4rho3sigma0;
            let t1612 = f64x8::splat(1.0) / t34 / t1070;
            let t1619 = t1612 * t211;
            let t1623 = t211 * t32;
            let t1624 = t1612 * t265;
            let t1629 = t1612 * t591;
            let t1634 = t1612 * t1153;
            let t1642 = t1097 * t211 * t32 * t1612 * t510;
            let t1650 = t513 * t714 * v_sigma;
            let t1663 = t210 * t264;
            let t1674 = -t1339 * t1259 * t1612 * t211 * t22 * t1119 / f64x8::splat(1944.0) - t1336 * t1619 * t1104 / f64x8::splat(81.0) + f64x8::splat(7.0) / f64x8::splat(972.0) * t1383 * t1623 * t1624 * t510 - t1574 * t1623 * t1629 * t510 / f64x8::splat(54.0) + t1347 * t1623 * t1634 * t510 / f64x8::splat(81.0) + t1390 * t1642 / f64x8::splat(27.0) - t1393 * t1642 / f64x8::splat(27.0) + t1329 * t1642 / f64x8::splat(81.0) - f64x8::splat(43.0) / f64x8::splat(9.0) * t612 * t1650 + f64x8::splat(43.0) / f64x8::splat(27.0) * t615 * t1650 + f64x8::splat(215.0) / f64x8::splat(54.0) * t508 * t1650 + f64x8::splat(43.0) / f64x8::splat(54.0) * t539 * t1650 - f64x8::splat(43.0) / f64x8::splat(18.0) * t542 * t1650 + f64x8::splat(43.0) / f64x8::splat(36.0) * t545 * t1650 + f64x8::splat(19.0) / f64x8::splat(81.0) * t312 * t1663 - f64x8::splat(19.0) / f64x8::splat(162.0) * t317 * t343 * t216 * t59 + f64x8::splat(19.0) / f64x8::splat(81.0) * t321 * t343 * t216 * t265;
            let t1675 = t715 * t884;
            let t1678 = t715 * t880;
            let t1681 = t715 * t890;
            let t1714 = -f64x8::splat(43.0) / f64x8::splat(18.0) * t596 * t1675 + f64x8::splat(43.0) / f64x8::splat(162.0) * t599 * t1678 + f64x8::splat(215.0) / f64x8::splat(54.0) * t603 * t1681 - f64x8::splat(215.0) / f64x8::splat(54.0) * t606 * t1675 + f64x8::splat(215.0) / f64x8::splat(648.0) * t609 * t1678 - f64x8::splat(43.0) / f64x8::splat(36.0) * t581 * t1675 + f64x8::splat(43.0) / f64x8::splat(216.0) * t586 * t1678 + f64x8::splat(43.0) / f64x8::splat(27.0) * t590 * t1681 - f64x8::splat(215.0) / f64x8::splat(27.0) * t525 * t1650 + f64x8::splat(215.0) / f64x8::splat(108.0) * t529 * t1650 + f64x8::splat(43.0) / f64x8::splat(324.0) * t533 * t1678 + f64x8::splat(43.0) / f64x8::splat(27.0) * t549 * t1650 - f64x8::splat(19.0) / f64x8::splat(81.0) * t308 * t1663 + f64x8::splat(43.0) / f64x8::splat(108.0) * t619 * t1681 - f64x8::splat(43.0) / f64x8::splat(108.0) * t623 * t1675 - f64x8::splat(43.0) / f64x8::splat(54.0) * t629 * t1650 + f64x8::splat(43.0) / f64x8::splat(108.0) * t633 * t1650 + f64x8::splat(43.0) / f64x8::splat(108.0) * t637 * t1650;
            let t1719 = t1619 * t22 * t1119;
            let t1723 = t1629 * t211 * t1104;
            let t1727 = t1624 * t211 * t1104;
            let t1744 = t1097 * t1612 * t211 * t1104;
            let t1761 = f64x8::splat(43.0) / f64x8::splat(648.0) * t641 * t1678 - t1137 * t1719 / f64x8::splat(648.0) - f64x8::splat(2.0) / f64x8::splat(27.0) * t1140 * t1723 + f64x8::splat(7.0) / f64x8::splat(162.0) * t1145 * t1727 - t1148 * t1719 / f64x8::splat(486.0) + f64x8::splat(5.0) / f64x8::splat(81.0) * t1152 * t1634 * t211 * t1104 - f64x8::splat(5.0) / f64x8::splat(27.0) * t1158 * t1723 + f64x8::splat(35.0) / f64x8::splat(486.0) * t1161 * t1727 - f64x8::splat(5.0) / f64x8::splat(1944.0) * t1165 * t1719 - f64x8::splat(2.0) / f64x8::splat(81.0) * t1123 * t1744 + t1126 * t1744 / f64x8::splat(9.0) - t1129 * t1744 / f64x8::splat(27.0) - f64x8::splat(4.0) / f64x8::splat(27.0) * t1172 * t1744 + f64x8::splat(2.0) / f64x8::splat(9.0) * t1175 * t1744 - f64x8::splat(4.0) / f64x8::splat(81.0) * t1178 * t1744 + f64x8::splat(5.0) / f64x8::splat(81.0) * t1186 * t1744 - f64x8::splat(10.0) / f64x8::splat(27.0) * t1189 * t1744;
            let t1771 = t316 * t343 * t216;
            let t1784 = t210 * t266;
            let t1789 = t210 * t271;
            let t1802 = f64x8::splat(10.0) / f64x8::splat(27.0) * t1095 * t1744 - f64x8::splat(5.0) / f64x8::splat(81.0) * t1110 * t1744 - t1115 * t1719 / f64x8::splat(972.0) + f64x8::splat(7.0) / f64x8::splat(324.0) * t1132 * t1727 - f64x8::splat(38.0) / f64x8::splat(81.0) * t430 * t1771 + f64x8::splat(19.0) / f64x8::splat(27.0) * t435 * t1771 - f64x8::splat(19.0) / f64x8::splat(27.0) * t438 * t1771 + f64x8::splat(38.0) / f64x8::splat(27.0) * t441 * t1771 - f64x8::splat(76.0) / f64x8::splat(81.0) * t444 * t1771 + f64x8::splat(190.0) / f64x8::splat(81.0) * t447 * t1771 + f64x8::splat(38.0) / f64x8::splat(27.0) * t474 * t1784 - f64x8::splat(95.0) / f64x8::splat(81.0) * t450 * t1771 - f64x8::splat(19.0) / f64x8::splat(81.0) * t461 * t1789 - f64x8::splat(95.0) / f64x8::splat(162.0) * t483 * t1789 + f64x8::splat(19.0) / f64x8::splat(27.0) * t466 * t1784 - f64x8::splat(19.0) / f64x8::splat(54.0) * t471 * t1789 - f64x8::splat(38.0) / f64x8::splat(81.0) * t477 * t1789 + f64x8::splat(190.0) / f64x8::splat(81.0) * t480 * t1784;
            let t1809 = ((t2).select(f64x8::splat(0.0), t6 * t192 * t486 / f64x8::splat(12.0) - t6 * t80 * t961 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * (t1674 + t1714 + t1761 + t1802)));
            let tv4rho2sigma20 = f64x8::splat(2.0) * v_rho * t1809 + f64x8::splat(4.0) * t966;
            acc_v4rho2sigma2 = tv4rho2sigma20;
            let t1817 = f64x8::splat(1.0) / t34 / t516;
            let t1820 = t1097 * v_sigma * t32 * t1817 * t510;
            let t1827 = t1817 * t265;
            let t1832 = t1817 * t591;
            let t1837 = t1817 * t1153;
            let t1842 = t1817 * t22;
            let t1844 = t27 * v_sigma * t32;
            let t1850 = t89 * t59;
            let t1854 = t532 * t879;
            let t1858 = t513 * t851 * t59;
            let t1862 = t513 * t851 * t265;
            let t1869 = -t1390 * t1820 / f64x8::splat(72.0) + t1393 * t1820 / f64x8::splat(72.0) - t1329 * t1820 / f64x8::splat(216.0) - f64x8::splat(7.0) / f64x8::splat(2592.0) * t1383 * t89 * t1827 * t510 + t1574 * t89 * t1832 * t510 / f64x8::splat(144.0) - t1347 * t89 * t1837 * t510 / f64x8::splat(216.0) + t1336 * t1842 * t1844 / f64x8::splat(216.0) + t1339 * t1259 * t1817 * t116 * t1850 / f64x8::splat(5184.0) - f64x8::splat(5.0) / f64x8::splat(12.0) * t996 * t1854 - t999 * t1858 / f64x8::splat(36.0) + t1004 * t1862 / f64x8::splat(4.0) - t1009 * t1858 / f64x8::splat(24.0) - t978 * t1854 / f64x8::splat(4.0);
            let t1884 = t513 * t851 * t591;
            let t1897 = -t981 * t1854 / f64x8::splat(3.0) + t984 * t1854 - t987 * t1854 / f64x8::splat(3.0) - f64x8::splat(5.0) / f64x8::splat(6.0) * t990 * t1854 - t969 * t1854 / f64x8::splat(6.0) + t975 * t1854 / f64x8::splat(2.0) - f64x8::splat(5.0) / f64x8::splat(72.0) * t1030 * t1858 - t1013 * t1884 / f64x8::splat(3.0) + t1018 * t1862 / f64x8::splat(2.0) - t1021 * t1858 / f64x8::splat(18.0) - f64x8::splat(5.0) / f64x8::splat(6.0) * t1024 * t1884 + f64x8::splat(5.0) / f64x8::splat(6.0) * t1027 * t1862 + f64x8::splat(5.0) / f64x8::splat(3.0) * t993 * t1854;
            let t1900 = t1827 * t22 * t1844;
            let t1904 = t1842 * t27 * t1850;
            let t1909 = t1097 * t1817 * t22 * t1844;
            let t1932 = -f64x8::splat(35.0) / f64x8::splat(1296.0) * t1161 * t1900 + f64x8::splat(5.0) / f64x8::splat(5184.0) * t1165 * t1904 + t1123 * t1909 / f64x8::splat(108.0) - t1126 * t1909 / f64x8::splat(24.0) + t1129 * t1909 / f64x8::splat(72.0) + t1172 * t1909 / f64x8::splat(18.0) - t1175 * t1909 / f64x8::splat(12.0) + t1178 * t1909 / f64x8::splat(54.0) - f64x8::splat(5.0) / f64x8::splat(216.0) * t1186 * t1909 + f64x8::splat(5.0) / f64x8::splat(36.0) * t1189 * t1909 - f64x8::splat(5.0) / f64x8::splat(36.0) * t1095 * t1909 + f64x8::splat(5.0) / f64x8::splat(216.0) * t1110 * t1909 + t1115 * t1904 / f64x8::splat(2592.0);
            let t1938 = t1832 * t22 * t1844;
            let t1960 = t513 * t851;
            let t1967 = -f64x8::splat(7.0) / f64x8::splat(864.0) * t1132 * t1900 + t1137 * t1904 / f64x8::splat(1728.0) + t1140 * t1938 / f64x8::splat(36.0) - f64x8::splat(7.0) / f64x8::splat(432.0) * t1145 * t1900 + t1148 * t1904 / f64x8::splat(1296.0) - f64x8::splat(5.0) / f64x8::splat(216.0) * t1152 * t1837 * t22 * t1844 + f64x8::splat(5.0) / f64x8::splat(72.0) * t1158 * t1938 - t641 * t879 * t59 / f64x8::splat(72.0) + t623 * t879 * t265 / f64x8::splat(12.0) - t619 * t879 * t591 / f64x8::splat(12.0) - t637 * t1960 / f64x8::splat(12.0) + t629 * t1960 / f64x8::splat(6.0) - t633 * t1960 / f64x8::splat(12.0);
            let t1974 = ((t2).select(f64x8::splat(0.0), -t6 * t80 * t1050 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * (t1869 + t1897 + t1932 + t1967)));
            let tv4rhosigma30 = f64x8::splat(2.0) * v_rho * t1974 + f64x8::splat(2.0) * t1054;
            acc_v4rhosigma3 = tv4rhosigma30;
            let t1978 = f64x8::splat(1.0) / t34 / t713;
            let t1980 = t1097 * t1978 * t1104;
            let t1988 = t1978 * t22 * t1119;
            let t1991 = t1978 * t265;
            let t1992 = t1991 * t1104;
            let t1997 = t1978 * t591;
            let t1998 = t1997 * t1104;
            let t2005 = t1978 * t1153;
            let t2017 = t1175 * t1980 / f64x8::splat(32.0) + f64x8::splat(5.0) / f64x8::splat(96.0) * t1095 * t1980 - f64x8::splat(5.0) / f64x8::splat(576.0) * t1110 * t1980 - t1115 * t1988 / f64x8::splat(6912.0) + f64x8::splat(7.0) / f64x8::splat(2304.0) * t1132 * t1992 - t1137 * t1988 / f64x8::splat(4608.0) - t1140 * t1998 / f64x8::splat(96.0) + f64x8::splat(7.0) / f64x8::splat(1152.0) * t1145 * t1992 - t1148 * t1988 / f64x8::splat(3456.0) + f64x8::splat(5.0) / f64x8::splat(576.0) * t1152 * t2005 * t1104 - f64x8::splat(5.0) / f64x8::splat(192.0) * t1158 * t1998 + f64x8::splat(35.0) / f64x8::splat(3456.0) * t1161 * t1992 - f64x8::splat(5.0) / f64x8::splat(13824.0) * t1165 * t1988 - t1123 * t1980 / f64x8::splat(288.0);
            let t2040 = t27 * t1097 * t32 * t1978 * t510;
            let t2044 = t1097 * t32;
            let t2065 = t1126 * t1980 / f64x8::splat(64.0) - t1129 * t1980 / f64x8::splat(192.0) - t1172 * t1980 / f64x8::splat(48.0) - t1178 * t1980 / f64x8::splat(144.0) + f64x8::splat(5.0) / f64x8::splat(576.0) * t1186 * t1980 - f64x8::splat(5.0) / f64x8::splat(96.0) * t1189 * t1980 - t1335 * t1113 * t1980 / f64x8::splat(576.0) - t1339 * t1259 * t1988 / f64x8::splat(13824.0) + t1326 * t1327 * t2040 / f64x8::splat(576.0) + f64x8::splat(7.0) / f64x8::splat(6912.0) * t1266 * t116 * t2044 * t1991 * t510 - t1258 * t116 * t2044 * t1997 * t510 / f64x8::splat(384.0) + t1345 * t116 * t2044 * t2005 * t510 / f64x8::splat(576.0) + t1272 * t1327 * t2040 / f64x8::splat(192.0) - t1277 * t1327 * t2040 / f64x8::splat(192.0);
            let t2070 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * (t2017 + t2065)));
            let tv4sigma40 = f64x8::splat(2.0) * v_rho * t2070;
            acc_v4sigma4 = tv4sigma40;
        }
        store_add(zk, ip, m, acc_zk);
        store_add(vrho, ip, m, acc_vrho);
        store_add(vsigma, ip, m, acc_vsigma);
        store_add(v2rho2, ip, m, acc_v2rho2);
        store_add(v2rhosigma, ip, m, acc_v2rhosigma);
        store_add(v2sigma2, ip, m, acc_v2sigma2);
        store_add(v3rho3, ip, m, acc_v3rho3);
        store_add(v3rho2sigma, ip, m, acc_v3rho2sigma);
        store_add(v3rhosigma2, ip, m, acc_v3rhosigma2);
        store_add(v3sigma3, ip, m, acc_v3sigma3);
        store_add(v4rho4, ip, m, acc_v4rho4);
        store_add(v4rho3sigma, ip, m, acc_v4rho3sigma);
        store_add(v4rho2sigma2, ip, m, acc_v4rho2sigma2);
        store_add(v4rhosigma3, ip, m, acc_v4rhosigma3);
        store_add(v4sigma4, ip, m, acc_v4sigma4);
        ip += 8;
    }
}
