//! GGA_C_AM05 vxc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_am05.c`
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
pub fn gga_c_am05_vxc_pol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    param_alpha: f64,
    param_gamma: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_alpha = f64x8::splat(param_alpha);
    let param_gamma = f64x8::splat(param_gamma);
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
        let mut acc_vrho_0 = V_ZERO;
        let mut acc_vrho_1 = V_ZERO;
        let mut acc_vsigma_0 = V_ZERO;
        let mut acc_vsigma_1 = V_ZERO;
        let mut acc_vsigma_2 = V_ZERO;
        {
            let t1 = f64x8::splat(M_CBRT3);
            let t2 = f64x8::splat(1.0) / f64x8::splat(M_PI);
            let t3 = (simd::cbrt(t2));
            let t4 = t1 * t3;
            let t5 = f64x8::splat(M_CBRT4);
            let t6 = t5 * t5;
            let t7 = v_rho0 + v_rho1;
            let t8 = (simd::cbrt(t7));
            let t11 = t4 * t6 / t8;
            let t13 = f64x8::splat(1.0) + f64x8::splat(0.053425) * t11;
            let t14 = ((t11).sqrt());
            let t17 = ((t11) * (t11).sqrt());
            let t19 = t1 * t1;
            let t20 = t3 * t3;
            let t21 = t19 * t20;
            let t22 = t8 * t8;
            let t25 = t21 * t5 / t22;
            let t27 = f64x8::splat(3.79785) * t14 + f64x8::splat(0.8969) * t11 + f64x8::splat(0.204775) * t17 + f64x8::splat(0.123235) * t25;
            let t30 = f64x8::splat(1.0) + f64x8::splat(16.081979498692537) / t27;
            let t31 = (simd::ln(t30));
            let t33 = f64x8::splat(0.0621814) * t13 * t31;
            let t34 = v_rho0 - v_rho1;
            let t35 = t34 * t34;
            let t36 = t35 * t35;
            let t37 = t7 * t7;
            let t38 = t37 * t37;
            let t39 = f64x8::splat(1.0) / t38;
            let t40 = t36 * t39;
            let t41 = f64x8::splat(1.0) / t7;
            let t42 = t34 * t41;
            let t43 = f64x8::splat(1.0) + t42;
            let t44 = (t43).simd_le(zeta_threshold);
            let t45 = (simd::cbrt(zeta_threshold));
            let t46 = t45 * zeta_threshold;
            let t47 = (simd::cbrt(t43));
            let t49 = ((t44).select(t46, t47 * t43));
            let t50 = f64x8::splat(1.0) - t42;
            let t51 = (t50).simd_le(zeta_threshold);
            let t52 = (simd::cbrt(t50));
            let t54 = ((t51).select(t46, t52 * t50));
            let t55 = t49 + t54 - f64x8::splat(2.0);
            let t56 = f64x8::splat(M_CBRT2);
            let t59 = f64x8::splat(1.0) / (f64x8::splat(2.0) * t56 - f64x8::splat(2.0));
            let t60 = t55 * t59;
            let t62 = f64x8::splat(1.0) + f64x8::splat(0.05137) * t11;
            let t67 = f64x8::splat(7.05945) * t14 + f64x8::splat(1.549425) * t11 + f64x8::splat(0.420775) * t17 + f64x8::splat(0.1562925) * t25;
            let t70 = f64x8::splat(1.0) + f64x8::splat(32.16395899738507) / t67;
            let t71 = (simd::ln(t70));
            let t75 = f64x8::splat(1.0) + f64x8::splat(0.0278125) * t11;
            let t80 = f64x8::splat(5.1785) * t14 + f64x8::splat(0.905775) * t11 + f64x8::splat(0.1100325) * t17 + f64x8::splat(0.1241775) * t25;
            let t83 = f64x8::splat(1.0) + f64x8::splat(29.608749977793437) / t80;
            let t84 = (simd::ln(t83));
            let t85 = t75 * t84;
            let t87 = -f64x8::splat(0.0310907) * t62 * t71 + t33 - f64x8::splat(0.0197516734986138) * t85;
            let t88 = t60 * t87;
            let t92 = -t33 + t40 * t88 + f64x8::splat(0.0197516734986138) * t60 * t85;
            let t93 = ((t44).select(zeta_threshold, t43));
            let t94 = f64x8::splat(M_CBRT6);
            let t95 = param_alpha * t94;
            let t96 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t97 = (simd::cbrt(t96));
            let t98 = t97 * t97;
            let t99 = f64x8::splat(1.0) / t98;
            let t100 = t99 * v_sigma0;
            let t101 = v_rho0 * v_rho0;
            let t102 = (simd::cbrt(v_rho0));
            let t103 = t102 * t102;
            let t105 = f64x8::splat(1.0) / t103 / t101;
            let t109 = f64x8::splat(1.0) + t95 * t100 * t105 / f64x8::splat(24.0);
            let t110 = f64x8::splat(1.0) / t109;
            let t113 = t110 + param_gamma * (f64x8::splat(1.0) - t110);
            let t115 = ((t51).select(zeta_threshold, t50));
            let t116 = t99 * v_sigma2;
            let t117 = v_rho1 * v_rho1;
            let t118 = (simd::cbrt(v_rho1));
            let t119 = t118 * t118;
            let t121 = f64x8::splat(1.0) / t119 / t117;
            let t125 = f64x8::splat(1.0) + t95 * t116 * t121 / f64x8::splat(24.0);
            let t126 = f64x8::splat(1.0) / t125;
            let t129 = t126 + param_gamma * (f64x8::splat(1.0) - t126);
            let t132 = t93 * t113 / f64x8::splat(2.0) + t115 * t129 / f64x8::splat(2.0);
            let tzk0 = t92 * t132;
            acc_zk = tzk0;
            let t134 = f64x8::splat(1.0) / t8 / t7;
            let t135 = t6 * t134;
            let t138 = f64x8::splat(0.0011073470983333333) * t4 * t135 * t31;
            let t139 = t27 * t27;
            let t140 = f64x8::splat(1.0) / t139;
            let t141 = t13 * t140;
            let t143 = f64x8::splat(1.0) / t14 * t1;
            let t144 = t3 * t6;
            let t145 = t144 * t134;
            let t146 = t143 * t145;
            let t148 = t4 * t135;
            let t150 = ((t11).sqrt());
            let t151 = t150 * t1;
            let t152 = t151 * t145;
            let t157 = t21 * t5 / t22 / t7;
            let t159 = -f64x8::splat(0.632975) * t146 - f64x8::splat(0.29896666666666666) * t148 - f64x8::splat(0.1023875) * t152 - f64x8::splat(0.08215666666666667) * t157;
            let t160 = f64x8::splat(1.0) / t30;
            let t161 = t159 * t160;
            let t163 = f64x8::splat(1.0) * t141 * t161;
            let t164 = t35 * t34;
            let t165 = t164 * t39;
            let t167 = f64x8::splat(4.0) * t165 * t88;
            let t168 = t38 * t7;
            let t169 = f64x8::splat(1.0) / t168;
            let t170 = t36 * t169;
            let t172 = f64x8::splat(4.0) * t170 * t88;
            let t173 = f64x8::splat(1.0) / t37;
            let t174 = t34 * t173;
            let t175 = t41 - t174;
            let t178 = ((t44).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t47 * t175));
            let t179 = -t175;
            let t182 = ((t51).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t52 * t179));
            let t184 = (t178 + t182) * t59;
            let t185 = t184 * t87;
            let t190 = t67 * t67;
            let t191 = f64x8::splat(1.0) / t190;
            let t192 = t62 * t191;
            let t197 = -f64x8::splat(1.176575) * t146 - f64x8::splat(0.516475) * t148 - f64x8::splat(0.2103875) * t152 - f64x8::splat(0.104195) * t157;
            let t198 = f64x8::splat(1.0) / t70;
            let t199 = t197 * t198;
            let t205 = t80 * t80;
            let t206 = f64x8::splat(1.0) / t205;
            let t207 = t75 * t206;
            let t212 = -f64x8::splat(0.8630833333333333) * t146 - f64x8::splat(0.301925) * t148 - f64x8::splat(0.05501625) * t152 - f64x8::splat(0.082785) * t157;
            let t213 = f64x8::splat(1.0) / t83;
            let t214 = t212 * t213;
            let t217 = f64x8::splat(0.0005323764196666666) * t4 * t135 * t71 + f64x8::splat(1.0) * t192 * t199 - t138 - t163 + f64x8::splat(0.00018311447306006544) * t4 * t135 * t84 + f64x8::splat(0.5848223622634646) * t207 * t214;
            let t218 = t60 * t217;
            let t219 = t40 * t218;
            let t222 = t60 * t1;
            let t224 = t144 * t134 * t84;
            let t226 = f64x8::splat(0.00018311447306006544) * t222 * t224;
            let t227 = t60 * t75;
            let t229 = t206 * t212 * t213;
            let t231 = f64x8::splat(0.5848223622634646) * t227 * t229;
            let t232 = t138 + t163 + t167 - t172 + t40 * t185 + t219 + f64x8::splat(0.0197516734986138) * t184 * t85 - t226 - t231;
            let t233 = t7 * t232;
            let t235 = t7 * t92;
            let t236 = ((t44).select(f64x8::splat(0.0), t175));
            let t238 = t109 * t109;
            let t239 = f64x8::splat(1.0) / t238;
            let t240 = t239 * param_alpha;
            let t241 = t240 * t94;
            let t242 = t101 * v_rho0;
            let t244 = f64x8::splat(1.0) / t103 / t242;
            let t248 = param_gamma * t239 * param_alpha;
            let t249 = t94 * t99;
            let t254 = -t248 * t249 * v_sigma0 * t244 / f64x8::splat(9.0) + t241 * t100 * t244 / f64x8::splat(9.0);
            let t256 = ((t51).select(f64x8::splat(0.0), t179));
            let t259 = t236 * t113 / f64x8::splat(2.0) + t256 * t129 / f64x8::splat(2.0) + t93 * t254 / f64x8::splat(2.0);
            let tvrho0 = t132 * t233 + t235 * t259 + tzk0;
            acc_vrho_0 = tvrho0;
            let t261 = -t41 - t174;
            let t264 = ((t44).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t47 * t261));
            let t265 = -t261;
            let t268 = ((t51).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t52 * t265));
            let t270 = (t264 + t268) * t59;
            let t271 = t270 * t87;
            let t275 = t138 + t163 - t167 - t172 + t40 * t271 + t219 + f64x8::splat(0.0197516734986138) * t270 * t85 - t226 - t231;
            let t276 = t7 * t275;
            let t278 = ((t44).select(f64x8::splat(0.0), t261));
            let t280 = ((t51).select(f64x8::splat(0.0), t265));
            let t282 = t125 * t125;
            let t283 = f64x8::splat(1.0) / t282;
            let t284 = t283 * param_alpha;
            let t285 = t284 * t94;
            let t286 = t117 * v_rho1;
            let t288 = f64x8::splat(1.0) / t119 / t286;
            let t292 = param_gamma * t283 * param_alpha;
            let t297 = -t292 * t249 * v_sigma2 * t288 / f64x8::splat(9.0) + t285 * t116 * t288 / f64x8::splat(9.0);
            let t300 = t278 * t113 / f64x8::splat(2.0) + t115 * t297 / f64x8::splat(2.0) + t280 * t129 / f64x8::splat(2.0);
            let tvrho1 = t132 * t276 + t235 * t300 + tzk0;
            acc_vrho_1 = tvrho1;
            let t302 = t249 * t105;
            let t306 = -t240 * t302 / f64x8::splat(24.0) + t248 * t302 / f64x8::splat(24.0);
            let t307 = t93 * t306;
            let tvsigma0 = t235 * t307 / f64x8::splat(2.0);
            acc_vsigma_0 = tvsigma0;
            let tvsigma1 = f64x8::splat(0.0);
            acc_vsigma_1 = tvsigma1;
            let t309 = t249 * t121;
            let t313 = -t284 * t309 / f64x8::splat(24.0) + t292 * t309 / f64x8::splat(24.0);
            let t314 = t115 * t313;
            let tvsigma2 = t235 * t314 / f64x8::splat(2.0);
            acc_vsigma_2 = tvsigma2;
        }
        store_add(zk, ip, m, acc_zk);
        store_strided(vrho, ip, m, 2, 0, acc_vrho_0);
        store_strided(vrho, ip, m, 2, 1, acc_vrho_1);
        store_strided(vsigma, ip, m, 3, 0, acc_vsigma_0);
        store_strided(vsigma, ip, m, 3, 1, acc_vsigma_1);
        store_strided(vsigma, ip, m, 3, 2, acc_vsigma_2);
        ip += 8;
    }
}
