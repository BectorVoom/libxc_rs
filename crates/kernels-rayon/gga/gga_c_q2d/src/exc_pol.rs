//! GGA_C_Q2D exc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_q2d.c`
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

/// Accumulate 8 elements with a given stride and offset.
///
/// `+=`, not `=`: the scalar kernel this was translated from writes
/// `out[ip * stride + offset] += v`, and a plain store is not the same
/// operation. It differs on the sign of zero -- `0.0 + -0.0` is `+0.0`
/// while a store of `-0.0` keeps the sign -- which is a bit difference
/// the fingerprint gate sees, and it would silently drop a caller's
/// existing contribution if one were ever there.
///
/// The read is not free on this path: a polarized `kxc`/`lxc` kernel
/// writes many strided outputs per point, and `lda_c_pw_erf kxc pol`
/// measured 84 -> 114 ns/pt (1.36x). It is charged anyway, because the
/// scalar kernel this is compared against does the same read. Gathering
/// into a vector, adding once and scattering back was tried and is no
/// faster (117 ns/pt), so the cost is the load itself, not scheduling.
#[inline(always)]
fn store_strided(s: &mut [f64], ip: usize, m: usize, stride: usize, offset: usize, acc: f64x8) {
    let a: [f64; 8] = acc.into();
    if m == 8 {
        let base = ip * stride + offset;
        s[base] += a[0];
        s[base + stride] += a[1];
        s[base + 2 * stride] += a[2];
        s[base + 3 * stride] += a[3];
        s[base + 4 * stride] += a[4];
        s[base + 5 * stride] += a[5];
        s[base + 6 * stride] += a[6];
        s[base + 7 * stride] += a[7];
    } else {
        for k in 0..m {
            s[(ip + k) * stride + offset] += a[k];
        }
    }
}

#[allow(unused_variables, non_snake_case)]
pub fn gga_c_q2d_exc_pol(
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
            let t2 = v_sigma0 + f64x8::splat(2.0) * v_sigma1 + v_sigma2;
            let t3 = t2 * t2;
            let t4 = v_rho0 + v_rho1;
            let t5 = t4 * t4;
            let t6 = t5 * t5;
            let t7 = (simd::cbrt(t4));
            let t8 = t7 * t7;
            let t10 = f64x8::splat(1.0) / t8 / t6;
            let t11 = t3 * t10;
            let t12 = f64x8::splat(M_CBRT2);
            let t13 = t12 * t12;
            let t14 = v_rho0 - v_rho1;
            let t15 = f64x8::splat(1.0) / t4;
            let t16 = t14 * t15;
            let t17 = f64x8::splat(1.0) + t16;
            let t18 = (t17).simd_le(zeta_threshold);
            let t19 = (simd::cbrt(zeta_threshold));
            let t20 = t19 * t19;
            let t21 = (simd::cbrt(t17));
            let t22 = t21 * t21;
            let t23 = ((t18).select(t20, t22));
            let t24 = f64x8::splat(1.0) - t16;
            let t25 = (t24).simd_le(zeta_threshold);
            let t26 = (simd::cbrt(t24));
            let t27 = t26 * t26;
            let t28 = ((t25).select(t20, t27));
            let t30 = t23 / f64x8::splat(2.0) + t28 / f64x8::splat(2.0);
            let t31 = t30 * t30;
            let t32 = t31 * t31;
            let t33 = f64x8::splat(1.0) / t32;
            let t34 = t13 * t33;
            let t35 = t11 * t34;
            let t36 = f64x8::splat(M_CBRT3);
            let t37 = f64x8::splat(1.0) / f64x8::splat(M_PI);
            let t38 = (simd::cbrt(t37));
            let t39 = t38 * t38;
            let t40 = f64x8::splat(1.0) / t39;
            let t41 = t36 * t40;
            let t42 = f64x8::splat(M_CBRT4);
            let t43 = t42 * t42;
            let t45 = f64x8::splat(1.0) / t7 / t5;
            let t46 = t2 * t45;
            let t48 = f64x8::splat(1.0) / t31;
            let t49 = t36 * t36;
            let t51 = f64x8::splat(1.0) / t38;
            let t52 = t51 * t42;
            let t53 = t48 * t49 * t52;
            let t55 = t46 * t12 * t53 / f64x8::splat(96.0);
            let t56 = f64x8::splat(1.0) + t55;
            let t57 = t43 * t56;
            let t58 = t3 * t2;
            let t59 = t5 * t4;
            let t60 = t6 * t59;
            let t61 = f64x8::splat(1.0) / t60;
            let t62 = t58 * t61;
            let t63 = t32 * t31;
            let t64 = f64x8::splat(1.0) / t63;
            let t65 = t64 * f64x8::splat(M_PI);
            let t68 = f64x8::splat(1000000.0) + t62 * t65 / f64x8::splat(12288.0);
            let t69 = f64x8::splat(1.0) / t68;
            let t70 = t57 * t69;
            let t71 = t41 * t70;
            let t74 = f64x8::splat(1.0) - t35 * t71 / f64x8::splat(3072.0);
            let t75 = t36 * t38;
            let t76 = f64x8::splat(1.0) / t7;
            let t78 = t75 * t43 * t76;
            let t80 = f64x8::splat(1.0) + f64x8::splat(0.053425) * t78;
            let t81 = ((t78).sqrt());
            let t84 = ((t78) * (t78).sqrt());
            let t86 = t49 * t39;
            let t87 = f64x8::splat(1.0) / t8;
            let t89 = t86 * t42 * t87;
            let t91 = f64x8::splat(3.79785) * t81 + f64x8::splat(0.8969) * t78 + f64x8::splat(0.204775) * t84 + f64x8::splat(0.123235) * t89;
            let t94 = f64x8::splat(1.0) + f64x8::splat(16.081979498692537) / t91;
            let t95 = (simd::ln(t94));
            let t97 = f64x8::splat(0.0621814) * t80 * t95;
            let t98 = t14 * t14;
            let t99 = t98 * t98;
            let t100 = f64x8::splat(1.0) / t6;
            let t101 = t99 * t100;
            let t102 = t19 * zeta_threshold;
            let t103 = t21 * t17;
            let t104 = ((t18).select(t102, t103));
            let t105 = t26 * t24;
            let t106 = ((t25).select(t102, t105));
            let t107 = t104 + t106 - f64x8::splat(2.0);
            let t110 = f64x8::splat(1.0) / (f64x8::splat(2.0) * t12 - f64x8::splat(2.0));
            let t111 = t107 * t110;
            let t113 = f64x8::splat(1.0) + f64x8::splat(0.05137) * t78;
            let t118 = f64x8::splat(7.05945) * t81 + f64x8::splat(1.549425) * t78 + f64x8::splat(0.420775) * t84 + f64x8::splat(0.1562925) * t89;
            let t121 = f64x8::splat(1.0) + f64x8::splat(32.16395899738507) / t118;
            let t122 = (simd::ln(t121));
            let t126 = f64x8::splat(1.0) + f64x8::splat(0.0278125) * t78;
            let t131 = f64x8::splat(5.1785) * t81 + f64x8::splat(0.905775) * t78 + f64x8::splat(0.1100325) * t84 + f64x8::splat(0.1241775) * t89;
            let t134 = f64x8::splat(1.0) + f64x8::splat(29.608749977793437) / t131;
            let t135 = (simd::ln(t134));
            let t136 = t126 * t135;
            let t138 = -f64x8::splat(0.0310907) * t113 * t122 + t97 - f64x8::splat(0.0197516734986138) * t136;
            let t139 = t111 * t138;
            let t140 = t101 * t139;
            let t142 = f64x8::splat(0.0197516734986138) * t111 * t136;
            let t143 = (simd::ln(f64x8::splat(2.0)));
            let t144 = f64x8::splat(1.0) - t143;
            let t145 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t146 = f64x8::splat(1.0) / t145;
            let t147 = t144 * t146;
            let t148 = t31 * t30;
            let t149 = f64x8::splat(1.0) / t144;
            let t151 = (-t97 + t140 + t142) * t149;
            let t152 = f64x8::splat(1.0) / t148;
            let t153 = t145 * t152;
            let t155 = (simd::exp(-t151 * t153));
            let t156 = t155 - f64x8::splat(1.0);
            let t157 = f64x8::splat(1.0) / t156;
            let t158 = t149 * t157;
            let t160 = t41 * t43;
            let t161 = t34 * t160;
            let t164 = t55 + f64x8::splat(0.0002143700905903487) * t158 * t11 * t161;
            let t165 = t164 * t149;
            let t168 = f64x8::splat(1.0) + f64x8::splat(0.6585449182935511) * t158 * t164;
            let t169 = f64x8::splat(1.0) / t168;
            let t172 = f64x8::splat(1.0) + f64x8::splat(0.6585449182935511) * t165 * t169;
            let t173 = (simd::ln(t172));
            let t176 = t147 * t148 * t173 + t140 + t142 - t97;
            let t177 = t74 * t176;
            let t178 = t34 * t36;
            let t179 = t11 * t178;
            let t180 = t40 * t43;
            let t181 = t56 * t69;
            let t182 = ((f64x8::splat(3.0)).sqrt());
            let t184 = f64x8::splat(M_CBRT6);
            let t185 = t184 * t184;
            let t186 = (simd::cbrt(t145));
            let t187 = f64x8::splat(1.0) / t186;
            let t188 = t185 * t187;
            let t189 = ((t2).sqrt());
            let t191 = f64x8::splat(1.0) / t7 / t4;
            let t193 = t188 * t189 * t191;
            let t194 = ((t193).sqrt());
            let t195 = t182 * t76 * t194;
            let t197 = f64x8::splat(1.0) / t5;
            let t198 = t197 * t185;
            let t199 = t187 * t189;
            let t200 = t198 * t199;
            let t202 = t182 * t15;
            let t203 = t194 * t193;
            let t204 = t202 * t203;
            let t206 = f64x8::splat(0.0245130624) * t195 + f64x8::splat(0.0138498611712) * t200 + f64x8::splat(0.0002310999830832) * t204;
            let t208 = ((t195) * (t195).sqrt());
            let t212 = f64x8::splat(0.2846248) * t195 - f64x8::splat(0.0031313960595450714) * t208 + f64x8::splat(0.08226186096) * t200 + f64x8::splat(0.00120051939264) * t204;
            let t214 = f64x8::splat(1.0) + f64x8::splat(1.0) / t212;
            let t215 = (simd::ln(t214));
            let t220 = -f64x8::splat(0.00963896) * t195 - f64x8::splat(0.0018553259352) * t200 - f64x8::splat(6.288223471953773e-06) * t204;
            let t224 = f64x8::splat(0.1173772) * t195 + f64x8::splat(0.0161747623056) * t200 + f64x8::splat(5.35938794688e-05) * t204;
            let t226 = f64x8::splat(1.0) + f64x8::splat(1.0) / t224;
            let t227 = (simd::ln(t226));
            let t229 = f64x8::splat(0.117331) + t220 * t227;
            let t230 = t229 * t98;
            let t235 = -f64x8::splat(0.010534412) * t195 + f64x8::splat(0.0039590320224) * t200 - f64x8::splat(0.0018717920348611111) * t204;
            let t238 = f64x8::splat(0.404501484) * t195 + f64x8::splat(0.079926897828288) * t204;
            let t240 = f64x8::splat(1.0) + f64x8::splat(1.0) / t238;
            let t241 = (simd::ln(t240));
            let t243 = f64x8::splat(0.0234188) + t235 * t241;
            let t244 = t243 * t99;
            let t247 = (simd::exp(-f64x8::splat(0.3801624) * t195));
            let t249 = f64x8::splat(M_SQRT2);
            let t250 = (t247 - f64x8::splat(1.0)) * t249;
            let t251 = t250 * t182;
            let t252 = f64x8::splat(1.0) / t194;
            let t253 = t7 * t252;
            let t254 = ((zeta_threshold).sqrt());
            let t255 = t254 * zeta_threshold;
            let t256 = ((t17).sqrt());
            let t257 = t256 * t17;
            let t258 = ((t18).select(t255, t257));
            let t260 = ((t24).sqrt());
            let t261 = t260 * t24;
            let t262 = ((t25).select(t255, t261));
            let t267 = t258 / f64x8::splat(2.0) + t262 / f64x8::splat(2.0) - f64x8::splat(1.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t98 * t197 - f64x8::splat(3.0) / f64x8::splat(128.0) * t101;
            let t271 = -f64x8::splat(0.1925) + t206 * t215 + t230 * t197 + t244 * t100 - f64x8::splat(0.4981375370638352) * t251 * t253 * t267;
            let t273 = t180 * t181 * t271;
            let t275 = t179 * t273 / f64x8::splat(3072.0);
            let tzk0 = t177 + t275;
            acc_zk = tzk0;
        }
        store_add(zk, ip, m, acc_zk);
        ip += 8;
    }
}
