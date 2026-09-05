//! GGA_C_ZVPBEINT vxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_zvpbeint.c`
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
pub fn gga_c_zvpbeint_vxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    param_alpha: f64,
    param_omega: f64,
    param_beta: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_alpha = f64x8::splat(param_alpha);
    let param_omega = f64x8::splat(param_omega);
    let param_beta = f64x8::splat(param_beta);
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
            let t58 = ((v_sigma).sqrt());
            let t59 = t58 * v_sigma;
            let t60 = param_alpha * t59;
            let t61 = v_rho * v_rho;
            let t62 = t61 * t61;
            let t63 = f64x8::splat(1.0) / t62;
            let t66 = f64x8::splat(1.0) / t13 / t10;
            let t67 = f64x8::splat(1.0) / t3;
            let t68 = t18 * t67;
            let t70 = t68 * t5 * t7;
            let t71 = ((t70).sqrt());
            let t72 = t66 * t71;
            let t74 = (((f64x8::splat(1e-20)).simd_lt(f64x8::splat(0.0))).select(f64x8::splat(0.0), f64x8::splat(1e-20)));
            let t76 = (simd::pow(t74, param_omega / f64x8::splat(2.0)));
            let t77 = t72 * t76;
            let t80 = (simd::exp(-t60 * t63 * t77 / f64x8::splat(16.0)));
            let t81 = (simd::ln(f64x8::splat(2.0)));
            let t82 = f64x8::splat(1.0) - t81;
            let t83 = t80 * t82;
            let t84 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t85 = f64x8::splat(1.0) / t84;
            let t86 = t34 * t34;
            let t87 = ((t33).select(t86, f64x8::splat(1.0)));
            let t88 = t87 * t87;
            let t89 = t88 * t87;
            let t90 = t85 * t89;
            let t92 = f64x8::splat(1.0) / t7 / t61;
            let t95 = f64x8::splat(1.0) / t88;
            let t97 = t67 * t5;
            let t98 = t95 * t18 * t97;
            let t101 = f64x8::splat(1.0) / t82;
            let t102 = param_beta * t101;
            let t105 = f64x8::splat(1.0) / t89;
            let t108 = (simd::exp(-(-t32 + t57) * t101 * t84 * t105));
            let t109 = t108 - f64x8::splat(1.0);
            let t110 = f64x8::splat(1.0) / t109;
            let t111 = t84 * t110;
            let t112 = v_sigma * v_sigma;
            let t114 = t102 * t111 * t112;
            let t116 = f64x8::splat(1.0) / t21 / t62;
            let t117 = t39 * t39;
            let t118 = t116 * t117;
            let t119 = t88 * t88;
            let t120 = f64x8::splat(1.0) / t119;
            let t121 = t118 * t120;
            let t122 = f64x8::splat(1.0) / t19;
            let t123 = t1 * t122;
            let t124 = t123 * t6;
            let t125 = t121 * t124;
            let t128 = v_sigma * t92 * t39 * t98 / f64x8::splat(96.0) + t114 * t125 / f64x8::splat(3072.0);
            let t129 = param_beta * t128;
            let t133 = t102 * t111 * t128 + f64x8::splat(1.0);
            let t134 = f64x8::splat(1.0) / t133;
            let t135 = t101 * t84 * t134;
            let t137 = t129 * t135 + f64x8::splat(1.0);
            let t138 = (simd::ln(t137));
            let t139 = t90 * t138;
            let t140 = t83 * t139;
            let tzk0 = -t32 + t57 + t140;
            acc_zk = tzk0;
            let t142 = f64x8::splat(1.0) / t7 / v_rho;
            let t143 = t6 * t142;
            let t145 = t4 * t143 * t30;
            let t146 = f64x8::splat(0.0011073470983333333) * t145;
            let t147 = t26 * t26;
            let t148 = f64x8::splat(1.0) / t147;
            let t149 = t12 * t148;
            let t151 = f64x8::splat(1.0) / t13 * t1;
            let t152 = t3 * t6;
            let t153 = t152 * t142;
            let t154 = t151 * t153;
            let t156 = t4 * t143;
            let t158 = ((t10).sqrt());
            let t159 = t158 * t1;
            let t160 = t159 * t153;
            let t164 = t5 / t21 / v_rho;
            let t165 = t20 * t164;
            let t167 = -f64x8::splat(0.632975) * t154 - f64x8::splat(0.29896666666666666) * t156 - f64x8::splat(0.1023875) * t160 - f64x8::splat(0.08215666666666667) * t165;
            let t168 = f64x8::splat(1.0) / t29;
            let t169 = t167 * t168;
            let t170 = t149 * t169;
            let t171 = f64x8::splat(1.0) * t170;
            let t172 = t43 * t1;
            let t175 = t172 * t152 * t142 * t54;
            let t176 = f64x8::splat(0.00018311447306006544) * t175;
            let t177 = t43 * t45;
            let t178 = t50 * t50;
            let t179 = f64x8::splat(1.0) / t178;
            let t184 = -f64x8::splat(0.8630833333333333) * t154 - f64x8::splat(0.301925) * t156 - f64x8::splat(0.05501625) * t160 - f64x8::splat(0.082785) * t165;
            let t186 = f64x8::splat(1.0) / t53;
            let t187 = t179 * t184 * t186;
            let t188 = t177 * t187;
            let t189 = f64x8::splat(0.5848223622634646) * t188;
            let t190 = t62 * v_rho;
            let t191 = f64x8::splat(1.0) / t190;
            let t196 = f64x8::splat(1.0) / t7 / t190;
            let t199 = f64x8::splat(1.0) / t13 / t24 / f64x8::splat(4.0);
            let t200 = t196 * t199;
            let t202 = t71 * t76;
            let t203 = t4 * t6;
            let t204 = t202 * t203;
            let t207 = t116 * t66;
            let t209 = f64x8::splat(1.0) / t71;
            let t210 = t209 * t76;
            let t211 = t68 * t5;
            let t212 = t210 * t211;
            let t215 = t60 * t191 * t77 / f64x8::splat(4.0) - t60 * t200 * t204 / f64x8::splat(32.0) - t60 * t207 * t212 / f64x8::splat(96.0);
            let t216 = t215 * t80;
            let t217 = t216 * t82;
            let t218 = t217 * t139;
            let t219 = t83 * t85;
            let t220 = t61 * v_rho;
            let t222 = f64x8::splat(1.0) / t7 / t220;
            let t227 = t82 * t82;
            let t228 = f64x8::splat(1.0) / t227;
            let t229 = param_beta * t228;
            let t230 = t84 * t84;
            let t231 = t229 * t230;
            let t232 = t109 * t109;
            let t233 = f64x8::splat(1.0) / t232;
            let t234 = t233 * t112;
            let t235 = t234 * t116;
            let t236 = t231 * t235;
            let t238 = f64x8::splat(1.0) / t119 / t89;
            let t239 = t117 * t238;
            let t240 = t239 * t1;
            let t241 = t122 * t6;
            let t242 = t146 + t171 - t176 - t189;
            let t243 = t242 * t108;
            let t244 = t241 * t243;
            let t245 = t240 * t244;
            let t249 = f64x8::splat(1.0) / t21 / t190;
            let t250 = t249 * t117;
            let t251 = t250 * t120;
            let t252 = t251 * t124;
            let t255 = -f64x8::splat(7.0) / f64x8::splat(288.0) * v_sigma * t222 * t39 * t98 + t236 * t245 / f64x8::splat(3072.0) - f64x8::splat(7.0) / f64x8::splat(4608.0) * t114 * t252;
            let t256 = param_beta * t255;
            let t258 = t129 * t101;
            let t259 = t133 * t133;
            let t260 = f64x8::splat(1.0) / t259;
            let t261 = t84 * t260;
            let t263 = t229 * t230 * t233;
            let t264 = t128 * t242;
            let t265 = t105 * t108;
            let t270 = t102 * t111 * t255 + t263 * t264 * t265;
            let t271 = t261 * t270;
            let t273 = t256 * t135 - t258 * t271;
            let t274 = t89 * t273;
            let t275 = f64x8::splat(1.0) / t137;
            let t276 = t274 * t275;
            let t277 = t219 * t276;
            let tvrho0 = -t32 + t57 + t140 + v_rho * (t146 + t171 - t176 - t189 + t218 + t277);
            acc_vrho = tvrho0;
            let t280 = param_alpha * t58;
            let t282 = t63 * t66 * t71;
            let t284 = t76 * t80;
            let t285 = t284 * t82;
            let t286 = t285 * t139;
            let t288 = f64x8::splat(3.0) / f64x8::splat(32.0) * t280 * t282 * t286;
            let t294 = t102 * t111 * v_sigma;
            let t297 = t92 * t39 * t95 * t211 / f64x8::splat(96.0) + t294 * t125 / f64x8::splat(1536.0);
            let t298 = param_beta * t297;
            let t300 = param_beta * param_beta;
            let t301 = t300 * t128;
            let t302 = t301 * t228;
            let t303 = t230 * t260;
            let t305 = t303 * t110 * t297;
            let t307 = t298 * t135 - t302 * t305;
            let t308 = t89 * t307;
            let t309 = t308 * t275;
            let t310 = t219 * t309;
            let tvsigma0 = v_rho * (-t288 + t310);
            acc_vsigma = tvsigma0;
        }
        store_add(zk, ip, m, acc_zk);
        store_add(vrho, ip, m, acc_vrho);
        store_add(vsigma, ip, m, acc_vsigma);
        ip += 8;
    }
}
