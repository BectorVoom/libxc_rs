//! GGA_C_GAPC exc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_gapc.c`
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

/// Store 8 elements with a given stride and offset.
#[inline(always)]
fn store_strided(s: &mut [f64], ip: usize, m: usize, stride: usize, offset: usize, acc: f64x8) {
    let a: [f64; 8] = acc.into();
    if m == 8 {
        let base = ip * stride + offset;
        s[base] = a[0];
        s[base + stride] = a[1];
        s[base + 2 * stride] = a[2];
        s[base + 3 * stride] = a[3];
        s[base + 4 * stride] = a[4];
        s[base + 5 * stride] = a[5];
        s[base + 6 * stride] = a[6];
        s[base + 7 * stride] = a[7];
    } else {
        for k in 0..m {
            s[(ip + k) * stride + offset] = a[k];
        }
    }
}

#[allow(unused_variables, non_snake_case)]
pub fn gga_c_gapc_exc_pol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
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
        {
            let t1 = f64x8::splat(M_CBRT3);
            let t2 = f64x8::splat(1.0) / f64x8::splat(M_PI);
            let t3 = (simd::cbrt(t2));
            let t4 = t1 * t3;
            let t5 = f64x8::splat(M_CBRT4);
            let t6 = t5 * t5;
            let t7 = v_rho0 + v_rho1;
            let t8 = (simd::cbrt(t7));
            let t9 = f64x8::splat(1.0) / t8;
            let t10 = t6 * t9;
            let t11 = t4 * t10;
            let t13 = f64x8::splat(1.0) + f64x8::splat(0.053425) * t11;
            let t14 = ((t11).sqrt());
            let t17 = ((t11) * (t11).sqrt());
            let t19 = t1 * t1;
            let t20 = t3 * t3;
            let t21 = t19 * t20;
            let t22 = t8 * t8;
            let t23 = f64x8::splat(1.0) / t22;
            let t25 = t21 * t5 * t23;
            let t27 = f64x8::splat(3.79785) * t14 + f64x8::splat(0.8969) * t11 + f64x8::splat(0.204775) * t17 + f64x8::splat(0.123235) * t25;
            let t30 = f64x8::splat(1.0) + f64x8::splat(16.081824322151103) / t27;
            let t31 = (simd::ln(t30));
            let t33 = f64x8::splat(0.062182) * t13 * t31;
            let t34 = (f64x8::splat(1.0)).simd_le(zeta_threshold);
            let t35 = (simd::cbrt(zeta_threshold));
            let t36 = t35 * zeta_threshold;
            let t37 = ((t34).select(t36, f64x8::splat(1.0)));
            let t40 = f64x8::splat(M_CBRT2);
            let t43 = f64x8::splat(1.0) / (f64x8::splat(2.0) * t40 - f64x8::splat(2.0));
            let t44 = (f64x8::splat(2.0) * t37 - f64x8::splat(2.0)) * t43;
            let t46 = f64x8::splat(1.0) + f64x8::splat(0.0278125) * t11;
            let t51 = f64x8::splat(5.1785) * t14 + f64x8::splat(0.905775) * t11 + f64x8::splat(0.1100325) * t17 + f64x8::splat(0.1241775) * t25;
            let t54 = f64x8::splat(1.0) + f64x8::splat(29.608574643216677) / t51;
            let t55 = (simd::ln(t54));
            let t56 = t46 * t55;
            let t58 = f64x8::splat(0.019751789702565206) * t44 * t56;
            let t60 = f64x8::splat(1.0) / t22 / t7;
            let t61 = t6 * t60;
            let t62 = f64x8::splat(1.0) / t7;
            let t65 = f64x8::splat(1.07924) + f64x8::splat(0.03964) * t14 + f64x8::splat(0.0123825) * t11;
            let t68 = f64x8::splat(1.0) + t14 * t65 / f64x8::splat(2.0);
            let t69 = t68 * t68;
            let t70 = f64x8::splat(1.0) / t69;
            let t73 = -t33 + t58;
            let t76 = t1 * t3 * t2;
            let t78 = f64x8::splat(1.0) / t8 / t7;
            let t79 = t6 * t78;
            let t80 = t76 * t79;
            let t83 = t19 * t20 * t2;
            let t84 = t5 * t60;
            let t85 = t83 * t84;
            let t87 = t7 * t7;
            let t88 = f64x8::splat(1.0) / t87;
            let t90 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t93 = t1 * t3 / t90;
            let t95 = f64x8::splat(1.0) / t8 / t87;
            let t96 = t6 * t95;
            let t97 = t93 * t96;
            let t99 = -f64x8::splat(0.005977859662531589) * t62 + f64x8::splat(0.001317375) * t80 - f64x8::splat(0.00023775) * t85 + f64x8::splat(6.474423634745383e-06) * t88 - f64x8::splat(5.40140625e-07) * t97;
            let t101 = f64x8::splat(0.0011713266981940448) * t62 * t70 - t73 * t99;
            let t102 = f64x8::splat(1.0) / t20;
            let t103 = t1 * t102;
            let t104 = (simd::pow(f64x8::splat(4.0), f64x8::splat(1.0) / f64x8::splat(6.0)));
            let t105 = t103 * t104;
            let t106 = t14 * t11;
            let t107 = t22 * t106;
            let t108 = f64x8::splat(1.0) / t68;
            let t112 = t73 * t73;
            let t114 = f64x8::splat(0.0019711289) * t105 * t107 * t108 - f64x8::splat(2.0) * t112;
            let t115 = f64x8::splat(1.0) / t114;
            let t116 = t101 * t115;
            let t117 = t35 * t35;
            let t118 = ((t34).select(t117, f64x8::splat(1.0)));
            let t119 = t116 * t118;
            let t120 = t61 * t119;
            let t122 = f64x8::splat(1.0) + f64x8::splat(0.025) * t11;
            let t124 = f64x8::splat(1.0) + f64x8::splat(0.04445) * t11;
            let t125 = f64x8::splat(1.0) / t124;
            let t126 = t122 * t125;
            let t128 = v_sigma0 + f64x8::splat(2.0) * v_sigma1 + v_sigma2;
            let t129 = t126 * t128;
            let t131 = f64x8::splat(1.0) / t22 / t87;
            let t132 = t6 * t131;
            let t134 = (simd::ln(t11 / f64x8::splat(4.0)));
            let t135 = t132 * t134;
            let t136 = t118 * t118;
            let t137 = f64x8::splat(1.0) / t136;
            let t138 = t128 * t137;
            let t139 = f64x8::splat(1.0) / t122;
            let t140 = t139 * t124;
            let t141 = t138 * t140;
            let t144 = f64x8::splat(30.0) + f64x8::splat(0.0072806316506996704) * t135 * t141;
            let t145 = t128 * t95;
            let t147 = f64x8::splat(1.0) / t3;
            let t148 = t137 * t19 * t147;
            let t151 = f64x8::splat(30.0) + t145 * t148 / f64x8::splat(48.0);
            let t152 = f64x8::splat(1.0) / t151;
            let t153 = t144 * t152;
            let t154 = t103 * t6;
            let t155 = t22 * t101;
            let t159 = ((f64x8::splat(4.0)).sqrt());
            let t160 = t73 * t159;
            let t161 = t106 * t108;
            let t164 = t6 * t22;
            let t168 = f64x8::splat(0.00619125) * t160 * t161 - f64x8::splat(0.07959333333333334) * t103 * t164 * t99;
            let t169 = t168 * t115;
            let t171 = f64x8::splat(0.07959333333333334) * t154 * t155 * t115 - t169 * t73;
            let t172 = f64x8::splat(1.0) / t171;
            let t173 = t153 * t172;
            let t174 = t129 * t173;
            let t177 = -t33 + t58 + f64x8::splat(0.0010427789137624512) * t120 * t174;
            let t178 = t169 * t118;
            let t179 = t178 * t129;
            let t181 = t95 * t19 * t147;
            let t182 = t181 * t173;
            let t185 = t116 * t136;
            let t186 = t122 * t122;
            let t187 = t124 * t124;
            let t188 = f64x8::splat(1.0) / t187;
            let t189 = t186 * t188;
            let t190 = t128 * t128;
            let t191 = t189 * t190;
            let t192 = t185 * t191;
            let t193 = t87 * t87;
            let t195 = f64x8::splat(1.0) / t22 / t193;
            let t197 = t195 * t1 * t102;
            let t198 = t144 * t144;
            let t199 = t151 * t151;
            let t200 = f64x8::splat(1.0) / t199;
            let t201 = t198 * t200;
            let t202 = t171 * t171;
            let t203 = f64x8::splat(1.0) / t202;
            let t204 = t201 * t203;
            let t205 = t197 * t204;
            let t208 = f64x8::splat(1.0) + f64x8::splat(0.0013900948042322753) * t179 * t182 - f64x8::splat(5.797090694260704e-06) * t192 * t205;
            let t209 = f64x8::splat(1.0) / t208;
            let t210 = t177 * t209;
            let t211 = v_rho0 - v_rho1;
            let t212 = t211 * t62;
            let t213 = f64x8::splat(1.0) + t212;
            let t214 = (t213).simd_le(zeta_threshold);
            let t215 = (simd::cbrt(t213));
            let t217 = ((t214).select(t36, t215 * t213));
            let t218 = f64x8::splat(1.0) - t212;
            let t219 = (t218).simd_le(zeta_threshold);
            let t220 = (simd::cbrt(t218));
            let t222 = ((t219).select(t36, t220 * t218));
            let t224 = (t217 + t222 - f64x8::splat(2.0)) * t43;
            let t225 = (f64x8::splat(2.0)).simd_le(zeta_threshold);
            let t227 = ((t225).select(t36, f64x8::splat(2.0) * t40));
            let t228 = (f64x8::splat(0.0)).simd_le(zeta_threshold);
            let t229 = ((t228).select(t36, f64x8::splat(0.0)));
            let t231 = (t227 + t229 - f64x8::splat(2.0)) * t43;
            let t233 = f64x8::splat(1.0) + f64x8::splat(0.05137) * t11;
            let t238 = f64x8::splat(7.05945) * t14 + f64x8::splat(1.549425) * t11 + f64x8::splat(0.420775) * t17 + f64x8::splat(0.1562925) * t25;
            let t241 = f64x8::splat(1.0) + f64x8::splat(32.1646831778707) / t238;
            let t242 = (simd::ln(t241));
            let t247 = t231 * (-f64x8::splat(0.03109) * t233 * t242 + t33 - f64x8::splat(0.019751789702565206) * t56);
            let t249 = f64x8::splat(0.019751789702565206) * t231 * t56;
            let t252 = f64x8::splat(1.49676) + f64x8::splat(0.00089527) * t14 + f64x8::splat(0.011799625) * t11;
            let t255 = f64x8::splat(1.0) + t14 * t252 / f64x8::splat(2.0);
            let t256 = t255 * t255;
            let t257 = f64x8::splat(1.0) / t256;
            let t260 = -t33 + t247 + t249;
            let t266 = -f64x8::splat(0.0077371026992393175) * t62 + f64x8::splat(0.00187495875) * t80 - f64x8::splat(0.000362780625) * t85 + f64x8::splat(1.0208501871552144e-05) * t88 - f64x8::splat(8.659659375e-07) * t97;
            let t268 = f64x8::splat(0.0010636476373080148) * t62 * t257 - t260 * t266;
            let t269 = f64x8::splat(1.0) / t255;
            let t273 = t260 * t260;
            let t275 = f64x8::splat(0.0005076591995833333) * t105 * t107 * t269 - f64x8::splat(2.0) * t273;
            let t276 = f64x8::splat(1.0) / t275;
            let t277 = t268 * t276;
            let t278 = t40 * t40;
            let t279 = ((t225).select(t117, t278));
            let t280 = ((t228).select(t117, f64x8::splat(0.0)));
            let t282 = t279 / f64x8::splat(2.0) + t280 / f64x8::splat(2.0);
            let t283 = t277 * t282;
            let t284 = t61 * t283;
            let t285 = t282 * t282;
            let t286 = f64x8::splat(1.0) / t285;
            let t287 = t128 * t286;
            let t288 = t287 * t140;
            let t291 = f64x8::splat(30.0) + f64x8::splat(0.0036401987395106744) * t135 * t288;
            let t293 = t286 * t19 * t147;
            let t296 = f64x8::splat(30.0) + t145 * t293 / f64x8::splat(48.0);
            let t297 = f64x8::splat(1.0) / t296;
            let t298 = t291 * t297;
            let t299 = t22 * t268;
            let t303 = t260 * t159;
            let t304 = t106 * t269;
            let t310 = f64x8::splat(0.0058998125) * t303 * t304 - f64x8::splat(0.021511666666666665) * t103 * t164 * t266;
            let t311 = t310 * t276;
            let t313 = f64x8::splat(0.021511666666666665) * t154 * t299 * t276 - t311 * t260;
            let t314 = f64x8::splat(1.0) / t313;
            let t315 = t298 * t314;
            let t316 = t129 * t315;
            let t319 = -t33 + t247 + t249 + f64x8::splat(0.000281831548704497) * t284 * t316;
            let t320 = t311 * t282;
            let t321 = t320 * t129;
            let t322 = t181 * t315;
            let t325 = t277 * t285;
            let t326 = t325 * t191;
            let t327 = t291 * t291;
            let t328 = t296 * t296;
            let t329 = f64x8::splat(1.0) / t328;
            let t330 = t327 * t329;
            let t331 = t313 * t313;
            let t332 = f64x8::splat(1.0) / t331;
            let t333 = t330 * t332;
            let t334 = t197 * t333;
            let t337 = f64x8::splat(1.0) + f64x8::splat(0.0013900948042322753) * t321 * t322 - f64x8::splat(5.797090694260704e-06) * t326 * t334;
            let t338 = f64x8::splat(1.0) / t337;
            let t340 = t319 * t338 - t210;
            let t341 = t224 * t340;
            let tzk0 = t210 + t341;
            acc_zk = tzk0;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        ip += 8;
    }
}
