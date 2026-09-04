//! GGA_X_BPCCAC fxc unpol kernel — explicit SIMD (bit-exact).
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

#[allow(unused_variables, non_snake_case)]
pub fn gga_x_bpccac_fxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    v2rho2: &mut [f64],
    v2rhosigma: &mut [f64],
    v2sigma2: &mut [f64],
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
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
            let t20 = ((v_sigma).sqrt());
            let t21 = f64x8::splat(M_CBRT2);
            let t24 = f64x8::splat(1.0) / t18 / v_rho;
            let t25 = t20 * t21 * t24;
            let t27 = (simd::exp(-t25 + f64x8::splat(19.0)));
            let t28 = f64x8::splat(1.0) + t27;
            let t29 = f64x8::splat(1.0) / t28;
            let t30 = f64x8::splat(1.0) - t29;
            let t31 = f64x8::splat(M_CBRT6);
            let t32 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t33 = (simd::cbrt(t32));
            let t34 = t33 * t33;
            let t35 = f64x8::splat(1.0) / t34;
            let t36 = t31 * t35;
            let t37 = t21 * t21;
            let t38 = v_sigma * t37;
            let t39 = v_rho * v_rho;
            let t40 = t18 * t18;
            let t42 = f64x8::splat(1.0) / t40 / t39;
            let t43 = t38 * t42;
            let t44 = t36 * t43;
            let t46 = f64x8::splat(1.227) + f64x8::splat(0.009146457198521547) * t44;
            let t49 = f64x8::splat(2.227) - f64x8::splat(1.505529) / t46;
            let t52 = (simd::exp(-f64x8::splat(25.0) / f64x8::splat(6.0) * t44));
            let t55 = (f64x8::splat(0.2743) - f64x8::splat(0.1508) * t52) * t31;
            let t56 = t55 * t35;
            let t59 = t31 * t31;
            let t61 = f64x8::splat(1.0) / t33 / t32;
            let t62 = t59 * t61;
            let t63 = v_sigma * v_sigma;
            let t64 = t63 * t21;
            let t65 = t39 * t39;
            let t66 = t65 * v_rho;
            let t68 = f64x8::splat(1.0) / t18 / t66;
            let t71 = f64x8::splat(1.388888888888889e-05) * t62 * t64 * t68;
            let t72 = t56 * t43 / f64x8::splat(24.0) - t71;
            let t74 = t59 / t33;
            let t75 = t74 * t20;
            let t76 = t21 * t24;
            let t79 = (simd::ln(f64x8::splat(0.6496333333333333) * t74 * t25 + ((((f64x8::splat(0.6496333333333333) * t74 * t25) * (f64x8::splat(0.6496333333333333) * t74 * t25)) + f64x8::splat(1.0)).sqrt())));
            let t80 = t76 * t79;
            let t83 = f64x8::splat(1.0) + f64x8::splat(0.016370833333333334) * t75 * t80 + t71;
            let t84 = f64x8::splat(1.0) / t83;
            let t86 = t72 * t84 + f64x8::splat(1.0);
            let t88 = t29 * t86 + t30 * t49;
            let t92 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t88));
            let tzk0 = f64x8::splat(2.0) * t92;
            acc_zk = tzk0;
            let t94 = t17 / t40;
            let t98 = t28 * t28;
            let t99 = f64x8::splat(1.0) / t98;
            let t100 = t99 * t20;
            let t101 = t100 * t21;
            let t103 = f64x8::splat(1.0) / t18 / t39;
            let t105 = t103 * t27 * t49;
            let t108 = t46 * t46;
            let t109 = f64x8::splat(1.0) / t108;
            let t111 = t30 * t109 * t31;
            let t112 = t35 * v_sigma;
            let t113 = t39 * v_rho;
            let t115 = f64x8::splat(1.0) / t40 / t113;
            let t116 = t37 * t115;
            let t120 = t99 * t86;
            let t121 = t120 * t20;
            let t122 = t21 * t103;
            let t123 = t122 * t27;
            let t126 = t62 * t63;
            let t127 = t65 * t39;
            let t129 = f64x8::splat(1.0) / t18 / t127;
            let t130 = t21 * t129;
            let t131 = t130 * t52;
            let t139 = f64x8::splat(7.407407407407407e-05) * t62 * t64 * t129;
            let t140 = -f64x8::splat(0.13962962962962963) * t126 * t131 - t56 * t38 * t115 / f64x8::splat(9.0) + t139;
            let t142 = t83 * t83;
            let t143 = f64x8::splat(1.0) / t142;
            let t144 = t72 * t143;
            let t145 = t122 * t79;
            let t148 = t36 * v_sigma;
            let t150 = f64x8::splat(2.532140806666667) * t44 + f64x8::splat(1.0);
            let t151 = ((t150).sqrt());
            let t152 = f64x8::splat(1.0) / t151;
            let t153 = t116 * t152;
            let t156 = -f64x8::splat(0.02182777777777778) * t75 * t145 - f64x8::splat(0.08508031222222222) * t148 * t153 - t139;
            let t158 = t140 * t84 - t144 * t156;
            let t160 = f64x8::splat(4.0) / f64x8::splat(3.0) * t101 * t105 - f64x8::splat(0.03672068415902118) * t111 * t112 * t116 - f64x8::splat(4.0) / f64x8::splat(3.0) * t121 * t123 + t29 * t158;
            let t165 = ((t2).select(f64x8::splat(0.0), -t6 * t94 * t88 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t160));
            let tvrho0 = f64x8::splat(2.0) * v_rho * t165 + f64x8::splat(2.0) * t92;
            acc_vrho = tvrho0;
            let t168 = f64x8::splat(1.0) / t20;
            let t169 = t99 * t168;
            let t170 = t169 * t21;
            let t172 = t24 * t27 * t49;
            let t175 = t35 * t37;
            let t176 = t175 * t42;
            let t179 = t120 * t168;
            let t180 = t76 * t27;
            let t183 = t62 * t21;
            let t190 = v_sigma * t21;
            let t193 = f64x8::splat(2.777777777777778e-05) * t62 * t190 * t68;
            let t194 = f64x8::splat(0.05236111111111111) * t183 * t68 * t52 * v_sigma + t55 * t176 / f64x8::splat(24.0) - t193;
            let t196 = t74 * t168;
            let t199 = t37 * t42;
            let t200 = t199 * t152;
            let t203 = f64x8::splat(0.008185416666666667) * t196 * t80 + f64x8::splat(0.03190511708333333) * t36 * t200 + t193;
            let t205 = -t144 * t203 + t194 * t84;
            let t207 = -t170 * t172 / f64x8::splat(2.0) + f64x8::splat(0.013770256559632944) * t111 * t176 + t179 * t180 / f64x8::splat(2.0) + t29 * t205;
            let t211 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t207));
            let tvsigma0 = f64x8::splat(2.0) * v_rho * t211;
            acc_vsigma = tvsigma0;
            let t216 = t17 / t40 / v_rho;
            let t224 = f64x8::splat(1.0) / t98 / t28;
            let t226 = t224 * v_sigma * t37;
            let t228 = f64x8::splat(1.0) / t40 / t65;
            let t229 = t27 * t27;
            let t231 = t228 * t229 * t49;
            let t235 = f64x8::splat(1.0) / t18 / t113;
            let t237 = t235 * t27 * t49;
            let t240 = t99 * v_sigma;
            let t241 = t240 * t37;
            let t243 = t228 * t27 * t49;
            let t246 = t20 * v_sigma;
            let t247 = t99 * t246;
            let t248 = f64x8::splat(1.0) / t127;
            let t251 = t27 * t109 * t36;
            let t255 = f64x8::splat(1.0) / t108 / t46;
            let t257 = t30 * t255 * t59;
            let t258 = t61 * t63;
            let t259 = t65 * t113;
            let t261 = f64x8::splat(1.0) / t18 / t259;
            let t262 = t21 * t261;
            let t266 = t37 * t228;
            let t270 = t224 * t86;
            let t271 = t270 * v_sigma;
            let t272 = t266 * t229;
            let t275 = t99 * t158;
            let t276 = t275 * t20;
            let t279 = t21 * t235;
            let t280 = t279 * t27;
            let t283 = t120 * v_sigma;
            let t284 = t266 * t27;
            let t287 = t262 * t52;
            let t290 = t32 * t32;
            let t291 = f64x8::splat(1.0) / t290;
            let t292 = t63 * v_sigma;
            let t293 = t291 * t292;
            let t294 = t65 * t65;
            let t295 = t294 * t39;
            let t296 = f64x8::splat(1.0) / t295;
            let t305 = f64x8::splat(0.0004691358024691358) * t62 * t64 * t261;
            let t306 = f64x8::splat(1.2566666666666666) * t126 * t287 - f64x8::splat(18.617283950617285) * t293 * t296 * t52 + f64x8::splat(11.0) / f64x8::splat(27.0) * t56 * t38 * t228 - t305;
            let t308 = t140 * t143;
            let t312 = f64x8::splat(1.0) / t142 / t83;
            let t313 = t72 * t312;
            let t314 = t156 * t156;
            let t317 = t279 * t79;
            let t320 = t266 * t152;
            let t324 = f64x8::splat(1.0) / t151 / t150;
            let t325 = t262 * t324;
            let t328 = f64x8::splat(0.05093148148148148) * t75 * t317 + f64x8::splat(0.4254015611111111) * t148 * t320 - f64x8::splat(0.5744942144582124) * t126 * t325 + t305;
            let t330 = -t144 * t328 - f64x8::splat(2.0) * t308 * t156 + t306 * t84 + f64x8::splat(2.0) * t313 * t314;
            let t332 = -f64x8::splat(32.0) / f64x8::splat(9.0) * t226 * t231 - f64x8::splat(28.0) / f64x8::splat(9.0) * t101 * t237 + f64x8::splat(16.0) / f64x8::splat(9.0) * t241 * t243 - f64x8::splat(0.19584364884811298) * t247 * t248 * t251 - f64x8::splat(0.0035825511035830976) * t257 * t258 * t262 + f64x8::splat(0.1346425085830777) * t111 * t112 * t266 + f64x8::splat(32.0) / f64x8::splat(9.0) * t271 * t272 - f64x8::splat(8.0) / f64x8::splat(3.0) * t276 * t123 + f64x8::splat(28.0) / f64x8::splat(9.0) * t121 * t280 - f64x8::splat(16.0) / f64x8::splat(9.0) * t283 * t284 + t29 * t330;
            let t337 = ((t2).select(f64x8::splat(0.0), t6 * t216 * t88 / f64x8::splat(12.0) - t6 * t94 * t160 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t332));
            let tv2rho20 = f64x8::splat(2.0) * v_rho * t337 + f64x8::splat(4.0) * t165;
            acc_v2rho2 = tv2rho20;
            let t343 = t224 * t37;
            let t345 = t115 * t229 * t49;
            let t350 = t99 * t37;
            let t352 = t115 * t27 * t49;
            let t355 = f64x8::splat(1.0) / t66;
            let t359 = t61 * t21;
            let t364 = t175 * t115;
            let t367 = t116 * t229;
            let t370 = t275 * t168;
            let t375 = t116 * t27;
            let t378 = t99 * t205;
            let t379 = t378 * t20;
            let t386 = t294 * v_rho;
            let t387 = f64x8::splat(1.0) / t386;
            let t388 = t291 * t387;
            let t389 = t63 * t52;
            let t396 = f64x8::splat(0.00014814814814814815) * t62 * t190 * t129;
            let t397 = -f64x8::splat(0.41888888888888887) * t183 * t129 * t52 * v_sigma + f64x8::splat(6.981481481481482) * t388 * t389 - t55 * t364 / f64x8::splat(9.0) + t396;
            let t399 = t194 * t143;
            let t402 = t203 * t156;
            let t413 = -f64x8::splat(0.01091388888888889) * t196 * t145 - f64x8::splat(0.12762046833333332) * t36 * t153 + f64x8::splat(0.21543533042182963) * t183 * t129 * t324 * v_sigma - t396;
            let t415 = -t144 * t413 - t399 * t156 - t308 * t203 + f64x8::splat(2.0) * t313 * t402 + t397 * t84;
            let t417 = f64x8::splat(4.0) / f64x8::splat(3.0) * t343 * t345 + f64x8::splat(2.0) / f64x8::splat(3.0) * t170 * t105 - f64x8::splat(2.0) / f64x8::splat(3.0) * t350 * t352 + f64x8::splat(0.07344136831804236) * t100 * t355 * t251 + f64x8::splat(0.0013434566638436617) * t257 * t359 * t129 * v_sigma - f64x8::splat(0.03672068415902118) * t111 * t364 - f64x8::splat(4.0) / f64x8::splat(3.0) * t270 * t367 + t370 * t180 / f64x8::splat(2.0) - f64x8::splat(2.0) / f64x8::splat(3.0) * t179 * t123 + f64x8::splat(2.0) / f64x8::splat(3.0) * t120 * t375 - f64x8::splat(4.0) / f64x8::splat(3.0) * t379 * t123 + t29 * t415;
            let t422 = ((t2).select(f64x8::splat(0.0), -t6 * t94 * t207 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t417));
            let tv2rhosigma0 = f64x8::splat(2.0) * v_rho * t422 + f64x8::splat(2.0) * t211;
            acc_v2rhosigma = tv2rhosigma0;
            let t425 = f64x8::splat(1.0) / v_sigma;
            let t426 = t224 * t425;
            let t427 = t426 * t37;
            let t429 = t42 * t229 * t49;
            let t432 = f64x8::splat(1.0) / t246;
            let t433 = t99 * t432;
            let t434 = t433 * t21;
            let t437 = t99 * t425;
            let t438 = t437 * t37;
            let t440 = t42 * t27 * t49;
            let t443 = f64x8::splat(1.0) / t65;
            let t450 = t270 * t425;
            let t451 = t199 * t229;
            let t454 = t378 * t168;
            let t456 = t120 * t432;
            let t459 = t120 * t425;
            let t460 = t199 * t27;
            let t463 = f64x8::splat(1.0) / t294;
            let t464 = t291 * t463;
            let t465 = t52 * v_sigma;
            let t468 = t21 * t68;
            let t473 = f64x8::splat(2.777777777777778e-05) * t62 * t468;
            let t474 = -f64x8::splat(2.6180555555555554) * t464 * t465 + f64x8::splat(0.10472222222222222) * t62 * t468 * t52 - t473;
            let t478 = t203 * t203;
            let t481 = t74 * t432;
            let t484 = t36 * t425;
            let t487 = t468 * t324;
            let t490 = -f64x8::splat(0.004092708333333334) * t481 * t80 + f64x8::splat(0.015952558541666665) * t484 * t200 - f64x8::splat(0.08078824890818612) * t62 * t487 + t473;
            let t492 = -t144 * t490 - f64x8::splat(2.0) * t399 * t203 + f64x8::splat(2.0) * t313 * t478 + t474 * t84;
            let t494 = -t427 * t429 / f64x8::splat(2.0) + t434 * t172 / f64x8::splat(4.0) + t438 * t440 / f64x8::splat(4.0) - f64x8::splat(0.027540513119265888) * t169 * t443 * t251 - f64x8::splat(0.0005037962489413731) * t257 * t359 * t68 + t450 * t451 / f64x8::splat(2.0) + t454 * t180 - t456 * t180 / f64x8::splat(4.0) - t459 * t460 / f64x8::splat(4.0) + t29 * t492;
            let t498 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t494));
            let tv2sigma20 = f64x8::splat(2.0) * v_rho * t498;
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
