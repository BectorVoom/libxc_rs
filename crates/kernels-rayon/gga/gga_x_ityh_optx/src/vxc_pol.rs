//! GGA_X_ITYH_OPTX vxc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_ityh_optx.c`
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
pub fn gga_x_ityh_optx_vxc_pol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    param_b: f64,
    param_a: f64,
    param_hyb_omega_0: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_b = f64x8::splat(param_b);
    let param_a = f64x8::splat(param_a);
    let param_hyb_omega_0 = f64x8::splat(param_hyb_omega_0);
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
            let t26 = t5 * t25;
            let t27 = (simd::cbrt(t6));
            let t28 = t2 * t2;
            let t29 = f64x8::splat(M_PI) * t28;
            let t30 = f64x8::splat(1.0) / f64x8::splat(M_PI);
            let t31 = (simd::cbrt(t30));
            let t32 = f64x8::splat(1.0) / t31;
            let t33 = f64x8::splat(M_CBRT4);
            let t34 = t32 * t33;
            let t35 = v_sigma0 * v_sigma0;
            let t36 = param_b * t35;
            let t37 = v_rho0 * v_rho0;
            let t38 = t37 * t37;
            let t39 = t38 * v_rho0;
            let t40 = (simd::cbrt(v_rho0));
            let t42 = f64x8::splat(1.0) / t40 / t39;
            let t43 = t40 * t40;
            let t48 = f64x8::splat(1.0) + f64x8::splat(6.0) * v_sigma0 / t43 / t37;
            let t49 = t48 * t48;
            let t50 = f64x8::splat(1.0) / t49;
            let t51 = t42 * t50;
            let t54 = param_a + f64x8::splat(36.0) * t36 * t51;
            let t57 = t29 * t34 / t54;
            let t58 = ((t57).sqrt());
            let t60 = param_hyb_omega_0 / t58;
            let t61 = f64x8::splat(M_CBRT2);
            let t62 = t19 * t6;
            let t63 = (simd::cbrt(t62));
            let t64 = f64x8::splat(1.0) / t63;
            let t65 = t61 * t64;
            let t67 = t60 * t65 / f64x8::splat(2.0);
            let t68 = (f64x8::splat(1.35)).simd_le(t67);
            let t69 = (f64x8::splat(1.35)).simd_lt(t67);
            let t70 = ((t69).select(t67, f64x8::splat(1.35)));
            let t71 = t70 * t70;
            let t74 = t71 * t71;
            let t75 = f64x8::splat(1.0) / t74;
            let t77 = t74 * t71;
            let t78 = f64x8::splat(1.0) / t77;
            let t80 = t74 * t74;
            let t81 = f64x8::splat(1.0) / t80;
            let t84 = f64x8::splat(1.0) / t80 / t71;
            let t87 = f64x8::splat(1.0) / t80 / t74;
            let t90 = f64x8::splat(1.0) / t80 / t77;
            let t92 = t80 * t80;
            let t93 = f64x8::splat(1.0) / t92;
            let t96 = ((t69).select(f64x8::splat(1.35), t67));
            let t97 = ((f64x8::splat(M_PI)).sqrt());
            let t98 = f64x8::splat(1.0) / t96;
            let t100 = (simd::erf(t98 / f64x8::splat(2.0)));
            let t102 = t96 * t96;
            let t103 = f64x8::splat(1.0) / t102;
            let t105 = (simd::exp(-t103 / f64x8::splat(4.0)));
            let t106 = t105 - f64x8::splat(1.0);
            let t109 = t105 - f64x8::splat(3.0) / f64x8::splat(2.0) - f64x8::splat(2.0) * t102 * t106;
            let t112 = t97 * t100 + f64x8::splat(2.0) * t96 * t109;
            let t116 = ((t68).select(f64x8::splat(1.0) / t71 / f64x8::splat(36.0) - t75 / f64x8::splat(960.0) + t78 / f64x8::splat(26880.0) - t81 / f64x8::splat(829440.0) + t84 / f64x8::splat(28385280.0) - t87 / f64x8::splat(1073479680.0) + t90 / f64x8::splat(44590694400.0) - t93 / f64x8::splat(2021444812800.0), f64x8::splat(1.0) - f64x8::splat(8.0) / f64x8::splat(3.0) * t96 * t112));
            let t117 = t27 * t116;
            let t118 = t117 * t54;
            let t121 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t26 * t118));
            let t122 = (v_rho1).simd_le(dens_threshold);
            let t123 = -t16;
            let t125 = ((t14).select(t11, (t10).select(t15, t123 * t7)));
            let t126 = f64x8::splat(1.0) + t125;
            let t127 = (t126).simd_le(zeta_threshold);
            let t128 = (simd::cbrt(t126));
            let t130 = ((t127).select(t22, t128 * t126));
            let t131 = t5 * t130;
            let t132 = v_sigma2 * v_sigma2;
            let t133 = param_b * t132;
            let t134 = v_rho1 * v_rho1;
            let t135 = t134 * t134;
            let t136 = t135 * v_rho1;
            let t137 = (simd::cbrt(v_rho1));
            let t139 = f64x8::splat(1.0) / t137 / t136;
            let t140 = t137 * t137;
            let t145 = f64x8::splat(1.0) + f64x8::splat(6.0) * v_sigma2 / t140 / t134;
            let t146 = t145 * t145;
            let t147 = f64x8::splat(1.0) / t146;
            let t148 = t139 * t147;
            let t151 = param_a + f64x8::splat(36.0) * t133 * t148;
            let t154 = t29 * t34 / t151;
            let t155 = ((t154).sqrt());
            let t157 = param_hyb_omega_0 / t155;
            let t158 = t126 * t6;
            let t159 = (simd::cbrt(t158));
            let t160 = f64x8::splat(1.0) / t159;
            let t161 = t61 * t160;
            let t163 = t157 * t161 / f64x8::splat(2.0);
            let t164 = (f64x8::splat(1.35)).simd_le(t163);
            let t165 = (f64x8::splat(1.35)).simd_lt(t163);
            let t166 = ((t165).select(t163, f64x8::splat(1.35)));
            let t167 = t166 * t166;
            let t170 = t167 * t167;
            let t171 = f64x8::splat(1.0) / t170;
            let t173 = t170 * t167;
            let t174 = f64x8::splat(1.0) / t173;
            let t176 = t170 * t170;
            let t177 = f64x8::splat(1.0) / t176;
            let t180 = f64x8::splat(1.0) / t176 / t167;
            let t183 = f64x8::splat(1.0) / t176 / t170;
            let t186 = f64x8::splat(1.0) / t176 / t173;
            let t188 = t176 * t176;
            let t189 = f64x8::splat(1.0) / t188;
            let t192 = ((t165).select(f64x8::splat(1.35), t163));
            let t193 = f64x8::splat(1.0) / t192;
            let t195 = (simd::erf(t193 / f64x8::splat(2.0)));
            let t197 = t192 * t192;
            let t198 = f64x8::splat(1.0) / t197;
            let t200 = (simd::exp(-t198 / f64x8::splat(4.0)));
            let t201 = t200 - f64x8::splat(1.0);
            let t204 = t200 - f64x8::splat(3.0) / f64x8::splat(2.0) - f64x8::splat(2.0) * t197 * t201;
            let t207 = f64x8::splat(2.0) * t192 * t204 + t97 * t195;
            let t211 = ((t164).select(f64x8::splat(1.0) / t167 / f64x8::splat(36.0) - t171 / f64x8::splat(960.0) + t174 / f64x8::splat(26880.0) - t177 / f64x8::splat(829440.0) + t180 / f64x8::splat(28385280.0) - t183 / f64x8::splat(1073479680.0) + t186 / f64x8::splat(44590694400.0) - t189 / f64x8::splat(2021444812800.0), f64x8::splat(1.0) - f64x8::splat(8.0) / f64x8::splat(3.0) * t192 * t207));
            let t212 = t27 * t211;
            let t213 = t212 * t151;
            let t216 = ((t122).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t131 * t213));
            let tzk0 = t121 + t216;
            acc_zk = tzk0;
            let t217 = t6 * t6;
            let t218 = f64x8::splat(1.0) / t217;
            let t219 = t16 * t218;
            let t221 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), t7 - t219)));
            let t224 = ((t20).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t221));
            let t225 = t5 * t224;
            let t228 = t27 * t27;
            let t229 = f64x8::splat(1.0) / t228;
            let t230 = t229 * t116;
            let t231 = t230 * t54;
            let t233 = t26 * t231 / f64x8::splat(8.0);
            let t234 = t71 * t70;
            let t235 = f64x8::splat(1.0) / t234;
            let t238 = param_hyb_omega_0 / t58 / t57;
            let t240 = t238 * t65 * f64x8::splat(M_PI);
            let t241 = t28 * t32;
            let t242 = t54 * t54;
            let t243 = f64x8::splat(1.0) / t242;
            let t244 = t33 * t243;
            let t245 = t38 * t37;
            let t247 = f64x8::splat(1.0) / t40 / t245;
            let t248 = t247 * t50;
            let t252 = param_b * t35 * v_sigma0;
            let t253 = t38 * t38;
            let t254 = t253 * v_rho0;
            let t255 = f64x8::splat(1.0) / t254;
            let t257 = f64x8::splat(1.0) / t49 / t48;
            let t258 = t255 * t257;
            let t261 = -f64x8::splat(192.0) * t36 * t248 + f64x8::splat(1152.0) * t252 * t258;
            let t262 = t244 * t261;
            let t267 = f64x8::splat(1.0) / t63 / t62;
            let t268 = t61 * t267;
            let t270 = t221 * t6 + t18 + f64x8::splat(1.0);
            let t274 = t240 * t241 * t262 / f64x8::splat(4.0) - t60 * t268 * t270 / f64x8::splat(6.0);
            let t275 = ((t69).select(t274, f64x8::splat(0.0)));
            let t278 = t74 * t70;
            let t279 = f64x8::splat(1.0) / t278;
            let t282 = t74 * t234;
            let t283 = f64x8::splat(1.0) / t282;
            let t287 = f64x8::splat(1.0) / t80 / t70;
            let t291 = f64x8::splat(1.0) / t80 / t234;
            let t295 = f64x8::splat(1.0) / t80 / t278;
            let t299 = f64x8::splat(1.0) / t80 / t282;
            let t303 = f64x8::splat(1.0) / t92 / t70;
            let t307 = ((t69).select(f64x8::splat(0.0), t274));
            let t309 = t105 * t103;
            let t313 = t102 * t96;
            let t314 = f64x8::splat(1.0) / t313;
            let t318 = t96 * t106;
            let t323 = t314 * t307 * t105 / f64x8::splat(2.0) - f64x8::splat(4.0) * t318 * t307 - t98 * t307 * t105;
            let t326 = f64x8::splat(2.0) * t307 * t109 - t309 * t307 + f64x8::splat(2.0) * t96 * t323;
            let t330 = ((t68).select(-t235 * t275 / f64x8::splat(18.0) + t279 * t275 / f64x8::splat(240.0) - t283 * t275 / f64x8::splat(4480.0) + t287 * t275 / f64x8::splat(103680.0) - t291 * t275 / f64x8::splat(2838528.0) + t295 * t275 / f64x8::splat(89456640.0) - t299 * t275 / f64x8::splat(3185049600.0) + t303 * t275 / f64x8::splat(126340300800.0), -f64x8::splat(8.0) / f64x8::splat(3.0) * t307 * t112 - f64x8::splat(8.0) / f64x8::splat(3.0) * t96 * t326));
            let t331 = t27 * t330;
            let t332 = t331 * t54;
            let t335 = t117 * t261;
            let t339 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t225 * t118 - t233 - f64x8::splat(3.0) / f64x8::splat(8.0) * t26 * t332 - f64x8::splat(3.0) / f64x8::splat(8.0) * t26 * t335));
            let t340 = t123 * t218;
            let t342 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), -t7 - t340)));
            let t345 = ((t127).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t128 * t342));
            let t346 = t5 * t345;
            let t349 = t229 * t211;
            let t350 = t349 * t151;
            let t352 = t131 * t350 / f64x8::splat(8.0);
            let t353 = t167 * t166;
            let t354 = f64x8::splat(1.0) / t353;
            let t356 = f64x8::splat(1.0) / t159 / t158;
            let t357 = t61 * t356;
            let t359 = t342 * t6 + t125 + f64x8::splat(1.0);
            let t362 = t157 * t357 * t359 / f64x8::splat(6.0);
            let t363 = ((t165).select(-t362, f64x8::splat(0.0)));
            let t366 = t170 * t166;
            let t367 = f64x8::splat(1.0) / t366;
            let t370 = t170 * t353;
            let t371 = f64x8::splat(1.0) / t370;
            let t375 = f64x8::splat(1.0) / t176 / t166;
            let t379 = f64x8::splat(1.0) / t176 / t353;
            let t383 = f64x8::splat(1.0) / t176 / t366;
            let t387 = f64x8::splat(1.0) / t176 / t370;
            let t391 = f64x8::splat(1.0) / t188 / t166;
            let t395 = ((t165).select(f64x8::splat(0.0), -t362));
            let t397 = t200 * t198;
            let t401 = t197 * t192;
            let t402 = f64x8::splat(1.0) / t401;
            let t406 = t192 * t201;
            let t411 = t402 * t395 * t200 / f64x8::splat(2.0) - f64x8::splat(4.0) * t406 * t395 - t193 * t395 * t200;
            let t414 = f64x8::splat(2.0) * t192 * t411 + f64x8::splat(2.0) * t395 * t204 - t397 * t395;
            let t418 = ((t164).select(-t354 * t363 / f64x8::splat(18.0) + t367 * t363 / f64x8::splat(240.0) - t371 * t363 / f64x8::splat(4480.0) + t375 * t363 / f64x8::splat(103680.0) - t379 * t363 / f64x8::splat(2838528.0) + t383 * t363 / f64x8::splat(89456640.0) - t387 * t363 / f64x8::splat(3185049600.0) + t391 * t363 / f64x8::splat(126340300800.0), -f64x8::splat(8.0) / f64x8::splat(3.0) * t192 * t414 - f64x8::splat(8.0) / f64x8::splat(3.0) * t395 * t207));
            let t419 = t27 * t418;
            let t420 = t419 * t151;
            let t424 = ((t122).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t346 * t213 - t352 - f64x8::splat(3.0) / f64x8::splat(8.0) * t131 * t420));
            let tvrho0 = t121 + t216 + t6 * (t339 + t424);
            acc_vrho_0 = tvrho0;
            let t428 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), -t7 - t219)));
            let t431 = ((t20).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t428));
            let t432 = t5 * t431;
            let t436 = t428 * t6 + t18 + f64x8::splat(1.0);
            let t437 = t268 * t436;
            let t439 = t60 * t437 / f64x8::splat(6.0);
            let t440 = ((t69).select(-t439, f64x8::splat(0.0)));
            let t443 = t279 * t440;
            let t445 = t283 * t440;
            let t447 = t287 * t440;
            let t449 = t291 * t440;
            let t451 = t295 * t440;
            let t453 = t299 * t440;
            let t455 = t303 * t440;
            let t458 = ((t69).select(f64x8::splat(0.0), -t439));
            let t470 = t314 * t458 * t105 / f64x8::splat(2.0) - f64x8::splat(4.0) * t318 * t458 - t98 * t458 * t105;
            let t473 = f64x8::splat(2.0) * t458 * t109 - t309 * t458 + f64x8::splat(2.0) * t96 * t470;
            let t477 = ((t68).select(-t235 * t440 / f64x8::splat(18.0) + t443 / f64x8::splat(240.0) - t445 / f64x8::splat(4480.0) + t447 / f64x8::splat(103680.0) - t449 / f64x8::splat(2838528.0) + t451 / f64x8::splat(89456640.0) - t453 / f64x8::splat(3185049600.0) + t455 / f64x8::splat(126340300800.0), -f64x8::splat(8.0) / f64x8::splat(3.0) * t458 * t112 - f64x8::splat(8.0) / f64x8::splat(3.0) * t96 * t473));
            let t478 = t27 * t477;
            let t479 = t478 * t54;
            let t483 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t432 * t118 - t233 - f64x8::splat(3.0) / f64x8::splat(8.0) * t26 * t479));
            let t485 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), t7 - t340)));
            let t488 = ((t127).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t128 * t485));
            let t489 = t5 * t488;
            let t494 = param_hyb_omega_0 / t155 / t154;
            let t496 = t494 * t161 * f64x8::splat(M_PI);
            let t497 = t151 * t151;
            let t498 = f64x8::splat(1.0) / t497;
            let t499 = t33 * t498;
            let t500 = t135 * t134;
            let t502 = f64x8::splat(1.0) / t137 / t500;
            let t503 = t502 * t147;
            let t507 = param_b * t132 * v_sigma2;
            let t508 = t135 * t135;
            let t509 = t508 * v_rho1;
            let t510 = f64x8::splat(1.0) / t509;
            let t512 = f64x8::splat(1.0) / t146 / t145;
            let t513 = t510 * t512;
            let t516 = -f64x8::splat(192.0) * t133 * t503 + f64x8::splat(1152.0) * t507 * t513;
            let t522 = t485 * t6 + t125 + f64x8::splat(1.0);
            let t526 = t496 * t241 * t499 * t516 / f64x8::splat(4.0) - t157 * t357 * t522 / f64x8::splat(6.0);
            let t527 = ((t165).select(t526, f64x8::splat(0.0)));
            let t530 = t367 * t527;
            let t532 = t371 * t527;
            let t534 = t375 * t527;
            let t536 = t379 * t527;
            let t538 = t383 * t527;
            let t540 = t387 * t527;
            let t542 = t391 * t527;
            let t545 = ((t165).select(f64x8::splat(0.0), t526));
            let t557 = t402 * t545 * t200 / f64x8::splat(2.0) - f64x8::splat(4.0) * t406 * t545 - t193 * t545 * t200;
            let t560 = f64x8::splat(2.0) * t192 * t557 + f64x8::splat(2.0) * t545 * t204 - t397 * t545;
            let t564 = ((t164).select(-t354 * t527 / f64x8::splat(18.0) + t530 / f64x8::splat(240.0) - t532 / f64x8::splat(4480.0) + t534 / f64x8::splat(103680.0) - t536 / f64x8::splat(2838528.0) + t538 / f64x8::splat(89456640.0) - t540 / f64x8::splat(3185049600.0) + t542 / f64x8::splat(126340300800.0), -f64x8::splat(8.0) / f64x8::splat(3.0) * t192 * t560 - f64x8::splat(8.0) / f64x8::splat(3.0) * t545 * t207));
            let t565 = t27 * t564;
            let t566 = t565 * t151;
            let t569 = t212 * t516;
            let t573 = ((t122).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t489 * t213 - t352 - f64x8::splat(3.0) / f64x8::splat(8.0) * t131 * t566 - f64x8::splat(3.0) / f64x8::splat(8.0) * t131 * t569));
            let tvrho1 = t121 + t216 + t6 * (t483 + t573);
            acc_vrho_1 = tvrho1;
            let t576 = param_b * v_sigma0;
            let t579 = f64x8::splat(1.0) / t253;
            let t580 = t579 * t257;
            let t583 = f64x8::splat(72.0) * t576 * t51 - f64x8::splat(432.0) * t36 * t580;
            let t587 = t240 * t241 * t244 * t583 / f64x8::splat(4.0);
            let t588 = ((t69).select(t587, f64x8::splat(0.0)));
            let t591 = t279 * t588;
            let t593 = t283 * t588;
            let t595 = t287 * t588;
            let t597 = t291 * t588;
            let t599 = t295 * t588;
            let t601 = t299 * t588;
            let t603 = t303 * t588;
            let t606 = ((t69).select(f64x8::splat(0.0), t587));
            let t618 = t314 * t606 * t105 / f64x8::splat(2.0) - f64x8::splat(4.0) * t318 * t606 - t98 * t606 * t105;
            let t621 = f64x8::splat(2.0) * t606 * t109 - t309 * t606 + f64x8::splat(2.0) * t96 * t618;
            let t625 = ((t68).select(-t235 * t588 / f64x8::splat(18.0) + t591 / f64x8::splat(240.0) - t593 / f64x8::splat(4480.0) + t595 / f64x8::splat(103680.0) - t597 / f64x8::splat(2838528.0) + t599 / f64x8::splat(89456640.0) - t601 / f64x8::splat(3185049600.0) + t603 / f64x8::splat(126340300800.0), -f64x8::splat(8.0) / f64x8::splat(3.0) * t606 * t112 - f64x8::splat(8.0) / f64x8::splat(3.0) * t96 * t621));
            let t626 = t27 * t625;
            let t627 = t626 * t54;
            let t629 = t117 * t583;
            let t633 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t26 * t627 - f64x8::splat(3.0) / f64x8::splat(8.0) * t26 * t629));
            let tvsigma0 = t6 * t633;
            acc_vsigma_0 = tvsigma0;
            let tvsigma1 = f64x8::splat(0.0);
            acc_vsigma_1 = tvsigma1;
            let t634 = param_b * v_sigma2;
            let t637 = f64x8::splat(1.0) / t508;
            let t638 = t637 * t512;
            let t641 = f64x8::splat(72.0) * t634 * t148 - f64x8::splat(432.0) * t133 * t638;
            let t645 = t496 * t241 * t499 * t641 / f64x8::splat(4.0);
            let t646 = ((t165).select(t645, f64x8::splat(0.0)));
            let t649 = t367 * t646;
            let t651 = t371 * t646;
            let t653 = t375 * t646;
            let t655 = t379 * t646;
            let t657 = t383 * t646;
            let t659 = t387 * t646;
            let t661 = t391 * t646;
            let t664 = ((t165).select(f64x8::splat(0.0), t645));
            let t676 = t402 * t664 * t200 / f64x8::splat(2.0) - f64x8::splat(4.0) * t406 * t664 - t193 * t664 * t200;
            let t679 = f64x8::splat(2.0) * t192 * t676 + f64x8::splat(2.0) * t664 * t204 - t397 * t664;
            let t683 = ((t164).select(-t354 * t646 / f64x8::splat(18.0) + t649 / f64x8::splat(240.0) - t651 / f64x8::splat(4480.0) + t653 / f64x8::splat(103680.0) - t655 / f64x8::splat(2838528.0) + t657 / f64x8::splat(89456640.0) - t659 / f64x8::splat(3185049600.0) + t661 / f64x8::splat(126340300800.0), -f64x8::splat(8.0) / f64x8::splat(3.0) * t192 * t679 - f64x8::splat(8.0) / f64x8::splat(3.0) * t664 * t207));
            let t684 = t27 * t683;
            let t685 = t684 * t151;
            let t687 = t212 * t641;
            let t691 = ((t122).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t131 * t685 - f64x8::splat(3.0) / f64x8::splat(8.0) * t131 * t687));
            let tvsigma2 = t6 * t691;
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
