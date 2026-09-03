//! GGA_X_SFAT_PBE vxc unpol kernel — explicit SIMD (bit-exact).
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

#[allow(unused_variables, non_snake_case)]
pub fn gga_x_sfat_pbe_vxc_unpol(
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
        let v_rho = load(rho, ip, np);
        let v_sigma = load(sigma, ip, np);
        let mut acc_zk = V_ZERO;
        let mut acc_vrho = V_ZERO;
        let mut acc_vsigma = V_ZERO;
        {
            let t2 = (v_rho / f64x8::splat(2.0)).simd_le(dens_threshold);
            let t3 = f64x8::splat(M_CBRT3);
            let t4 = f64x8::splat(M_CBRTPI);
            let t7 = (f64x8::splat(1.0)).simd_le(zeta_threshold);
            let t8 = zeta_threshold - f64x8::splat(1.0);
            let t10 = ((t7).select(t8, (t7).select(-t8, f64x8::splat(0.0))));
            let t11 = f64x8::splat(1.0) + t10;
            let t13 = (simd::cbrt(zeta_threshold));
            let t15 = (simd::cbrt(t11));
            let t17 = (((t11).simd_le(zeta_threshold)).select(t13 * zeta_threshold, t15 * t11));
            let t18 = t17 / t4 * t3;
            let t19 = (simd::cbrt(v_rho));
            let t20 = t3 * t3;
            let t22 = f64x8::splat(1.0) / f64x8::splat(M_PI);
            let t23 = (simd::cbrt(t22));
            let t24 = f64x8::splat(1.0) / t23;
            let t25 = f64x8::splat(M_CBRT4);
            let t27 = f64x8::splat(M_CBRT6);
            let t28 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t29 = (simd::cbrt(t28));
            let t30 = t29 * t29;
            let t31 = f64x8::splat(1.0) / t30;
            let t32 = t31 * t27;
            let t33 = f64x8::splat(M_CBRT2);
            let t34 = t33 * t33;
            let t35 = t34 * v_sigma;
            let t36 = v_rho * v_rho;
            let t37 = t19 * t19;
            let t39 = f64x8::splat(1.0) / t37 / t36;
            let t43 = f64x8::splat(0.804) + f64x8::splat(0.009146457198521547) * t39 * t35 * t32;
            let t46 = f64x8::splat(1.804) - f64x8::splat(0.646416) / t43;
            let t49 = f64x8::splat(1.0) / t46 * t25 * t24 * t20 * f64x8::splat(M_PI);
            let t50 = ((t49).sqrt());
            let t52 = f64x8::splat(1.0) / t50 * param_hyb_omega_0;
            let t53 = v_rho * t11;
            let t54 = (simd::cbrt(t53));
            let t55 = f64x8::splat(1.0) / t54;
            let t58 = t55 * t33 * t52 / f64x8::splat(2.0);
            let t59 = (f64x8::splat(1.92)).simd_le(t58);
            let t60 = (f64x8::splat(1.92)).simd_lt(t58);
            let t61 = ((t60).select(t58, f64x8::splat(1.92)));
            let t62 = t61 * t61;
            let t63 = t62 * t62;
            let t64 = f64x8::splat(1.0) / t63;
            let t66 = t63 * t62;
            let t67 = f64x8::splat(1.0) / t66;
            let t69 = t63 * t63;
            let t70 = f64x8::splat(1.0) / t69;
            let t72 = t69 * t62;
            let t73 = f64x8::splat(1.0) / t72;
            let t75 = t69 * t63;
            let t76 = f64x8::splat(1.0) / t75;
            let t78 = t69 * t66;
            let t79 = f64x8::splat(1.0) / t78;
            let t81 = t69 * t69;
            let t82 = f64x8::splat(1.0) / t81;
            let t85 = f64x8::splat(1.0) / t81 / t62;
            let t88 = f64x8::splat(1.0) / t81 / t63;
            let t91 = f64x8::splat(1.0) / t81 / t66;
            let t94 = f64x8::splat(1.0) / t81 / t69;
            let t97 = f64x8::splat(1.0) / t81 / t72;
            let t100 = f64x8::splat(1.0) / t81 / t75;
            let t103 = f64x8::splat(1.0) / t81 / t78;
            let t105 = t81 * t81;
            let t106 = f64x8::splat(1.0) / t105;
            let t109 = f64x8::splat(1.0) / t105 / t62;
            let t112 = f64x8::splat(1.0) / t105 / t63;
            let t116 = -t64 / f64x8::splat(30.0) + t67 / f64x8::splat(70.0) - t70 / f64x8::splat(135.0) + t73 / f64x8::splat(231.0) - t76 / f64x8::splat(364.0) + t79 / f64x8::splat(540.0) - t82 / f64x8::splat(765.0) + t85 / f64x8::splat(1045.0) - t88 / f64x8::splat(1386.0) + t91 / f64x8::splat(1794.0) - t94 / f64x8::splat(2275.0) + t97 / f64x8::splat(2835.0) - t100 / f64x8::splat(3480.0) + t103 / f64x8::splat(4216.0) - t106 / f64x8::splat(5049.0) + t109 / f64x8::splat(5985.0) - t112 / f64x8::splat(7030.0) + f64x8::splat(1.0) / t62 / f64x8::splat(9.0);
            let t117 = ((t60).select(f64x8::splat(1.92), t58));
            let t118 = (simd::atan2(f64x8::splat(1.0), t117));
            let t119 = t117 * t117;
            let t120 = t119 + f64x8::splat(3.0);
            let t121 = f64x8::splat(1.0) / t119;
            let t122 = f64x8::splat(1.0) + t121;
            let t123 = (simd::ln(t122));
            let t125 = -t123 * t120 + f64x8::splat(1.0);
            let t128 = t118 + t125 * t117 / f64x8::splat(4.0);
            let t132 = ((t59).select(t116, f64x8::splat(1.0) - f64x8::splat(8.0) / f64x8::splat(3.0) * t128 * t117));
            let t137 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t46 * t132 * t19 * t18));
            let tzk0 = f64x8::splat(2.0) * t137;
            acc_zk = tzk0;
            let t138 = f64x8::splat(1.0) / t37;
            let t143 = t63 * t61;
            let t144 = f64x8::splat(1.0) / t143;
            let t147 = f64x8::splat(1.0) / t50 / t49 * param_hyb_omega_0;
            let t149 = t24 * t20;
            let t150 = t25 * t149;
            let t151 = t150 * t55 * t147;
            let t152 = t46 * t46;
            let t153 = f64x8::splat(1.0) / t152;
            let t154 = t43 * t43;
            let t155 = f64x8::splat(1.0) / t154;
            let t157 = t27 * t155 * t153;
            let t158 = v_sigma * t31;
            let t159 = t36 * v_rho;
            let t161 = f64x8::splat(1.0) / t37 / t159;
            let t167 = f64x8::splat(1.0) / t54 / t53;
            let t172 = -f64x8::splat(0.02476587138536942) * t161 * t158 * t157 * t151 - t11 * t167 * t33 * t52 / f64x8::splat(6.0);
            let t173 = ((t60).select(t172, f64x8::splat(0.0)));
            let t176 = t62 * t61;
            let t177 = t63 * t176;
            let t178 = f64x8::splat(1.0) / t177;
            let t181 = t69 * t61;
            let t182 = f64x8::splat(1.0) / t181;
            let t185 = t69 * t176;
            let t186 = f64x8::splat(1.0) / t185;
            let t189 = t69 * t143;
            let t190 = f64x8::splat(1.0) / t189;
            let t193 = t69 * t177;
            let t194 = f64x8::splat(1.0) / t193;
            let t198 = f64x8::splat(1.0) / t81 / t61;
            let t202 = f64x8::splat(1.0) / t81 / t176;
            let t206 = f64x8::splat(1.0) / t81 / t143;
            let t210 = f64x8::splat(1.0) / t81 / t177;
            let t214 = f64x8::splat(1.0) / t81 / t181;
            let t218 = f64x8::splat(1.0) / t81 / t185;
            let t222 = f64x8::splat(1.0) / t81 / t189;
            let t226 = f64x8::splat(1.0) / t81 / t193;
            let t230 = f64x8::splat(1.0) / t105 / t61;
            let t234 = f64x8::splat(1.0) / t105 / t176;
            let t238 = f64x8::splat(1.0) / t105 / t143;
            let t241 = f64x8::splat(1.0) / t176;
            let t244 = f64x8::splat(2.0) / f64x8::splat(15.0) * t173 * t144 - f64x8::splat(3.0) / f64x8::splat(35.0) * t173 * t178 + f64x8::splat(8.0) / f64x8::splat(135.0) * t173 * t182 - f64x8::splat(10.0) / f64x8::splat(231.0) * t173 * t186 + f64x8::splat(3.0) / f64x8::splat(91.0) * t173 * t190 - f64x8::splat(7.0) / f64x8::splat(270.0) * t173 * t194 + f64x8::splat(16.0) / f64x8::splat(765.0) * t173 * t198 - f64x8::splat(18.0) / f64x8::splat(1045.0) * t173 * t202 + f64x8::splat(10.0) / f64x8::splat(693.0) * t173 * t206 - f64x8::splat(11.0) / f64x8::splat(897.0) * t173 * t210 + f64x8::splat(24.0) / f64x8::splat(2275.0) * t173 * t214 - f64x8::splat(26.0) / f64x8::splat(2835.0) * t173 * t218 + f64x8::splat(7.0) / f64x8::splat(870.0) * t173 * t222 - f64x8::splat(15.0) / f64x8::splat(2108.0) * t173 * t226 + f64x8::splat(32.0) / f64x8::splat(5049.0) * t173 * t230 - f64x8::splat(34.0) / f64x8::splat(5985.0) * t173 * t234 + f64x8::splat(18.0) / f64x8::splat(3515.0) * t173 * t238 - f64x8::splat(2.0) / f64x8::splat(9.0) * t173 * t241;
            let t245 = ((t60).select(f64x8::splat(0.0), t172));
            let t248 = f64x8::splat(1.0) / t122;
            let t254 = t119 * t117;
            let t255 = f64x8::splat(1.0) / t254;
            let t256 = t255 * t120;
            let t257 = t248 * t245;
            let t260 = -f64x8::splat(2.0) * t123 * t245 * t117 + f64x8::splat(2.0) * t257 * t256;
            let t263 = -t248 * t121 * t245 + t125 * t245 / f64x8::splat(4.0) + t260 * t117 / f64x8::splat(4.0);
            let t267 = ((t59).select(t244, -f64x8::splat(8.0) / f64x8::splat(3.0) * t263 * t117 - f64x8::splat(8.0) / f64x8::splat(3.0) * t128 * t245));
            let t272 = t17 * t3;
            let t274 = f64x8::splat(1.0) / t19 / t159;
            let t276 = t132 * t274 * t272;
            let t277 = t27 * t155;
            let t278 = t34 * t158;
            let t279 = t278 * t277;
            let t283 = ((t2).select(f64x8::splat(0.0), -t46 * t132 * t138 * t18 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t46 * t267 * t19 * t18 + f64x8::splat(0.0040369036088841095) * t279 * t276));
            let tvrho0 = f64x8::splat(2.0) * t283 * v_rho + f64x8::splat(2.0) * t137;
            acc_vrho = tvrho0;
            let t288 = t24 * t20 * t55 * t147;
            let t289 = t153 * t25;
            let t290 = t155 * t289;
            let t294 = f64x8::splat(0.009287201769513533) * t39 * t32 * t290 * t288;
            let t295 = ((t60).select(t294, f64x8::splat(0.0)));
            let t296 = t295 * t144;
            let t298 = t295 * t178;
            let t300 = t295 * t182;
            let t302 = t295 * t186;
            let t304 = t295 * t190;
            let t306 = t295 * t194;
            let t308 = t295 * t198;
            let t310 = t295 * t202;
            let t312 = t295 * t206;
            let t314 = t295 * t210;
            let t316 = t295 * t214;
            let t318 = t295 * t218;
            let t320 = t295 * t222;
            let t322 = t295 * t226;
            let t324 = t295 * t230;
            let t326 = t295 * t234;
            let t328 = t295 * t238;
            let t332 = f64x8::splat(2.0) / f64x8::splat(15.0) * t296 - f64x8::splat(3.0) / f64x8::splat(35.0) * t298 + f64x8::splat(8.0) / f64x8::splat(135.0) * t300 - f64x8::splat(10.0) / f64x8::splat(231.0) * t302 + f64x8::splat(3.0) / f64x8::splat(91.0) * t304 - f64x8::splat(7.0) / f64x8::splat(270.0) * t306 + f64x8::splat(16.0) / f64x8::splat(765.0) * t308 - f64x8::splat(18.0) / f64x8::splat(1045.0) * t310 + f64x8::splat(10.0) / f64x8::splat(693.0) * t312 - f64x8::splat(11.0) / f64x8::splat(897.0) * t314 + f64x8::splat(24.0) / f64x8::splat(2275.0) * t316 - f64x8::splat(26.0) / f64x8::splat(2835.0) * t318 + f64x8::splat(7.0) / f64x8::splat(870.0) * t320 - f64x8::splat(15.0) / f64x8::splat(2108.0) * t322 + f64x8::splat(32.0) / f64x8::splat(5049.0) * t324 - f64x8::splat(34.0) / f64x8::splat(5985.0) * t326 + f64x8::splat(18.0) / f64x8::splat(3515.0) * t328 - f64x8::splat(2.0) / f64x8::splat(9.0) * t295 * t241;
            let t333 = ((t60).select(f64x8::splat(0.0), t294));
            let t335 = t121 * t333;
            let t341 = t248 * t333;
            let t344 = -f64x8::splat(2.0) * t123 * t333 * t117 + f64x8::splat(2.0) * t341 * t256;
            let t347 = -t248 * t335 + t125 * t333 / f64x8::splat(4.0) + t344 * t117 / f64x8::splat(4.0);
            let t351 = ((t59).select(t332, -f64x8::splat(8.0) / f64x8::splat(3.0) * t347 * t117 - f64x8::splat(8.0) / f64x8::splat(3.0) * t128 * t333));
            let t357 = f64x8::splat(1.0) / t19 / t36;
            let t360 = t34 * t31;
            let t361 = t360 * t277;
            let t365 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t46 * t351 * t19 * t18 - f64x8::splat(0.0015138388533315413) * t361 * t132 * t357 * t272));
            let tvsigma0 = f64x8::splat(2.0) * t365 * v_rho;
            acc_vsigma = tvsigma0;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_vrho.into(); vrho[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_vsigma.into(); vsigma[ip..ip + m].copy_from_slice(&a[..m]); }
        ip += 8;
    }
}
