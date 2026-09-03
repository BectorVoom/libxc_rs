//! GGA_C_GAPLOC exc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_gaploc.c`
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
pub fn gga_c_gaploc_exc_pol(
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
            let t59 = t3 * t2;
            let t60 = f64x8::splat(1.0) / t59;
            let t61 = t19 * t60;
            let t62 = t8 * t7;
            let t63 = t6 * t62;
            let t64 = f64x8::splat(1.0) / t7;
            let t67 = f64x8::splat(1.07924) + f64x8::splat(0.03964) * t14 + f64x8::splat(0.0123825) * t11;
            let t70 = f64x8::splat(1.0) + t14 * t67 / f64x8::splat(2.0);
            let t71 = t70 * t70;
            let t72 = f64x8::splat(1.0) / t71;
            let t75 = -t33 + t58;
            let t77 = t1 * t59;
            let t78 = f64x8::splat(1.0) / t62;
            let t79 = t6 * t78;
            let t80 = t77 * t79;
            let t83 = t19 * t20 * t2;
            let t85 = f64x8::splat(1.0) / t22 / t7;
            let t86 = t5 * t85;
            let t87 = t83 * t86;
            let t89 = t7 * t7;
            let t90 = f64x8::splat(1.0) / t89;
            let t92 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t95 = t1 * t3 / t92;
            let t97 = f64x8::splat(1.0) / t8 / t89;
            let t98 = t6 * t97;
            let t99 = t95 * t98;
            let t101 = -f64x8::splat(0.005977859662531589) * t64 + f64x8::splat(0.001317375) * t80 - f64x8::splat(0.00023775) * t87 + f64x8::splat(6.474423634745383e-06) * t90 - f64x8::splat(5.40140625e-07) * t99;
            let t103 = f64x8::splat(0.0011713266981940448) * t64 * t72 - t75 * t101;
            let t105 = t61 * t63 * t103;
            let t106 = f64x8::splat(1.0) / t20;
            let t107 = t1 * t106;
            let t108 = (simd::pow(f64x8::splat(4.0), f64x8::splat(1.0) / f64x8::splat(6.0)));
            let t109 = t107 * t108;
            let t110 = t14 * t11;
            let t111 = t22 * t110;
            let t112 = f64x8::splat(1.0) / t70;
            let t116 = t75 * t75;
            let t118 = f64x8::splat(0.0019711289) * t109 * t111 * t112 - f64x8::splat(2.0) * t116;
            let t119 = f64x8::splat(1.0) / t118;
            let t120 = (simd::cbrt(f64x8::splat(9.0)));
            let t121 = t120 * t120;
            let t122 = t119 * t121;
            let t123 = f64x8::splat(M_CBRT6);
            let t124 = t123 * t123;
            let t125 = (simd::cbrt(t92));
            let t129 = v_sigma0 + f64x8::splat(2.0) * v_sigma1 + v_sigma2;
            let t130 = ((t129).sqrt());
            let t134 = t124 / t125 * t130 * t78 * t40 / f64x8::splat(12.0);
            let t135 = t130 * t129;
            let t136 = t89 * t7;
            let t137 = ((t7).sqrt());
            let t139 = f64x8::splat(1.0) / t137 / t136;
            let t140 = t135 * t139;
            let t141 = t35 * t35;
            let t142 = ((t34).select(t141, f64x8::splat(1.0)));
            let t143 = t142 * t142;
            let t145 = f64x8::splat(1.0) / t143 / t142;
            let t146 = ((f64x8::splat(3.0)).sqrt());
            let t148 = ((t2).sqrt());
            let t149 = f64x8::splat(1.0) / t148;
            let t151 = t140 * t145 * t146 * t149;
            let t153 = f64x8::splat(8.54613) + t151 / f64x8::splat(64.0);
            let t155 = f64x8::splat(1.0) + t151 / f64x8::splat(192.0);
            let t156 = f64x8::splat(1.0) / t155;
            let t157 = t153 * t156;
            let t158 = (simd::pow(t134, t157));
            let t159 = t125 * t125;
            let t160 = f64x8::splat(1.0) / t159;
            let t161 = t123 * t160;
            let t163 = f64x8::splat(1.0) / t22 / t89;
            let t165 = t40 * t40;
            let t169 = f64x8::splat(14.709046) + t161 * t129 * t163 * t165 / f64x8::splat(24.0);
            let t171 = f64x8::splat(1.0) + t158;
            let t172 = f64x8::splat(1.0) / t171;
            let t174 = t122 * t158 * t169 * t172;
            let t177 = -t33 + t58 + f64x8::splat(0.02845500663567615) * t105 * t174;
            let t178 = ((f64x8::splat(4.0)).sqrt());
            let t179 = t75 * t178;
            let t180 = t110 * t112;
            let t183 = t6 * t22;
            let t187 = f64x8::splat(0.00619125) * t179 * t180 - f64x8::splat(0.07959333333333334) * t107 * t183 * t101;
            let t188 = t187 * t119;
            let t189 = t121 * t158;
            let t190 = t188 * t189;
            let t191 = t22 * t169;
            let t192 = t191 * t172;
            let t193 = t107 * t192;
            let t196 = t103 * t119;
            let t197 = t158 * t158;
            let t198 = t120 * t197;
            let t199 = t196 * t198;
            let t200 = t169 * t169;
            let t201 = t62 * t200;
            let t202 = t171 * t171;
            let t203 = f64x8::splat(1.0) / t202;
            let t204 = t201 * t203;
            let t205 = t61 * t204;
            let t208 = f64x8::splat(1.0) + f64x8::splat(0.3575048995185043) * t190 * t193 - f64x8::splat(1.1502877786176224) * t199 * t205;
            let t209 = f64x8::splat(1.0) / t208;
            let t210 = t177 * t209;
            let t211 = v_rho0 - v_rho1;
            let t212 = t211 * t64;
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
            let t266 = -f64x8::splat(0.0077371026992393175) * t64 + f64x8::splat(0.00187495875) * t80 - f64x8::splat(0.000362780625) * t87 + f64x8::splat(1.0208501871552144e-05) * t90 - f64x8::splat(8.659659375e-07) * t99;
            let t268 = f64x8::splat(0.0010636476373080148) * t64 * t257 - t260 * t266;
            let t270 = t61 * t63 * t268;
            let t271 = f64x8::splat(1.0) / t255;
            let t275 = t260 * t260;
            let t277 = f64x8::splat(0.0005076591995833333) * t109 * t111 * t271 - f64x8::splat(2.0) * t275;
            let t278 = f64x8::splat(1.0) / t277;
            let t279 = t278 * t121;
            let t280 = ((t225).select(t141, t165));
            let t281 = ((t228).select(t141, f64x8::splat(0.0)));
            let t283 = t280 / f64x8::splat(2.0) + t281 / f64x8::splat(2.0);
            let t284 = t283 * t283;
            let t286 = f64x8::splat(1.0) / t284 / t283;
            let t289 = t140 * t286 * t146 * t149;
            let t291 = f64x8::splat(8.54613) + t289 / f64x8::splat(64.0);
            let t293 = f64x8::splat(1.0) + t289 / f64x8::splat(192.0);
            let t294 = f64x8::splat(1.0) / t293;
            let t295 = t291 * t294;
            let t296 = (simd::pow(t134, t295));
            let t298 = f64x8::splat(1.0) + t296;
            let t299 = f64x8::splat(1.0) / t298;
            let t301 = t279 * t296 * t169 * t299;
            let t304 = -t33 + t247 + t249 + f64x8::splat(0.007690526230142224) * t270 * t301;
            let t305 = t260 * t178;
            let t306 = t110 * t271;
            let t312 = f64x8::splat(0.0058998125) * t305 * t306 - f64x8::splat(0.021511666666666665) * t107 * t183 * t266;
            let t313 = t312 * t278;
            let t314 = t121 * t296;
            let t315 = t313 * t314;
            let t316 = t191 * t299;
            let t317 = t107 * t316;
            let t320 = t268 * t278;
            let t321 = t296 * t296;
            let t322 = t120 * t321;
            let t323 = t320 * t322;
            let t324 = t298 * t298;
            let t325 = f64x8::splat(1.0) / t324;
            let t326 = t201 * t325;
            let t327 = t61 * t326;
            let t330 = f64x8::splat(1.0) + f64x8::splat(0.3575048995185043) * t315 * t317 - f64x8::splat(1.1502877786176224) * t323 * t327;
            let t331 = f64x8::splat(1.0) / t330;
            let t333 = t304 * t331 - t210;
            let t334 = t224 * t333;
            let tzk0 = t210 + t334;
            acc_zk = tzk0;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        ip += 8;
    }
}
