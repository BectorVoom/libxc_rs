//! MGGA_X_RPPSCAN vxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_rppscan.c`
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
pub fn mgga_x_rppscan_vxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    vlapl: &mut [f64],
    vtau: &mut [f64],
    param_c2: f64,
    param_d: f64,
    param_eta: f64,
    param_k1: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_c2 = f64x8::splat(param_c2);
    let param_d = f64x8::splat(param_d);
    let param_eta = f64x8::splat(param_eta);
    let param_k1 = f64x8::splat(param_k1);
    let dens_threshold = f64x8::splat(dens_threshold);
    let zeta_threshold = f64x8::splat(zeta_threshold);
    let mut ip = 0usize;
    while ip < np {
        let m = (np - ip).min(8);
        let v_rho = load(rho, ip, np);
        let v_sigma = load(sigma, ip, np);
        let v_lapl = load(lapl, ip, np);
        let v_tau = load(tau, ip, np);
        let mut acc_zk = V_ZERO;
        let mut acc_vrho = V_ZERO;
        let mut acc_vsigma = V_ZERO;
        let mut acc_vlapl = V_ZERO;
        let mut acc_vtau = V_ZERO;
        {
            let t3 = (v_rho / f64x8::splat(2.0)).simd_le(dens_threshold);
            let t4 = f64x8::splat(M_CBRT3);
            let t5 = f64x8::splat(M_CBRTPI);
            let t8 = (f64x8::splat(1.0)).simd_le(zeta_threshold);
            let t9 = zeta_threshold - f64x8::splat(1.0);
            let t11 = ((t8).select(t9, (t8).select(-t9, f64x8::splat(0.0))));
            let t12 = f64x8::splat(1.0) + t11;
            let t14 = (simd::cbrt(zeta_threshold));
            let t16 = (simd::cbrt(t12));
            let t18 = (((t12).simd_le(zeta_threshold)).select(t14 * zeta_threshold, t16 * t12));
            let t19 = t4 / t5 * t18;
            let t20 = (simd::cbrt(v_rho));
            let t21 = f64x8::splat(M_CBRT6);
            let t22 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t23 = (simd::cbrt(t22));
            let t24 = t23 * t23;
            let t25 = f64x8::splat(1.0) / t24;
            let t26 = t21 * t25;
            let t27 = f64x8::splat(M_CBRT2);
            let t28 = t27 * t27;
            let t29 = v_sigma * t28;
            let t30 = v_rho * v_rho;
            let t31 = t20 * t20;
            let t33 = f64x8::splat(1.0) / t31 / t30;
            let t34 = t29 * t33;
            let t35 = t26 * t34;
            let t39 = f64x8::splat(100.0) / f64x8::splat(6561.0) / param_k1 - f64x8::splat(73.0) / f64x8::splat(648.0);
            let t40 = t21 * t21;
            let t42 = t23 * t22;
            let t43 = f64x8::splat(1.0) / t42;
            let t44 = t39 * t40 * t43;
            let t45 = v_sigma * v_sigma;
            let t46 = t45 * t27;
            let t47 = t30 * t30;
            let t48 = t47 * v_rho;
            let t50 = f64x8::splat(1.0) / t20 / t48;
            let t55 = (simd::exp(-f64x8::splat(27.0) / f64x8::splat(80.0) * t39 * t21 * t25 * t34));
            let t56 = t50 * t55;
            let t60 = ((f64x8::splat(146.0)).sqrt());
            let t61 = t60 * t21;
            let t62 = t61 * t25;
            let t65 = v_tau * t28;
            let t66 = t31 * v_rho;
            let t67 = f64x8::splat(1.0) / t66;
            let t70 = t65 * t67 - t34 / f64x8::splat(8.0);
            let t73 = param_eta * v_sigma;
            let t74 = t28 * t33;
            let t77 = f64x8::splat(3.0) / f64x8::splat(10.0) * t40 * t24 + t73 * t74 / f64x8::splat(8.0);
            let t78 = f64x8::splat(1.0) / t77;
            let t79 = t70 * t78;
            let t80 = f64x8::splat(1.0) - t79;
            let t82 = t80 * t80;
            let t84 = (simd::exp(-t82 / f64x8::splat(2.0)));
            let t87 = f64x8::splat(7.0) / f64x8::splat(12960.0) * t62 * t34 + t60 * t80 * t84 / f64x8::splat(100.0);
            let t88 = t87 * t87;
            let t89 = param_k1 + f64x8::splat(5.0) / f64x8::splat(972.0) * t35 + t44 * t46 * t56 / f64x8::splat(288.0) + t88;
            let t94 = f64x8::splat(1.0) + param_k1 * (f64x8::splat(1.0) - param_k1 / t89);
            let t95 = (t79).simd_le(f64x8::splat(2.5));
            let t96 = (f64x8::splat(2.5)).simd_lt(t79);
            let t97 = ((t96).select(f64x8::splat(2.5), t79));
            let t99 = t97 * t97;
            let t101 = t99 * t97;
            let t103 = t99 * t99;
            let t105 = t103 * t97;
            let t107 = t103 * t99;
            let t112 = ((t96).select(t79, f64x8::splat(2.5)));
            let t113 = f64x8::splat(1.0) - t112;
            let t116 = (simd::exp(param_c2 / t113));
            let t118 = ((t95).select(f64x8::splat(1.0) - f64x8::splat(0.667) * t97 - f64x8::splat(0.4445555) * t99 - f64x8::splat(0.663086601049) * t101 + f64x8::splat(1.45129704449) * t103 - f64x8::splat(0.887998041597) * t105 + f64x8::splat(0.234528941479) * t107 - f64x8::splat(0.023185843322) * t103 * t101, -param_d * t116));
            let t119 = f64x8::splat(1.0) - t118;
            let t122 = t94 * t119 + f64x8::splat(1.174) * t118;
            let t124 = ((f64x8::splat(3.0)).sqrt());
            let t125 = f64x8::splat(1.0) / t23;
            let t126 = t40 * t125;
            let t127 = ((v_sigma).sqrt());
            let t128 = t127 * t27;
            let t130 = f64x8::splat(1.0) / t20 / v_rho;
            let t132 = t126 * t128 * t130;
            let t133 = ((t132).sqrt());
            let t137 = (simd::exp(-f64x8::splat(9.8958) * t124 / t133));
            let t138 = f64x8::splat(1.0) - t137;
            let t142 = ((t3).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t19 * t20 * t122 * t138));
            let tzk0 = f64x8::splat(2.0) * t142;
            acc_zk = tzk0;
            let t143 = f64x8::splat(1.0) / t31;
            let t148 = param_k1 * param_k1;
            let t149 = t89 * t89;
            let t151 = t148 / t149;
            let t152 = t30 * v_rho;
            let t154 = f64x8::splat(1.0) / t31 / t152;
            let t155 = t29 * t154;
            let t158 = t47 * t30;
            let t160 = f64x8::splat(1.0) / t20 / t158;
            let t161 = t160 * t55;
            let t165 = t39 * t39;
            let t166 = t22 * t22;
            let t167 = f64x8::splat(1.0) / t166;
            let t168 = t165 * t167;
            let t169 = t45 * v_sigma;
            let t170 = t47 * t47;
            let t171 = t170 * v_rho;
            let t172 = f64x8::splat(1.0) / t171;
            let t182 = -f64x8::splat(5.0) / f64x8::splat(3.0) * t65 * t33 + t155 / f64x8::splat(3.0);
            let t184 = t77 * t77;
            let t185 = f64x8::splat(1.0) / t184;
            let t186 = t70 * t185;
            let t187 = t186 * param_eta;
            let t190 = -t182 * t78 - t187 * t155 / f64x8::splat(3.0);
            let t194 = t60 * t82;
            let t195 = t190 * t84;
            let t198 = -f64x8::splat(7.0) / f64x8::splat(4860.0) * t62 * t155 + t60 * t190 * t84 / f64x8::splat(100.0) - t194 * t195 / f64x8::splat(100.0);
            let t201 = -f64x8::splat(10.0) / f64x8::splat(729.0) * t26 * t155 - t44 * t46 * t161 / f64x8::splat(54.0) + f64x8::splat(3.0) / f64x8::splat(80.0) * t168 * t169 * t172 * t55 + f64x8::splat(2.0) * t87 * t198;
            let t202 = t201 * t119;
            let t204 = -t190;
            let t205 = ((t96).select(f64x8::splat(0.0), t204));
            let t207 = t97 * t205;
            let t209 = t99 * t205;
            let t211 = t101 * t205;
            let t213 = t103 * t205;
            let t215 = t105 * t205;
            let t220 = param_d * param_c2;
            let t221 = t113 * t113;
            let t222 = f64x8::splat(1.0) / t221;
            let t223 = ((t96).select(t204, f64x8::splat(0.0)));
            let t227 = ((t95).select(-f64x8::splat(0.667) * t205 - f64x8::splat(0.889111) * t207 - f64x8::splat(1.989259803147) * t209 + f64x8::splat(5.80518817796) * t211 - f64x8::splat(4.439990207985) * t213 + f64x8::splat(1.407173648874) * t215 - f64x8::splat(0.162300903254) * t107 * t205, -t220 * t222 * t223 * t116));
            let t230 = t151 * t202 - t94 * t227 + f64x8::splat(1.174) * t227;
            let t235 = (simd::pow(f64x8::splat(3.0), f64x8::splat(1.0) / f64x8::splat(6.0)));
            let t236 = t235 * t235;
            let t237 = t236 * t236;
            let t239 = t237 * t235 * t18;
            let t240 = f64x8::splat(1.0) / t30;
            let t241 = t240 * t122;
            let t243 = f64x8::splat(1.0) / t133 / t132;
            let t245 = t239 * t241 * t243;
            let t247 = t126 * t128 * t137;
            let t251 = ((t3).select(f64x8::splat(0.0), -t19 * t143 * t122 * t138 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t19 * t20 * t230 * t138 - f64x8::splat(1.6891736332904388) * t245 * t247));
            let tvrho0 = f64x8::splat(2.0) * v_rho * t251 + f64x8::splat(2.0) * t142;
            acc_vrho = tvrho0;
            let t256 = v_sigma * t27;
            let t260 = f64x8::splat(1.0) / t170;
            let t265 = t25 * t28;
            let t269 = t74 * t78;
            let t270 = param_eta * t28;
            let t271 = t270 * t33;
            let t274 = t186 * t271 / f64x8::splat(8.0) + t269 / f64x8::splat(8.0);
            let t275 = t60 * t274;
            let t278 = t274 * t84;
            let t281 = f64x8::splat(7.0) / f64x8::splat(12960.0) * t61 * t265 * t33 + t275 * t84 / f64x8::splat(100.0) - t194 * t278 / f64x8::splat(100.0);
            let t284 = f64x8::splat(5.0) / f64x8::splat(972.0) * t26 * t74 + t44 * t256 * t56 / f64x8::splat(144.0) - f64x8::splat(9.0) / f64x8::splat(640.0) * t168 * t45 * t260 * t55 + f64x8::splat(2.0) * t87 * t281;
            let t285 = t284 * t119;
            let t287 = -t274;
            let t288 = ((t96).select(f64x8::splat(0.0), t287));
            let t290 = t97 * t288;
            let t292 = t99 * t288;
            let t294 = t101 * t288;
            let t296 = t103 * t288;
            let t298 = t105 * t288;
            let t303 = ((t96).select(t287, f64x8::splat(0.0)));
            let t307 = ((t95).select(-f64x8::splat(0.667) * t288 - f64x8::splat(0.889111) * t290 - f64x8::splat(1.989259803147) * t292 + f64x8::splat(5.80518817796) * t294 - f64x8::splat(4.439990207985) * t296 + f64x8::splat(1.407173648874) * t298 - f64x8::splat(0.162300903254) * t107 * t288, -t220 * t222 * t303 * t116));
            let t310 = t151 * t285 - t94 * t307 + f64x8::splat(1.174) * t307;
            let t315 = f64x8::splat(1.0) / v_rho;
            let t316 = t315 * t122;
            let t318 = t239 * t316 * t243;
            let t319 = f64x8::splat(1.0) / t127;
            let t322 = t126 * t319 * t27 * t137;
            let t326 = ((t3).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t19 * t20 * t310 * t138 + f64x8::splat(0.6334401124839145) * t318 * t322));
            let tvsigma0 = f64x8::splat(2.0) * v_rho * t326;
            acc_vsigma = tvsigma0;
            let tvlapl0 = f64x8::splat(0.0);
            acc_vlapl = tvlapl0;
            let t328 = t60 * t28;
            let t329 = t67 * t78;
            let t330 = t329 * t84;
            let t332 = t194 * t28;
            let t335 = -t328 * t330 / f64x8::splat(100.0) + t332 * t330 / f64x8::splat(100.0);
            let t336 = t87 * t335;
            let t340 = t28 * t67;
            let t341 = t340 * t78;
            let t342 = ((t96).select(f64x8::splat(0.0), t341));
            let t344 = t97 * t342;
            let t346 = t99 * t342;
            let t348 = t101 * t342;
            let t350 = t103 * t342;
            let t352 = t105 * t342;
            let t357 = ((t96).select(t341, f64x8::splat(0.0)));
            let t361 = ((t95).select(-f64x8::splat(0.667) * t342 - f64x8::splat(0.889111) * t344 - f64x8::splat(1.989259803147) * t346 + f64x8::splat(5.80518817796) * t348 - f64x8::splat(4.439990207985) * t350 + f64x8::splat(1.407173648874) * t352 - f64x8::splat(0.162300903254) * t107 * t342, -t220 * t222 * t357 * t116));
            let t364 = f64x8::splat(2.0) * t151 * t336 * t119 - t94 * t361 + f64x8::splat(1.174) * t361;
            let t369 = ((t3).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t19 * t20 * t364 * t138));
            let tvtau0 = f64x8::splat(2.0) * v_rho * t369;
            acc_vtau = tvtau0;
        }
        store_add(zk, ip, m, acc_zk);
        store_add(vrho, ip, m, acc_vrho);
        store_add(vsigma, ip, m, acc_vsigma);
        store_add(vlapl, ip, m, acc_vlapl);
        store_add(vtau, ip, m, acc_vtau);
        ip += 8;
    }
}
