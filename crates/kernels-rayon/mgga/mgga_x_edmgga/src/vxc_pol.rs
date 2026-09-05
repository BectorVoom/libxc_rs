//! MGGA_X_EDMGGA vxc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_edmgga.c`
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
pub fn mgga_x_edmgga_vxc_pol(
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
            let t27 = (simd::cbrt(t7));
            let t28 = t26 * t27;
            let t29 = f64x8::splat(M_CBRT4);
            let t30 = t3 * t3;
            let t32 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t33 = (simd::cbrt(t32));
            let t35 = t29 * t30 * t33 / f64x8::splat(9.0);
            let t36 = f64x8::splat(1.0) - t35;
            let t37 = (simd::cbrt(v_rho0));
            let t38 = t37 * t37;
            let t40 = f64x8::splat(1.0) / t38 / v_rho0;
            let t42 = v_rho0 * v_rho0;
            let t44 = f64x8::splat(1.0) / t38 / t42;
            let t50 = f64x8::splat(M_CBRT6);
            let t52 = t33 * t33;
            let t53 = f64x8::splat(1.0) / t52;
            let t54 = (v_tau0 * t40 - v_sigma0 * t44 / f64x8::splat(8.0) - v_lapl0 * t40 / f64x8::splat(4.0)) * t50 * t53;
            let t55 = f64x8::splat(5.0) / f64x8::splat(9.0) * t54;
            let t56 = (-t55).simd_lt(-f64x8::splat(14205.545454545454));
            let t57 = f64x8::splat(0.39111111111111113) * t54;
            let t59 = (f64x8::splat(0.0)).simd_lt(f64x8::splat(0.7041420454545455) - t57);
            let t61 = ((t59).select(-f64x8::splat(0.00014204545454545454), f64x8::splat(0.704) - t57));
            let t64 = t61 * t61;
            let t65 = t64 * t61;
            let t66 = f64x8::splat(1.0) / t65;
            let t69 = f64x8::splat(1.0) - t55;
            let t70 = t69 * t69;
            let t72 = f64x8::splat(1.0) + f64x8::splat(0.495616) * t70;
            let t73 = ((t72).sqrt());
            let t75 = ((t56).select(-f64x8::splat(1.0) / t61 / f64x8::splat(2.0) + t66 / f64x8::splat(8.0), f64x8::splat(0.704) - t57 + t73));
            let t76 = t36 * t75;
            let t77 = ((f64x8::splat(30.0)).sqrt());
            let t78 = t36 * t77;
            let t79 = ((t75).sqrt());
            let t80 = t36 * t36;
            let t83 = f64x8::splat(1.0) / t80 / t36 * t77;
            let t85 = f64x8::splat(0.6018478308354863) * t80 - f64x8::splat(0.0206514);
            let t86 = t75 - f64x8::splat(1.0);
            let t90 = (simd::ln(f64x8::splat(0.3910293204892512) * t83 * t85 * t86 + ((((f64x8::splat(0.3910293204892512) * t83 * t85 * t86) * (f64x8::splat(0.3910293204892512) * t83 * t85 * t86)) + f64x8::splat(1.0)).sqrt())));
            let t94 = f64x8::splat(1.0) + f64x8::splat(0.14163895778062927) * t78 * t79 * t90;
            let t95 = f64x8::splat(1.0) / t94;
            let t97 = t76 * t95 + t35;
            let t101 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t28 * t97));
            let t102 = (v_rho1).simd_le(dens_threshold);
            let t103 = -t17;
            let t105 = ((t15).select(t12, (t11).select(t16, t103 * t8)));
            let t106 = f64x8::splat(1.0) + t105;
            let t107 = (t106).simd_le(zeta_threshold);
            let t108 = (simd::cbrt(t106));
            let t110 = ((t107).select(t23, t108 * t106));
            let t111 = t110 * t27;
            let t112 = (simd::cbrt(v_rho1));
            let t113 = t112 * t112;
            let t115 = f64x8::splat(1.0) / t113 / v_rho1;
            let t117 = v_rho1 * v_rho1;
            let t119 = f64x8::splat(1.0) / t113 / t117;
            let t126 = (v_tau1 * t115 - v_sigma2 * t119 / f64x8::splat(8.0) - v_lapl1 * t115 / f64x8::splat(4.0)) * t50 * t53;
            let t127 = f64x8::splat(5.0) / f64x8::splat(9.0) * t126;
            let t128 = (-t127).simd_lt(-f64x8::splat(14205.545454545454));
            let t129 = f64x8::splat(0.39111111111111113) * t126;
            let t131 = (f64x8::splat(0.0)).simd_lt(f64x8::splat(0.7041420454545455) - t129);
            let t133 = ((t131).select(-f64x8::splat(0.00014204545454545454), f64x8::splat(0.704) - t129));
            let t136 = t133 * t133;
            let t137 = t136 * t133;
            let t138 = f64x8::splat(1.0) / t137;
            let t141 = f64x8::splat(1.0) - t127;
            let t142 = t141 * t141;
            let t144 = f64x8::splat(1.0) + f64x8::splat(0.495616) * t142;
            let t145 = ((t144).sqrt());
            let t147 = ((t128).select(-f64x8::splat(1.0) / t133 / f64x8::splat(2.0) + t138 / f64x8::splat(8.0), f64x8::splat(0.704) - t129 + t145));
            let t148 = t36 * t147;
            let t149 = ((t147).sqrt());
            let t150 = t147 - f64x8::splat(1.0);
            let t154 = (simd::ln(f64x8::splat(0.3910293204892512) * t83 * t85 * t150 + ((((f64x8::splat(0.3910293204892512) * t83 * t85 * t150) * (f64x8::splat(0.3910293204892512) * t83 * t85 * t150)) + f64x8::splat(1.0)).sqrt())));
            let t158 = f64x8::splat(1.0) + f64x8::splat(0.14163895778062927) * t78 * t149 * t154;
            let t159 = f64x8::splat(1.0) / t158;
            let t161 = t148 * t159 + t35;
            let t165 = ((t102).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t111 * t161));
            let tzk0 = t101 + t165;
            acc_zk = tzk0;
            let t166 = t7 * t7;
            let t167 = f64x8::splat(1.0) / t166;
            let t168 = t17 * t167;
            let t170 = ((t11).select(f64x8::splat(0.0), (t15).select(f64x8::splat(0.0), t8 - t168)));
            let t173 = ((t21).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t24 * t170));
            let t174 = t173 * t27;
            let t178 = t27 * t27;
            let t179 = f64x8::splat(1.0) / t178;
            let t180 = t26 * t179;
            let t183 = t6 * t180 * t97 / f64x8::splat(8.0);
            let t184 = f64x8::splat(1.0) / t64;
            let t187 = t42 * v_rho0;
            let t189 = f64x8::splat(1.0) / t38 / t187;
            let t194 = -f64x8::splat(5.0) / f64x8::splat(3.0) * v_tau0 * t44 + v_sigma0 * t189 / f64x8::splat(3.0) + f64x8::splat(5.0) / f64x8::splat(12.0) * v_lapl0 * t44;
            let t196 = t194 * t50 * t53;
            let t197 = f64x8::splat(0.39111111111111113) * t196;
            let t198 = ((t59).select(f64x8::splat(0.0), -t197));
            let t201 = t64 * t64;
            let t202 = f64x8::splat(1.0) / t201;
            let t206 = f64x8::splat(1.0) / t73;
            let t207 = t206 * t69;
            let t211 = ((t56).select(t184 * t198 / f64x8::splat(2.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t202 * t198, -t197 - f64x8::splat(0.2753422222222222) * t207 * t196));
            let t212 = t36 * t211;
            let t214 = t94 * t94;
            let t215 = f64x8::splat(1.0) / t214;
            let t216 = f64x8::splat(1.0) / t79;
            let t217 = t216 * t90;
            let t221 = f64x8::splat(1.0) / t80;
            let t222 = t221 * t79;
            let t223 = t85 * t211;
            let t224 = t80 * t80;
            let t225 = t224 * t80;
            let t227 = t85 * t85;
            let t228 = f64x8::splat(1.0) / t225 * t227;
            let t229 = t86 * t86;
            let t232 = f64x8::splat(4.587117884468566) * t228 * t229 + f64x8::splat(1.0);
            let t233 = ((t232).sqrt());
            let t234 = f64x8::splat(1.0) / t233;
            let t238 = f64x8::splat(0.07081947889031463) * t78 * t217 * t211 + f64x8::splat(1.661549562472956) * t222 * t223 * t234;
            let t239 = t215 * t238;
            let t241 = t212 * t95 - t76 * t239;
            let t246 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t174 * t97 - t183 - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t28 * t241));
            let t247 = t103 * t167;
            let t249 = ((t15).select(f64x8::splat(0.0), (t11).select(f64x8::splat(0.0), -t8 - t247)));
            let t252 = ((t107).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t108 * t249));
            let t253 = t252 * t27;
            let t257 = t110 * t179;
            let t260 = t6 * t257 * t161 / f64x8::splat(8.0);
            let t262 = ((t102).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t253 * t161 - t260));
            let tvrho0 = t101 + t165 + t7 * (t246 + t262);
            acc_vrho_0 = tvrho0;
            let t266 = ((t11).select(f64x8::splat(0.0), (t15).select(f64x8::splat(0.0), -t8 - t168)));
            let t269 = ((t21).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t24 * t266));
            let t270 = t269 * t27;
            let t275 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t270 * t97 - t183));
            let t277 = ((t15).select(f64x8::splat(0.0), (t11).select(f64x8::splat(0.0), t8 - t247)));
            let t280 = ((t107).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t108 * t277));
            let t281 = t280 * t27;
            let t285 = f64x8::splat(1.0) / t136;
            let t288 = t117 * v_rho1;
            let t290 = f64x8::splat(1.0) / t113 / t288;
            let t295 = -f64x8::splat(5.0) / f64x8::splat(3.0) * v_tau1 * t119 + v_sigma2 * t290 / f64x8::splat(3.0) + f64x8::splat(5.0) / f64x8::splat(12.0) * v_lapl1 * t119;
            let t297 = t295 * t50 * t53;
            let t298 = f64x8::splat(0.39111111111111113) * t297;
            let t299 = ((t131).select(f64x8::splat(0.0), -t298));
            let t302 = t136 * t136;
            let t303 = f64x8::splat(1.0) / t302;
            let t307 = f64x8::splat(1.0) / t145;
            let t308 = t307 * t141;
            let t312 = ((t128).select(t285 * t299 / f64x8::splat(2.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t303 * t299, -t298 - f64x8::splat(0.2753422222222222) * t308 * t297));
            let t313 = t36 * t312;
            let t315 = t158 * t158;
            let t316 = f64x8::splat(1.0) / t315;
            let t317 = f64x8::splat(1.0) / t149;
            let t318 = t317 * t154;
            let t322 = t221 * t149;
            let t323 = t85 * t312;
            let t324 = t150 * t150;
            let t327 = f64x8::splat(4.587117884468566) * t228 * t324 + f64x8::splat(1.0);
            let t328 = ((t327).sqrt());
            let t329 = f64x8::splat(1.0) / t328;
            let t333 = f64x8::splat(0.07081947889031463) * t78 * t318 * t312 + f64x8::splat(1.661549562472956) * t322 * t323 * t329;
            let t334 = t316 * t333;
            let t336 = -t148 * t334 + t313 * t159;
            let t341 = ((t102).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t281 * t161 - t260 - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t111 * t336));
            let tvrho1 = t101 + t165 + t7 * (t275 + t341);
            acc_vrho_1 = tvrho1;
            let t345 = t44 * t50 * t53;
            let t346 = f64x8::splat(0.04888888888888889) * t345;
            let t347 = ((t59).select(f64x8::splat(0.0), t346));
            let t350 = t202 * t347;
            let t353 = t207 * t345;
            let t356 = ((t56).select(t184 * t347 / f64x8::splat(2.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t350, t346 + f64x8::splat(0.034417777777777776) * t353));
            let t357 = t36 * t356;
            let t366 = f64x8::splat(0.07081947889031463) * t78 * t217 * t356 + f64x8::splat(1.661549562472956) * t222 * t85 * t356 * t234;
            let t367 = t215 * t366;
            let t369 = t357 * t95 - t76 * t367;
            let t373 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t28 * t369));
            let tvsigma0 = t7 * t373;
            acc_vsigma_0 = tvsigma0;
            let tvsigma1 = f64x8::splat(0.0);
            acc_vsigma_1 = tvsigma1;
            let t375 = t119 * t50 * t53;
            let t376 = f64x8::splat(0.04888888888888889) * t375;
            let t377 = ((t131).select(f64x8::splat(0.0), t376));
            let t380 = t303 * t377;
            let t383 = t308 * t375;
            let t386 = ((t128).select(t285 * t377 / f64x8::splat(2.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t380, t376 + f64x8::splat(0.034417777777777776) * t383));
            let t387 = t36 * t386;
            let t396 = f64x8::splat(0.07081947889031463) * t78 * t318 * t386 + f64x8::splat(1.661549562472956) * t322 * t85 * t386 * t329;
            let t397 = t316 * t396;
            let t399 = -t148 * t397 + t387 * t159;
            let t403 = ((t102).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t111 * t399));
            let tvsigma2 = t7 * t403;
            acc_vsigma_2 = tvsigma2;
            let t405 = t40 * t50 * t53;
            let t406 = f64x8::splat(0.09777777777777778) * t405;
            let t407 = ((t59).select(f64x8::splat(0.0), t406));
            let t410 = t202 * t407;
            let t413 = t207 * t405;
            let t416 = ((t56).select(t184 * t407 / f64x8::splat(2.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t410, t406 + f64x8::splat(0.06883555555555555) * t413));
            let t417 = t36 * t416;
            let t426 = f64x8::splat(0.07081947889031463) * t78 * t217 * t416 + f64x8::splat(1.661549562472956) * t222 * t85 * t416 * t234;
            let t427 = t215 * t426;
            let t429 = t417 * t95 - t76 * t427;
            let t433 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t28 * t429));
            let tvlapl0 = t7 * t433;
            acc_vlapl_0 = tvlapl0;
            let t435 = t115 * t50 * t53;
            let t436 = f64x8::splat(0.09777777777777778) * t435;
            let t437 = ((t131).select(f64x8::splat(0.0), t436));
            let t440 = t303 * t437;
            let t443 = t308 * t435;
            let t446 = ((t128).select(t285 * t437 / f64x8::splat(2.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t440, t436 + f64x8::splat(0.06883555555555555) * t443));
            let t447 = t36 * t446;
            let t456 = f64x8::splat(0.07081947889031463) * t78 * t318 * t446 + f64x8::splat(1.661549562472956) * t322 * t85 * t446 * t329;
            let t457 = t316 * t456;
            let t459 = -t148 * t457 + t447 * t159;
            let t463 = ((t102).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t111 * t459));
            let tvlapl1 = t7 * t463;
            acc_vlapl_1 = tvlapl1;
            let t464 = f64x8::splat(0.39111111111111113) * t405;
            let t465 = ((t59).select(f64x8::splat(0.0), -t464));
            let t468 = t202 * t465;
            let t473 = ((t56).select(t184 * t465 / f64x8::splat(2.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t468, -t464 - f64x8::splat(0.2753422222222222) * t413));
            let t474 = t36 * t473;
            let t483 = f64x8::splat(0.07081947889031463) * t78 * t217 * t473 + f64x8::splat(1.661549562472956) * t222 * t85 * t473 * t234;
            let t484 = t215 * t483;
            let t486 = t474 * t95 - t76 * t484;
            let t490 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t28 * t486));
            let tvtau0 = t7 * t490;
            acc_vtau_0 = tvtau0;
            let t491 = f64x8::splat(0.39111111111111113) * t435;
            let t492 = ((t131).select(f64x8::splat(0.0), -t491));
            let t495 = t303 * t492;
            let t500 = ((t128).select(t285 * t492 / f64x8::splat(2.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t495, -t491 - f64x8::splat(0.2753422222222222) * t443));
            let t501 = t36 * t500;
            let t510 = f64x8::splat(0.07081947889031463) * t78 * t318 * t500 + f64x8::splat(1.661549562472956) * t322 * t85 * t500 * t329;
            let t511 = t316 * t510;
            let t513 = -t148 * t511 + t501 * t159;
            let t517 = ((t102).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t111 * t513));
            let tvtau1 = t7 * t517;
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
