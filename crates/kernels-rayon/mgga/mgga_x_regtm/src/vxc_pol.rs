//! MGGA_X_REGTM vxc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_regtm.c`
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
pub fn mgga_x_regtm_vxc_pol(
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
            let t29 = (simd::cbrt(v_rho0));
            let t30 = t29 * t29;
            let t32 = f64x8::splat(1.0) / t30 / v_rho0;
            let t33 = v_tau0 * t32;
            let t34 = v_rho0 * v_rho0;
            let t36 = f64x8::splat(1.0) / t30 / t34;
            let t37 = v_sigma0 * t36;
            let t39 = t33 - t37 / f64x8::splat(8.0);
            let t40 = f64x8::splat(M_CBRT6);
            let t41 = t39 * t40;
            let t42 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t43 = (simd::cbrt(t42));
            let t44 = t43 * t43;
            let t45 = f64x8::splat(1.0) / t44;
            let t46 = t40 * t45;
            let t47 = t46 * t37;
            let t49 = t41 * t45;
            let t51 = f64x8::splat(1.0) - f64x8::splat(5.0) / f64x8::splat(9.0) * t49;
            let t52 = t51 * t51;
            let t53 = t52 * t51;
            let t54 = t39 * t39;
            let t55 = t40 * t40;
            let t56 = t54 * t55;
            let t58 = f64x8::splat(1.0) / t43 / t42;
            let t61 = f64x8::splat(1.0) + f64x8::splat(0.6714891975308642) * t56 * t58;
            let t62 = ((t61).sqrt());
            let t64 = f64x8::splat(1.0) / t62 / t61;
            let t65 = t53 * t64;
            let t67 = (simd::exp(-t47 / f64x8::splat(8.0)));
            let t69 = t47 / f64x8::splat(24.0) + t65 * t67;
            let t71 = t45 / t69;
            let t74 = f64x8::splat(1.0) + t41 * t71 / f64x8::splat(3.0);
            let t75 = t74 * t74;
            let t77 = t75 * t74;
            let t78 = f64x8::splat(1.0) / t77;
            let t80 = f64x8::splat(1.0) / t75 + f64x8::splat(3.0) * t78;
            let t81 = f64x8::splat(1.0) + t78;
            let t82 = t81 * t81;
            let t83 = f64x8::splat(1.0) / t82;
            let t84 = t80 * t83;
            let t86 = t55 * t58;
            let t87 = v_sigma0 * v_sigma0;
            let t88 = t34 * t34;
            let t89 = t88 * v_rho0;
            let t91 = f64x8::splat(1.0) / t29 / t89;
            let t95 = f64x8::splat(1.0) + f64x8::splat(0.1504548888888889) * t47 + f64x8::splat(0.002689949046226295) * t86 * t87 * t91;
            let t96 = (simd::pow(t95, f64x8::splat(1.0) / f64x8::splat(5.0)));
            let t101 = f64x8::splat(0.256337604) * t55 * t44;
            let t107 = f64x8::splat(1.0) + f64x8::splat(0.06394332777777778) * t47 - f64x8::splat(5.0) / f64x8::splat(9.0) * (f64x8::splat(0.14554132) * t33 + t101 + f64x8::splat(0.011867481666666667) * t37) * t40 * t45;
            let t108 = t96 * t96;
            let t109 = f64x8::splat(1.0) / t108;
            let t112 = f64x8::splat(1.0) / t96 + f64x8::splat(7.0) / f64x8::splat(9.0) * t107 * t109;
            let t114 = f64x8::splat(1.0) - t84;
            let t117 = (f64x8::splat(10.0) / f64x8::splat(81.0) + f64x8::splat(25.0) / f64x8::splat(8748.0) * t47) * t40;
            let t118 = t45 * v_sigma0;
            let t124 = t49 / f64x8::splat(4.0) - f64x8::splat(9.0) / f64x8::splat(20.0) + t47 / f64x8::splat(36.0);
            let t125 = t124 * t124;
            let t127 = f64x8::splat(1.0) / v_rho0;
            let t128 = v_sigma0 * t127;
            let t129 = f64x8::splat(1.0) / v_tau0;
            let t131 = t128 * t129 / f64x8::splat(8.0);
            let t132 = (t131).simd_lt(f64x8::splat(1.0));
            let t133 = ((t132).select(t131, f64x8::splat(1.0)));
            let t134 = t124 * t133;
            let t135 = f64x8::splat(1.0) - t133;
            let t138 = f64x8::splat(1.0) + f64x8::splat(5.0) / f64x8::splat(12.0) * t117 * t118 * t36 + f64x8::splat(292.0) / f64x8::splat(405.0) * t125 - f64x8::splat(146.0) / f64x8::splat(135.0) * t134 * t135;
            let t139 = (simd::pow(t138, f64x8::splat(1.0) / f64x8::splat(10.0)));
            let t141 = t84 * t112 + t114 * t139;
            let t145 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t28 * t141));
            let t146 = (v_rho1).simd_le(dens_threshold);
            let t147 = -t17;
            let t149 = ((t15).select(t12, (t11).select(t16, t147 * t8)));
            let t150 = f64x8::splat(1.0) + t149;
            let t151 = (t150).simd_le(zeta_threshold);
            let t152 = (simd::cbrt(t150));
            let t154 = ((t151).select(t23, t152 * t150));
            let t155 = t154 * t27;
            let t156 = (simd::cbrt(v_rho1));
            let t157 = t156 * t156;
            let t159 = f64x8::splat(1.0) / t157 / v_rho1;
            let t160 = v_tau1 * t159;
            let t161 = v_rho1 * v_rho1;
            let t163 = f64x8::splat(1.0) / t157 / t161;
            let t164 = v_sigma2 * t163;
            let t166 = t160 - t164 / f64x8::splat(8.0);
            let t167 = t166 * t40;
            let t168 = t46 * t164;
            let t170 = t167 * t45;
            let t172 = f64x8::splat(1.0) - f64x8::splat(5.0) / f64x8::splat(9.0) * t170;
            let t173 = t172 * t172;
            let t174 = t173 * t172;
            let t175 = t166 * t166;
            let t176 = t175 * t55;
            let t179 = f64x8::splat(1.0) + f64x8::splat(0.6714891975308642) * t176 * t58;
            let t180 = ((t179).sqrt());
            let t182 = f64x8::splat(1.0) / t180 / t179;
            let t183 = t174 * t182;
            let t185 = (simd::exp(-t168 / f64x8::splat(8.0)));
            let t187 = t168 / f64x8::splat(24.0) + t183 * t185;
            let t189 = t45 / t187;
            let t192 = f64x8::splat(1.0) + t167 * t189 / f64x8::splat(3.0);
            let t193 = t192 * t192;
            let t195 = t193 * t192;
            let t196 = f64x8::splat(1.0) / t195;
            let t198 = f64x8::splat(1.0) / t193 + f64x8::splat(3.0) * t196;
            let t199 = f64x8::splat(1.0) + t196;
            let t200 = t199 * t199;
            let t201 = f64x8::splat(1.0) / t200;
            let t202 = t198 * t201;
            let t204 = v_sigma2 * v_sigma2;
            let t205 = t161 * t161;
            let t206 = t205 * v_rho1;
            let t208 = f64x8::splat(1.0) / t156 / t206;
            let t212 = f64x8::splat(1.0) + f64x8::splat(0.1504548888888889) * t168 + f64x8::splat(0.002689949046226295) * t86 * t204 * t208;
            let t213 = (simd::pow(t212, f64x8::splat(1.0) / f64x8::splat(5.0)));
            let t222 = f64x8::splat(1.0) + f64x8::splat(0.06394332777777778) * t168 - f64x8::splat(5.0) / f64x8::splat(9.0) * (f64x8::splat(0.14554132) * t160 + t101 + f64x8::splat(0.011867481666666667) * t164) * t40 * t45;
            let t223 = t213 * t213;
            let t224 = f64x8::splat(1.0) / t223;
            let t227 = f64x8::splat(1.0) / t213 + f64x8::splat(7.0) / f64x8::splat(9.0) * t222 * t224;
            let t229 = f64x8::splat(1.0) - t202;
            let t232 = (f64x8::splat(10.0) / f64x8::splat(81.0) + f64x8::splat(25.0) / f64x8::splat(8748.0) * t168) * t40;
            let t233 = t45 * v_sigma2;
            let t239 = t170 / f64x8::splat(4.0) - f64x8::splat(9.0) / f64x8::splat(20.0) + t168 / f64x8::splat(36.0);
            let t240 = t239 * t239;
            let t242 = f64x8::splat(1.0) / v_rho1;
            let t243 = v_sigma2 * t242;
            let t244 = f64x8::splat(1.0) / v_tau1;
            let t246 = t243 * t244 / f64x8::splat(8.0);
            let t247 = (t246).simd_lt(f64x8::splat(1.0));
            let t248 = ((t247).select(t246, f64x8::splat(1.0)));
            let t249 = t239 * t248;
            let t250 = f64x8::splat(1.0) - t248;
            let t253 = f64x8::splat(1.0) + f64x8::splat(5.0) / f64x8::splat(12.0) * t232 * t233 * t163 + f64x8::splat(292.0) / f64x8::splat(405.0) * t240 - f64x8::splat(146.0) / f64x8::splat(135.0) * t249 * t250;
            let t254 = (simd::pow(t253, f64x8::splat(1.0) / f64x8::splat(10.0)));
            let t256 = t202 * t227 + t229 * t254;
            let t260 = ((t146).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t155 * t256));
            let tzk0 = t145 + t260;
            acc_zk = tzk0;
            let t261 = t7 * t7;
            let t262 = f64x8::splat(1.0) / t261;
            let t263 = t17 * t262;
            let t265 = ((t11).select(f64x8::splat(0.0), (t15).select(f64x8::splat(0.0), t8 - t263)));
            let t268 = ((t21).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t24 * t265));
            let t269 = t268 * t27;
            let t273 = t27 * t27;
            let t274 = f64x8::splat(1.0) / t273;
            let t275 = t26 * t274;
            let t278 = t6 * t275 * t141 / f64x8::splat(8.0);
            let t279 = v_tau0 * t36;
            let t281 = t34 * v_rho0;
            let t283 = f64x8::splat(1.0) / t30 / t281;
            let t284 = v_sigma0 * t283;
            let t286 = -f64x8::splat(5.0) / f64x8::splat(3.0) * t279 + t284 / f64x8::splat(3.0);
            let t287 = t286 * t40;
            let t289 = t69 * t69;
            let t291 = t45 / t289;
            let t292 = t46 * t284;
            let t294 = t52 * t64;
            let t295 = t294 * t67;
            let t296 = t287 * t45;
            let t299 = t61 * t61;
            let t301 = f64x8::splat(1.0) / t62 / t299;
            let t302 = t53 * t301;
            let t303 = t302 * t67;
            let t304 = t39 * t55;
            let t305 = t58 * t286;
            let t306 = t304 * t305;
            let t309 = t65 * t40;
            let t310 = t283 * t67;
            let t311 = t118 * t310;
            let t314 = -t292 / f64x8::splat(9.0) - f64x8::splat(5.0) / f64x8::splat(3.0) * t295 * t296 - f64x8::splat(2.0144675925925926) * t303 * t306 + t309 * t311 / f64x8::splat(3.0);
            let t315 = t291 * t314;
            let t318 = t287 * t71 / f64x8::splat(3.0) - t41 * t315 / f64x8::splat(3.0);
            let t321 = t75 * t75;
            let t322 = f64x8::splat(1.0) / t321;
            let t323 = t322 * t318;
            let t325 = -f64x8::splat(2.0) * t78 * t318 - f64x8::splat(9.0) * t323;
            let t326 = t325 * t83;
            let t329 = f64x8::splat(1.0) / t82 / t81;
            let t330 = t80 * t329;
            let t331 = t112 * t322;
            let t332 = t331 * t318;
            let t336 = f64x8::splat(1.0) / t96 / t95;
            let t338 = t88 * t34;
            let t340 = f64x8::splat(1.0) / t29 / t338;
            let t342 = t86 * t87 * t340;
            let t344 = -f64x8::splat(0.40121303703703703) * t292 - f64x8::splat(0.014346394913206906) * t342;
            let t354 = -f64x8::splat(0.17051554074074074) * t292 - f64x8::splat(5.0) / f64x8::splat(9.0) * (-f64x8::splat(0.24256886666666666) * t279 - f64x8::splat(0.031646617777777775) * t284) * t40 * t45;
            let t358 = f64x8::splat(1.0) / t108 / t95;
            let t359 = t107 * t358;
            let t362 = -t336 * t344 / f64x8::splat(5.0) + f64x8::splat(7.0) / f64x8::splat(9.0) * t354 * t109 - f64x8::splat(14.0) / f64x8::splat(45.0) * t359 * t344;
            let t366 = -f64x8::splat(6.0) * t330 * t323 - t326;
            let t368 = t139 * t139;
            let t369 = t368 * t368;
            let t370 = t369 * t369;
            let t371 = t370 * t139;
            let t372 = f64x8::splat(1.0) / t371;
            let t373 = t114 * t372;
            let t380 = t296 / f64x8::splat(4.0) - f64x8::splat(2.0) / f64x8::splat(27.0) * t292;
            let t383 = t380 * t133;
            let t386 = f64x8::splat(1.0) / t34;
            let t387 = v_sigma0 * t386;
            let t390 = ((t132).select(-t387 * t129 / f64x8::splat(8.0), f64x8::splat(0.0)));
            let t396 = -f64x8::splat(125.0) / f64x8::splat(39366.0) * t342 - f64x8::splat(10.0) / f64x8::splat(9.0) * t117 * t118 * t283 + f64x8::splat(584.0) / f64x8::splat(405.0) * t124 * t380 - f64x8::splat(146.0) / f64x8::splat(135.0) * t383 * t135 - f64x8::splat(146.0) / f64x8::splat(135.0) * t124 * t390 * t135 + f64x8::splat(146.0) / f64x8::splat(135.0) * t134 * t390;
            let t399 = t326 * t112 + f64x8::splat(6.0) * t330 * t332 + t84 * t362 + t366 * t139 + t373 * t396 / f64x8::splat(10.0);
            let t404 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t269 * t141 - t278 - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t28 * t399));
            let t405 = t147 * t262;
            let t407 = ((t15).select(f64x8::splat(0.0), (t11).select(f64x8::splat(0.0), -t8 - t405)));
            let t410 = ((t151).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t152 * t407));
            let t411 = t410 * t27;
            let t415 = t154 * t274;
            let t418 = t6 * t415 * t256 / f64x8::splat(8.0);
            let t420 = ((t146).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t411 * t256 - t418));
            let tvrho0 = t145 + t260 + t7 * (t404 + t420);
            acc_vrho_0 = tvrho0;
            let t424 = ((t11).select(f64x8::splat(0.0), (t15).select(f64x8::splat(0.0), -t8 - t263)));
            let t427 = ((t21).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t24 * t424));
            let t428 = t427 * t27;
            let t433 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t428 * t141 - t278));
            let t435 = ((t15).select(f64x8::splat(0.0), (t11).select(f64x8::splat(0.0), t8 - t405)));
            let t438 = ((t151).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t152 * t435));
            let t439 = t438 * t27;
            let t443 = v_tau1 * t163;
            let t445 = t161 * v_rho1;
            let t447 = f64x8::splat(1.0) / t157 / t445;
            let t448 = v_sigma2 * t447;
            let t450 = -f64x8::splat(5.0) / f64x8::splat(3.0) * t443 + t448 / f64x8::splat(3.0);
            let t451 = t450 * t40;
            let t453 = t187 * t187;
            let t455 = t45 / t453;
            let t456 = t46 * t448;
            let t458 = t173 * t182;
            let t459 = t458 * t185;
            let t460 = t451 * t45;
            let t463 = t179 * t179;
            let t465 = f64x8::splat(1.0) / t180 / t463;
            let t466 = t174 * t465;
            let t467 = t466 * t185;
            let t468 = t166 * t55;
            let t469 = t58 * t450;
            let t470 = t468 * t469;
            let t473 = t183 * t40;
            let t474 = t447 * t185;
            let t478 = -t456 / f64x8::splat(9.0) - f64x8::splat(5.0) / f64x8::splat(3.0) * t459 * t460 - f64x8::splat(2.0144675925925926) * t467 * t470 + t473 * t233 * t474 / f64x8::splat(3.0);
            let t479 = t455 * t478;
            let t482 = -t167 * t479 / f64x8::splat(3.0) + t451 * t189 / f64x8::splat(3.0);
            let t485 = t193 * t193;
            let t486 = f64x8::splat(1.0) / t485;
            let t487 = t486 * t482;
            let t489 = -f64x8::splat(2.0) * t196 * t482 - f64x8::splat(9.0) * t487;
            let t490 = t489 * t201;
            let t493 = f64x8::splat(1.0) / t200 / t199;
            let t494 = t198 * t493;
            let t495 = t227 * t486;
            let t496 = t495 * t482;
            let t500 = f64x8::splat(1.0) / t213 / t212;
            let t502 = t205 * t161;
            let t504 = f64x8::splat(1.0) / t156 / t502;
            let t506 = t86 * t204 * t504;
            let t508 = -f64x8::splat(0.40121303703703703) * t456 - f64x8::splat(0.014346394913206906) * t506;
            let t518 = -f64x8::splat(0.17051554074074074) * t456 - f64x8::splat(5.0) / f64x8::splat(9.0) * (-f64x8::splat(0.24256886666666666) * t443 - f64x8::splat(0.031646617777777775) * t448) * t40 * t45;
            let t522 = f64x8::splat(1.0) / t223 / t212;
            let t523 = t222 * t522;
            let t526 = -t500 * t508 / f64x8::splat(5.0) + f64x8::splat(7.0) / f64x8::splat(9.0) * t518 * t224 - f64x8::splat(14.0) / f64x8::splat(45.0) * t523 * t508;
            let t530 = -f64x8::splat(6.0) * t494 * t487 - t490;
            let t532 = t254 * t254;
            let t533 = t532 * t532;
            let t534 = t533 * t533;
            let t535 = t534 * t254;
            let t536 = f64x8::splat(1.0) / t535;
            let t537 = t229 * t536;
            let t544 = t460 / f64x8::splat(4.0) - f64x8::splat(2.0) / f64x8::splat(27.0) * t456;
            let t547 = t544 * t248;
            let t550 = f64x8::splat(1.0) / t161;
            let t551 = v_sigma2 * t550;
            let t554 = ((t247).select(-t551 * t244 / f64x8::splat(8.0), f64x8::splat(0.0)));
            let t560 = -f64x8::splat(125.0) / f64x8::splat(39366.0) * t506 - f64x8::splat(10.0) / f64x8::splat(9.0) * t232 * t233 * t447 + f64x8::splat(584.0) / f64x8::splat(405.0) * t239 * t544 - f64x8::splat(146.0) / f64x8::splat(135.0) * t547 * t250 - f64x8::splat(146.0) / f64x8::splat(135.0) * t239 * t554 * t250 + f64x8::splat(146.0) / f64x8::splat(135.0) * t249 * t554;
            let t563 = t490 * t227 + f64x8::splat(6.0) * t494 * t496 + t202 * t526 + t530 * t254 + t537 * t560 / f64x8::splat(10.0);
            let t568 = ((t146).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t439 * t256 - t418 - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t155 * t563));
            let tvrho1 = t145 + t260 + t7 * (t433 + t568);
            acc_vrho_1 = tvrho1;
            let t571 = t36 * t40;
            let t572 = t571 * t71;
            let t574 = t571 * t45;
            let t576 = t295 * t574;
            let t578 = t58 * t36;
            let t579 = t304 * t578;
            let t580 = t303 * t579;
            let t583 = t46 * t67;
            let t586 = t574 / f64x8::splat(24.0) + f64x8::splat(5.0) / f64x8::splat(24.0) * t576 + f64x8::splat(0.25180844907407407) * t580 - t65 * t36 * t583 / f64x8::splat(8.0);
            let t587 = t291 * t586;
            let t590 = -t572 / f64x8::splat(24.0) - t41 * t587 / f64x8::splat(3.0);
            let t593 = t322 * t590;
            let t595 = -f64x8::splat(2.0) * t78 * t590 - f64x8::splat(9.0) * t593;
            let t596 = t595 * t83;
            let t598 = t331 * t590;
            let t602 = v_sigma0 * t91;
            let t603 = t86 * t602;
            let t605 = f64x8::splat(0.1504548888888889) * t574 + f64x8::splat(0.00537989809245259) * t603;
            let t608 = t45 * t109;
            let t613 = -t336 * t605 / f64x8::splat(5.0) + f64x8::splat(0.04460577520576132) * t571 * t608 - f64x8::splat(14.0) / f64x8::splat(45.0) * t359 * t605;
            let t617 = -f64x8::splat(6.0) * t330 * t593 - t596;
            let t624 = t124 * t36 * t46;
            let t626 = t45 * t133;
            let t627 = t626 * t135;
            let t628 = t571 * t627;
            let t632 = ((t132).select(t127 * t129 / f64x8::splat(8.0), f64x8::splat(0.0)));
            let t633 = t124 * t632;
            let t638 = f64x8::splat(125.0) / f64x8::splat(104976.0) * t603 + f64x8::splat(5.0) / f64x8::splat(12.0) * t117 * t45 * t36 - f64x8::splat(73.0) / f64x8::splat(14580.0) * t624 + f64x8::splat(73.0) / f64x8::splat(19440.0) * t628 - f64x8::splat(146.0) / f64x8::splat(135.0) * t633 * t135 + f64x8::splat(146.0) / f64x8::splat(135.0) * t134 * t632;
            let t641 = t596 * t112 + f64x8::splat(6.0) * t330 * t598 + t84 * t613 + t617 * t139 + t373 * t638 / f64x8::splat(10.0);
            let t645 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t28 * t641));
            let tvsigma0 = t7 * t645;
            acc_vsigma_0 = tvsigma0;
            let tvsigma1 = f64x8::splat(0.0);
            acc_vsigma_1 = tvsigma1;
            let t646 = t163 * t40;
            let t647 = t646 * t189;
            let t649 = t646 * t45;
            let t651 = t459 * t649;
            let t653 = t58 * t163;
            let t654 = t468 * t653;
            let t655 = t467 * t654;
            let t658 = t46 * t185;
            let t661 = t649 / f64x8::splat(24.0) + f64x8::splat(5.0) / f64x8::splat(24.0) * t651 + f64x8::splat(0.25180844907407407) * t655 - t183 * t163 * t658 / f64x8::splat(8.0);
            let t662 = t455 * t661;
            let t665 = -t647 / f64x8::splat(24.0) - t167 * t662 / f64x8::splat(3.0);
            let t668 = t486 * t665;
            let t670 = -f64x8::splat(2.0) * t196 * t665 - f64x8::splat(9.0) * t668;
            let t671 = t670 * t201;
            let t673 = t495 * t665;
            let t677 = v_sigma2 * t208;
            let t678 = t86 * t677;
            let t680 = f64x8::splat(0.1504548888888889) * t649 + f64x8::splat(0.00537989809245259) * t678;
            let t683 = t45 * t224;
            let t688 = -t500 * t680 / f64x8::splat(5.0) + f64x8::splat(0.04460577520576132) * t646 * t683 - f64x8::splat(14.0) / f64x8::splat(45.0) * t523 * t680;
            let t692 = -f64x8::splat(6.0) * t494 * t668 - t671;
            let t699 = t239 * t163 * t46;
            let t701 = t45 * t248;
            let t702 = t701 * t250;
            let t703 = t646 * t702;
            let t707 = ((t247).select(t242 * t244 / f64x8::splat(8.0), f64x8::splat(0.0)));
            let t708 = t239 * t707;
            let t713 = f64x8::splat(125.0) / f64x8::splat(104976.0) * t678 + f64x8::splat(5.0) / f64x8::splat(12.0) * t232 * t45 * t163 - f64x8::splat(73.0) / f64x8::splat(14580.0) * t699 + f64x8::splat(73.0) / f64x8::splat(19440.0) * t703 - f64x8::splat(146.0) / f64x8::splat(135.0) * t708 * t250 + f64x8::splat(146.0) / f64x8::splat(135.0) * t249 * t707;
            let t716 = t671 * t227 + f64x8::splat(6.0) * t494 * t673 + t202 * t688 + t692 * t254 + t537 * t713 / f64x8::splat(10.0);
            let t720 = ((t146).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t155 * t716));
            let tvsigma2 = t7 * t720;
            acc_vsigma_2 = tvsigma2;
            let tvlapl0 = f64x8::splat(0.0);
            acc_vlapl_0 = tvlapl0;
            let tvlapl1 = f64x8::splat(0.0);
            acc_vlapl_1 = tvlapl1;
            let t721 = t32 * t40;
            let t723 = t721 * t45;
            let t726 = t58 * t32;
            let t727 = t304 * t726;
            let t730 = -f64x8::splat(5.0) / f64x8::splat(3.0) * t295 * t723 - f64x8::splat(2.0144675925925926) * t303 * t727;
            let t731 = t291 * t730;
            let t734 = -t41 * t731 / f64x8::splat(3.0) + t721 * t71 / f64x8::splat(3.0);
            let t737 = t322 * t734;
            let t739 = -f64x8::splat(2.0) * t78 * t734 - f64x8::splat(9.0) * t737;
            let t740 = t739 * t83;
            let t742 = t331 * t734;
            let t745 = t84 * t32;
            let t746 = t46 * t109;
            let t751 = -f64x8::splat(6.0) * t330 * t737 - t740;
            let t758 = v_tau0 * v_tau0;
            let t759 = f64x8::splat(1.0) / t758;
            let t762 = ((t132).select(-t128 * t759 / f64x8::splat(8.0), f64x8::splat(0.0)));
            let t763 = t124 * t762;
            let t768 = f64x8::splat(146.0) / f64x8::splat(405.0) * t124 * t32 * t46 - f64x8::splat(73.0) / f64x8::splat(270.0) * t721 * t627 - f64x8::splat(146.0) / f64x8::splat(135.0) * t763 * t135 + f64x8::splat(146.0) / f64x8::splat(135.0) * t134 * t762;
            let t771 = t740 * t112 + f64x8::splat(6.0) * t330 * t742 - f64x8::splat(0.06288822469135802) * t745 * t746 + t751 * t139 + t373 * t768 / f64x8::splat(10.0);
            let t775 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t28 * t771));
            let tvtau0 = t7 * t775;
            acc_vtau_0 = tvtau0;
            let t776 = t159 * t40;
            let t778 = t776 * t45;
            let t781 = t58 * t159;
            let t782 = t468 * t781;
            let t785 = -f64x8::splat(5.0) / f64x8::splat(3.0) * t459 * t778 - f64x8::splat(2.0144675925925926) * t467 * t782;
            let t786 = t455 * t785;
            let t789 = -t167 * t786 / f64x8::splat(3.0) + t776 * t189 / f64x8::splat(3.0);
            let t792 = t486 * t789;
            let t794 = -f64x8::splat(2.0) * t196 * t789 - f64x8::splat(9.0) * t792;
            let t795 = t794 * t201;
            let t797 = t495 * t789;
            let t800 = t202 * t159;
            let t801 = t46 * t224;
            let t806 = -f64x8::splat(6.0) * t494 * t792 - t795;
            let t813 = v_tau1 * v_tau1;
            let t814 = f64x8::splat(1.0) / t813;
            let t817 = ((t247).select(-t243 * t814 / f64x8::splat(8.0), f64x8::splat(0.0)));
            let t818 = t239 * t817;
            let t823 = f64x8::splat(146.0) / f64x8::splat(405.0) * t239 * t159 * t46 - f64x8::splat(73.0) / f64x8::splat(270.0) * t776 * t702 - f64x8::splat(146.0) / f64x8::splat(135.0) * t818 * t250 + f64x8::splat(146.0) / f64x8::splat(135.0) * t249 * t817;
            let t826 = t795 * t227 + f64x8::splat(6.0) * t494 * t797 - f64x8::splat(0.06288822469135802) * t800 * t801 + t806 * t254 + t537 * t823 / f64x8::splat(10.0);
            let t830 = ((t146).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t155 * t826));
            let tvtau1 = t7 * t830;
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
