//! MGGA_C_SCAN vxc unpol kernel — explicit SIMD (exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_c_scan.c`
//! by tools/translate_rayon/from_maple.py, then rewritten to
//! `wide::f64x8` by simd.py (exact math). Eight grid points per step; every lane runs maple2c's expression
//! sequence in its original order.
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]
use libxc_rkernel_math::constants::*;
use libxc_rkernel_math::simd;
use libxc_rkernel_math::wide::{f64x8, CmpEq, CmpGe, CmpGt, CmpLe, CmpLt, CmpNe};

const V_ZERO: f64x8 = f64x8::new([0.0; 8]);
const V_ONE: f64x8 = f64x8::new([1.0; 8]);

// `exp`, `ln` and the cube-root family come from `libxc_rkernel_math::simd`,
// which is bit-identical per lane to the scalar calls the scalar kernel makes
// (exp/ln to glibc's `_fma` ifuncs, cbrt to `powers::cbrt_f64`). Only
// `atan`/`tanh`-class calls still use `wide`'s ~1 ulp forms; a kernel with
// none of those produces output bit-identical to its scalar form.

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
pub fn mgga_c_scan_vxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    vlapl: &mut [f64],
    vtau: &mut [f64],
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
        let v_lapl = load(lapl, ip, np);
        let v_tau = load(tau, ip, np);
        let mut acc_zk = V_ZERO;
        let mut acc_vrho = V_ZERO;
        let mut acc_vsigma = V_ZERO;
        let mut acc_vlapl = V_ZERO;
        let mut acc_vtau = V_ZERO;
        {
            let t2 = f64x8::splat(M_CBRT3);
            let t3 = f64x8::splat(1.0) / f64x8::splat(M_PI);
            let t4 = (simd::cbrt(t3));
            let t5 = t2 * t4;
            let t6 = f64x8::splat(M_CBRT4);
            let t7 = t6 * t6;
            let t8 = (simd::cbrt(v_rho));
            let t11 = t5 * t7 / t8;
            let t13 = f64x8::splat(1.0) + f64x8::splat(0.053425) * t11;
            let t14 = ((t11).sqrt());
            let t17 = ((t11) * (t11).sqrt());
            let t19 = t2 * t2;
            let t20 = t4 * t4;
            let t21 = t19 * t20;
            let t22 = t8 * t8;
            let t25 = t21 * t6 / t22;
            let t27 = f64x8::splat(3.79785) * t14 + f64x8::splat(0.8969) * t11 + f64x8::splat(0.204775) * t17 + f64x8::splat(0.123235) * t25;
            let t30 = f64x8::splat(1.0) + f64x8::splat(16.081979498692537) / t27;
            let t31 = (simd::ln(t30));
            let t33 = f64x8::splat(0.0621814) * t13 * t31;
            let t34 = (f64x8::splat(1.0)).simd_le(zeta_threshold);
            let t35 = (simd::cbrt(zeta_threshold));
            let t37 = ((t34).select(t35 * zeta_threshold, f64x8::splat(1.0)));
            let t39 = f64x8::splat(2.0) * t37 - f64x8::splat(2.0);
            let t40 = f64x8::splat(M_CBRT2);
            let t41 = t40 - f64x8::splat(1.0);
            let t43 = f64x8::splat(1.0) / t41 / f64x8::splat(2.0);
            let t44 = t39 * t43;
            let t46 = f64x8::splat(1.0) + f64x8::splat(0.0278125) * t11;
            let t51 = f64x8::splat(5.1785) * t14 + f64x8::splat(0.905775) * t11 + f64x8::splat(0.1100325) * t17 + f64x8::splat(0.1241775) * t25;
            let t54 = f64x8::splat(1.0) + f64x8::splat(29.608749977793437) / t51;
            let t55 = (simd::ln(t54));
            let t58 = f64x8::splat(0.0197516734986138) * t44 * t46 * t55;
            let t59 = (simd::ln(f64x8::splat(2.0)));
            let t60 = f64x8::splat(1.0) - t59;
            let t61 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t63 = t60 / t61;
            let t64 = t35 * t35;
            let t65 = ((t34).select(t64, f64x8::splat(1.0)));
            let t66 = t65 * t65;
            let t67 = t66 * t65;
            let t69 = f64x8::splat(1.0) + f64x8::splat(0.025) * t11;
            let t71 = f64x8::splat(1.0) + f64x8::splat(0.04445) * t11;
            let t72 = f64x8::splat(1.0) / t71;
            let t73 = t69 * t72;
            let t74 = f64x8::splat(1.0) / t60;
            let t77 = f64x8::splat(1.0) / t67;
            let t78 = t61 * t77;
            let t80 = (simd::exp(-(-t33 + t58) * t74 * t78));
            let t81 = t80 - f64x8::splat(1.0);
            let t82 = f64x8::splat(1.0) / t81;
            let t83 = t74 * t82;
            let t84 = t83 * v_sigma;
            let t85 = t73 * t84;
            let t86 = v_rho * v_rho;
            let t88 = f64x8::splat(1.0) / t8 / t86;
            let t89 = t88 * t40;
            let t90 = f64x8::splat(1.0) / t66;
            let t92 = f64x8::splat(1.0) / t4;
            let t94 = t19 * t92 * t6;
            let t98 = f64x8::splat(1.0) + f64x8::splat(0.027439371595564633) * t85 * t89 * t90 * t94;
            let t99 = ((t98).sqrt().sqrt());
            let t101 = f64x8::splat(1.0) - f64x8::splat(1.0) / t99;
            let t104 = f64x8::splat(1.0) + f64x8::splat(1.0) * t101 * t81;
            let t105 = (simd::ln(t104));
            let t107 = t63 * t67 * t105;
            let t109 = f64x8::splat(1.0) / t22 / v_rho;
            let t112 = f64x8::splat(1.0) / t22 / t86;
            let t116 = f64x8::splat(M_CBRT6);
            let t118 = (simd::cbrt(t61));
            let t119 = t118 * t118;
            let t120 = f64x8::splat(1.0) / t119;
            let t121 = t40 * t40;
            let t122 = t120 * t121;
            let t124 = f64x8::splat(5.0) / f64x8::splat(9.0) * (v_tau * t109 - v_sigma * t112 / f64x8::splat(8.0)) * t116 * t122;
            let t125 = (t124).simd_le(f64x8::splat(1.0));
            let t126 = (simd::ln(f64x8::splat(f64::EPSILON)));
            let t129 = t126 / (-t126 + f64x8::splat(0.64));
            let t130 = (-t129).simd_lt(t124);
            let t131 = (t124).simd_lt(-t129);
            let t132 = ((t131).select(t124, -t129));
            let t133 = f64x8::splat(1.0) - t132;
            let t134 = f64x8::splat(1.0) / t133;
            let t137 = (simd::exp(-f64x8::splat(0.64) * t132 * t134));
            let t138 = ((t130).select(f64x8::splat(0.0), t137));
            let t140 = (simd::ln(f64x8::splat(1.4285714285714286) * f64x8::splat(f64::EPSILON)));
            let t143 = (-t140 + f64x8::splat(1.5)) / t140;
            let t144 = (t124).simd_lt(-t143);
            let t145 = ((t144).select(-t143, t124));
            let t146 = f64x8::splat(1.0) - t145;
            let t149 = (simd::exp(f64x8::splat(1.5) / t146));
            let t151 = ((t144).select(f64x8::splat(0.0), -f64x8::splat(0.7) * t149));
            let t152 = ((t125).select(t138, t151));
            let t155 = f64x8::splat(1.0) + f64x8::splat(0.04445) * t14 + f64x8::splat(0.03138525) * t11;
            let t156 = f64x8::splat(1.0) / t155;
            let t159 = (simd::exp(f64x8::splat(1.0) * t156));
            let t160 = t159 - f64x8::splat(1.0);
            let t161 = t116 * t120;
            let t162 = t121 * v_sigma;
            let t166 = f64x8::splat(1.0) + f64x8::splat(0.02133764210437636) * t161 * t162 * t112;
            let t167 = ((t166).sqrt().sqrt());
            let t169 = f64x8::splat(1.0) - f64x8::splat(1.0) / t167;
            let t171 = t160 * t169 + f64x8::splat(1.0);
            let t172 = (simd::ln(t171));
            let t178 = f64x8::splat(1.0) - f64x8::splat(2.363) * t41 * t39 * t43;
            let t180 = (-f64x8::splat(0.0285764) * t156 + f64x8::splat(0.0285764) * t172) * t178 + t33 - t58 - t107;
            let t181 = t152 * t180;
            let tzk0 = -t33 + t58 + t107 + t181;
            acc_zk = tzk0;
            let t183 = f64x8::splat(1.0) / t8 / v_rho;
            let t184 = t7 * t183;
            let t186 = t5 * t184 * t31;
            let t187 = f64x8::splat(0.0011073470983333333) * t186;
            let t188 = t27 * t27;
            let t189 = f64x8::splat(1.0) / t188;
            let t190 = t13 * t189;
            let t192 = f64x8::splat(1.0) / t14 * t2;
            let t193 = t4 * t7;
            let t194 = t193 * t183;
            let t195 = t192 * t194;
            let t197 = t5 * t184;
            let t199 = ((t11).sqrt());
            let t200 = t199 * t2;
            let t201 = t200 * t194;
            let t204 = t21 * t6 * t109;
            let t206 = -f64x8::splat(0.632975) * t195 - f64x8::splat(0.29896666666666666) * t197 - f64x8::splat(0.1023875) * t201 - f64x8::splat(0.08215666666666667) * t204;
            let t207 = f64x8::splat(1.0) / t30;
            let t208 = t206 * t207;
            let t209 = t190 * t208;
            let t210 = f64x8::splat(1.0) * t209;
            let t211 = t44 * t2;
            let t214 = t211 * t193 * t183 * t55;
            let t215 = f64x8::splat(0.00018311447306006544) * t214;
            let t216 = t44 * t46;
            let t217 = t51 * t51;
            let t218 = f64x8::splat(1.0) / t217;
            let t223 = -f64x8::splat(0.8630833333333333) * t195 - f64x8::splat(0.301925) * t197 - f64x8::splat(0.05501625) * t201 - f64x8::splat(0.082785) * t204;
            let t225 = f64x8::splat(1.0) / t54;
            let t226 = t218 * t223 * t225;
            let t227 = t216 * t226;
            let t228 = f64x8::splat(0.5848223622634646) * t227;
            let t230 = f64x8::splat(1.0) / t99 / t98;
            let t231 = t86 * v_rho;
            let t233 = f64x8::splat(1.0) / t22 / t231;
            let t234 = t233 * t72;
            let t237 = t40 * t90;
            let t238 = t82 * v_sigma * t237;
            let t241 = t71 * t71;
            let t242 = f64x8::splat(1.0) / t241;
            let t243 = t69 * t242;
            let t244 = t243 * t83;
            let t245 = v_sigma * t233;
            let t249 = t60 * t60;
            let t250 = f64x8::splat(1.0) / t249;
            let t251 = t73 * t250;
            let t252 = t81 * t81;
            let t253 = f64x8::splat(1.0) / t252;
            let t254 = t253 * v_sigma;
            let t255 = t254 * t89;
            let t256 = t251 * t255;
            let t257 = t66 * t66;
            let t259 = f64x8::splat(1.0) / t257 / t65;
            let t260 = t259 * t19;
            let t261 = t260 * t92;
            let t262 = t187 + t210 - t215 - t228;
            let t264 = t61 * t80;
            let t265 = t6 * t262 * t264;
            let t266 = t261 * t265;
            let t270 = f64x8::splat(1.0) / t8 / t231;
            let t271 = t270 * t40;
            let t276 = -f64x8::splat(0.002743937159556463) * t234 * t74 * t238 + f64x8::splat(0.004878720269691391) * t244 * t245 * t237 + f64x8::splat(0.027439371595564633) * t256 * t266 - f64x8::splat(0.0640252003896508) * t85 * t271 * t90 * t94;
            let t277 = t230 * t276;
            let t282 = t78 * t80;
            let t285 = f64x8::splat(0.25) * t277 * t81 - f64x8::splat(1.0) * t101 * t262 * t74 * t282;
            let t287 = f64x8::splat(1.0) / t104;
            let t289 = t63 * t67 * t285 * t287;
            let t296 = f64x8::splat(5.0) / f64x8::splat(9.0) * (-f64x8::splat(5.0) / f64x8::splat(3.0) * v_tau * t112 + t245 / f64x8::splat(3.0)) * t116 * t122;
            let t297 = ((t131).select(t296, f64x8::splat(0.0)));
            let t300 = t133 * t133;
            let t301 = f64x8::splat(1.0) / t300;
            let t302 = t132 * t301;
            let t305 = -f64x8::splat(0.64) * t297 * t134 - f64x8::splat(0.64) * t302 * t297;
            let t306 = t305 * t137;
            let t307 = ((t130).select(f64x8::splat(0.0), t306));
            let t308 = t146 * t146;
            let t309 = f64x8::splat(1.0) / t308;
            let t310 = ((t144).select(f64x8::splat(0.0), t296));
            let t314 = ((t144).select(f64x8::splat(0.0), -f64x8::splat(1.05) * t309 * t310 * t149));
            let t315 = ((t125).select(t307, t314));
            let t316 = t315 * t180;
            let t317 = t155 * t155;
            let t318 = f64x8::splat(1.0) / t317;
            let t321 = -f64x8::splat(0.007408333333333334) * t195 - f64x8::splat(0.01046175) * t197;
            let t322 = t318 * t321;
            let t324 = t159 * t169;
            let t328 = f64x8::splat(1.0) / t167 / t166;
            let t329 = t160 * t328;
            let t330 = t329 * t116;
            let t334 = -f64x8::splat(1.0) * t322 * t324 - f64x8::splat(0.014225094736250906) * t330 * t122 * t245;
            let t335 = f64x8::splat(1.0) / t171;
            let t340 = (f64x8::splat(0.0285764) * t322 + f64x8::splat(0.0285764) * t334 * t335) * t178 - t187 - t210 + t215 + t228 - t289;
            let t341 = t152 * t340;
            let tvrho0 = -t33 + t58 + t107 + t181 + v_rho * (t187 + t210 - t215 - t228 + t289 + t316 + t341);
            acc_vrho = tvrho0;
            let t344 = t65 * t230;
            let t345 = t73 * t88;
            let t346 = t344 * t345;
            let t347 = t40 * t19;
            let t348 = t92 * t6;
            let t349 = t348 * t287;
            let t350 = t347 * t349;
            let t352 = f64x8::splat(0.0006950474021161377) * t346 * t350;
            let t353 = t112 * t116;
            let t354 = t353 * t122;
            let t355 = f64x8::splat(5.0) / f64x8::splat(72.0) * t354;
            let t356 = ((t131).select(-t355, f64x8::splat(0.0)));
            let t361 = -f64x8::splat(0.64) * t356 * t134 - f64x8::splat(0.64) * t302 * t356;
            let t362 = t361 * t137;
            let t363 = ((t130).select(f64x8::splat(0.0), t362));
            let t364 = ((t144).select(f64x8::splat(0.0), -t355));
            let t368 = ((t144).select(f64x8::splat(0.0), -f64x8::splat(1.05) * t309 * t364 * t149));
            let t369 = ((t125).select(t363, t368));
            let t370 = t369 * t180;
            let t371 = t329 * t353;
            let t372 = t335 * t178;
            let t373 = t122 * t372;
            let t376 = f64x8::splat(0.00015243824895787514) * t371 * t373 - t352;
            let t377 = t152 * t376;
            let tvsigma0 = v_rho * (t352 + t370 + t377);
            acc_vsigma = tvsigma0;
            let tvlapl0 = f64x8::splat(0.0);
            acc_vlapl = tvlapl0;
            let t381 = f64x8::splat(5.0) / f64x8::splat(9.0) * t109 * t116 * t122;
            let t382 = ((t131).select(t381, f64x8::splat(0.0)));
            let t387 = -f64x8::splat(0.64) * t382 * t134 - f64x8::splat(0.64) * t302 * t382;
            let t388 = t387 * t137;
            let t389 = ((t130).select(f64x8::splat(0.0), t388));
            let t390 = ((t144).select(f64x8::splat(0.0), t381));
            let t394 = ((t144).select(f64x8::splat(0.0), -f64x8::splat(1.05) * t309 * t390 * t149));
            let t395 = ((t125).select(t389, t394));
            let t396 = v_rho * t395;
            let tvtau0 = t396 * t180;
            acc_vtau = tvtau0;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_vrho.into(); vrho[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_vsigma.into(); vsigma[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_vlapl.into(); vlapl[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_vtau.into(); vtau[ip..ip + m].copy_from_slice(&a[..m]); }
        ip += 8;
    }
}
