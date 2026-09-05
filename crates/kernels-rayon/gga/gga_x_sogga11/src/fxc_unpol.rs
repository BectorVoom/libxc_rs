//! GGA_X_SOGGA11 fxc unpol kernel — explicit SIMD (bit-exact).
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

#[allow(unused_variables, non_snake_case)]
pub fn gga_x_sogga11_fxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    v2rho2: &mut [f64],
    v2rhosigma: &mut [f64],
    v2sigma2: &mut [f64],
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
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_vrho.into(); vrho[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_vsigma.into(); vsigma[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v2rho2.into(); v2rho2[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v2rhosigma.into(); v2rhosigma[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v2sigma2.into(); v2sigma2[ip..ip + m].copy_from_slice(&a[..m]); }
        ip += 8;
    }
}
