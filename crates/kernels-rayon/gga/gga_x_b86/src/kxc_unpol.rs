//! GGA_X_B86 kxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_b86.c`
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
pub fn gga_x_b86_kxc_unpol(
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
    param_beta: f64,
    param_gamma: f64,
    param_omega: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_beta = f64x8::splat(param_beta);
    let param_gamma = f64x8::splat(param_gamma);
    let param_omega = f64x8::splat(param_omega);
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
            let t20 = param_beta * v_sigma;
            let t21 = f64x8::splat(M_CBRT2);
            let t22 = t21 * t21;
            let t23 = v_rho * v_rho;
            let t24 = t18 * t18;
            let t26 = f64x8::splat(1.0) / t24 / t23;
            let t27 = t22 * t26;
            let t30 = param_gamma * v_sigma * t27 + f64x8::splat(1.0);
            let t31 = (simd::pow(t30, param_omega));
            let t32 = f64x8::splat(1.0) / t31;
            let t35 = t20 * t27 * t32 + f64x8::splat(1.0);
            let t39 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t35));
            let tzk0 = f64x8::splat(2.0) * t39;
            acc_zk = tzk0;
            let t41 = t17 / t24;
            let t45 = t23 * v_rho;
            let t47 = f64x8::splat(1.0) / t24 / t45;
            let t52 = v_sigma * v_sigma;
            let t53 = param_beta * t52;
            let t54 = t23 * t23;
            let t55 = t54 * t23;
            let t57 = f64x8::splat(1.0) / t18 / t55;
            let t60 = t32 * param_omega;
            let t61 = f64x8::splat(1.0) / t30;
            let t63 = t60 * param_gamma * t61;
            let t66 = -f64x8::splat(8.0) / f64x8::splat(3.0) * t20 * t22 * t47 * t32 + f64x8::splat(16.0) / f64x8::splat(3.0) * t53 * t21 * t57 * t63;
            let t71 = ((t2).select(f64x8::splat(0.0), -t6 * t41 * t35 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t66));
            let tvrho0 = f64x8::splat(2.0) * v_rho * t71 + f64x8::splat(2.0) * t39;
            acc_vrho = tvrho0;
            let t74 = param_beta * t22;
            let t77 = t54 * v_rho;
            let t79 = f64x8::splat(1.0) / t18 / t77;
            let t84 = -f64x8::splat(2.0) * t20 * t21 * t79 * t63 + t74 * t26 * t32;
            let t88 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t84));
            let tvsigma0 = f64x8::splat(2.0) * v_rho * t88;
            acc_vsigma = tvsigma0;
            let t93 = t17 / t24 / v_rho;
            let t101 = f64x8::splat(1.0) / t24 / t54;
            let t106 = t54 * t45;
            let t108 = f64x8::splat(1.0) / t18 / t106;
            let t113 = t52 * v_sigma;
            let t114 = param_beta * t113;
            let t115 = t54 * t54;
            let t116 = t115 * t23;
            let t117 = f64x8::splat(1.0) / t116;
            let t118 = t114 * t117;
            let t119 = param_omega * param_omega;
            let t120 = t32 * t119;
            let t121 = param_gamma * param_gamma;
            let t122 = t30 * t30;
            let t123 = f64x8::splat(1.0) / t122;
            let t124 = t121 * t123;
            let t125 = t120 * t124;
            let t128 = t60 * t124;
            let t131 = f64x8::splat(88.0) / f64x8::splat(9.0) * t20 * t22 * t101 * t32 - f64x8::splat(48.0) * t53 * t21 * t108 * t63 + f64x8::splat(256.0) / f64x8::splat(9.0) * t118 * t125 + f64x8::splat(256.0) / f64x8::splat(9.0) * t118 * t128;
            let t136 = ((t2).select(f64x8::splat(0.0), t6 * t93 * t35 / f64x8::splat(12.0) - t6 * t41 * t66 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t131));
            let tv2rho20 = f64x8::splat(2.0) * v_rho * t136 + f64x8::splat(4.0) * t71;
            acc_v2rho2 = tv2rho20;
            let t145 = param_beta * t21;
            let t150 = param_omega * param_gamma * v_sigma * t61;
            let t153 = t115 * v_rho;
            let t154 = f64x8::splat(1.0) / t153;
            let t155 = t53 * t154;
            let t160 = -f64x8::splat(8.0) / f64x8::splat(3.0) * t74 * t47 * t32 + f64x8::splat(16.0) * t145 * t57 * t32 * t150 - f64x8::splat(32.0) / f64x8::splat(3.0) * t155 * t125 - f64x8::splat(32.0) / f64x8::splat(3.0) * t155 * t128;
            let t165 = ((t2).select(f64x8::splat(0.0), -t6 * t41 * t84 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t160));
            let tv2rhosigma0 = f64x8::splat(2.0) * v_rho * t165 + f64x8::splat(2.0) * t88;
            acc_v2rhosigma = tv2rhosigma0;
            let t170 = f64x8::splat(1.0) / t115;
            let t171 = t20 * t170;
            let t175 = -f64x8::splat(4.0) * t145 * t79 * t63 + f64x8::splat(4.0) * t171 * t125 + f64x8::splat(4.0) * t171 * t128;
            let t179 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t175));
            let tv2sigma20 = f64x8::splat(2.0) * v_rho * t179;
            acc_v2sigma2 = tv2sigma20;
            let t182 = t17 * t26;
            let t193 = f64x8::splat(1.0) / t24 / t77;
            let t199 = f64x8::splat(1.0) / t18 / t115;
            let t204 = t115 * t45;
            let t205 = f64x8::splat(1.0) / t204;
            let t206 = t114 * t205;
            let t211 = t52 * t52;
            let t212 = param_beta * t211;
            let t213 = t115 * t77;
            let t215 = f64x8::splat(1.0) / t24 / t213;
            let t217 = t212 * t215 * t32;
            let t218 = t119 * param_omega;
            let t219 = t121 * param_gamma;
            let t222 = f64x8::splat(1.0) / t122 / t30;
            let t223 = t222 * t22;
            let t224 = t218 * t219 * t223;
            let t228 = t119 * t219 * t223;
            let t232 = param_omega * t219 * t223;
            let t235 = -f64x8::splat(1232.0) / f64x8::splat(27.0) * t20 * t22 * t193 * t32 + f64x8::splat(10912.0) / f64x8::splat(27.0) * t53 * t21 * t199 * t63 - f64x8::splat(4864.0) / f64x8::splat(9.0) * t206 * t125 - f64x8::splat(4864.0) / f64x8::splat(9.0) * t206 * t128 + f64x8::splat(2048.0) / f64x8::splat(27.0) * t217 * t224 + f64x8::splat(2048.0) / f64x8::splat(9.0) * t217 * t228 + f64x8::splat(4096.0) / f64x8::splat(27.0) * t217 * t232;
            let t240 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(5.0) / f64x8::splat(36.0) * t6 * t182 * t35 + t6 * t93 * t66 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t41 * t131 - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t235));
            let tv3rho30 = f64x8::splat(2.0) * v_rho * t240 + f64x8::splat(6.0) * t136;
            acc_v3rho3 = tv3rho30;
            let t258 = param_beta * t117 * t32;
            let t259 = t119 * t121;
            let t260 = t52 * t123;
            let t261 = t259 * t260;
            let t264 = param_omega * t121;
            let t265 = t264 * t260;
            let t268 = t115 * t54;
            let t270 = f64x8::splat(1.0) / t24 / t268;
            let t272 = t114 * t270 * t32;
            let t279 = f64x8::splat(88.0) / f64x8::splat(9.0) * t74 * t101 * t32 - f64x8::splat(1040.0) / f64x8::splat(9.0) * t145 * t108 * t32 * t150 + f64x8::splat(544.0) / f64x8::splat(3.0) * t258 * t261 + f64x8::splat(544.0) / f64x8::splat(3.0) * t258 * t265 - f64x8::splat(256.0) / f64x8::splat(9.0) * t272 * t224 - f64x8::splat(256.0) / f64x8::splat(3.0) * t272 * t228 - f64x8::splat(512.0) / f64x8::splat(9.0) * t272 * t232;
            let t284 = ((t2).select(f64x8::splat(0.0), t6 * t93 * t84 / f64x8::splat(12.0) - t6 * t41 * t160 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t279));
            let tv3rho2sigma0 = f64x8::splat(2.0) * v_rho * t284 + f64x8::splat(4.0) * t165;
            acc_v3rho2sigma = tv3rho2sigma0;
            let t294 = param_beta * t154 * t32;
            let t295 = t123 * v_sigma;
            let t296 = t259 * t295;
            let t299 = t264 * t295;
            let t303 = f64x8::splat(1.0) / t24 / t204;
            let t305 = t53 * t303 * t32;
            let t312 = f64x8::splat(64.0) / f64x8::splat(3.0) * t145 * t57 * t63 - f64x8::splat(160.0) / f64x8::splat(3.0) * t294 * t296 - f64x8::splat(160.0) / f64x8::splat(3.0) * t294 * t299 + f64x8::splat(32.0) / f64x8::splat(3.0) * t305 * t224 + f64x8::splat(32.0) * t305 * t228 + f64x8::splat(64.0) / f64x8::splat(3.0) * t305 * t232;
            let t317 = ((t2).select(f64x8::splat(0.0), -t6 * t41 * t175 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t312));
            let tv3rhosigma20 = f64x8::splat(2.0) * v_rho * t317 + f64x8::splat(2.0) * t179;
            acc_v3rhosigma2 = tv3rhosigma20;
            let t321 = param_beta * t170 * t32;
            let t322 = t259 * t123;
            let t325 = t264 * t123;
            let t329 = f64x8::splat(1.0) / t24 / t116;
            let t331 = t20 * t329 * t32;
            let t338 = -f64x8::splat(4.0) * t331 * t224 - f64x8::splat(12.0) * t331 * t228 - f64x8::splat(8.0) * t331 * t232 + f64x8::splat(12.0) * t321 * t322 + f64x8::splat(12.0) * t321 * t325;
            let t342 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t338));
            let tv3sigma30 = f64x8::splat(2.0) * v_rho * t342;
            acc_v3sigma3 = tv3sigma30;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_vrho.into(); vrho[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_vsigma.into(); vsigma[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v2rho2.into(); v2rho2[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v2rhosigma.into(); v2rhosigma[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v2sigma2.into(); v2sigma2[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v3rho3.into(); v3rho3[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v3rho2sigma.into(); v3rho2sigma[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v3rhosigma2.into(); v3rhosigma2[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v3sigma3.into(); v3sigma3[ip..ip + m].copy_from_slice(&a[..m]); }
        ip += 8;
    }
}
