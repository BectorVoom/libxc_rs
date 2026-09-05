//! GGA_C_REGTPSS vxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_regtpss.c`
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
pub fn gga_c_regtpss_vxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
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
        {
            let t1 = f64x8::splat(M_CBRT3);
            let t2 = f64x8::splat(1.0) / f64x8::splat(M_PI);
            let t3 = (simd::cbrt(t2));
            let t4 = t1 * t3;
            let t5 = f64x8::splat(M_CBRT4);
            let t6 = t5 * t5;
            let t7 = (simd::cbrt(v_rho));
            let t10 = t4 * t6 / t7;
            let t12 = f64x8::splat(1.0) + f64x8::splat(0.053425) * t10;
            let t13 = ((t10).sqrt());
            let t16 = ((t10) * (t10).sqrt());
            let t18 = t1 * t1;
            let t19 = t3 * t3;
            let t20 = t18 * t19;
            let t21 = t7 * t7;
            let t24 = t20 * t5 / t21;
            let t26 = f64x8::splat(3.79785) * t13 + f64x8::splat(0.8969) * t10 + f64x8::splat(0.204775) * t16 + f64x8::splat(0.123235) * t24;
            let t29 = f64x8::splat(1.0) + f64x8::splat(16.081979498692537) / t26;
            let t30 = (simd::ln(t29));
            let t32 = f64x8::splat(0.0621814) * t12 * t30;
            let t33 = (f64x8::splat(1.0)).simd_le(zeta_threshold);
            let t34 = (simd::cbrt(zeta_threshold));
            let t36 = ((t33).select(t34 * zeta_threshold, f64x8::splat(1.0)));
            let t39 = f64x8::splat(M_CBRT2);
            let t43 = (f64x8::splat(2.0) * t36 - f64x8::splat(2.0)) / (f64x8::splat(2.0) * t39 - f64x8::splat(2.0));
            let t45 = f64x8::splat(1.0) + f64x8::splat(0.0278125) * t10;
            let t50 = f64x8::splat(5.1785) * t13 + f64x8::splat(0.905775) * t10 + f64x8::splat(0.1100325) * t16 + f64x8::splat(0.1241775) * t24;
            let t53 = f64x8::splat(1.0) + f64x8::splat(29.608749977793437) / t50;
            let t54 = (simd::ln(t53));
            let t57 = f64x8::splat(0.0197516734986138) * t43 * t45 * t54;
            let t58 = (simd::ln(f64x8::splat(2.0)));
            let t59 = f64x8::splat(1.0) - t58;
            let t60 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t61 = f64x8::splat(1.0) / t60;
            let t62 = t59 * t61;
            let t63 = t34 * t34;
            let t64 = ((t33).select(t63, f64x8::splat(1.0)));
            let t65 = t64 * t64;
            let t66 = t65 * t64;
            let t68 = f64x8::splat(1.0) + f64x8::splat(0.025) * t10;
            let t70 = f64x8::splat(1.0) + f64x8::splat(0.04445) * t10;
            let t71 = f64x8::splat(1.0) / t70;
            let t72 = t68 * t71;
            let t73 = v_rho * v_rho;
            let t75 = f64x8::splat(1.0) / t7 / t73;
            let t78 = f64x8::splat(1.0) / t65;
            let t80 = f64x8::splat(1.0) / t3;
            let t81 = t80 * t5;
            let t82 = t78 * t18 * t81;
            let t85 = f64x8::splat(1.0) / t59;
            let t88 = f64x8::splat(1.0) / t66;
            let t89 = t60 * t88;
            let t91 = (simd::exp(-(-t32 + t57) * t85 * t89));
            let t92 = t91 - f64x8::splat(1.0);
            let t93 = f64x8::splat(1.0) / t92;
            let t94 = t85 * t93;
            let t95 = v_sigma * v_sigma;
            let t96 = t94 * t95;
            let t97 = t72 * t96;
            let t98 = t73 * t73;
            let t100 = f64x8::splat(1.0) / t21 / t98;
            let t101 = t39 * t39;
            let t102 = t100 * t101;
            let t103 = t65 * t65;
            let t104 = f64x8::splat(1.0) / t103;
            let t105 = t102 * t104;
            let t106 = f64x8::splat(1.0) / t19;
            let t107 = t1 * t106;
            let t108 = t107 * t6;
            let t109 = t105 * t108;
            let t112 = v_sigma * t75 * t39 * t82 / f64x8::splat(96.0) + f64x8::splat(0.0002143700905903487) * t97 * t109;
            let t113 = t112 * t85;
            let t114 = t94 * t112;
            let t117 = f64x8::splat(1.0) + f64x8::splat(0.6585449182935511) * t72 * t114;
            let t118 = f64x8::splat(1.0) / t117;
            let t119 = t113 * t118;
            let t122 = f64x8::splat(1.0) + f64x8::splat(0.6585449182935511) * t72 * t119;
            let t123 = (simd::ln(t122));
            let t125 = t62 * t66 * t123;
            let tzk0 = -t32 + t57 + t125;
            acc_zk = tzk0;
            let t127 = f64x8::splat(1.0) / t7 / v_rho;
            let t128 = t6 * t127;
            let t130 = t4 * t128 * t30;
            let t131 = f64x8::splat(0.0011073470983333333) * t130;
            let t132 = t26 * t26;
            let t133 = f64x8::splat(1.0) / t132;
            let t134 = t12 * t133;
            let t136 = f64x8::splat(1.0) / t13 * t1;
            let t137 = t3 * t6;
            let t138 = t137 * t127;
            let t139 = t136 * t138;
            let t141 = t4 * t128;
            let t143 = ((t10).sqrt());
            let t144 = t143 * t1;
            let t145 = t144 * t138;
            let t150 = t20 * t5 / t21 / v_rho;
            let t152 = -f64x8::splat(0.632975) * t139 - f64x8::splat(0.29896666666666666) * t141 - f64x8::splat(0.1023875) * t145 - f64x8::splat(0.08215666666666667) * t150;
            let t153 = f64x8::splat(1.0) / t29;
            let t154 = t152 * t153;
            let t155 = t134 * t154;
            let t156 = f64x8::splat(1.0) * t155;
            let t157 = t43 * t1;
            let t160 = t157 * t137 * t127 * t54;
            let t161 = f64x8::splat(0.00018311447306006544) * t160;
            let t162 = t43 * t45;
            let t163 = t50 * t50;
            let t164 = f64x8::splat(1.0) / t163;
            let t169 = -f64x8::splat(0.8630833333333333) * t139 - f64x8::splat(0.301925) * t141 - f64x8::splat(0.05501625) * t145 - f64x8::splat(0.082785) * t150;
            let t171 = f64x8::splat(1.0) / t53;
            let t172 = t164 * t169 * t171;
            let t173 = t162 * t172;
            let t174 = f64x8::splat(0.5848223622634646) * t173;
            let t175 = t71 * t112;
            let t176 = t85 * t118;
            let t177 = t175 * t176;
            let t180 = t70 * t70;
            let t181 = f64x8::splat(1.0) / t180;
            let t182 = t68 * t181;
            let t183 = t182 * t113;
            let t184 = t118 * t1;
            let t185 = t184 * t138;
            let t188 = t73 * v_rho;
            let t190 = f64x8::splat(1.0) / t7 / t188;
            let t195 = t18 * t80;
            let t196 = t98 * t73;
            let t197 = f64x8::splat(1.0) / t196;
            let t200 = t195 * t5 * t197 * t71;
            let t201 = t95 * t101;
            let t203 = t94 * t201 * t104;
            let t206 = t182 * t96;
            let t207 = t197 * t101;
            let t208 = t207 * t104;
            let t209 = t195 * t5;
            let t210 = t208 * t209;
            let t213 = t59 * t59;
            let t214 = f64x8::splat(1.0) / t213;
            let t215 = t72 * t214;
            let t216 = t92 * t92;
            let t217 = f64x8::splat(1.0) / t216;
            let t218 = t217 * t95;
            let t219 = t218 * t102;
            let t220 = t215 * t219;
            let t222 = f64x8::splat(1.0) / t103 / t66;
            let t223 = t222 * t1;
            let t224 = t223 * t106;
            let t225 = t131 + t156 - t161 - t174;
            let t227 = t60 * t91;
            let t228 = t6 * t225 * t227;
            let t229 = t224 * t228;
            let t232 = t98 * v_rho;
            let t234 = f64x8::splat(1.0) / t21 / t232;
            let t235 = t234 * t101;
            let t236 = t235 * t104;
            let t237 = t236 * t108;
            let t240 = -f64x8::splat(7.0) / f64x8::splat(288.0) * v_sigma * t190 * t39 * t82 - f64x8::splat(7.145669686344956e-06) * t200 * t203 + f64x8::splat(1.2705000702321332e-05) * t206 * t210 + f64x8::splat(0.0002143700905903487) * t220 * t229 - f64x8::splat(0.0010003937560882938) * t97 * t237;
            let t241 = t240 * t85;
            let t242 = t241 * t118;
            let t245 = t72 * t112;
            let t246 = t117 * t117;
            let t247 = f64x8::splat(1.0) / t246;
            let t248 = t85 * t247;
            let t249 = t71 * t85;
            let t250 = t93 * t112;
            let t251 = t249 * t250;
            let t254 = t182 * t94;
            let t255 = t112 * t1;
            let t259 = t214 * t217;
            let t260 = t72 * t259;
            let t261 = t112 * t225;
            let t262 = t89 * t91;
            let t263 = t261 * t262;
            let t266 = t94 * t240;
            let t269 = -f64x8::splat(0.005487874319112926) * t141 * t251 + f64x8::splat(0.009757440539382782) * t254 * t255 * t138 + f64x8::splat(0.6585449182935511) * t260 * t263 + f64x8::splat(0.6585449182935511) * t72 * t266;
            let t270 = t248 * t269;
            let t273 = -f64x8::splat(0.005487874319112926) * t141 * t177 + f64x8::splat(0.009757440539382782) * t183 * t185 + f64x8::splat(0.6585449182935511) * t72 * t242 - f64x8::splat(0.6585449182935511) * t245 * t270;
            let t275 = f64x8::splat(1.0) / t122;
            let t277 = t62 * t66 * t273 * t275;
            let tvrho0 = -t32 + t57 + t125 + v_rho * (t131 + t156 - t161 - t174 + t277);
            acc_vrho = tvrho0;
            let t280 = v_rho * t59;
            let t281 = t280 * t61;
            let t286 = t94 * v_sigma;
            let t287 = t72 * t286;
            let t290 = t75 * t39 * t78 * t209 / f64x8::splat(96.0) + f64x8::splat(0.0004287401811806974) * t287 * t109;
            let t291 = t290 * t85;
            let t292 = t291 * t118;
            let t295 = t68 * t68;
            let t296 = t295 * t181;
            let t297 = t296 * t112;
            let t298 = t214 * t247;
            let t299 = t93 * t290;
            let t300 = t298 * t299;
            let t303 = f64x8::splat(0.6585449182935511) * t72 * t292 - f64x8::splat(0.43368140941025995) * t297 * t300;
            let t304 = t66 * t303;
            let t305 = t304 * t275;
            let tvsigma0 = t281 * t305;
            acc_vsigma = tvsigma0;
        }
        store_add(zk, ip, m, acc_zk);
        store_add(vrho, ip, m, acc_vrho);
        store_add(vsigma, ip, m, acc_vsigma);
        ip += 8;
    }
}
