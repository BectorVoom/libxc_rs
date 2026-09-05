//! GGA_X_C09X kxc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_c09x.c`
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
pub fn gga_x_c09x_kxc_pol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    v2rho2: &mut [f64],
    v2rhosigma: &mut [f64],
    v2sigma2: &mut [f64],
    v3rho3: &mut [f64],
    v3rho2sigma: &mut [f64],
    v3rhosigma2: &mut [f64],
    v3sigma3: &mut [f64],
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
        let mut acc_vrho_0 = V_ZERO;
        let mut acc_vrho_1 = V_ZERO;
        let mut acc_vsigma_0 = V_ZERO;
        let mut acc_vsigma_1 = V_ZERO;
        let mut acc_vsigma_2 = V_ZERO;
        let mut acc_v2rho2_0 = V_ZERO;
        let mut acc_v2rho2_1 = V_ZERO;
        let mut acc_v2rho2_2 = V_ZERO;
        let mut acc_v2rhosigma_0 = V_ZERO;
        let mut acc_v2rhosigma_1 = V_ZERO;
        let mut acc_v2rhosigma_2 = V_ZERO;
        let mut acc_v2rhosigma_3 = V_ZERO;
        let mut acc_v2rhosigma_4 = V_ZERO;
        let mut acc_v2rhosigma_5 = V_ZERO;
        let mut acc_v2sigma2_0 = V_ZERO;
        let mut acc_v2sigma2_1 = V_ZERO;
        let mut acc_v2sigma2_2 = V_ZERO;
        let mut acc_v2sigma2_3 = V_ZERO;
        let mut acc_v2sigma2_4 = V_ZERO;
        let mut acc_v2sigma2_5 = V_ZERO;
        let mut acc_v3rho3_0 = V_ZERO;
        let mut acc_v3rho3_1 = V_ZERO;
        let mut acc_v3rho3_2 = V_ZERO;
        let mut acc_v3rho3_3 = V_ZERO;
        let mut acc_v3rho2sigma_0 = V_ZERO;
        let mut acc_v3rho2sigma_1 = V_ZERO;
        let mut acc_v3rho2sigma_2 = V_ZERO;
        let mut acc_v3rho2sigma_3 = V_ZERO;
        let mut acc_v3rho2sigma_4 = V_ZERO;
        let mut acc_v3rho2sigma_5 = V_ZERO;
        let mut acc_v3rho2sigma_6 = V_ZERO;
        let mut acc_v3rho2sigma_7 = V_ZERO;
        let mut acc_v3rho2sigma_8 = V_ZERO;
        let mut acc_v3rhosigma2_0 = V_ZERO;
        let mut acc_v3rhosigma2_1 = V_ZERO;
        let mut acc_v3rhosigma2_2 = V_ZERO;
        let mut acc_v3rhosigma2_3 = V_ZERO;
        let mut acc_v3rhosigma2_4 = V_ZERO;
        let mut acc_v3rhosigma2_5 = V_ZERO;
        let mut acc_v3rhosigma2_6 = V_ZERO;
        let mut acc_v3rhosigma2_7 = V_ZERO;
        let mut acc_v3rhosigma2_8 = V_ZERO;
        let mut acc_v3rhosigma2_9 = V_ZERO;
        let mut acc_v3rhosigma2_10 = V_ZERO;
        let mut acc_v3rhosigma2_11 = V_ZERO;
        let mut acc_v3sigma3_0 = V_ZERO;
        let mut acc_v3sigma3_1 = V_ZERO;
        let mut acc_v3sigma3_2 = V_ZERO;
        let mut acc_v3sigma3_3 = V_ZERO;
        let mut acc_v3sigma3_4 = V_ZERO;
        let mut acc_v3sigma3_5 = V_ZERO;
        let mut acc_v3sigma3_6 = V_ZERO;
        let mut acc_v3sigma3_7 = V_ZERO;
        let mut acc_v3sigma3_8 = V_ZERO;
        let mut acc_v3sigma3_9 = V_ZERO;
        {
            let t1 = (v_rho0).simd_le(dens_threshold);
            let t2 = f64x8::splat(M_CBRT3);
            let t3 = f64x8::splat(M_CBRTPI);
            let t5 = t2 / t3;
            let t6 = v_rho0 + v_rho1;
            let t7 = f64x8::splat(1.0) / t6;
            let t10 = (f64x8::splat(2.0) * v_rho0 * t7).simd_le(zeta_threshold);
            let t11 = zeta_threshold - f64x8::splat(1.0);
            let t14 = (f64x8::splat(2.0) * v_rho1 * t7).simd_le(zeta_threshold);
            let t15 = -t11;
            let t16 = v_rho0 - v_rho1;
            let t18 = ((t10).select(t11, (t14).select(t15, t16 * t7)));
            let t19 = f64x8::splat(1.0) + t18;
            let t20 = (t19).simd_le(zeta_threshold);
            let t21 = (simd::cbrt(zeta_threshold));
            let t22 = t21 * zeta_threshold;
            let t23 = (simd::cbrt(t19));
            let t25 = ((t20).select(t22, t23 * t19));
            let t26 = (simd::cbrt(t6));
            let t27 = t25 * t26;
            let t28 = f64x8::splat(M_CBRT6);
            let t29 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t30 = (simd::cbrt(t29));
            let t31 = t30 * t30;
            let t32 = f64x8::splat(1.0) / t31;
            let t33 = t28 * t32;
            let t34 = v_rho0 * v_rho0;
            let t35 = (simd::cbrt(v_rho0));
            let t36 = t35 * t35;
            let t38 = f64x8::splat(1.0) / t36 / t34;
            let t39 = v_sigma0 * t38;
            let t40 = t33 * t39;
            let t42 = (simd::exp(-f64x8::splat(0.0020125) * t40));
            let t47 = (simd::exp(-f64x8::splat(0.00100625) * t40));
            let t49 = f64x8::splat(2.245) + f64x8::splat(0.0025708333333333334) * t33 * t39 * t42 - f64x8::splat(1.245) * t47;
            let t53 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t27 * t49));
            let t54 = (v_rho1).simd_le(dens_threshold);
            let t55 = -t16;
            let t57 = ((t14).select(t11, (t10).select(t15, t55 * t7)));
            let t58 = f64x8::splat(1.0) + t57;
            let t59 = (t58).simd_le(zeta_threshold);
            let t60 = (simd::cbrt(t58));
            let t62 = ((t59).select(t22, t60 * t58));
            let t63 = t62 * t26;
            let t64 = v_rho1 * v_rho1;
            let t65 = (simd::cbrt(v_rho1));
            let t66 = t65 * t65;
            let t68 = f64x8::splat(1.0) / t66 / t64;
            let t69 = v_sigma2 * t68;
            let t70 = t33 * t69;
            let t72 = (simd::exp(-f64x8::splat(0.0020125) * t70));
            let t77 = (simd::exp(-f64x8::splat(0.00100625) * t70));
            let t79 = f64x8::splat(2.245) + f64x8::splat(0.0025708333333333334) * t33 * t69 * t72 - f64x8::splat(1.245) * t77;
            let t83 = ((t54).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t63 * t79));
            let tzk0 = t53 + t83;
            acc_zk = tzk0;
            let t84 = t6 * t6;
            let t85 = f64x8::splat(1.0) / t84;
            let t86 = t16 * t85;
            let t88 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), t7 - t86)));
            let t91 = ((t20).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t88));
            let t92 = t91 * t26;
            let t96 = t26 * t26;
            let t97 = f64x8::splat(1.0) / t96;
            let t98 = t25 * t97;
            let t101 = t5 * t98 * t49 / f64x8::splat(8.0);
            let t102 = t34 * v_rho0;
            let t104 = f64x8::splat(1.0) / t36 / t102;
            let t105 = v_sigma0 * t104;
            let t109 = t28 * t28;
            let t112 = t109 / t30 / t29;
            let t113 = v_sigma0 * v_sigma0;
            let t114 = t34 * t34;
            let t115 = t114 * t34;
            let t117 = f64x8::splat(1.0) / t35 / t115;
            let t125 = -f64x8::splat(0.006855555555555556) * t33 * t105 * t42 + f64x8::splat(1.3796805555555556e-05) * t112 * t113 * t117 * t42 - f64x8::splat(0.00334075) * t33 * t105 * t47;
            let t130 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t92 * t49 - t101 - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t27 * t125));
            let t131 = t55 * t85;
            let t133 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), -t7 - t131)));
            let t136 = ((t59).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t60 * t133));
            let t137 = t136 * t26;
            let t141 = t62 * t97;
            let t144 = t5 * t141 * t79 / f64x8::splat(8.0);
            let t146 = ((t54).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t137 * t79 - t144));
            let tvrho0 = t53 + t83 + t6 * (t130 + t146);
            acc_vrho_0 = tvrho0;
            let t150 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), -t7 - t86)));
            let t153 = ((t20).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t150));
            let t154 = t153 * t26;
            let t159 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t154 * t49 - t101));
            let t161 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), t7 - t131)));
            let t164 = ((t59).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t60 * t161));
            let t165 = t164 * t26;
            let t169 = t64 * v_rho1;
            let t171 = f64x8::splat(1.0) / t66 / t169;
            let t172 = v_sigma2 * t171;
            let t176 = v_sigma2 * v_sigma2;
            let t177 = t64 * t64;
            let t178 = t177 * t64;
            let t180 = f64x8::splat(1.0) / t65 / t178;
            let t188 = -f64x8::splat(0.006855555555555556) * t33 * t172 * t72 + f64x8::splat(1.3796805555555556e-05) * t112 * t176 * t180 * t72 - f64x8::splat(0.00334075) * t33 * t172 * t77;
            let t193 = ((t54).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t165 * t79 - t144 - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t63 * t188));
            let tvrho1 = t53 + t83 + t6 * (t159 + t193);
            acc_vrho_1 = tvrho1;
            let t199 = t114 * v_rho0;
            let t201 = f64x8::splat(1.0) / t35 / t199;
            let t209 = f64x8::splat(0.0025708333333333334) * t33 * t38 * t42 - f64x8::splat(5.173802083333333e-06) * t112 * v_sigma0 * t201 * t42 + f64x8::splat(0.00125278125) * t33 * t38 * t47;
            let t213 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t27 * t209));
            let tvsigma0 = t6 * t213;
            acc_vsigma_0 = tvsigma0;
            let tvsigma1 = f64x8::splat(0.0);
            acc_vsigma_1 = tvsigma1;
            let t217 = t177 * v_rho1;
            let t219 = f64x8::splat(1.0) / t65 / t217;
            let t227 = f64x8::splat(0.0025708333333333334) * t33 * t68 * t72 - f64x8::splat(5.173802083333333e-06) * t112 * v_sigma2 * t219 * t72 + f64x8::splat(0.00125278125) * t33 * t68 * t77;
            let t231 = ((t54).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t63 * t227));
            let tvsigma2 = t6 * t231;
            acc_vsigma_2 = tvsigma2;
            let t234 = t23 * t23;
            let t235 = f64x8::splat(1.0) / t234;
            let t236 = t88 * t88;
            let t239 = t84 * t6;
            let t240 = f64x8::splat(1.0) / t239;
            let t241 = t16 * t240;
            let t244 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), -f64x8::splat(2.0) * t85 + f64x8::splat(2.0) * t241)));
            let t248 = ((t20).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(9.0) * t235 * t236 + f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t244));
            let t249 = t248 * t26;
            let t253 = t91 * t97;
            let t255 = t5 * t253 * t49;
            let t261 = f64x8::splat(1.0) / t96 / t6;
            let t262 = t25 * t261;
            let t265 = t5 * t262 * t49 / f64x8::splat(12.0);
            let t267 = t5 * t98 * t125;
            let t270 = f64x8::splat(1.0) / t36 / t114;
            let t271 = v_sigma0 * t270;
            let t275 = t114 * t102;
            let t277 = f64x8::splat(1.0) / t35 / t275;
            let t278 = t113 * t277;
            let t282 = t113 * v_sigma0;
            let t283 = t114 * t114;
            let t284 = t283 * t34;
            let t285 = f64x8::splat(1.0) / t284;
            let t295 = f64x8::splat(0.025137037037037038) * t33 * t271 * t42 - f64x8::splat(0.00012417125) * t112 * t278 * t42 + f64x8::splat(4.560735904350167e-09) * t282 * t285 * t42 + f64x8::splat(0.012249416666666667) * t33 * t271 * t47 - f64x8::splat(8.964345833333334e-06) * t112 * t278 * t47;
            let t300 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t249 * t49 - t255 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(4.0) * t5 * t92 * t125 + t265 - t267 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t27 * t295));
            let t301 = t60 * t60;
            let t302 = f64x8::splat(1.0) / t301;
            let t303 = t133 * t133;
            let t306 = t55 * t240;
            let t309 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), f64x8::splat(2.0) * t85 + f64x8::splat(2.0) * t306)));
            let t313 = ((t59).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(9.0) * t302 * t303 + f64x8::splat(4.0) / f64x8::splat(3.0) * t60 * t309));
            let t314 = t313 * t26;
            let t318 = t136 * t97;
            let t320 = t5 * t318 * t79;
            let t322 = t62 * t261;
            let t325 = t5 * t322 * t79 / f64x8::splat(12.0);
            let t327 = ((t54).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t314 * t79 - t320 / f64x8::splat(4.0) + t325));
            let tv2rho20 = f64x8::splat(2.0) * t130 + f64x8::splat(2.0) * t146 + t6 * (t300 + t327);
            acc_v2rho2_0 = tv2rho20;
            let t330 = t235 * t150;
            let t334 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), f64x8::splat(2.0) * t241)));
            let t338 = ((t20).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(9.0) * t330 * t88 + f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t334));
            let t339 = t338 * t26;
            let t343 = t153 * t97;
            let t345 = t5 * t343 * t49;
            let t353 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t339 * t49 - t345 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t154 * t125 - t255 / f64x8::splat(8.0) + t265 - t267 / f64x8::splat(8.0)));
            let t354 = t302 * t161;
            let t358 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), f64x8::splat(2.0) * t306)));
            let t362 = ((t59).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(9.0) * t354 * t133 + f64x8::splat(4.0) / f64x8::splat(3.0) * t60 * t358));
            let t363 = t362 * t26;
            let t367 = t164 * t97;
            let t369 = t5 * t367 * t79;
            let t376 = t5 * t141 * t188;
            let t379 = ((t54).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t363 * t79 - t369 / f64x8::splat(8.0) - t320 / f64x8::splat(8.0) + t325 - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t137 * t188 - t376 / f64x8::splat(8.0)));
            let tv2rho21 = t130 + t146 + t159 + t193 + t6 * (t353 + t379);
            acc_v2rho2_1 = tv2rho21;
            let t384 = t150 * t150;
            let t389 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), f64x8::splat(2.0) * t85 + f64x8::splat(2.0) * t241)));
            let t393 = ((t20).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(9.0) * t235 * t384 + f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t389));
            let t394 = t393 * t26;
            let t400 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t394 * t49 - t345 / f64x8::splat(4.0) + t265));
            let t401 = t161 * t161;
            let t406 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), -f64x8::splat(2.0) * t85 + f64x8::splat(2.0) * t306)));
            let t410 = ((t59).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(9.0) * t302 * t401 + f64x8::splat(4.0) / f64x8::splat(3.0) * t60 * t406));
            let t411 = t410 * t26;
            let t421 = f64x8::splat(1.0) / t66 / t177;
            let t422 = v_sigma2 * t421;
            let t426 = t177 * t169;
            let t428 = f64x8::splat(1.0) / t65 / t426;
            let t429 = t176 * t428;
            let t433 = t176 * v_sigma2;
            let t434 = t177 * t177;
            let t435 = t434 * t64;
            let t436 = f64x8::splat(1.0) / t435;
            let t446 = f64x8::splat(0.025137037037037038) * t33 * t422 * t72 - f64x8::splat(0.00012417125) * t112 * t429 * t72 + f64x8::splat(4.560735904350167e-09) * t433 * t436 * t72 + f64x8::splat(0.012249416666666667) * t33 * t422 * t77 - f64x8::splat(8.964345833333334e-06) * t112 * t429 * t77;
            let t451 = ((t54).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t411 * t79 - t369 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(4.0) * t5 * t165 * t188 + t325 - t376 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t63 * t446));
            let tv2rho22 = f64x8::splat(2.0) * t159 + f64x8::splat(2.0) * t193 + t6 * (t400 + t451);
            acc_v2rho2_2 = tv2rho22;
            let t459 = t5 * t98 * t209 / f64x8::splat(8.0);
            let t463 = t117 * v_sigma0;
            let t467 = t283 * v_rho0;
            let t468 = f64x8::splat(1.0) / t467;
            let t478 = -f64x8::splat(0.006855555555555556) * t33 * t104 * t42 + f64x8::splat(4.1390416666666666e-05) * t112 * t463 * t42 - f64x8::splat(1.7102759641313128e-09) * t113 * t468 * t42 - f64x8::splat(0.00334075) * t33 * t104 * t47 + f64x8::splat(3.3616296875e-06) * t112 * t463 * t47;
            let t483 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t92 * t209 - t459 - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t27 * t478));
            let tv2rhosigma0 = t483 * t6 + t213;
            acc_v2rhosigma_0 = tv2rhosigma0;
            let tv2rhosigma1 = f64x8::splat(0.0);
            acc_v2rhosigma_1 = tv2rhosigma1;
            let t490 = t5 * t141 * t227 / f64x8::splat(8.0);
            let t492 = ((t54).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t137 * t227 - t490));
            let tv2rhosigma2 = t492 * t6 + t231;
            acc_v2rhosigma_2 = tv2rhosigma2;
            let t498 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t154 * t209 - t459));
            let tv2rhosigma3 = t498 * t6 + t213;
            acc_v2rhosigma_3 = tv2rhosigma3;
            let tv2rhosigma4 = f64x8::splat(0.0);
            acc_v2rhosigma_4 = tv2rhosigma4;
            let t506 = t180 * v_sigma2;
            let t510 = t434 * v_rho1;
            let t511 = f64x8::splat(1.0) / t510;
            let t521 = -f64x8::splat(0.006855555555555556) * t33 * t171 * t72 + f64x8::splat(4.1390416666666666e-05) * t112 * t506 * t72 - f64x8::splat(1.7102759641313128e-09) * t176 * t511 * t72 - f64x8::splat(0.00334075) * t33 * t171 * t77 + f64x8::splat(3.3616296875e-06) * t112 * t506 * t77;
            let t526 = ((t54).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t165 * t227 - t490 - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t63 * t521));
            let tv2rhosigma5 = t526 * t6 + t231;
            acc_v2rhosigma_5 = tv2rhosigma5;
            let t531 = f64x8::splat(1.0) / t283;
            let t538 = -f64x8::splat(1.0347604166666667e-05) * t112 * t201 * t42 + f64x8::splat(6.413534865492423e-10) * v_sigma0 * t531 * t42 - f64x8::splat(1.2606111328125e-06) * t112 * t201 * t47;
            let t542 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t27 * t538));
            let tv2sigma20 = t6 * t542;
            acc_v2sigma2_0 = tv2sigma20;
            let tv2sigma21 = f64x8::splat(0.0);
            acc_v2sigma2_1 = tv2sigma21;
            let tv2sigma22 = f64x8::splat(0.0);
            acc_v2sigma2_2 = tv2sigma22;
            let tv2sigma23 = f64x8::splat(0.0);
            acc_v2sigma2_3 = tv2sigma23;
            let tv2sigma24 = f64x8::splat(0.0);
            acc_v2sigma2_4 = tv2sigma24;
            let t546 = f64x8::splat(1.0) / t434;
            let t553 = -f64x8::splat(1.0347604166666667e-05) * t112 * t219 * t72 + f64x8::splat(6.413534865492423e-10) * v_sigma2 * t546 * t72 - f64x8::splat(1.2606111328125e-06) * t112 * t219 * t77;
            let t557 = ((t54).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t63 * t553));
            let tv2sigma25 = t6 * t557;
            acc_v2sigma2_5 = tv2sigma25;
            let t561 = f64x8::splat(1.0) / t234 / t19;
            let t562 = t236 * t88;
            let t565 = t235 * t88;
            let t568 = t84 * t84;
            let t569 = f64x8::splat(1.0) / t568;
            let t570 = t16 * t569;
            let t573 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), f64x8::splat(6.0) * t240 - f64x8::splat(6.0) * t570)));
            let t577 = ((t20).select(f64x8::splat(0.0), -f64x8::splat(8.0) / f64x8::splat(27.0) * t561 * t562 + f64x8::splat(4.0) / f64x8::splat(3.0) * t565 * t244 + f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t573));
            let t578 = t577 * t26;
            let t582 = t248 * t97;
            let t584 = t5 * t582 * t49;
            let t589 = t91 * t261;
            let t591 = t5 * t589 * t49;
            let t594 = t5 * t253 * t125;
            let t600 = f64x8::splat(1.0) / t96 / t84;
            let t601 = t25 * t600;
            let t604 = f64x8::splat(5.0) / f64x8::splat(36.0) * t5 * t601 * t49;
            let t606 = t5 * t262 * t125;
            let t609 = t5 * t98 * t295;
            let t612 = f64x8::splat(1.0) / t36 / t199;
            let t613 = v_sigma0 * t612;
            let t618 = f64x8::splat(1.0) / t35 / t283;
            let t619 = t113 * t618;
            let t623 = t283 * t102;
            let t624 = f64x8::splat(1.0) / t623;
            let t625 = t282 * t624;
            let t628 = t113 * t113;
            let t629 = t283 * t199;
            let t631 = f64x8::splat(1.0) / t36 / t629;
            let t633 = t33 * t42;
            let t644 = -f64x8::splat(0.11730617283950617) * t33 * t613 * t42 + f64x8::splat(0.0010454912654320988) * t112 * t619 * t42 - f64x8::splat(8.665398218265318e-08) * t625 * t42 + f64x8::splat(2.44759493533459e-11) * t628 * t631 * t633 - f64x8::splat(0.05716394444444445) * t33 * t613 * t47 + f64x8::splat(9.860780416666666e-05) * t112 * t619 * t47 - f64x8::splat(1.4816478255226407e-09) * t625 * t47;
            let t649 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t578 * t49 - f64x8::splat(3.0) / f64x8::splat(8.0) * t584 - f64x8::splat(9.0) / f64x8::splat(8.0) * t5 * t249 * t125 + t591 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(4.0) * t594 - f64x8::splat(9.0) / f64x8::splat(8.0) * t5 * t92 * t295 - t604 + t606 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t609 - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t27 * t644));
            let t651 = f64x8::splat(1.0) / t301 / t58;
            let t652 = t303 * t133;
            let t655 = t302 * t133;
            let t658 = t55 * t569;
            let t661 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), -f64x8::splat(6.0) * t240 - f64x8::splat(6.0) * t658)));
            let t665 = ((t59).select(f64x8::splat(0.0), -f64x8::splat(8.0) / f64x8::splat(27.0) * t651 * t652 + f64x8::splat(4.0) / f64x8::splat(3.0) * t655 * t309 + f64x8::splat(4.0) / f64x8::splat(3.0) * t60 * t661));
            let t666 = t665 * t26;
            let t670 = t313 * t97;
            let t672 = t5 * t670 * t79;
            let t674 = t136 * t261;
            let t676 = t5 * t674 * t79;
            let t678 = t62 * t600;
            let t681 = f64x8::splat(5.0) / f64x8::splat(36.0) * t5 * t678 * t79;
            let t683 = ((t54).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t666 * t79 - f64x8::splat(3.0) / f64x8::splat(8.0) * t672 + t676 / f64x8::splat(4.0) - t681));
            let tv3rho30 = f64x8::splat(3.0) * t300 + f64x8::splat(3.0) * t327 + t6 * (t649 + t683);
            acc_v3rho3_0 = tv3rho30;
            let t686 = f64x8::splat(2.0) * t353;
            let t687 = f64x8::splat(2.0) * t379;
            let t688 = t561 * t150;
            let t691 = t235 * t334;
            let t696 = f64x8::splat(2.0) * t240;
            let t697 = f64x8::splat(6.0) * t570;
            let t699 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), t696 - t697)));
            let t703 = ((t20).select(f64x8::splat(0.0), -f64x8::splat(8.0) / f64x8::splat(27.0) * t688 * t236 + f64x8::splat(8.0) / f64x8::splat(9.0) * t691 * t88 + f64x8::splat(4.0) / f64x8::splat(9.0) * t330 * t244 + f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t699));
            let t704 = t703 * t26;
            let t708 = t338 * t97;
            let t711 = t5 * t708 * t49 / f64x8::splat(4.0);
            let t715 = t153 * t261;
            let t717 = t5 * t715 * t49;
            let t721 = t5 * t343 * t125 / f64x8::splat(4.0);
            let t730 = -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t704 * t49 - t711 - f64x8::splat(3.0) / f64x8::splat(4.0) * t5 * t339 * t125 + t717 / f64x8::splat(12.0) - t721 - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t154 * t295 - t584 / f64x8::splat(8.0) + t591 / f64x8::splat(6.0) - t594 / f64x8::splat(4.0) - t604 + t606 / f64x8::splat(6.0) - t609 / f64x8::splat(8.0);
            let t731 = ((t1).select(f64x8::splat(0.0), t730));
            let t732 = t651 * t161;
            let t735 = t302 * t358;
            let t740 = f64x8::splat(6.0) * t658;
            let t742 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), -t696 - t740)));
            let t746 = ((t59).select(f64x8::splat(0.0), -f64x8::splat(8.0) / f64x8::splat(27.0) * t732 * t303 + f64x8::splat(8.0) / f64x8::splat(9.0) * t735 * t133 + f64x8::splat(4.0) / f64x8::splat(9.0) * t354 * t309 + f64x8::splat(4.0) / f64x8::splat(3.0) * t60 * t742));
            let t747 = t746 * t26;
            let t751 = t362 * t97;
            let t754 = t5 * t751 * t79 / f64x8::splat(4.0);
            let t755 = t164 * t261;
            let t757 = t5 * t755 * t79;
            let t766 = t5 * t318 * t188 / f64x8::splat(4.0);
            let t768 = t5 * t322 * t188;
            let t771 = ((t54).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t747 * t79 - t754 + t757 / f64x8::splat(12.0) - t672 / f64x8::splat(8.0) + t676 / f64x8::splat(6.0) - t681 - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t314 * t188 - t766 + t768 / f64x8::splat(12.0)));
            let tv3rho31 = t300 + t327 + t686 + t687 + t6 * (t731 + t771);
            acc_v3rho3_1 = tv3rho31;
            let t774 = t561 * t384;
            let t779 = t235 * t389;
            let t783 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), -t696 - t697)));
            let t787 = ((t20).select(f64x8::splat(0.0), -f64x8::splat(8.0) / f64x8::splat(27.0) * t774 * t88 + f64x8::splat(8.0) / f64x8::splat(9.0) * t330 * t334 + f64x8::splat(4.0) / f64x8::splat(9.0) * t779 * t88 + f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t783));
            let t788 = t787 * t26;
            let t792 = t393 * t97;
            let t794 = t5 * t792 * t49;
            let t803 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t788 * t49 - t794 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t394 * t125 - t711 + t717 / f64x8::splat(6.0) - t721 + t591 / f64x8::splat(12.0) - t604 + t606 / f64x8::splat(12.0)));
            let t804 = t651 * t401;
            let t809 = t302 * t406;
            let t813 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), t696 - t740)));
            let t817 = ((t59).select(f64x8::splat(0.0), -f64x8::splat(8.0) / f64x8::splat(27.0) * t804 * t133 + f64x8::splat(8.0) / f64x8::splat(9.0) * t354 * t358 + f64x8::splat(4.0) / f64x8::splat(9.0) * t809 * t133 + f64x8::splat(4.0) / f64x8::splat(3.0) * t60 * t813));
            let t818 = t817 * t26;
            let t822 = t410 * t97;
            let t824 = t5 * t822 * t79;
            let t831 = t5 * t367 * t188;
            let t839 = t5 * t141 * t446;
            let t841 = -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t818 * t79 - t824 / f64x8::splat(8.0) - t754 + t757 / f64x8::splat(6.0) - f64x8::splat(3.0) / f64x8::splat(4.0) * t5 * t363 * t188 - t831 / f64x8::splat(4.0) + t676 / f64x8::splat(12.0) - t681 - t766 + t768 / f64x8::splat(6.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t137 * t446 - t839 / f64x8::splat(8.0);
            let t842 = ((t54).select(f64x8::splat(0.0), t841));
            let tv3rho32 = t686 + t687 + t400 + t451 + t6 * (t803 + t842);
            acc_v3rho3_2 = tv3rho32;
            let t847 = t384 * t150;
            let t854 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), -f64x8::splat(6.0) * t240 - f64x8::splat(6.0) * t570)));
            let t858 = ((t20).select(f64x8::splat(0.0), -f64x8::splat(8.0) / f64x8::splat(27.0) * t561 * t847 + f64x8::splat(4.0) / f64x8::splat(3.0) * t330 * t389 + f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t854));
            let t859 = t858 * t26;
            let t866 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t859 * t49 - f64x8::splat(3.0) / f64x8::splat(8.0) * t794 + t717 / f64x8::splat(4.0) - t604));
            let t867 = t401 * t161;
            let t874 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), f64x8::splat(6.0) * t240 - f64x8::splat(6.0) * t658)));
            let t878 = ((t59).select(f64x8::splat(0.0), -f64x8::splat(8.0) / f64x8::splat(27.0) * t651 * t867 + f64x8::splat(4.0) / f64x8::splat(3.0) * t354 * t406 + f64x8::splat(4.0) / f64x8::splat(3.0) * t60 * t874));
            let t879 = t878 * t26;
            let t895 = f64x8::splat(1.0) / t66 / t217;
            let t896 = v_sigma2 * t895;
            let t901 = f64x8::splat(1.0) / t65 / t434;
            let t902 = t176 * t901;
            let t906 = t434 * t169;
            let t907 = f64x8::splat(1.0) / t906;
            let t908 = t433 * t907;
            let t911 = t176 * t176;
            let t912 = t434 * t217;
            let t914 = f64x8::splat(1.0) / t66 / t912;
            let t916 = t33 * t72;
            let t927 = -f64x8::splat(0.11730617283950617) * t33 * t896 * t72 + f64x8::splat(0.0010454912654320988) * t112 * t902 * t72 - f64x8::splat(8.665398218265318e-08) * t908 * t72 + f64x8::splat(2.44759493533459e-11) * t911 * t914 * t916 - f64x8::splat(0.05716394444444445) * t33 * t896 * t77 + f64x8::splat(9.860780416666666e-05) * t112 * t902 * t77 - f64x8::splat(1.4816478255226407e-09) * t908 * t77;
            let t932 = ((t54).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t879 * t79 - f64x8::splat(3.0) / f64x8::splat(8.0) * t824 - f64x8::splat(9.0) / f64x8::splat(8.0) * t5 * t411 * t188 + t757 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(4.0) * t831 - f64x8::splat(9.0) / f64x8::splat(8.0) * t5 * t165 * t446 - t681 + t768 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t839 - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t63 * t927));
            let tv3rho33 = f64x8::splat(3.0) * t400 + f64x8::splat(3.0) * t451 + t6 * (t866 + t932);
            acc_v3rho3_3 = tv3rho33;
            let t940 = t5 * t253 * t209;
            let t947 = t5 * t262 * t209 / f64x8::splat(12.0);
            let t949 = t5 * t98 * t478;
            let t954 = t277 * v_sigma0;
            let t958 = t285 * t113;
            let t961 = t283 * t114;
            let t963 = f64x8::splat(1.0) / t36 / t961;
            let t975 = f64x8::splat(0.025137037037037038) * t33 * t270 * t42 - f64x8::splat(0.00029893078703703704) * t112 * t954 * t42 + f64x8::splat(2.907469139023232e-08) * t958 * t42 - f64x8::splat(9.178481007504712e-12) * t282 * t963 * t633 + f64x8::splat(0.012249416666666667) * t33 * t270 * t47 - f64x8::splat(3.02546671875e-05) * t112 * t954 * t47 + f64x8::splat(5.556179345709902e-10) * t958 * t47;
            let t980 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t249 * t209 - t940 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(4.0) * t5 * t92 * t478 + t947 - t949 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t27 * t975));
            let tv3rho2sigma0 = t6 * t980 + f64x8::splat(2.0) * t483;
            acc_v3rho2sigma_0 = tv3rho2sigma0;
            let tv3rho2sigma1 = f64x8::splat(0.0);
            acc_v3rho2sigma_1 = tv3rho2sigma1;
            let t987 = t5 * t318 * t227;
            let t991 = t5 * t322 * t227 / f64x8::splat(12.0);
            let t993 = ((t54).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t314 * t227 - t987 / f64x8::splat(4.0) + t991));
            let tv3rho2sigma2 = t6 * t993 + f64x8::splat(2.0) * t492;
            acc_v3rho2sigma_2 = tv3rho2sigma2;
            let t999 = t5 * t343 * t209;
            let t1007 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t339 * t209 - t999 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t154 * t478 - t940 / f64x8::splat(8.0) + t947 - t949 / f64x8::splat(8.0)));
            let tv3rho2sigma3 = t1007 * t6 + t483 + t498;
            acc_v3rho2sigma_3 = tv3rho2sigma3;
            let tv3rho2sigma4 = f64x8::splat(0.0);
            acc_v3rho2sigma_4 = tv3rho2sigma4;
            let t1013 = t5 * t367 * t227;
            let t1020 = t5 * t141 * t521;
            let t1023 = ((t54).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t363 * t227 - t1013 / f64x8::splat(8.0) - t987 / f64x8::splat(8.0) + t991 - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t137 * t521 - t1020 / f64x8::splat(8.0)));
            let tv3rho2sigma5 = t1023 * t6 + t492 + t526;
            acc_v3rho2sigma_5 = tv3rho2sigma5;
            let t1031 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t394 * t209 - t999 / f64x8::splat(4.0) + t947));
            let tv3rho2sigma6 = t1031 * t6 + f64x8::splat(2.0) * t498;
            acc_v3rho2sigma_6 = tv3rho2sigma6;
            let tv3rho2sigma7 = f64x8::splat(0.0);
            acc_v3rho2sigma_7 = tv3rho2sigma7;
            let t1045 = t428 * v_sigma2;
            let t1049 = t436 * t176;
            let t1052 = t434 * t177;
            let t1054 = f64x8::splat(1.0) / t66 / t1052;
            let t1066 = f64x8::splat(0.025137037037037038) * t33 * t421 * t72 - f64x8::splat(0.00029893078703703704) * t112 * t1045 * t72 + f64x8::splat(2.907469139023232e-08) * t1049 * t72 - f64x8::splat(9.178481007504712e-12) * t433 * t1054 * t916 + f64x8::splat(0.012249416666666667) * t33 * t421 * t77 - f64x8::splat(3.02546671875e-05) * t112 * t1045 * t77 + f64x8::splat(5.556179345709902e-10) * t1049 * t77;
            let t1071 = ((t54).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t411 * t227 - t1013 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(4.0) * t5 * t165 * t521 + t991 - t1020 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t63 * t1066));
            let tv3rho2sigma8 = t1071 * t6 + f64x8::splat(2.0) * t526;
            acc_v3rho2sigma_8 = tv3rho2sigma8;
            let t1078 = t5 * t98 * t538 / f64x8::splat(8.0);
            let t1082 = t468 * v_sigma0;
            let t1086 = f64x8::splat(1.0) / t36 / t623;
            let t1095 = f64x8::splat(5.5187222222222224e-05) * t112 * t117 * t42 - f64x8::splat(8.551379820656565e-09) * t1082 * t42 + f64x8::splat(3.441930377814267e-12) * t113 * t1086 * t633 + f64x8::splat(6.723259375e-06) * t112 * t117 * t47 - f64x8::splat(2.0835672546412136e-10) * t1082 * t47;
            let t1100 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t92 * t538 - t1078 - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t27 * t1095));
            let tv3rhosigma20 = t1100 * t6 + t542;
            acc_v3rhosigma2_0 = tv3rhosigma20;
            let tv3rhosigma21 = f64x8::splat(0.0);
            acc_v3rhosigma2_1 = tv3rhosigma21;
            let tv3rhosigma22 = f64x8::splat(0.0);
            acc_v3rhosigma2_2 = tv3rhosigma22;
            let tv3rhosigma23 = f64x8::splat(0.0);
            acc_v3rhosigma2_3 = tv3rhosigma23;
            let tv3rhosigma24 = f64x8::splat(0.0);
            acc_v3rhosigma2_4 = tv3rhosigma24;
            let t1107 = t5 * t141 * t553 / f64x8::splat(8.0);
            let t1109 = ((t54).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t137 * t553 - t1107));
            let tv3rhosigma25 = t1109 * t6 + t557;
            acc_v3rhosigma2_5 = tv3rhosigma25;
            let t1115 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t154 * t538 - t1078));
            let tv3rhosigma26 = t1115 * t6 + t542;
            acc_v3rhosigma2_6 = tv3rhosigma26;
            let tv3rhosigma27 = f64x8::splat(0.0);
            acc_v3rhosigma2_7 = tv3rhosigma27;
            let tv3rhosigma28 = f64x8::splat(0.0);
            acc_v3rhosigma2_8 = tv3rhosigma28;
            let tv3rhosigma29 = f64x8::splat(0.0);
            acc_v3rhosigma2_9 = tv3rhosigma29;
            let tv3rhosigma210 = f64x8::splat(0.0);
            acc_v3rhosigma2_10 = tv3rhosigma210;
            let t1123 = t511 * v_sigma2;
            let t1127 = f64x8::splat(1.0) / t66 / t906;
            let t1136 = f64x8::splat(5.5187222222222224e-05) * t112 * t180 * t72 - f64x8::splat(8.551379820656565e-09) * t1123 * t72 + f64x8::splat(3.441930377814267e-12) * t176 * t1127 * t916 + f64x8::splat(6.723259375e-06) * t112 * t180 * t77 - f64x8::splat(2.0835672546412136e-10) * t1123 * t77;
            let t1141 = ((t54).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t165 * t553 - t1107 - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t63 * t1136));
            let tv3rhosigma211 = t1141 * t6 + t557;
            acc_v3rhosigma2_11 = tv3rhosigma211;
            let t1146 = f64x8::splat(1.0) / t36 / t284;
            let t1152 = f64x8::splat(1.924060459647727e-09) * t531 * t42 - f64x8::splat(1.29072389168035e-12) * v_sigma0 * t1146 * t633 + f64x8::splat(7.81337720490455e-11) * t531 * t47;
            let t1156 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t27 * t1152));
            let tv3sigma30 = t6 * t1156;
            acc_v3sigma3_0 = tv3sigma30;
            let tv3sigma31 = f64x8::splat(0.0);
            acc_v3sigma3_1 = tv3sigma31;
            let tv3sigma32 = f64x8::splat(0.0);
            acc_v3sigma3_2 = tv3sigma32;
            let tv3sigma33 = f64x8::splat(0.0);
            acc_v3sigma3_3 = tv3sigma33;
            let tv3sigma34 = f64x8::splat(0.0);
            acc_v3sigma3_4 = tv3sigma34;
            let tv3sigma35 = f64x8::splat(0.0);
            acc_v3sigma3_5 = tv3sigma35;
            let tv3sigma36 = f64x8::splat(0.0);
            acc_v3sigma3_6 = tv3sigma36;
            let tv3sigma37 = f64x8::splat(0.0);
            acc_v3sigma3_7 = tv3sigma37;
            let tv3sigma38 = f64x8::splat(0.0);
            acc_v3sigma3_8 = tv3sigma38;
            let t1160 = f64x8::splat(1.0) / t66 / t435;
            let t1166 = f64x8::splat(1.924060459647727e-09) * t546 * t72 - f64x8::splat(1.29072389168035e-12) * v_sigma2 * t1160 * t916 + f64x8::splat(7.81337720490455e-11) * t546 * t77;
            let t1170 = ((t54).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t63 * t1166));
            let tv3sigma39 = t6 * t1170;
            acc_v3sigma3_9 = tv3sigma39;
        }
        store_add(zk, ip, m, acc_zk);
        store_strided(vrho, ip, m, 2, 0, acc_vrho_0);
        store_strided(vrho, ip, m, 2, 1, acc_vrho_1);
        store_strided(vsigma, ip, m, 3, 0, acc_vsigma_0);
        store_strided(vsigma, ip, m, 3, 1, acc_vsigma_1);
        store_strided(vsigma, ip, m, 3, 2, acc_vsigma_2);
        store_strided(v2rho2, ip, m, 3, 0, acc_v2rho2_0);
        store_strided(v2rho2, ip, m, 3, 1, acc_v2rho2_1);
        store_strided(v2rho2, ip, m, 3, 2, acc_v2rho2_2);
        store_strided(v2rhosigma, ip, m, 6, 0, acc_v2rhosigma_0);
        store_strided(v2rhosigma, ip, m, 6, 1, acc_v2rhosigma_1);
        store_strided(v2rhosigma, ip, m, 6, 2, acc_v2rhosigma_2);
        store_strided(v2rhosigma, ip, m, 6, 3, acc_v2rhosigma_3);
        store_strided(v2rhosigma, ip, m, 6, 4, acc_v2rhosigma_4);
        store_strided(v2rhosigma, ip, m, 6, 5, acc_v2rhosigma_5);
        store_strided(v2sigma2, ip, m, 6, 0, acc_v2sigma2_0);
        store_strided(v2sigma2, ip, m, 6, 1, acc_v2sigma2_1);
        store_strided(v2sigma2, ip, m, 6, 2, acc_v2sigma2_2);
        store_strided(v2sigma2, ip, m, 6, 3, acc_v2sigma2_3);
        store_strided(v2sigma2, ip, m, 6, 4, acc_v2sigma2_4);
        store_strided(v2sigma2, ip, m, 6, 5, acc_v2sigma2_5);
        store_strided(v3rho3, ip, m, 4, 0, acc_v3rho3_0);
        store_strided(v3rho3, ip, m, 4, 1, acc_v3rho3_1);
        store_strided(v3rho3, ip, m, 4, 2, acc_v3rho3_2);
        store_strided(v3rho3, ip, m, 4, 3, acc_v3rho3_3);
        store_strided(v3rho2sigma, ip, m, 9, 0, acc_v3rho2sigma_0);
        store_strided(v3rho2sigma, ip, m, 9, 1, acc_v3rho2sigma_1);
        store_strided(v3rho2sigma, ip, m, 9, 2, acc_v3rho2sigma_2);
        store_strided(v3rho2sigma, ip, m, 9, 3, acc_v3rho2sigma_3);
        store_strided(v3rho2sigma, ip, m, 9, 4, acc_v3rho2sigma_4);
        store_strided(v3rho2sigma, ip, m, 9, 5, acc_v3rho2sigma_5);
        store_strided(v3rho2sigma, ip, m, 9, 6, acc_v3rho2sigma_6);
        store_strided(v3rho2sigma, ip, m, 9, 7, acc_v3rho2sigma_7);
        store_strided(v3rho2sigma, ip, m, 9, 8, acc_v3rho2sigma_8);
        store_strided(v3rhosigma2, ip, m, 12, 0, acc_v3rhosigma2_0);
        store_strided(v3rhosigma2, ip, m, 12, 1, acc_v3rhosigma2_1);
        store_strided(v3rhosigma2, ip, m, 12, 2, acc_v3rhosigma2_2);
        store_strided(v3rhosigma2, ip, m, 12, 3, acc_v3rhosigma2_3);
        store_strided(v3rhosigma2, ip, m, 12, 4, acc_v3rhosigma2_4);
        store_strided(v3rhosigma2, ip, m, 12, 5, acc_v3rhosigma2_5);
        store_strided(v3rhosigma2, ip, m, 12, 6, acc_v3rhosigma2_6);
        store_strided(v3rhosigma2, ip, m, 12, 7, acc_v3rhosigma2_7);
        store_strided(v3rhosigma2, ip, m, 12, 8, acc_v3rhosigma2_8);
        store_strided(v3rhosigma2, ip, m, 12, 9, acc_v3rhosigma2_9);
        store_strided(v3rhosigma2, ip, m, 12, 10, acc_v3rhosigma2_10);
        store_strided(v3rhosigma2, ip, m, 12, 11, acc_v3rhosigma2_11);
        store_strided(v3sigma3, ip, m, 10, 0, acc_v3sigma3_0);
        store_strided(v3sigma3, ip, m, 10, 1, acc_v3sigma3_1);
        store_strided(v3sigma3, ip, m, 10, 2, acc_v3sigma3_2);
        store_strided(v3sigma3, ip, m, 10, 3, acc_v3sigma3_3);
        store_strided(v3sigma3, ip, m, 10, 4, acc_v3sigma3_4);
        store_strided(v3sigma3, ip, m, 10, 5, acc_v3sigma3_5);
        store_strided(v3sigma3, ip, m, 10, 6, acc_v3sigma3_6);
        store_strided(v3sigma3, ip, m, 10, 7, acc_v3sigma3_7);
        store_strided(v3sigma3, ip, m, 10, 8, acc_v3sigma3_8);
        store_strided(v3sigma3, ip, m, 10, 9, acc_v3sigma3_9);
        ip += 8;
    }
}
