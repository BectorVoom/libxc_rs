//! GGA_C_SCAN_E0 vxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_scan_e0.c`
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
pub fn gga_c_scan_e0_vxc_unpol(
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
            let t62 = t59 / t60;
            let t63 = t34 * t34;
            let t64 = ((t33).select(t63, f64x8::splat(1.0)));
            let t65 = t64 * t64;
            let t66 = t65 * t64;
            let t68 = f64x8::splat(1.0) + f64x8::splat(0.025) * t10;
            let t70 = f64x8::splat(1.0) + f64x8::splat(0.04445) * t10;
            let t71 = f64x8::splat(1.0) / t70;
            let t72 = t68 * t71;
            let t73 = f64x8::splat(1.0) / t59;
            let t76 = f64x8::splat(1.0) / t66;
            let t77 = t60 * t76;
            let t79 = (simd::exp(-(-t32 + t57) * t73 * t77));
            let t80 = t79 - f64x8::splat(1.0);
            let t81 = f64x8::splat(1.0) / t80;
            let t82 = t73 * t81;
            let t83 = t82 * v_sigma;
            let t84 = t72 * t83;
            let t85 = v_rho * v_rho;
            let t87 = f64x8::splat(1.0) / t7 / t85;
            let t88 = t87 * t39;
            let t89 = f64x8::splat(1.0) / t65;
            let t91 = f64x8::splat(1.0) / t3;
            let t93 = t18 * t91 * t5;
            let t97 = f64x8::splat(1.0) + f64x8::splat(0.027439371595564633) * t84 * t88 * t89 * t93;
            let t98 = ((t97).sqrt().sqrt());
            let t100 = f64x8::splat(1.0) - f64x8::splat(1.0) / t98;
            let t103 = f64x8::splat(1.0) + f64x8::splat(1.0) * t100 * t80;
            let t104 = (simd::ln(t103));
            let t106 = t62 * t66 * t104;
            let tzk0 = -t32 + t57 + t106;
            acc_zk = tzk0;
            let t108 = f64x8::splat(1.0) / t7 / v_rho;
            let t109 = t6 * t108;
            let t111 = t4 * t109 * t30;
            let t112 = f64x8::splat(0.0011073470983333333) * t111;
            let t113 = t26 * t26;
            let t114 = f64x8::splat(1.0) / t113;
            let t115 = t12 * t114;
            let t117 = f64x8::splat(1.0) / t13 * t1;
            let t118 = t3 * t6;
            let t119 = t118 * t108;
            let t120 = t117 * t119;
            let t122 = t4 * t109;
            let t124 = ((t10).sqrt());
            let t125 = t124 * t1;
            let t126 = t125 * t119;
            let t131 = t20 * t5 / t21 / v_rho;
            let t133 = -f64x8::splat(0.632975) * t120 - f64x8::splat(0.29896666666666666) * t122 - f64x8::splat(0.1023875) * t126 - f64x8::splat(0.08215666666666667) * t131;
            let t134 = f64x8::splat(1.0) / t29;
            let t135 = t133 * t134;
            let t136 = t115 * t135;
            let t137 = f64x8::splat(1.0) * t136;
            let t138 = t43 * t1;
            let t141 = t138 * t118 * t108 * t54;
            let t142 = f64x8::splat(0.00018311447306006544) * t141;
            let t143 = t43 * t45;
            let t144 = t50 * t50;
            let t145 = f64x8::splat(1.0) / t144;
            let t150 = -f64x8::splat(0.8630833333333333) * t120 - f64x8::splat(0.301925) * t122 - f64x8::splat(0.05501625) * t126 - f64x8::splat(0.082785) * t131;
            let t152 = f64x8::splat(1.0) / t53;
            let t153 = t145 * t150 * t152;
            let t154 = t143 * t153;
            let t155 = f64x8::splat(0.5848223622634646) * t154;
            let t157 = f64x8::splat(1.0) / t98 / t97;
            let t158 = t85 * v_rho;
            let t160 = f64x8::splat(1.0) / t21 / t158;
            let t161 = t160 * t71;
            let t164 = t39 * t89;
            let t165 = t81 * v_sigma * t164;
            let t168 = t70 * t70;
            let t169 = f64x8::splat(1.0) / t168;
            let t170 = t68 * t169;
            let t171 = t170 * t82;
            let t176 = t59 * t59;
            let t177 = f64x8::splat(1.0) / t176;
            let t178 = t72 * t177;
            let t179 = t80 * t80;
            let t180 = f64x8::splat(1.0) / t179;
            let t181 = t180 * v_sigma;
            let t182 = t181 * t88;
            let t183 = t178 * t182;
            let t184 = t65 * t65;
            let t186 = f64x8::splat(1.0) / t184 / t64;
            let t187 = t186 * t18;
            let t188 = t187 * t91;
            let t189 = t112 + t137 - t142 - t155;
            let t191 = t60 * t79;
            let t192 = t5 * t189 * t191;
            let t193 = t188 * t192;
            let t197 = f64x8::splat(1.0) / t7 / t158;
            let t198 = t197 * t39;
            let t203 = -f64x8::splat(0.002743937159556463) * t161 * t73 * t165 + f64x8::splat(0.004878720269691391) * t171 * v_sigma * t160 * t164 + f64x8::splat(0.027439371595564633) * t183 * t193 - f64x8::splat(0.0640252003896508) * t84 * t198 * t89 * t93;
            let t204 = t157 * t203;
            let t209 = t77 * t79;
            let t212 = f64x8::splat(0.25) * t204 * t80 - f64x8::splat(1.0) * t100 * t189 * t73 * t209;
            let t214 = f64x8::splat(1.0) / t103;
            let t216 = t62 * t66 * t212 * t214;
            let tvrho0 = -t32 + t57 + t106 + v_rho * (t112 + t137 - t142 - t155 + t216);
            acc_vrho = tvrho0;
            let t219 = t108 * t64;
            let t220 = t157 * t68;
            let t222 = t219 * t220 * t71;
            let t223 = t39 * t18;
            let t224 = t91 * t5;
            let t225 = t224 * t214;
            let t226 = t223 * t225;
            let tvsigma0 = f64x8::splat(0.0006950474021161377) * t222 * t226;
            acc_vsigma = tvsigma0;
        }
        store_add(zk, ip, m, acc_zk);
        store_add(vrho, ip, m, acc_vrho);
        store_add(vsigma, ip, m, acc_vsigma);
        ip += 8;
    }
}
