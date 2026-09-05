//! GGA_C_ZPBEINT vxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_zpbeint.c`
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
pub fn gga_c_zpbeint_vxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    param_alpha: f64,
    param_beta: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_alpha = f64x8::splat(param_alpha);
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
            let t58 = t34 * t34;
            let t59 = ((t33).select(t58, f64x8::splat(1.0)));
            let t60 = ((v_sigma).sqrt());
            let t61 = t60 * v_sigma;
            let t62 = param_alpha * t61;
            let t63 = v_rho * v_rho;
            let t64 = t63 * t63;
            let t65 = f64x8::splat(1.0) / t64;
            let t66 = t59 * t59;
            let t67 = t66 * t59;
            let t68 = f64x8::splat(1.0) / t67;
            let t71 = f64x8::splat(1.0) / t13 / t10;
            let t75 = (simd::pow(t59, t62 * t65 * t68 * t71 / f64x8::splat(16.0)));
            let t76 = (simd::ln(f64x8::splat(2.0)));
            let t77 = f64x8::splat(1.0) - t76;
            let t78 = t75 * t77;
            let t79 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t80 = f64x8::splat(1.0) / t79;
            let t81 = t80 * t67;
            let t83 = f64x8::splat(1.0) / t7 / t63;
            let t86 = f64x8::splat(1.0) / t66;
            let t88 = f64x8::splat(1.0) / t3;
            let t90 = t86 * t18 * t88 * t5;
            let t93 = f64x8::splat(1.0) / t77;
            let t94 = param_beta * t93;
            let t99 = (simd::exp(-(-t32 + t57) * t93 * t79 * t68));
            let t100 = t99 - f64x8::splat(1.0);
            let t101 = f64x8::splat(1.0) / t100;
            let t102 = t79 * t101;
            let t103 = v_sigma * v_sigma;
            let t105 = t94 * t102 * t103;
            let t107 = f64x8::splat(1.0) / t21 / t64;
            let t108 = t39 * t39;
            let t109 = t107 * t108;
            let t110 = t66 * t66;
            let t111 = f64x8::splat(1.0) / t110;
            let t112 = t109 * t111;
            let t113 = f64x8::splat(1.0) / t19;
            let t114 = t1 * t113;
            let t115 = t114 * t6;
            let t116 = t112 * t115;
            let t119 = v_sigma * t83 * t39 * t90 / f64x8::splat(96.0) + t105 * t116 / f64x8::splat(3072.0);
            let t120 = param_beta * t119;
            let t124 = t94 * t102 * t119 + f64x8::splat(1.0);
            let t125 = f64x8::splat(1.0) / t124;
            let t126 = t93 * t79 * t125;
            let t128 = t120 * t126 + f64x8::splat(1.0);
            let t129 = (simd::ln(t128));
            let t131 = t78 * t81 * t129;
            let tzk0 = -t32 + t57 + t131;
            acc_zk = tzk0;
            let t133 = f64x8::splat(1.0) / t7 / v_rho;
            let t134 = t6 * t133;
            let t136 = t4 * t134 * t30;
            let t137 = f64x8::splat(0.0011073470983333333) * t136;
            let t138 = t26 * t26;
            let t139 = f64x8::splat(1.0) / t138;
            let t140 = t12 * t139;
            let t142 = f64x8::splat(1.0) / t13 * t1;
            let t143 = t3 * t6;
            let t144 = t143 * t133;
            let t145 = t142 * t144;
            let t147 = t4 * t134;
            let t149 = ((t10).sqrt());
            let t150 = t149 * t1;
            let t151 = t150 * t144;
            let t155 = t5 / t21 / v_rho;
            let t156 = t20 * t155;
            let t158 = -f64x8::splat(0.632975) * t145 - f64x8::splat(0.29896666666666666) * t147 - f64x8::splat(0.1023875) * t151 - f64x8::splat(0.08215666666666667) * t156;
            let t159 = f64x8::splat(1.0) / t29;
            let t160 = t158 * t159;
            let t161 = t140 * t160;
            let t162 = f64x8::splat(1.0) * t161;
            let t163 = t43 * t1;
            let t166 = t163 * t143 * t133 * t54;
            let t167 = f64x8::splat(0.00018311447306006544) * t166;
            let t168 = t43 * t45;
            let t169 = t50 * t50;
            let t170 = f64x8::splat(1.0) / t169;
            let t175 = -f64x8::splat(0.8630833333333333) * t145 - f64x8::splat(0.301925) * t147 - f64x8::splat(0.05501625) * t151 - f64x8::splat(0.082785) * t156;
            let t177 = f64x8::splat(1.0) / t53;
            let t178 = t170 * t175 * t177;
            let t179 = t168 * t178;
            let t180 = f64x8::splat(0.5848223622634646) * t179;
            let t181 = t64 * v_rho;
            let t182 = f64x8::splat(1.0) / t181;
            let t188 = f64x8::splat(1.0) / t7 / t181;
            let t193 = f64x8::splat(1.0) / t13 / t24 / f64x8::splat(4.0);
            let t195 = t193 * t1 * t143;
            let t198 = -t62 * t182 * t68 * t71 / f64x8::splat(4.0) + t62 * t188 * t68 * t195 / f64x8::splat(32.0);
            let t199 = t75 * t198;
            let t200 = (simd::ln(t59));
            let t202 = t77 * t80;
            let t204 = t202 * t67 * t129;
            let t205 = t199 * t200 * t204;
            let t206 = t78 * t80;
            let t207 = t63 * v_rho;
            let t209 = f64x8::splat(1.0) / t7 / t207;
            let t214 = t77 * t77;
            let t215 = f64x8::splat(1.0) / t214;
            let t216 = param_beta * t215;
            let t217 = t79 * t79;
            let t218 = t216 * t217;
            let t219 = t100 * t100;
            let t220 = f64x8::splat(1.0) / t219;
            let t221 = t220 * t103;
            let t222 = t221 * t107;
            let t223 = t218 * t222;
            let t225 = f64x8::splat(1.0) / t110 / t67;
            let t226 = t108 * t225;
            let t227 = t226 * t1;
            let t228 = t113 * t6;
            let t229 = t137 + t162 - t167 - t180;
            let t230 = t229 * t99;
            let t231 = t228 * t230;
            let t232 = t227 * t231;
            let t236 = f64x8::splat(1.0) / t21 / t181;
            let t237 = t236 * t108;
            let t238 = t237 * t111;
            let t239 = t238 * t115;
            let t242 = -f64x8::splat(7.0) / f64x8::splat(288.0) * v_sigma * t209 * t39 * t90 + t223 * t232 / f64x8::splat(3072.0) - f64x8::splat(7.0) / f64x8::splat(4608.0) * t105 * t239;
            let t243 = param_beta * t242;
            let t245 = t120 * t93;
            let t246 = t124 * t124;
            let t247 = f64x8::splat(1.0) / t246;
            let t248 = t79 * t247;
            let t250 = t216 * t217 * t220;
            let t251 = t119 * t229;
            let t252 = t68 * t99;
            let t257 = t94 * t102 * t242 + t250 * t251 * t252;
            let t258 = t248 * t257;
            let t260 = t243 * t126 - t245 * t258;
            let t262 = f64x8::splat(1.0) / t128;
            let t264 = t206 * t67 * t260 * t262;
            let tvrho0 = -t32 + t57 + t131 + v_rho * (t137 + t162 - t167 - t180 + t205 + t264);
            acc_vrho = tvrho0;
            let t267 = t75 * param_alpha;
            let t268 = t60 * t65;
            let t271 = t202 * t129;
            let t272 = t71 * t200 * t271;
            let t274 = f64x8::splat(3.0) / f64x8::splat(32.0) * t267 * t268 * t272;
            let t278 = t18 * t88 * t5;
            let t282 = t94 * t102 * v_sigma;
            let t285 = t83 * t39 * t86 * t278 / f64x8::splat(96.0) + t282 * t116 / f64x8::splat(1536.0);
            let t286 = param_beta * t285;
            let t288 = param_beta * param_beta;
            let t289 = t288 * t119;
            let t290 = t289 * t215;
            let t291 = t217 * t247;
            let t293 = t291 * t101 * t285;
            let t295 = t286 * t126 - t290 * t293;
            let t296 = t67 * t295;
            let t298 = t206 * t296 * t262;
            let tvsigma0 = v_rho * (t274 + t298);
            acc_vsigma = tvsigma0;
        }
        store_add(zk, ip, m, acc_zk);
        store_add(vrho, ip, m, acc_vrho);
        store_add(vsigma, ip, m, acc_vsigma);
        ip += 8;
    }
}
