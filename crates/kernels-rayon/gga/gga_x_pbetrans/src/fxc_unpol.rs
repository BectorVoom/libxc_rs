//! GGA_X_PBETRANS fxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_pbetrans.c`
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
pub fn gga_x_pbetrans_fxc_unpol(
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
            let t20 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t21 = (simd::cbrt(t20));
            let t23 = f64x8::splat(M_CBRT6);
            let t24 = t23 * t23;
            let t27 = ((v_sigma).sqrt());
            let t28 = f64x8::splat(M_CBRT2);
            let t29 = t27 * t28;
            let t31 = f64x8::splat(1.0) / t18 / v_rho;
            let t38 = (simd::exp(-f64x8::splat(2.0) * t3 * t21 * (t24 / t21 * t29 * t31 / f64x8::splat(12.0) - f64x8::splat(3.0))));
            let t39 = f64x8::splat(1.0) + t38;
            let t41 = f64x8::splat(0.413) / t39;
            let t42 = f64x8::splat(1.227) - t41;
            let t43 = t21 * t21;
            let t45 = t23 / t43;
            let t46 = t28 * t28;
            let t47 = v_sigma * t46;
            let t48 = v_rho * v_rho;
            let t49 = t18 * t18;
            let t51 = f64x8::splat(1.0) / t49 / t48;
            let t55 = f64x8::splat(1.227) - t41 + f64x8::splat(0.009125) * t45 * t47 * t51;
            let t56 = f64x8::splat(1.0) / t55;
            let t58 = -t42 * t56 + f64x8::splat(1.0);
            let t60 = t42 * t58 + f64x8::splat(1.0);
            let t64 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t60));
            let tzk0 = f64x8::splat(2.0) * t64;
            acc_zk = tzk0;
            let t66 = t17 / t49;
            let t70 = t39 * t39;
            let t71 = f64x8::splat(1.0) / t70;
            let t72 = t71 * t3;
            let t73 = t24 * t27;
            let t74 = t72 * t73;
            let t76 = f64x8::splat(1.0) / t18 / t48;
            let t77 = t28 * t76;
            let t78 = t38 * t58;
            let t79 = t77 * t78;
            let t82 = t38 * t56;
            let t83 = t77 * t82;
            let t86 = t55 * t55;
            let t87 = f64x8::splat(1.0) / t86;
            let t88 = t42 * t87;
            let t89 = t72 * t24;
            let t90 = t76 * t38;
            let t94 = t48 * v_rho;
            let t96 = f64x8::splat(1.0) / t49 / t94;
            let t100 = f64x8::splat(0.09177777777777778) * t89 * t29 * t90 - f64x8::splat(0.024333333333333332) * t45 * t47 * t96;
            let t102 = -f64x8::splat(0.09177777777777778) * t74 * t83 + t88 * t100;
            let t104 = f64x8::splat(0.09177777777777778) * t74 * t79 + t42 * t102;
            let t109 = ((t2).select(f64x8::splat(0.0), -t6 * t66 * t60 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t104));
            let tvrho0 = f64x8::splat(2.0) * v_rho * t109 + f64x8::splat(2.0) * t64;
            acc_vrho = tvrho0;
            let t112 = f64x8::splat(1.0) / t27;
            let t113 = t24 * t112;
            let t114 = t72 * t113;
            let t115 = t28 * t31;
            let t116 = t115 * t78;
            let t119 = t115 * t82;
            let t122 = t112 * t28;
            let t123 = t31 * t38;
            let t127 = t46 * t51;
            let t130 = -f64x8::splat(0.034416666666666665) * t89 * t122 * t123 + f64x8::splat(0.009125) * t45 * t127;
            let t132 = f64x8::splat(0.034416666666666665) * t114 * t119 + t88 * t130;
            let t134 = -f64x8::splat(0.034416666666666665) * t114 * t116 + t42 * t132;
            let t138 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t134));
            let tvsigma0 = f64x8::splat(2.0) * v_rho * t138;
            acc_vsigma = tvsigma0;
            let t143 = t17 / t49 / v_rho;
            let t151 = f64x8::splat(1.0) / t70 / t39;
            let t152 = t3 * t3;
            let t153 = t151 * t152;
            let t154 = t23 * v_sigma;
            let t155 = t153 * t154;
            let t156 = t48 * t48;
            let t158 = f64x8::splat(1.0) / t49 / t156;
            let t159 = t46 * t158;
            let t160 = t38 * t38;
            let t161 = t160 * t58;
            let t162 = t159 * t161;
            let t166 = f64x8::splat(1.0) / t18 / t94;
            let t167 = t28 * t166;
            let t168 = t167 * t78;
            let t171 = t71 * t152;
            let t172 = t171 * t154;
            let t173 = t159 * t78;
            let t176 = t38 * t102;
            let t177 = t77 * t176;
            let t180 = t160 * t56;
            let t181 = t159 * t180;
            let t184 = t167 * t82;
            let t187 = t159 * t82;
            let t190 = t38 * t87;
            let t191 = t190 * t100;
            let t192 = t77 * t191;
            let t196 = f64x8::splat(1.0) / t86 / t55;
            let t197 = t42 * t196;
            let t198 = t100 * t100;
            let t201 = t153 * t23;
            let t202 = t158 * t160;
            let t206 = t166 * t38;
            let t210 = t171 * t23;
            let t211 = t158 * t38;
            let t218 = -f64x8::splat(0.24474074074074073) * t201 * t47 * t202 - f64x8::splat(0.21414814814814814) * t89 * t29 * t206 + f64x8::splat(0.12237037037037037) * t210 * t47 * t211 + f64x8::splat(0.08922222222222222) * t45 * t47 * t158;
            let t220 = f64x8::splat(0.24474074074074073) * t155 * t181 + f64x8::splat(0.21414814814814814) * t74 * t184 - f64x8::splat(0.12237037037037037) * t172 * t187 + f64x8::splat(0.18355555555555556) * t74 * t192 - f64x8::splat(2.0) * t197 * t198 + t88 * t218;
            let t222 = -f64x8::splat(0.24474074074074073) * t155 * t162 - f64x8::splat(0.21414814814814814) * t74 * t168 + f64x8::splat(0.12237037037037037) * t172 * t173 + f64x8::splat(0.18355555555555556) * t74 * t177 + t42 * t220;
            let t227 = ((t2).select(f64x8::splat(0.0), t6 * t143 * t60 / f64x8::splat(12.0) - t6 * t66 * t104 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t222));
            let tv2rho20 = f64x8::splat(2.0) * v_rho * t227 + f64x8::splat(4.0) * t109;
            acc_v2rho2 = tv2rho20;
            let t233 = t46 * t96;
            let t234 = t233 * t161;
            let t239 = t233 * t78;
            let t242 = t115 * t176;
            let t245 = t38 * t132;
            let t246 = t77 * t245;
            let t249 = t233 * t180;
            let t254 = t233 * t82;
            let t257 = t115 * t191;
            let t260 = t190 * t130;
            let t261 = t77 * t260;
            let t264 = t130 * t100;
            let t278 = f64x8::splat(0.09177777777777778) * t201 * t233 * t160 + f64x8::splat(0.04588888888888889) * t89 * t122 * t90 - f64x8::splat(0.04588888888888889) * t210 * t233 * t38 - f64x8::splat(0.024333333333333332) * t45 * t233;
            let t280 = -f64x8::splat(0.09177777777777778) * t201 * t249 - f64x8::splat(0.04588888888888889) * t114 * t83 + f64x8::splat(0.04588888888888889) * t210 * t254 - f64x8::splat(0.034416666666666665) * t114 * t257 + f64x8::splat(0.09177777777777778) * t74 * t261 - f64x8::splat(2.0) * t197 * t264 + t88 * t278;
            let t282 = f64x8::splat(0.09177777777777778) * t201 * t234 + f64x8::splat(0.04588888888888889) * t114 * t79 - f64x8::splat(0.04588888888888889) * t210 * t239 - f64x8::splat(0.034416666666666665) * t114 * t242 + f64x8::splat(0.09177777777777778) * t74 * t246 + t42 * t280;
            let t287 = ((t2).select(f64x8::splat(0.0), -t6 * t66 * t134 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t282));
            let tv2rhosigma0 = f64x8::splat(2.0) * v_rho * t287 + f64x8::splat(2.0) * t138;
            acc_v2rhosigma = tv2rhosigma0;
            let t290 = f64x8::splat(1.0) / v_sigma;
            let t291 = t23 * t290;
            let t292 = t153 * t291;
            let t293 = t127 * t161;
            let t296 = t27 * v_sigma;
            let t297 = f64x8::splat(1.0) / t296;
            let t298 = t24 * t297;
            let t299 = t72 * t298;
            let t302 = t171 * t291;
            let t303 = t127 * t78;
            let t306 = t115 * t245;
            let t309 = t127 * t180;
            let t314 = t127 * t82;
            let t317 = t115 * t260;
            let t320 = t130 * t130;
            let t323 = t290 * t46;
            let t324 = t51 * t160;
            let t328 = t297 * t28;
            let t332 = t51 * t38;
            let t336 = -f64x8::splat(0.034416666666666665) * t201 * t323 * t324 + f64x8::splat(0.017208333333333332) * t89 * t328 * t123 + f64x8::splat(0.017208333333333332) * t210 * t323 * t332;
            let t338 = f64x8::splat(0.034416666666666665) * t292 * t309 - f64x8::splat(0.017208333333333332) * t299 * t119 - f64x8::splat(0.017208333333333332) * t302 * t314 - f64x8::splat(0.06883333333333333) * t114 * t317 - f64x8::splat(2.0) * t197 * t320 + t88 * t336;
            let t340 = -f64x8::splat(0.034416666666666665) * t292 * t293 + f64x8::splat(0.017208333333333332) * t299 * t116 + f64x8::splat(0.017208333333333332) * t302 * t303 - f64x8::splat(0.06883333333333333) * t114 * t306 + t42 * t338;
            let t344 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t340));
            let tv2sigma20 = f64x8::splat(2.0) * v_rho * t344;
            acc_v2sigma2 = tv2sigma20;
        }
        store_add(zk, ip, m, acc_zk);
        store_add(vrho, ip, m, acc_vrho);
        store_add(vsigma, ip, m, acc_vsigma);
        store_add(v2rho2, ip, m, acc_v2rho2);
        store_add(v2rhosigma, ip, m, acc_v2rhosigma);
        store_add(v2sigma2, ip, m, acc_v2sigma2);
        ip += 8;
    }
}
