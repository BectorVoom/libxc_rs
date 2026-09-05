//! GGA_C_GAPC exc unpol kernel — explicit SIMD (bit-exact).
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
pub fn gga_c_gapc_exc_unpol(
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
        let v_rho = load(rho, ip, np);
        let v_sigma = load(sigma, ip, np);
        let mut acc_zk = V_ZERO;
        {
            let t1 = f64x8::splat(M_CBRT3);
            let t2 = f64x8::splat(1.0) / f64x8::splat(M_PI);
            let t3 = (simd::cbrt(t2));
            let t4 = t1 * t3;
            let t5 = f64x8::splat(M_CBRT4);
            let t6 = t5 * t5;
            let t7 = (simd::cbrt(v_rho));
            let t8 = f64x8::splat(1.0) / t7;
            let t9 = t6 * t8;
            let t10 = t4 * t9;
            let t12 = f64x8::splat(1.0) + f64x8::splat(0.053425) * t10;
            let t13 = ((t10).sqrt());
            let t16 = ((t10) * (t10).sqrt());
            let t18 = t1 * t1;
            let t19 = t3 * t3;
            let t20 = t18 * t19;
            let t21 = t7 * t7;
            let t22 = f64x8::splat(1.0) / t21;
            let t24 = t20 * t5 * t22;
            let t26 = f64x8::splat(3.79785) * t13 + f64x8::splat(0.8969) * t10 + f64x8::splat(0.204775) * t16 + f64x8::splat(0.123235) * t24;
            let t29 = f64x8::splat(1.0) + f64x8::splat(16.081824322151103) / t26;
            let t30 = (simd::ln(t29));
            let t32 = f64x8::splat(0.062182) * t12 * t30;
            let t33 = (f64x8::splat(1.0)).simd_le(zeta_threshold);
            let t34 = (simd::cbrt(zeta_threshold));
            let t35 = t34 * zeta_threshold;
            let t36 = ((t33).select(t35, f64x8::splat(1.0)));
            let t39 = f64x8::splat(M_CBRT2);
            let t42 = f64x8::splat(1.0) / (f64x8::splat(2.0) * t39 - f64x8::splat(2.0));
            let t43 = (f64x8::splat(2.0) * t36 - f64x8::splat(2.0)) * t42;
            let t45 = f64x8::splat(1.0) + f64x8::splat(0.0278125) * t10;
            let t50 = f64x8::splat(5.1785) * t13 + f64x8::splat(0.905775) * t10 + f64x8::splat(0.1100325) * t16 + f64x8::splat(0.1241775) * t24;
            let t53 = f64x8::splat(1.0) + f64x8::splat(29.608574643216677) / t50;
            let t54 = (simd::ln(t53));
            let t55 = t45 * t54;
            let t57 = f64x8::splat(0.019751789702565206) * t43 * t55;
            let t59 = f64x8::splat(1.0) / t21 / v_rho;
            let t60 = t6 * t59;
            let t61 = f64x8::splat(1.0) / v_rho;
            let t64 = f64x8::splat(1.07924) + f64x8::splat(0.03964) * t13 + f64x8::splat(0.0123825) * t10;
            let t67 = f64x8::splat(1.0) + t13 * t64 / f64x8::splat(2.0);
            let t68 = t67 * t67;
            let t69 = f64x8::splat(1.0) / t68;
            let t72 = -t32 + t57;
            let t75 = t1 * t3 * t2;
            let t77 = f64x8::splat(1.0) / t7 / v_rho;
            let t78 = t6 * t77;
            let t79 = t75 * t78;
            let t82 = t18 * t19 * t2;
            let t83 = t5 * t59;
            let t84 = t82 * t83;
            let t86 = v_rho * v_rho;
            let t87 = f64x8::splat(1.0) / t86;
            let t89 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t92 = t1 * t3 / t89;
            let t94 = f64x8::splat(1.0) / t7 / t86;
            let t95 = t6 * t94;
            let t96 = t92 * t95;
            let t98 = -f64x8::splat(0.005977859662531589) * t61 + f64x8::splat(0.001317375) * t79 - f64x8::splat(0.00023775) * t84 + f64x8::splat(6.474423634745383e-06) * t87 - f64x8::splat(5.40140625e-07) * t96;
            let t100 = f64x8::splat(0.0011713266981940448) * t61 * t69 - t72 * t98;
            let t101 = f64x8::splat(1.0) / t19;
            let t102 = t1 * t101;
            let t103 = (simd::pow(f64x8::splat(4.0), f64x8::splat(1.0) / f64x8::splat(6.0)));
            let t104 = t102 * t103;
            let t105 = t13 * t10;
            let t106 = t21 * t105;
            let t107 = f64x8::splat(1.0) / t67;
            let t111 = t72 * t72;
            let t113 = f64x8::splat(0.0019711289) * t104 * t106 * t107 - f64x8::splat(2.0) * t111;
            let t114 = f64x8::splat(1.0) / t113;
            let t115 = t100 * t114;
            let t116 = t34 * t34;
            let t117 = ((t33).select(t116, f64x8::splat(1.0)));
            let t118 = t115 * t117;
            let t119 = t60 * t118;
            let t121 = f64x8::splat(1.0) + f64x8::splat(0.025) * t10;
            let t123 = f64x8::splat(1.0) + f64x8::splat(0.04445) * t10;
            let t124 = f64x8::splat(1.0) / t123;
            let t125 = t121 * t124;
            let t126 = t125 * v_sigma;
            let t128 = f64x8::splat(1.0) / t21 / t86;
            let t129 = t6 * t128;
            let t131 = (simd::ln(t10 / f64x8::splat(4.0)));
            let t132 = t129 * t131;
            let t133 = t117 * t117;
            let t134 = f64x8::splat(1.0) / t133;
            let t135 = v_sigma * t134;
            let t136 = f64x8::splat(1.0) / t121;
            let t137 = t136 * t123;
            let t138 = t135 * t137;
            let t141 = f64x8::splat(30.0) + f64x8::splat(0.0072806316506996704) * t132 * t138;
            let t142 = v_sigma * t94;
            let t143 = t134 * t18;
            let t144 = f64x8::splat(1.0) / t3;
            let t145 = t143 * t144;
            let t148 = f64x8::splat(30.0) + t142 * t145 / f64x8::splat(48.0);
            let t149 = f64x8::splat(1.0) / t148;
            let t150 = t141 * t149;
            let t151 = t102 * t6;
            let t152 = t21 * t100;
            let t156 = ((f64x8::splat(4.0)).sqrt());
            let t157 = t72 * t156;
            let t158 = t105 * t107;
            let t161 = t6 * t21;
            let t165 = f64x8::splat(0.00619125) * t157 * t158 - f64x8::splat(0.07959333333333334) * t102 * t161 * t98;
            let t166 = t165 * t114;
            let t168 = f64x8::splat(0.07959333333333334) * t151 * t152 * t114 - t166 * t72;
            let t169 = f64x8::splat(1.0) / t168;
            let t170 = t150 * t169;
            let t171 = t126 * t170;
            let t174 = -t32 + t57 + f64x8::splat(0.0010427789137624512) * t119 * t171;
            let t175 = t166 * t117;
            let t176 = t175 * t126;
            let t178 = t94 * t18 * t144;
            let t179 = t178 * t170;
            let t182 = t115 * t133;
            let t183 = t121 * t121;
            let t184 = t123 * t123;
            let t185 = f64x8::splat(1.0) / t184;
            let t186 = t183 * t185;
            let t187 = v_sigma * v_sigma;
            let t188 = t186 * t187;
            let t189 = t182 * t188;
            let t190 = t86 * t86;
            let t192 = f64x8::splat(1.0) / t21 / t190;
            let t194 = t192 * t1 * t101;
            let t195 = t141 * t141;
            let t196 = t148 * t148;
            let t197 = f64x8::splat(1.0) / t196;
            let t198 = t195 * t197;
            let t199 = t168 * t168;
            let t200 = f64x8::splat(1.0) / t199;
            let t201 = t198 * t200;
            let t202 = t194 * t201;
            let t205 = f64x8::splat(1.0) + f64x8::splat(0.0013900948042322753) * t176 * t179 - f64x8::splat(5.797090694260704e-06) * t189 * t202;
            let t206 = f64x8::splat(1.0) / t205;
            let t207 = t174 * t206;
            let t208 = (f64x8::splat(2.0)).simd_le(zeta_threshold);
            let t210 = ((t208).select(t35, f64x8::splat(2.0) * t39));
            let t211 = (f64x8::splat(0.0)).simd_le(zeta_threshold);
            let t212 = ((t211).select(t35, f64x8::splat(0.0)));
            let t214 = (t210 + t212 - f64x8::splat(2.0)) * t42;
            let t216 = f64x8::splat(1.0) + f64x8::splat(0.05137) * t10;
            let t221 = f64x8::splat(7.05945) * t13 + f64x8::splat(1.549425) * t10 + f64x8::splat(0.420775) * t16 + f64x8::splat(0.1562925) * t24;
            let t224 = f64x8::splat(1.0) + f64x8::splat(32.1646831778707) / t221;
            let t225 = (simd::ln(t224));
            let t230 = t214 * (-f64x8::splat(0.03109) * t216 * t225 + t32 - f64x8::splat(0.019751789702565206) * t55);
            let t232 = f64x8::splat(0.019751789702565206) * t214 * t55;
            let t235 = f64x8::splat(1.49676) + f64x8::splat(0.00089527) * t13 + f64x8::splat(0.011799625) * t10;
            let t238 = f64x8::splat(1.0) + t13 * t235 / f64x8::splat(2.0);
            let t239 = t238 * t238;
            let t240 = f64x8::splat(1.0) / t239;
            let t243 = -t32 + t230 + t232;
            let t249 = -f64x8::splat(0.0077371026992393175) * t61 + f64x8::splat(0.00187495875) * t79 - f64x8::splat(0.000362780625) * t84 + f64x8::splat(1.0208501871552144e-05) * t87 - f64x8::splat(8.659659375e-07) * t96;
            let t251 = f64x8::splat(0.0010636476373080148) * t61 * t240 - t243 * t249;
            let t252 = f64x8::splat(1.0) / t238;
            let t256 = t243 * t243;
            let t258 = f64x8::splat(0.0005076591995833333) * t104 * t106 * t252 - f64x8::splat(2.0) * t256;
            let t259 = f64x8::splat(1.0) / t258;
            let t260 = t251 * t259;
            let t261 = t39 * t39;
            let t262 = ((t208).select(t116, t261));
            let t263 = ((t211).select(t116, f64x8::splat(0.0)));
            let t265 = t262 / f64x8::splat(2.0) + t263 / f64x8::splat(2.0);
            let t266 = t260 * t265;
            let t267 = t60 * t266;
            let t268 = t265 * t265;
            let t269 = f64x8::splat(1.0) / t268;
            let t270 = v_sigma * t269;
            let t271 = t270 * t137;
            let t274 = f64x8::splat(30.0) + f64x8::splat(0.0036401987395106744) * t132 * t271;
            let t276 = t269 * t18 * t144;
            let t279 = f64x8::splat(30.0) + t142 * t276 / f64x8::splat(48.0);
            let t280 = f64x8::splat(1.0) / t279;
            let t281 = t274 * t280;
            let t282 = t21 * t251;
            let t286 = t243 * t156;
            let t287 = t105 * t252;
            let t293 = f64x8::splat(0.0058998125) * t286 * t287 - f64x8::splat(0.021511666666666665) * t102 * t161 * t249;
            let t294 = t293 * t259;
            let t296 = f64x8::splat(0.021511666666666665) * t151 * t282 * t259 - t294 * t243;
            let t297 = f64x8::splat(1.0) / t296;
            let t298 = t281 * t297;
            let t299 = t126 * t298;
            let t302 = -t32 + t230 + t232 + f64x8::splat(0.000281831548704497) * t267 * t299;
            let t303 = t294 * t265;
            let t304 = t303 * t126;
            let t305 = t178 * t298;
            let t308 = t260 * t268;
            let t309 = t308 * t188;
            let t310 = t274 * t274;
            let t311 = t279 * t279;
            let t312 = f64x8::splat(1.0) / t311;
            let t313 = t310 * t312;
            let t314 = t296 * t296;
            let t315 = f64x8::splat(1.0) / t314;
            let t316 = t313 * t315;
            let t317 = t194 * t316;
            let t320 = f64x8::splat(1.0) + f64x8::splat(0.0013900948042322753) * t304 * t305 - f64x8::splat(5.797090694260704e-06) * t309 * t317;
            let t321 = f64x8::splat(1.0) / t320;
            let t324 = t43 * (t302 * t321 - t207);
            let tzk0 = t207 + t324;
            acc_zk = tzk0;
        }
        store_add(zk, ip, m, acc_zk);
        ip += 8;
    }
}
