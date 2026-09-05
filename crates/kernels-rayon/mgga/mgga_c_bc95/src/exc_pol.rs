//! MGGA_C_BC95 exc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_c_bc95.c`
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
pub fn mgga_c_bc95_exc_pol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    param_copp: f64,
    param_css: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_copp = f64x8::splat(param_copp);
    let param_css = f64x8::splat(param_css);
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
        let v_lapl0 = load_strided(lapl, ip, np, 2, 0);
        let v_lapl1 = load_strided(lapl, ip, np, 2, 1);
        let v_tau0 = load_strided(tau, ip, np, 2, 0);
        let v_tau1 = load_strided(tau, ip, np, 2, 1);
        let mut acc_zk = V_ZERO;
        {
            let t3 = v_rho0 - v_rho1;
            let t4 = v_rho0 + v_rho1;
            let t5 = f64x8::splat(1.0) / t4;
            let t6 = t3 * t5;
            let t7 = f64x8::splat(1.0) + t6;
            let t8 = (t7).simd_le(zeta_threshold);
            let t9 = ((v_rho0).simd_le(dens_threshold)) | (t8);
            let t10 = ((t8).select(zeta_threshold, t7));
            let t11 = f64x8::splat(M_CBRT3);
            let t12 = f64x8::splat(1.0) / f64x8::splat(M_PI);
            let t13 = (simd::cbrt(t12));
            let t14 = t11 * t13;
            let t15 = f64x8::splat(M_CBRT4);
            let t16 = t15 * t15;
            let t17 = t14 * t16;
            let t18 = (simd::cbrt(t4));
            let t19 = f64x8::splat(1.0) / t18;
            let t20 = f64x8::splat(M_CBRT2);
            let t21 = t19 * t20;
            let t22 = (simd::cbrt(zeta_threshold));
            let t23 = f64x8::splat(1.0) / t22;
            let t24 = (simd::cbrt(t7));
            let t26 = ((t8).select(t23, f64x8::splat(1.0) / t24));
            let t28 = t17 * t21 * t26;
            let t30 = f64x8::splat(1.0) + f64x8::splat(0.053425) * t28;
            let t31 = ((t28).sqrt());
            let t34 = ((t28) * (t28).sqrt());
            let t36 = t11 * t11;
            let t37 = t13 * t13;
            let t38 = t36 * t37;
            let t39 = t38 * t15;
            let t40 = t18 * t18;
            let t41 = f64x8::splat(1.0) / t40;
            let t42 = t20 * t20;
            let t43 = t41 * t42;
            let t44 = t26 * t26;
            let t46 = t39 * t43 * t44;
            let t48 = f64x8::splat(3.79785) * t31 + f64x8::splat(0.8969) * t28 + f64x8::splat(0.204775) * t34 + f64x8::splat(0.123235) * t46;
            let t51 = f64x8::splat(1.0) + f64x8::splat(16.081979498692537) / t48;
            let t52 = (simd::ln(t51));
            let t54 = f64x8::splat(0.0621814) * t30 * t52;
            let t56 = t22 * zeta_threshold;
            let t58 = (((f64x8::splat(2.0)).simd_le(zeta_threshold)).select(t56, f64x8::splat(2.0) * t20));
            let t60 = (((f64x8::splat(0.0)).simd_le(zeta_threshold)).select(t56, f64x8::splat(0.0)));
            let t64 = f64x8::splat(1.0) / (f64x8::splat(2.0) * t20 - f64x8::splat(2.0));
            let t65 = (t58 + t60 - f64x8::splat(2.0)) * t64;
            let t67 = f64x8::splat(1.0) + f64x8::splat(0.05137) * t28;
            let t72 = f64x8::splat(7.05945) * t31 + f64x8::splat(1.549425) * t28 + f64x8::splat(0.420775) * t34 + f64x8::splat(0.1562925) * t46;
            let t75 = f64x8::splat(1.0) + f64x8::splat(32.16395899738507) / t72;
            let t76 = (simd::ln(t75));
            let t80 = f64x8::splat(1.0) + f64x8::splat(0.0278125) * t28;
            let t85 = f64x8::splat(5.1785) * t31 + f64x8::splat(0.905775) * t28 + f64x8::splat(0.1100325) * t34 + f64x8::splat(0.1241775) * t46;
            let t88 = f64x8::splat(1.0) + f64x8::splat(29.608749977793437) / t85;
            let t89 = (simd::ln(t88));
            let t90 = t80 * t89;
            let t96 = -t54 + t65 * (-f64x8::splat(0.0310907) * t67 * t76 + t54 - f64x8::splat(0.0197516734986138) * t90) + f64x8::splat(0.0197516734986138) * t65 * t90;
            let t99 = ((t9).select(f64x8::splat(0.0), t10 * t96 / f64x8::splat(2.0)));
            let t100 = t99 * v_tau0;
            let t101 = (simd::cbrt(v_rho0));
            let t102 = t101 * t101;
            let t104 = f64x8::splat(1.0) / t102 / v_rho0;
            let t108 = f64x8::splat(1.0) / v_tau0;
            let t111 = f64x8::splat(1.0) - v_sigma0 / v_rho0 * t108 / f64x8::splat(8.0);
            let t112 = f64x8::splat(M_CBRT6);
            let t113 = t111 * t112;
            let t114 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t115 = (simd::cbrt(t114));
            let t116 = t115 * t115;
            let t117 = f64x8::splat(1.0) / t116;
            let t118 = param_css * v_sigma0;
            let t119 = v_rho0 * v_rho0;
            let t121 = f64x8::splat(1.0) / t102 / t119;
            let t123 = t118 * t121 + f64x8::splat(1.0);
            let t124 = t123 * t123;
            let t125 = f64x8::splat(1.0) / t124;
            let t126 = t117 * t125;
            let t127 = t113 * t126;
            let t129 = f64x8::splat(5.0) / f64x8::splat(9.0) * t100 * t104 * t127;
            let t131 = f64x8::splat(1.0) - t6;
            let t132 = (t131).simd_le(zeta_threshold);
            let t133 = ((v_rho1).simd_le(dens_threshold)) | (t132);
            let t134 = ((t132).select(zeta_threshold, t131));
            let t135 = (simd::cbrt(t131));
            let t137 = ((t132).select(t23, f64x8::splat(1.0) / t135));
            let t139 = t17 * t21 * t137;
            let t141 = f64x8::splat(1.0) + f64x8::splat(0.053425) * t139;
            let t142 = ((t139).sqrt());
            let t145 = ((t139) * (t139).sqrt());
            let t147 = t137 * t137;
            let t149 = t39 * t43 * t147;
            let t151 = f64x8::splat(3.79785) * t142 + f64x8::splat(0.8969) * t139 + f64x8::splat(0.204775) * t145 + f64x8::splat(0.123235) * t149;
            let t154 = f64x8::splat(1.0) + f64x8::splat(16.081979498692537) / t151;
            let t155 = (simd::ln(t154));
            let t157 = f64x8::splat(0.0621814) * t141 * t155;
            let t159 = f64x8::splat(1.0) + f64x8::splat(0.05137) * t139;
            let t164 = f64x8::splat(7.05945) * t142 + f64x8::splat(1.549425) * t139 + f64x8::splat(0.420775) * t145 + f64x8::splat(0.1562925) * t149;
            let t167 = f64x8::splat(1.0) + f64x8::splat(32.16395899738507) / t164;
            let t168 = (simd::ln(t167));
            let t172 = f64x8::splat(1.0) + f64x8::splat(0.0278125) * t139;
            let t177 = f64x8::splat(5.1785) * t142 + f64x8::splat(0.905775) * t139 + f64x8::splat(0.1100325) * t145 + f64x8::splat(0.1241775) * t149;
            let t180 = f64x8::splat(1.0) + f64x8::splat(29.608749977793437) / t177;
            let t181 = (simd::ln(t180));
            let t182 = t172 * t181;
            let t188 = -t157 + t65 * (-f64x8::splat(0.0310907) * t159 * t168 + t157 - f64x8::splat(0.0197516734986138) * t182) + f64x8::splat(0.0197516734986138) * t65 * t182;
            let t191 = ((t133).select(f64x8::splat(0.0), t134 * t188 / f64x8::splat(2.0)));
            let t192 = t191 * v_tau1;
            let t193 = (simd::cbrt(v_rho1));
            let t194 = t193 * t193;
            let t196 = f64x8::splat(1.0) / t194 / v_rho1;
            let t200 = f64x8::splat(1.0) / v_tau1;
            let t203 = f64x8::splat(1.0) - v_sigma2 / v_rho1 * t200 / f64x8::splat(8.0);
            let t204 = t203 * t112;
            let t205 = param_css * v_sigma2;
            let t206 = v_rho1 * v_rho1;
            let t208 = f64x8::splat(1.0) / t194 / t206;
            let t210 = t205 * t208 + f64x8::splat(1.0);
            let t211 = t210 * t210;
            let t212 = f64x8::splat(1.0) / t211;
            let t213 = t117 * t212;
            let t214 = t204 * t213;
            let t216 = f64x8::splat(5.0) / f64x8::splat(9.0) * t192 * t196 * t214;
            let t218 = t14 * t16 * t19;
            let t220 = f64x8::splat(1.0) + f64x8::splat(0.053425) * t218;
            let t221 = ((t218).sqrt());
            let t224 = ((t218) * (t218).sqrt());
            let t227 = t38 * t15 * t41;
            let t229 = f64x8::splat(3.79785) * t221 + f64x8::splat(0.8969) * t218 + f64x8::splat(0.204775) * t224 + f64x8::splat(0.123235) * t227;
            let t232 = f64x8::splat(1.0) + f64x8::splat(16.081979498692537) / t229;
            let t233 = (simd::ln(t232));
            let t235 = f64x8::splat(0.0621814) * t220 * t233;
            let t236 = t3 * t3;
            let t237 = t236 * t236;
            let t238 = t4 * t4;
            let t239 = t238 * t238;
            let t240 = f64x8::splat(1.0) / t239;
            let t241 = t237 * t240;
            let t242 = t24 * t7;
            let t243 = ((t8).select(t56, t242));
            let t244 = t135 * t131;
            let t245 = ((t132).select(t56, t244));
            let t246 = t243 + t245 - f64x8::splat(2.0);
            let t247 = t246 * t64;
            let t249 = f64x8::splat(1.0) + f64x8::splat(0.05137) * t218;
            let t254 = f64x8::splat(7.05945) * t221 + f64x8::splat(1.549425) * t218 + f64x8::splat(0.420775) * t224 + f64x8::splat(0.1562925) * t227;
            let t257 = f64x8::splat(1.0) + f64x8::splat(32.16395899738507) / t254;
            let t258 = (simd::ln(t257));
            let t262 = f64x8::splat(1.0) + f64x8::splat(0.0278125) * t218;
            let t267 = f64x8::splat(5.1785) * t221 + f64x8::splat(0.905775) * t218 + f64x8::splat(0.1100325) * t224 + f64x8::splat(0.1241775) * t227;
            let t270 = f64x8::splat(1.0) + f64x8::splat(29.608749977793437) / t267;
            let t271 = (simd::ln(t270));
            let t272 = t262 * t271;
            let t274 = -f64x8::splat(0.0310907) * t249 * t258 + t235 - f64x8::splat(0.0197516734986138) * t272;
            let t275 = t247 * t274;
            let t279 = -t235 + t241 * t275 + f64x8::splat(0.0197516734986138) * t247 * t272 - t99 - t191;
            let t284 = f64x8::splat(1.0) + param_copp * (v_sigma0 * t121 + v_sigma2 * t208);
            let t285 = f64x8::splat(1.0) / t284;
            let t286 = t279 * t285;
            let tzk0 = t129 + t216 + t286;
            acc_zk = tzk0;
        }
        store_add(zk, ip, m, acc_zk);
        ip += 8;
    }
}
