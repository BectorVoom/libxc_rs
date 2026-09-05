//! MGGA_X_PBE_GX vxc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_pbe_gx.c`
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
pub fn mgga_x_pbe_gx_vxc_pol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    vlapl: &mut [f64],
    vtau: &mut [f64],
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
        let v_lapl0 = load_strided(lapl, ip, np, 2, 0);
        let v_lapl1 = load_strided(lapl, ip, np, 2, 1);
        let v_tau0 = load_strided(tau, ip, np, 2, 0);
        let v_tau1 = load_strided(tau, ip, np, 2, 1);
        let mut acc_zk = V_ZERO;
        let mut acc_vrho_0 = V_ZERO;
        let mut acc_vrho_1 = V_ZERO;
        let mut acc_vsigma_0 = V_ZERO;
        let mut acc_vsigma_1 = V_ZERO;
        let mut acc_vsigma_2 = V_ZERO;
        let mut acc_vlapl_0 = V_ZERO;
        let mut acc_vlapl_1 = V_ZERO;
        let mut acc_vtau_0 = V_ZERO;
        let mut acc_vtau_1 = V_ZERO;
        {
            let t2 = (v_rho0).simd_le(dens_threshold);
            let t3 = f64x8::splat(M_CBRT3);
            let t4 = f64x8::splat(M_CBRTPI);
            let t6 = t3 / t4;
            let t7 = v_rho0 + v_rho1;
            let t8 = f64x8::splat(1.0) / t7;
            let t11 = (f64x8::splat(2.0) * v_rho0 * t8).simd_le(zeta_threshold);
            let t12 = zeta_threshold - f64x8::splat(1.0);
            let t15 = (f64x8::splat(2.0) * v_rho1 * t8).simd_le(zeta_threshold);
            let t16 = -t12;
            let t17 = v_rho0 - v_rho1;
            let t19 = ((t11).select(t12, (t15).select(t16, t17 * t8)));
            let t20 = f64x8::splat(1.0) + t19;
            let t21 = (t20).simd_le(zeta_threshold);
            let t22 = (simd::cbrt(zeta_threshold));
            let t23 = t22 * zeta_threshold;
            let t24 = (simd::cbrt(t20));
            let t26 = ((t21).select(t23, t24 * t20));
            let t27 = t6 * t26;
            let t28 = (simd::cbrt(t7));
            let t29 = f64x8::splat(M_CBRT2);
            let t30 = t3 * t3;
            let t32 = f64x8::splat(M_CBRT4);
            let t34 = f64x8::splat(8.0) / f64x8::splat(27.0) * t29 * t30 * t32;
            let t35 = (simd::cbrt(v_rho0));
            let t36 = t35 * t35;
            let t38 = f64x8::splat(1.0) / t36 / v_rho0;
            let t40 = v_rho0 * v_rho0;
            let t42 = f64x8::splat(1.0) / t36 / t40;
            let t43 = v_sigma0 * t42;
            let t45 = v_tau0 * t38 - t43 / f64x8::splat(8.0);
            let t46 = f64x8::splat(M_CBRT6);
            let t48 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t49 = (simd::cbrt(t48));
            let t50 = t49 * t49;
            let t51 = f64x8::splat(1.0) / t50;
            let t52 = t45 * t46 * t51;
            let t54 = f64x8::splat(0.827411) - f64x8::splat(0.3575333333333333) * t52;
            let t56 = f64x8::splat(1.0) - f64x8::splat(0.45341611111111113) * t52;
            let t57 = f64x8::splat(1.0) / t56;
            let t59 = f64x8::splat(1.0) - t34;
            let t60 = t54 * t57 * t59;
            let t63 = t34 + f64x8::splat(5.0) / f64x8::splat(9.0) * t52 * t60;
            let t64 = f64x8::splat(5.0) / f64x8::splat(9.0) * t52;
            let t65 = f64x8::splat(1.0) - t64;
            let t66 = ((t65).simd_ge(V_ZERO).select(V_ONE, V_ZERO));
            let t68 = f64x8::splat(1.0) + t64;
            let t69 = f64x8::splat(1.0) / t68;
            let t72 = f64x8::splat(1.0) + f64x8::splat(0.148) * t65 * t69;
            let t73 = -t65;
            let t74 = ((t73).simd_ge(V_ZERO).select(V_ONE, V_ZERO));
            let t76 = t63 * t66 + t72 * t74;
            let t79 = f64x8::splat(1.0) + f64x8::splat(0.001015549) * t43;
            let t80 = f64x8::splat(1.0) / t79;
            let t81 = t28 * t76 * t80;
            let t84 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t27 * t81));
            let t85 = (v_rho1).simd_le(dens_threshold);
            let t86 = -t17;
            let t88 = ((t15).select(t12, (t11).select(t16, t86 * t8)));
            let t89 = f64x8::splat(1.0) + t88;
            let t90 = (t89).simd_le(zeta_threshold);
            let t91 = (simd::cbrt(t89));
            let t93 = ((t90).select(t23, t91 * t89));
            let t94 = t6 * t93;
            let t95 = (simd::cbrt(v_rho1));
            let t96 = t95 * t95;
            let t98 = f64x8::splat(1.0) / t96 / v_rho1;
            let t100 = v_rho1 * v_rho1;
            let t102 = f64x8::splat(1.0) / t96 / t100;
            let t103 = v_sigma2 * t102;
            let t105 = v_tau1 * t98 - t103 / f64x8::splat(8.0);
            let t107 = t105 * t46 * t51;
            let t109 = f64x8::splat(0.827411) - f64x8::splat(0.3575333333333333) * t107;
            let t111 = f64x8::splat(1.0) - f64x8::splat(0.45341611111111113) * t107;
            let t112 = f64x8::splat(1.0) / t111;
            let t114 = t109 * t112 * t59;
            let t117 = t34 + f64x8::splat(5.0) / f64x8::splat(9.0) * t107 * t114;
            let t118 = f64x8::splat(5.0) / f64x8::splat(9.0) * t107;
            let t119 = f64x8::splat(1.0) - t118;
            let t120 = ((t119).simd_ge(V_ZERO).select(V_ONE, V_ZERO));
            let t122 = f64x8::splat(1.0) + t118;
            let t123 = f64x8::splat(1.0) / t122;
            let t126 = f64x8::splat(1.0) + f64x8::splat(0.148) * t119 * t123;
            let t127 = -t119;
            let t128 = ((t127).simd_ge(V_ZERO).select(V_ONE, V_ZERO));
            let t130 = t117 * t120 + t126 * t128;
            let t133 = f64x8::splat(1.0) + f64x8::splat(0.001015549) * t103;
            let t134 = f64x8::splat(1.0) / t133;
            let t135 = t28 * t130 * t134;
            let t138 = ((t85).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t94 * t135));
            let tzk0 = t84 + t138;
            acc_zk = tzk0;
            let t139 = t7 * t7;
            let t140 = f64x8::splat(1.0) / t139;
            let t141 = t17 * t140;
            let t143 = ((t11).select(f64x8::splat(0.0), (t15).select(f64x8::splat(0.0), t8 - t141)));
            let t146 = ((t21).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t24 * t143));
            let t147 = t6 * t146;
            let t150 = t28 * t28;
            let t151 = f64x8::splat(1.0) / t150;
            let t153 = t151 * t76 * t80;
            let t155 = t27 * t153 / f64x8::splat(8.0);
            let t158 = t40 * v_rho0;
            let t160 = f64x8::splat(1.0) / t36 / t158;
            let t161 = v_sigma0 * t160;
            let t163 = -f64x8::splat(5.0) / f64x8::splat(3.0) * v_tau0 * t42 + t161 / f64x8::splat(3.0);
            let t164 = t163 * t46;
            let t165 = t164 * t51;
            let t168 = t46 * t46;
            let t171 = f64x8::splat(1.0) / t49 / t48;
            let t172 = t45 * t168 * t171;
            let t174 = t163 * t57 * t59;
            let t177 = t56 * t56;
            let t178 = f64x8::splat(1.0) / t177;
            let t179 = t54 * t178;
            let t180 = t59 * t163;
            let t181 = t179 * t180;
            let t184 = f64x8::splat(5.0) / f64x8::splat(9.0) * t165 * t60 - f64x8::splat(0.19862962962962963) * t172 * t174 + f64x8::splat(0.25189783950617284) * t172 * t181;
            let t186 = f64x8::splat(0.0);
            let t187 = t63 * t186;
            let t190 = t51 * t69;
            let t193 = t68 * t68;
            let t194 = f64x8::splat(1.0) / t193;
            let t195 = t65 * t194;
            let t198 = -f64x8::splat(0.08222222222222222) * t164 * t190 - f64x8::splat(0.08222222222222222) * t195 * t165;
            let t200 = t72 * t186;
            let t203 = t184 * t66 - f64x8::splat(5.0) / f64x8::splat(9.0) * t187 * t165 + t198 * t74 + f64x8::splat(5.0) / f64x8::splat(9.0) * t200 * t165;
            let t205 = t28 * t203 * t80;
            let t208 = t3 * t26;
            let t209 = t208 * t28;
            let t210 = t79 * t79;
            let t211 = f64x8::splat(1.0) / t210;
            let t212 = t76 * t211;
            let t213 = t212 * t161;
            let t217 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t147 * t81 - t155 - f64x8::splat(3.0) / f64x8::splat(8.0) * t27 * t205 - f64x8::splat(0.0006934006726548522) * t209 * t213));
            let t218 = t86 * t140;
            let t220 = ((t15).select(f64x8::splat(0.0), (t11).select(f64x8::splat(0.0), -t8 - t218)));
            let t223 = ((t90).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t91 * t220));
            let t224 = t6 * t223;
            let t228 = t151 * t130 * t134;
            let t230 = t94 * t228 / f64x8::splat(8.0);
            let t232 = ((t85).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t224 * t135 - t230));
            let tvrho0 = t84 + t138 + t7 * (t217 + t232);
            acc_vrho_0 = tvrho0;
            let t236 = ((t11).select(f64x8::splat(0.0), (t15).select(f64x8::splat(0.0), -t8 - t141)));
            let t239 = ((t21).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t24 * t236));
            let t240 = t6 * t239;
            let t244 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t240 * t81 - t155));
            let t246 = ((t15).select(f64x8::splat(0.0), (t11).select(f64x8::splat(0.0), t8 - t218)));
            let t249 = ((t90).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t91 * t246));
            let t250 = t6 * t249;
            let t255 = t100 * v_rho1;
            let t257 = f64x8::splat(1.0) / t96 / t255;
            let t258 = v_sigma2 * t257;
            let t260 = -f64x8::splat(5.0) / f64x8::splat(3.0) * v_tau1 * t102 + t258 / f64x8::splat(3.0);
            let t261 = t260 * t46;
            let t262 = t261 * t51;
            let t266 = t105 * t168 * t171;
            let t268 = t260 * t112 * t59;
            let t271 = t111 * t111;
            let t272 = f64x8::splat(1.0) / t271;
            let t273 = t109 * t272;
            let t274 = t59 * t260;
            let t275 = t273 * t274;
            let t278 = f64x8::splat(5.0) / f64x8::splat(9.0) * t262 * t114 - f64x8::splat(0.19862962962962963) * t266 * t268 + f64x8::splat(0.25189783950617284) * t266 * t275;
            let t280 = f64x8::splat(0.0);
            let t281 = t117 * t280;
            let t284 = t51 * t123;
            let t287 = t122 * t122;
            let t288 = f64x8::splat(1.0) / t287;
            let t289 = t119 * t288;
            let t292 = -f64x8::splat(0.08222222222222222) * t261 * t284 - f64x8::splat(0.08222222222222222) * t289 * t262;
            let t294 = t126 * t280;
            let t297 = t278 * t120 - f64x8::splat(5.0) / f64x8::splat(9.0) * t281 * t262 + t292 * t128 + f64x8::splat(5.0) / f64x8::splat(9.0) * t294 * t262;
            let t299 = t28 * t297 * t134;
            let t302 = t3 * t93;
            let t303 = t302 * t28;
            let t304 = t133 * t133;
            let t305 = f64x8::splat(1.0) / t304;
            let t306 = t130 * t305;
            let t307 = t306 * t258;
            let t311 = ((t85).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t250 * t135 - t230 - f64x8::splat(3.0) / f64x8::splat(8.0) * t94 * t299 - f64x8::splat(0.0006934006726548522) * t303 * t307));
            let tvrho1 = t84 + t138 + t7 * (t244 + t311);
            acc_vrho_1 = tvrho1;
            let t314 = t42 * t46;
            let t315 = t314 * t51;
            let t316 = t315 * t60;
            let t320 = t172 * t42 * t57 * t59;
            let t322 = t59 * t42;
            let t324 = t172 * t179 * t322;
            let t326 = -f64x8::splat(5.0) / f64x8::splat(72.0) * t316 + f64x8::splat(0.024828703703703704) * t320 - f64x8::splat(0.031487229938271605) * t324;
            let t328 = t187 * t315;
            let t330 = t314 * t190;
            let t332 = t195 * t315;
            let t334 = f64x8::splat(0.010277777777777778) * t330 + f64x8::splat(0.010277777777777778) * t332;
            let t336 = t200 * t315;
            let t338 = t326 * t66 + f64x8::splat(5.0) / f64x8::splat(72.0) * t328 + t334 * t74 - f64x8::splat(5.0) / f64x8::splat(72.0) * t336;
            let t340 = t28 * t338 * t80;
            let t343 = t212 * t42;
            let t347 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t27 * t340 + f64x8::splat(0.0002600252522455696) * t209 * t343));
            let tvsigma0 = t7 * t347;
            acc_vsigma_0 = tvsigma0;
            let tvsigma1 = f64x8::splat(0.0);
            acc_vsigma_1 = tvsigma1;
            let t348 = t102 * t46;
            let t349 = t348 * t51;
            let t350 = t349 * t114;
            let t354 = t266 * t102 * t112 * t59;
            let t356 = t59 * t102;
            let t358 = t266 * t273 * t356;
            let t360 = -f64x8::splat(5.0) / f64x8::splat(72.0) * t350 + f64x8::splat(0.024828703703703704) * t354 - f64x8::splat(0.031487229938271605) * t358;
            let t362 = t281 * t349;
            let t364 = t348 * t284;
            let t366 = t289 * t349;
            let t368 = f64x8::splat(0.010277777777777778) * t364 + f64x8::splat(0.010277777777777778) * t366;
            let t370 = t294 * t349;
            let t372 = t360 * t120 + f64x8::splat(5.0) / f64x8::splat(72.0) * t362 + t368 * t128 - f64x8::splat(5.0) / f64x8::splat(72.0) * t370;
            let t374 = t28 * t372 * t134;
            let t377 = t306 * t102;
            let t381 = ((t85).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t94 * t374 + f64x8::splat(0.0002600252522455696) * t303 * t377));
            let tvsigma2 = t7 * t381;
            acc_vsigma_2 = tvsigma2;
            let tvlapl0 = f64x8::splat(0.0);
            acc_vlapl_0 = tvlapl0;
            let tvlapl1 = f64x8::splat(0.0);
            acc_vlapl_1 = tvlapl1;
            let t382 = t38 * t46;
            let t383 = t382 * t51;
            let t390 = t59 * t38;
            let t394 = f64x8::splat(5.0) / f64x8::splat(9.0) * t383 * t60 - f64x8::splat(0.19862962962962963) * t172 * t38 * t57 * t59 + f64x8::splat(0.25189783950617284) * t172 * t179 * t390;
            let t402 = -f64x8::splat(0.08222222222222222) * t382 * t190 - f64x8::splat(0.08222222222222222) * t195 * t383;
            let t406 = t394 * t66 - f64x8::splat(5.0) / f64x8::splat(9.0) * t187 * t383 + t402 * t74 + f64x8::splat(5.0) / f64x8::splat(9.0) * t200 * t383;
            let t408 = t28 * t406 * t80;
            let t411 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t27 * t408));
            let tvtau0 = t7 * t411;
            acc_vtau_0 = tvtau0;
            let t412 = t98 * t46;
            let t413 = t412 * t51;
            let t420 = t59 * t98;
            let t424 = f64x8::splat(5.0) / f64x8::splat(9.0) * t413 * t114 - f64x8::splat(0.19862962962962963) * t266 * t98 * t112 * t59 + f64x8::splat(0.25189783950617284) * t266 * t273 * t420;
            let t432 = -f64x8::splat(0.08222222222222222) * t412 * t284 - f64x8::splat(0.08222222222222222) * t289 * t413;
            let t436 = t424 * t120 - f64x8::splat(5.0) / f64x8::splat(9.0) * t281 * t413 + t432 * t128 + f64x8::splat(5.0) / f64x8::splat(9.0) * t294 * t413;
            let t438 = t28 * t436 * t134;
            let t441 = ((t85).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t94 * t438));
            let tvtau1 = t7 * t441;
            acc_vtau_1 = tvtau1;
        }
        store_add(zk, ip, m, acc_zk);
        store_strided(vrho, ip, m, 2, 0, acc_vrho_0);
        store_strided(vrho, ip, m, 2, 1, acc_vrho_1);
        store_strided(vsigma, ip, m, 3, 0, acc_vsigma_0);
        store_strided(vsigma, ip, m, 3, 1, acc_vsigma_1);
        store_strided(vsigma, ip, m, 3, 2, acc_vsigma_2);
        store_strided(vlapl, ip, m, 2, 0, acc_vlapl_0);
        store_strided(vlapl, ip, m, 2, 1, acc_vlapl_1);
        store_strided(vtau, ip, m, 2, 0, acc_vtau_0);
        store_strided(vtau, ip, m, 2, 1, acc_vtau_1);
        ip += 8;
    }
}
