//! GGA_C_OP_PW91 vxc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_op_pw91.c`
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
pub fn gga_c_op_pw91_vxc_pol(
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
            let t1 = v_rho0 - v_rho1;
            let t2 = v_rho0 + v_rho1;
            let t3 = f64x8::splat(1.0) / t2;
            let t4 = t1 * t3;
            let t5 = ((t4).abs());
            let t11 = ((f64x8::splat(1.0) - t5).simd_le(zeta_threshold)) | (((v_rho0).simd_le(dens_threshold)) & ((v_rho1).simd_le(dens_threshold)));
            let t13 = (f64x8::splat(1.0) + t4).simd_le(zeta_threshold);
            let t14 = zeta_threshold - f64x8::splat(1.0);
            let t16 = (f64x8::splat(1.0) - t4).simd_le(zeta_threshold);
            let t17 = -t14;
            let t18 = ((t13).select(t14, (t16).select(t17, t4)));
            let t19 = t18 * t18;
            let t20 = f64x8::splat(1.0) - t19;
            let t21 = t20 * t2;
            let t24 = (f64x8::splat(2.0) * v_rho0 * t3).simd_le(zeta_threshold);
            let t27 = (f64x8::splat(2.0) * v_rho1 * t3).simd_le(zeta_threshold);
            let t28 = ((t24).select(t14, (t27).select(t17, t4)));
            let t29 = f64x8::splat(1.0) + t28;
            let t32 = (t29 * t2 / f64x8::splat(2.0)).simd_le(dens_threshold);
            let t33 = f64x8::splat(M_CBRT3);
            let t34 = t33 * t33;
            let t36 = (simd::cbrt(f64x8::splat(1.0) / f64x8::splat(M_PI)));
            let t38 = t34 / t36;
            let t39 = f64x8::splat(M_CBRT4);
            let t40 = t38 * t39;
            let t41 = f64x8::splat(M_CBRT2);
            let t42 = (t29).simd_le(zeta_threshold);
            let t43 = f64x8::splat(1.0) - t28;
            let t44 = (t43).simd_le(zeta_threshold);
            let t45 = ((t42).select(t14, (t44).select(t17, t28)));
            let t46 = f64x8::splat(1.0) + t45;
            let t47 = t46 * t2;
            let t48 = (simd::cbrt(t47));
            let t49 = f64x8::splat(1.0) / t48;
            let t50 = t41 * t49;
            let t51 = f64x8::splat(M_CBRT6);
            let t52 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t53 = (simd::cbrt(t52));
            let t54 = t53 * t53;
            let t55 = f64x8::splat(1.0) / t54;
            let t56 = t51 * t55;
            let t57 = v_rho0 * v_rho0;
            let t58 = (simd::cbrt(v_rho0));
            let t59 = t58 * t58;
            let t61 = f64x8::splat(1.0) / t59 / t57;
            let t63 = t56 * v_sigma0 * t61;
            let t65 = (simd::exp(-f64x8::splat(25.0) / f64x8::splat(6.0) * t63));
            let t68 = (f64x8::splat(0.2743) - f64x8::splat(0.1508) * t65) * t51;
            let t69 = t55 * v_sigma0;
            let t73 = t51 * t51;
            let t75 = f64x8::splat(1.0) / t53 / t52;
            let t76 = t73 * t75;
            let t77 = v_sigma0 * v_sigma0;
            let t78 = t57 * t57;
            let t79 = t78 * v_rho0;
            let t81 = f64x8::splat(1.0) / t58 / t79;
            let t84 = f64x8::splat(6.944444444444445e-06) * t76 * t77 * t81;
            let t85 = t68 * t69 * t61 / f64x8::splat(24.0) - t84;
            let t87 = t73 / t53;
            let t88 = ((v_sigma0).sqrt());
            let t90 = f64x8::splat(1.0) / t58 / v_rho0;
            let t91 = t88 * t90;
            let t94 = (simd::ln(f64x8::splat(0.6496333333333333) * t87 * t91 + ((((f64x8::splat(0.6496333333333333) * t87 * t91) * (f64x8::splat(0.6496333333333333) * t87 * t91)) + f64x8::splat(1.0)).sqrt())));
            let t98 = f64x8::splat(1.0) + f64x8::splat(0.016370833333333334) * t87 * t91 * t94 + t84;
            let t99 = f64x8::splat(1.0) / t98;
            let t101 = t85 * t99 + f64x8::splat(1.0);
            let t102 = f64x8::splat(1.0) / t101;
            let t106 = ((t32).select(f64x8::splat(0.0), t40 * t50 * t102 / f64x8::splat(9.0)));
            let t110 = (t43 * t2 / f64x8::splat(2.0)).simd_le(dens_threshold);
            let t111 = ((t44).select(t14, (t42).select(t17, -t28)));
            let t112 = f64x8::splat(1.0) + t111;
            let t113 = t112 * t2;
            let t114 = (simd::cbrt(t113));
            let t115 = f64x8::splat(1.0) / t114;
            let t116 = t41 * t115;
            let t117 = v_rho1 * v_rho1;
            let t118 = (simd::cbrt(v_rho1));
            let t119 = t118 * t118;
            let t121 = f64x8::splat(1.0) / t119 / t117;
            let t123 = t56 * v_sigma2 * t121;
            let t125 = (simd::exp(-f64x8::splat(25.0) / f64x8::splat(6.0) * t123));
            let t128 = (f64x8::splat(0.2743) - f64x8::splat(0.1508) * t125) * t51;
            let t129 = t55 * v_sigma2;
            let t133 = v_sigma2 * v_sigma2;
            let t134 = t117 * t117;
            let t135 = t134 * v_rho1;
            let t137 = f64x8::splat(1.0) / t118 / t135;
            let t140 = f64x8::splat(6.944444444444445e-06) * t76 * t133 * t137;
            let t141 = t128 * t129 * t121 / f64x8::splat(24.0) - t140;
            let t142 = ((v_sigma2).sqrt());
            let t144 = f64x8::splat(1.0) / t118 / v_rho1;
            let t145 = t142 * t144;
            let t148 = (simd::ln(f64x8::splat(0.6496333333333333) * t87 * t145 + ((((f64x8::splat(0.6496333333333333) * t87 * t145) * (f64x8::splat(0.6496333333333333) * t87 * t145)) + f64x8::splat(1.0)).sqrt())));
            let t152 = f64x8::splat(1.0) + f64x8::splat(0.016370833333333334) * t87 * t145 * t148 + t140;
            let t153 = f64x8::splat(1.0) / t152;
            let t155 = t141 * t153 + f64x8::splat(1.0);
            let t156 = f64x8::splat(1.0) / t155;
            let t160 = ((t110).select(f64x8::splat(0.0), t40 * t116 * t156 / f64x8::splat(9.0)));
            let t161 = t106 + t160;
            let t162 = (t161).simd_eq(f64x8::splat(0.0));
            let t163 = ((t162).select(f64x8::splat(f64::EPSILON), t161));
            let t166 = f64x8::splat(3.60663084) / t163 + f64x8::splat(0.5764);
            let t167 = t163 * t163;
            let t168 = t167 * t167;
            let t169 = f64x8::splat(1.0) / t168;
            let t171 = t167 * t163;
            let t172 = f64x8::splat(1.0) / t171;
            let t174 = f64x8::splat(1.0) / t167;
            let t176 = f64x8::splat(31.58152667175181) * t169 + f64x8::splat(15.032732091624375) * t172 + f64x8::splat(1.788764629788) * t174;
            let t177 = f64x8::splat(1.0) / t176;
            let t178 = t166 * t177;
            let tzk0 = ((t11).select(f64x8::splat(0.0), -f64x8::splat(0.25) * t21 * t178));
            acc_zk = tzk0;
            let t181 = t2 * t2;
            let t182 = f64x8::splat(1.0) / t181;
            let t183 = t1 * t182;
            let t184 = t3 - t183;
            let t185 = ((t13).select(f64x8::splat(0.0), (t16).select(f64x8::splat(0.0), t184)));
            let t186 = t18 * t185;
            let t187 = t2 * t166;
            let t188 = t187 * t177;
            let t191 = t20 * t166;
            let t193 = f64x8::splat(0.25) * t191 * t177;
            let t195 = f64x8::splat(1.0) / t48 / t47;
            let t196 = t41 * t195;
            let t197 = ((t24).select(f64x8::splat(0.0), (t27).select(f64x8::splat(0.0), t184)));
            let t198 = ((t42).select(f64x8::splat(0.0), (t44).select(f64x8::splat(0.0), t197)));
            let t200 = t198 * t2 + t45 + f64x8::splat(1.0);
            let t205 = t101 * t101;
            let t206 = f64x8::splat(1.0) / t205;
            let t207 = t78 * t57;
            let t209 = f64x8::splat(1.0) / t58 / t207;
            let t210 = t77 * t209;
            let t214 = t57 * v_rho0;
            let t216 = f64x8::splat(1.0) / t59 / t214;
            let t221 = f64x8::splat(3.7037037037037037e-05) * t76 * t210;
            let t222 = -f64x8::splat(0.06981481481481482) * t76 * t210 * t65 - t68 * t69 * t216 / f64x8::splat(9.0) + t221;
            let t224 = t98 * t98;
            let t225 = f64x8::splat(1.0) / t224;
            let t226 = t85 * t225;
            let t228 = f64x8::splat(1.0) / t58 / t57;
            let t235 = f64x8::splat(2.532140806666667) * t63 + f64x8::splat(1.0);
            let t236 = ((t235).sqrt());
            let t237 = f64x8::splat(1.0) / t236;
            let t241 = -f64x8::splat(0.02182777777777778) * t87 * t88 * t228 * t94 - f64x8::splat(0.08508031222222222) * t56 * v_sigma0 * t216 * t237 - t221;
            let t243 = t222 * t99 - t226 * t241;
            let t249 = ((t32).select(f64x8::splat(0.0), -t40 * t196 * t102 * t200 / f64x8::splat(27.0) - t40 * t50 * t206 * t243 / f64x8::splat(9.0)));
            let t251 = f64x8::splat(1.0) / t114 / t113;
            let t252 = t41 * t251;
            let t253 = ((t44).select(f64x8::splat(0.0), (t42).select(f64x8::splat(0.0), -t197)));
            let t255 = t2 * t253 + t111 + f64x8::splat(1.0);
            let t260 = ((t110).select(f64x8::splat(0.0), -t40 * t252 * t156 * t255 / f64x8::splat(27.0)));
            let t262 = ((t162).select(f64x8::splat(0.0), t249 + t260));
            let t263 = t174 * t262;
            let t264 = t263 * t177;
            let t267 = t176 * t176;
            let t268 = f64x8::splat(1.0) / t267;
            let t269 = t166 * t268;
            let t271 = f64x8::splat(1.0) / t168 / t163;
            let t272 = t271 * t262;
            let t274 = t169 * t262;
            let t276 = t172 * t262;
            let t278 = -f64x8::splat(126.32610668700724) * t272 - f64x8::splat(45.098196274873125) * t274 - f64x8::splat(3.577529259576) * t276;
            let t279 = t269 * t278;
            let t283 = ((t11).select(f64x8::splat(0.0), f64x8::splat(0.5) * t186 * t188 - t193 + f64x8::splat(0.90165771) * t21 * t264 + f64x8::splat(0.25) * t21 * t279));
            let tvrho0 = t2 * t283 + tzk0;
            acc_vrho_0 = tvrho0;
            let t285 = -t3 - t183;
            let t286 = ((t13).select(f64x8::splat(0.0), (t16).select(f64x8::splat(0.0), t285)));
            let t287 = t18 * t286;
            let t290 = ((t24).select(f64x8::splat(0.0), (t27).select(f64x8::splat(0.0), t285)));
            let t291 = ((t42).select(f64x8::splat(0.0), (t44).select(f64x8::splat(0.0), t290)));
            let t293 = t2 * t291 + t45 + f64x8::splat(1.0);
            let t298 = ((t32).select(f64x8::splat(0.0), -t40 * t196 * t102 * t293 / f64x8::splat(27.0)));
            let t299 = ((t44).select(f64x8::splat(0.0), (t42).select(f64x8::splat(0.0), -t290)));
            let t301 = t2 * t299 + t111 + f64x8::splat(1.0);
            let t306 = t155 * t155;
            let t307 = f64x8::splat(1.0) / t306;
            let t308 = t134 * t117;
            let t310 = f64x8::splat(1.0) / t118 / t308;
            let t311 = t133 * t310;
            let t315 = t117 * v_rho1;
            let t317 = f64x8::splat(1.0) / t119 / t315;
            let t322 = f64x8::splat(3.7037037037037037e-05) * t76 * t311;
            let t323 = -f64x8::splat(0.06981481481481482) * t76 * t311 * t125 - t128 * t129 * t317 / f64x8::splat(9.0) + t322;
            let t325 = t152 * t152;
            let t326 = f64x8::splat(1.0) / t325;
            let t327 = t141 * t326;
            let t329 = f64x8::splat(1.0) / t118 / t117;
            let t336 = f64x8::splat(2.532140806666667) * t123 + f64x8::splat(1.0);
            let t337 = ((t336).sqrt());
            let t338 = f64x8::splat(1.0) / t337;
            let t342 = -f64x8::splat(0.02182777777777778) * t87 * t142 * t329 * t148 - f64x8::splat(0.08508031222222222) * t56 * v_sigma2 * t317 * t338 - t322;
            let t344 = t153 * t323 - t327 * t342;
            let t350 = ((t110).select(f64x8::splat(0.0), -t40 * t252 * t156 * t301 / f64x8::splat(27.0) - t40 * t116 * t307 * t344 / f64x8::splat(9.0)));
            let t352 = ((t162).select(f64x8::splat(0.0), t298 + t350));
            let t353 = t174 * t352;
            let t354 = t353 * t177;
            let t357 = t271 * t352;
            let t359 = t169 * t352;
            let t361 = t172 * t352;
            let t363 = -f64x8::splat(126.32610668700724) * t357 - f64x8::splat(45.098196274873125) * t359 - f64x8::splat(3.577529259576) * t361;
            let t364 = t269 * t363;
            let t368 = ((t11).select(f64x8::splat(0.0), f64x8::splat(0.5) * t287 * t188 - t193 + f64x8::splat(0.90165771) * t21 * t354 + f64x8::splat(0.25) * t21 * t364));
            let tvrho1 = t2 * t368 + tzk0;
            acc_vrho_1 = tvrho1;
            let t370 = t81 * t65;
            let t379 = f64x8::splat(1.388888888888889e-05) * t76 * v_sigma0 * t81;
            let t380 = f64x8::splat(0.026180555555555554) * t76 * t370 * v_sigma0 + t68 * t55 * t61 / f64x8::splat(24.0) - t379;
            let t382 = f64x8::splat(1.0) / t88;
            let t390 = f64x8::splat(0.008185416666666667) * t87 * t382 * t90 * t94 + f64x8::splat(0.03190511708333333) * t56 * t61 * t237 + t379;
            let t392 = -t226 * t390 + t380 * t99;
            let t397 = ((t32).select(f64x8::splat(0.0), -t40 * t50 * t206 * t392 / f64x8::splat(9.0)));
            let t398 = ((t162).select(f64x8::splat(0.0), t397));
            let t399 = t174 * t398;
            let t400 = t399 * t177;
            let t403 = t271 * t398;
            let t405 = t169 * t398;
            let t407 = t172 * t398;
            let t409 = -f64x8::splat(126.32610668700724) * t403 - f64x8::splat(45.098196274873125) * t405 - f64x8::splat(3.577529259576) * t407;
            let t410 = t269 * t409;
            let t414 = ((t11).select(f64x8::splat(0.0), f64x8::splat(0.90165771) * t21 * t400 + f64x8::splat(0.25) * t21 * t410));
            let tvsigma0 = t2 * t414;
            acc_vsigma_0 = tvsigma0;
            let tvsigma1 = f64x8::splat(0.0);
            acc_vsigma_1 = tvsigma1;
            let t415 = t137 * t125;
            let t424 = f64x8::splat(1.388888888888889e-05) * t76 * v_sigma2 * t137;
            let t425 = f64x8::splat(0.026180555555555554) * t76 * t415 * v_sigma2 + t128 * t55 * t121 / f64x8::splat(24.0) - t424;
            let t427 = f64x8::splat(1.0) / t142;
            let t435 = f64x8::splat(0.008185416666666667) * t87 * t427 * t144 * t148 + f64x8::splat(0.03190511708333333) * t56 * t121 * t338 + t424;
            let t437 = t153 * t425 - t327 * t435;
            let t442 = ((t110).select(f64x8::splat(0.0), -t40 * t116 * t307 * t437 / f64x8::splat(9.0)));
            let t443 = ((t162).select(f64x8::splat(0.0), t442));
            let t444 = t174 * t443;
            let t445 = t444 * t177;
            let t448 = t271 * t443;
            let t450 = t169 * t443;
            let t452 = t172 * t443;
            let t454 = -f64x8::splat(126.32610668700724) * t448 - f64x8::splat(45.098196274873125) * t450 - f64x8::splat(3.577529259576) * t452;
            let t455 = t269 * t454;
            let t459 = ((t11).select(f64x8::splat(0.0), f64x8::splat(0.90165771) * t21 * t445 + f64x8::splat(0.25) * t21 * t455));
            let tvsigma2 = t2 * t459;
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
