//! MGGA_C_RMGGAC vxc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_c_rmggac.c`
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
pub fn mgga_c_rmggac_vxc_pol(
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
            let t2 = f64x8::splat(M_CBRT3);
            let t3 = f64x8::splat(1.0) / f64x8::splat(M_PI);
            let t4 = (simd::cbrt(t3));
            let t5 = t2 * t4;
            let t6 = f64x8::splat(M_CBRT4);
            let t7 = t6 * t6;
            let t8 = v_rho0 + v_rho1;
            let t9 = (simd::cbrt(t8));
            let t12 = t5 * t7 / t9;
            let t13 = ((t12).sqrt());
            let t16 = f64x8::splat(1.0) + f64x8::splat(0.04445) * t13 + f64x8::splat(0.03138525) * t12;
            let t17 = f64x8::splat(1.0) / t16;
            let t20 = (simd::exp(f64x8::splat(1.0) * t17));
            let t21 = t20 - f64x8::splat(1.0);
            let t22 = f64x8::splat(M_CBRT6);
            let t23 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t24 = (simd::cbrt(t23));
            let t25 = t24 * t24;
            let t26 = f64x8::splat(1.0) / t25;
            let t27 = t22 * t26;
            let t28 = f64x8::splat(M_CBRT2);
            let t29 = t28 * t28;
            let t31 = v_sigma0 + f64x8::splat(2.0) * v_sigma1 + v_sigma2;
            let t32 = t29 * t31;
            let t33 = t8 * t8;
            let t34 = t9 * t9;
            let t36 = f64x8::splat(1.0) / t34 / t33;
            let t38 = t27 * t32 * t36;
            let t40 = f64x8::splat(1.0) + f64x8::splat(0.02133764210437636) * t38;
            let t41 = ((t40).sqrt().sqrt());
            let t43 = f64x8::splat(1.0) - f64x8::splat(1.0) / t41;
            let t45 = t21 * t43 + f64x8::splat(1.0);
            let t46 = (simd::ln(t45));
            let t48 = -f64x8::splat(0.0285764) * t17 + f64x8::splat(0.0285764) * t46;
            let t49 = t28 - f64x8::splat(1.0);
            let t50 = v_rho0 - v_rho1;
            let t51 = f64x8::splat(1.0) / t8;
            let t52 = t50 * t51;
            let t53 = f64x8::splat(1.0) + t52;
            let t54 = (t53).simd_le(zeta_threshold);
            let t55 = (simd::cbrt(zeta_threshold));
            let t56 = t55 * zeta_threshold;
            let t57 = (simd::cbrt(t53));
            let t58 = t57 * t53;
            let t59 = ((t54).select(t56, t58));
            let t60 = f64x8::splat(1.0) - t52;
            let t61 = (t60).simd_le(zeta_threshold);
            let t62 = (simd::cbrt(t60));
            let t63 = t62 * t60;
            let t64 = ((t61).select(t56, t63));
            let t65 = t59 + t64 - f64x8::splat(2.0);
            let t68 = f64x8::splat(1.0) / t49 / f64x8::splat(2.0);
            let t71 = f64x8::splat(1.0) - f64x8::splat(2.363) * t49 * t65 * t68;
            let t72 = t48 * t71;
            let t73 = t50 * t50;
            let t74 = t73 * t73;
            let t75 = t74 * t74;
            let t76 = t75 * t74;
            let t77 = t33 * t33;
            let t78 = t77 * t77;
            let t79 = t78 * t77;
            let t80 = f64x8::splat(1.0) / t79;
            let t82 = -t76 * t80 + f64x8::splat(1.0);
            let t83 = (simd::cbrt(v_rho0));
            let t84 = t83 * t83;
            let t86 = f64x8::splat(1.0) / t84 / v_rho0;
            let t87 = v_tau0 * t86;
            let t88 = t53 / f64x8::splat(2.0);
            let t89 = (simd::cbrt(t88));
            let t90 = t89 * t89;
            let t91 = t90 * t88;
            let t94 = (simd::cbrt(v_rho1));
            let t95 = t94 * t94;
            let t97 = f64x8::splat(1.0) / t95 / v_rho1;
            let t98 = v_tau1 * t97;
            let t99 = t60 / f64x8::splat(2.0);
            let t100 = (simd::cbrt(t99));
            let t101 = t100 * t100;
            let t102 = t101 * t99;
            let t107 = f64x8::splat(2.0) * t87 * t91 + f64x8::splat(2.0) * t98 * t102 - t31 * t36 / f64x8::splat(4.0);
            let t108 = t107 * t107;
            let t109 = t108 * t107;
            let t114 = f64x8::splat(0.08) + f64x8::splat(5.0) / f64x8::splat(18.0) * t107 * t29 * t27 + f64x8::splat(0.0125) * t38;
            let t115 = t114 * t114;
            let t116 = t115 * t114;
            let t117 = f64x8::splat(1.0) / t116;
            let t118 = t109 * t117;
            let t120 = t108 * t108;
            let t121 = t120 * t108;
            let t122 = t115 * t115;
            let t124 = f64x8::splat(1.0) / t122 / t115;
            let t127 = f64x8::splat(1.0) + f64x8::splat(0.006652356501035449) * t118 + f64x8::splat(4.42538470168686e-05) * t121 * t124;
            let t128 = f64x8::splat(1.0) / t127;
            let t129 = t118 * t128;
            let t131 = f64x8::splat(1.0) - f64x8::splat(0.01995706950310635) * t129;
            let t132 = t82 * t131;
            let t133 = t72 * t132;
            let t135 = f64x8::splat(1.0) + f64x8::splat(0.053425) * t12;
            let t138 = ((t12) * (t12).sqrt());
            let t140 = t2 * t2;
            let t141 = t4 * t4;
            let t142 = t140 * t141;
            let t145 = t142 * t6 / t34;
            let t147 = f64x8::splat(3.79785) * t13 + f64x8::splat(0.8969) * t12 + f64x8::splat(0.204775) * t138 + f64x8::splat(0.123235) * t145;
            let t150 = f64x8::splat(1.0) + f64x8::splat(16.081979498692537) / t147;
            let t151 = (simd::ln(t150));
            let t153 = f64x8::splat(0.0621814) * t135 * t151;
            let t154 = f64x8::splat(1.0) / t77;
            let t155 = t74 * t154;
            let t156 = t65 * t68;
            let t158 = f64x8::splat(1.0) + f64x8::splat(0.05137) * t12;
            let t163 = f64x8::splat(7.05945) * t13 + f64x8::splat(1.549425) * t12 + f64x8::splat(0.420775) * t138 + f64x8::splat(0.1562925) * t145;
            let t166 = f64x8::splat(1.0) + f64x8::splat(32.16395899738507) / t163;
            let t167 = (simd::ln(t166));
            let t171 = f64x8::splat(1.0) + f64x8::splat(0.0278125) * t12;
            let t176 = f64x8::splat(5.1785) * t13 + f64x8::splat(0.905775) * t12 + f64x8::splat(0.1100325) * t138 + f64x8::splat(0.1241775) * t145;
            let t179 = f64x8::splat(1.0) + f64x8::splat(29.608749977793437) / t176;
            let t180 = (simd::ln(t179));
            let t181 = t171 * t180;
            let t183 = -f64x8::splat(0.0310907) * t158 * t167 + t153 - f64x8::splat(0.0197516734986138) * t181;
            let t184 = t156 * t183;
            let t185 = t155 * t184;
            let t187 = f64x8::splat(0.0197516734986138) * t156 * t181;
            let t188 = t55 * t55;
            let t189 = t57 * t57;
            let t190 = ((t54).select(t188, t189));
            let t191 = t62 * t62;
            let t192 = ((t61).select(t188, t191));
            let t194 = t190 / f64x8::splat(2.0) + t192 / f64x8::splat(2.0);
            let t195 = t194 * t194;
            let t196 = t195 * t194;
            let t197 = -t153 + t185 + t187;
            let t198 = f64x8::splat(1.0) / t196;
            let t201 = (simd::exp(-f64x8::splat(32.16364864430221) * t197 * t198));
            let t202 = t201 - f64x8::splat(1.0);
            let t203 = (simd::ln(f64x8::splat(2.0)));
            let t204 = f64x8::splat(1.0) - t203;
            let t205 = f64x8::splat(1.0) / t204;
            let t206 = t197 * t205;
            let t207 = t23 * t198;
            let t209 = (simd::exp(-t206 * t207));
            let t210 = t209 - f64x8::splat(1.0);
            let t211 = f64x8::splat(1.0) / t210;
            let t212 = t205 * t211;
            let t214 = f64x8::splat(1.0) / t9 / t33;
            let t215 = t31 * t214;
            let t217 = f64x8::splat(1.0) / t195;
            let t219 = f64x8::splat(1.0) / t4;
            let t220 = t140 * t219;
            let t221 = t220 * t6;
            let t222 = t28 * t217 * t221;
            let t225 = f64x8::splat(1.0) + f64x8::splat(0.02743955640261198) * t212 * t215 * t222;
            let t226 = ((t225).sqrt().sqrt());
            let t228 = f64x8::splat(1.0) - f64x8::splat(1.0) / t226;
            let t230 = t202 * t228 + f64x8::splat(1.0);
            let t231 = (simd::ln(t230));
            let t234 = -t153 + t185 + t187 + f64x8::splat(0.031091) * t196 * t231;
            let t235 = t234 * t109;
            let t236 = t117 * t128;
            let t238 = f64x8::splat(0.01995706950310635) * t235 * t236;
            let tzk0 = t133 + t238;
            acc_zk = tzk0;
            let t239 = t16 * t16;
            let t240 = f64x8::splat(1.0) / t239;
            let t242 = f64x8::splat(1.0) / t13 * t2;
            let t243 = t4 * t7;
            let t245 = f64x8::splat(1.0) / t9 / t8;
            let t246 = t243 * t245;
            let t247 = t242 * t246;
            let t249 = t7 * t245;
            let t250 = t5 * t249;
            let t252 = -f64x8::splat(0.007408333333333334) * t247 - f64x8::splat(0.01046175) * t250;
            let t253 = t240 * t252;
            let t255 = t20 * t43;
            let t259 = f64x8::splat(1.0) / t41 / t40;
            let t260 = t21 * t259;
            let t261 = t260 * t22;
            let t262 = t26 * t29;
            let t263 = t33 * t8;
            let t265 = f64x8::splat(1.0) / t34 / t263;
            let t266 = t31 * t265;
            let t270 = -f64x8::splat(1.0) * t253 * t255 - f64x8::splat(0.014225094736250906) * t261 * t262 * t266;
            let t271 = f64x8::splat(1.0) / t45;
            let t274 = f64x8::splat(0.0285764) * t253 + f64x8::splat(0.0285764) * t270 * t271;
            let t275 = t274 * t71;
            let t276 = t275 * t132;
            let t277 = t48 * t49;
            let t278 = f64x8::splat(1.0) / t33;
            let t279 = t50 * t278;
            let t280 = t51 - t279;
            let t283 = ((t54).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t57 * t280));
            let t284 = -t280;
            let t287 = ((t61).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t62 * t284));
            let t288 = t283 + t287;
            let t289 = t277 * t288;
            let t290 = t68 * t82;
            let t291 = t290 * t131;
            let t292 = t289 * t291;
            let t293 = f64x8::splat(2.363) * t292;
            let t294 = t73 * t50;
            let t295 = t75 * t294;
            let t296 = t295 * t80;
            let t297 = t77 * t8;
            let t298 = t78 * t297;
            let t299 = f64x8::splat(1.0) / t298;
            let t300 = t76 * t299;
            let t302 = -f64x8::splat(12.0) * t296 + f64x8::splat(12.0) * t300;
            let t303 = t302 * t131;
            let t304 = t72 * t303;
            let t305 = t108 * t117;
            let t306 = v_rho0 * v_rho0;
            let t308 = f64x8::splat(1.0) / t84 / t306;
            let t309 = v_tau0 * t308;
            let t312 = t280 / f64x8::splat(2.0);
            let t313 = t90 * t312;
            let t316 = -t312;
            let t317 = t101 * t316;
            let t320 = f64x8::splat(2.0) / f64x8::splat(3.0) * t266;
            let t321 = -f64x8::splat(10.0) / f64x8::splat(3.0) * t309 * t91 + f64x8::splat(10.0) / f64x8::splat(3.0) * t87 * t313 + f64x8::splat(10.0) / f64x8::splat(3.0) * t98 * t317 + t320;
            let t322 = t128 * t321;
            let t323 = t305 * t322;
            let t325 = f64x8::splat(1.0) / t122;
            let t326 = t109 * t325;
            let t328 = t321 * t29 * t27;
            let t331 = t27 * t32 * t265;
            let t332 = f64x8::splat(0.03333333333333333) * t331;
            let t333 = f64x8::splat(5.0) / f64x8::splat(18.0) * t328 - t332;
            let t334 = t128 * t333;
            let t335 = t326 * t334;
            let t337 = t127 * t127;
            let t338 = f64x8::splat(1.0) / t337;
            let t343 = t120 * t107;
            let t344 = t343 * t124;
            let t348 = f64x8::splat(1.0) / t122 / t116;
            let t349 = t121 * t348;
            let t352 = f64x8::splat(0.01995706950310635) * t305 * t321 - f64x8::splat(0.01995706950310635) * t326 * t333 + f64x8::splat(0.0002655230821012116) * t344 * t321 - f64x8::splat(0.0002655230821012116) * t349 * t333;
            let t353 = t338 * t352;
            let t354 = t118 * t353;
            let t356 = -f64x8::splat(0.05987120850931904) * t323 + f64x8::splat(0.05987120850931904) * t335 + f64x8::splat(0.01995706950310635) * t354;
            let t357 = t82 * t356;
            let t358 = t72 * t357;
            let t361 = f64x8::splat(0.0011073470983333333) * t5 * t249 * t151;
            let t362 = t147 * t147;
            let t363 = f64x8::splat(1.0) / t362;
            let t364 = t135 * t363;
            let t367 = ((t12).sqrt());
            let t368 = t367 * t2;
            let t369 = t368 * t246;
            let t374 = t142 * t6 / t34 / t8;
            let t376 = -f64x8::splat(0.632975) * t247 - f64x8::splat(0.29896666666666666) * t250 - f64x8::splat(0.1023875) * t369 - f64x8::splat(0.08215666666666667) * t374;
            let t377 = f64x8::splat(1.0) / t150;
            let t378 = t376 * t377;
            let t380 = f64x8::splat(1.0) * t364 * t378;
            let t381 = t294 * t154;
            let t383 = f64x8::splat(4.0) * t381 * t184;
            let t384 = f64x8::splat(1.0) / t297;
            let t385 = t74 * t384;
            let t387 = f64x8::splat(4.0) * t385 * t184;
            let t388 = t288 * t68;
            let t389 = t388 * t183;
            let t390 = t155 * t389;
            let t394 = t163 * t163;
            let t395 = f64x8::splat(1.0) / t394;
            let t396 = t158 * t395;
            let t401 = -f64x8::splat(1.176575) * t247 - f64x8::splat(0.516475) * t250 - f64x8::splat(0.2103875) * t369 - f64x8::splat(0.104195) * t374;
            let t402 = f64x8::splat(1.0) / t166;
            let t403 = t401 * t402;
            let t409 = t176 * t176;
            let t410 = f64x8::splat(1.0) / t409;
            let t411 = t171 * t410;
            let t416 = -f64x8::splat(0.8630833333333333) * t247 - f64x8::splat(0.301925) * t250 - f64x8::splat(0.05501625) * t369 - f64x8::splat(0.082785) * t374;
            let t417 = f64x8::splat(1.0) / t179;
            let t418 = t416 * t417;
            let t421 = f64x8::splat(0.0005323764196666666) * t5 * t249 * t167 + f64x8::splat(1.0) * t396 * t403 - t361 - t380 + f64x8::splat(0.00018311447306006544) * t5 * t249 * t180 + f64x8::splat(0.5848223622634646) * t411 * t418;
            let t422 = t156 * t421;
            let t423 = t155 * t422;
            let t425 = f64x8::splat(0.0197516734986138) * t388 * t181;
            let t426 = t156 * t2;
            let t428 = t243 * t245 * t180;
            let t430 = f64x8::splat(0.00018311447306006544) * t426 * t428;
            let t431 = t156 * t171;
            let t433 = t410 * t416 * t417;
            let t435 = f64x8::splat(0.5848223622634646) * t431 * t433;
            let t436 = t195 * t231;
            let t437 = f64x8::splat(1.0) / t57;
            let t440 = ((t54).select(f64x8::splat(0.0), f64x8::splat(2.0) / f64x8::splat(3.0) * t437 * t280));
            let t441 = f64x8::splat(1.0) / t62;
            let t444 = ((t61).select(f64x8::splat(0.0), f64x8::splat(2.0) / f64x8::splat(3.0) * t441 * t284));
            let t446 = t440 / f64x8::splat(2.0) + t444 / f64x8::splat(2.0);
            let t449 = t361 + t380 + t383 - t387 + t390 + t423 + t425 - t430 - t435;
            let t452 = t195 * t195;
            let t453 = f64x8::splat(1.0) / t452;
            let t454 = t197 * t453;
            let t457 = -f64x8::splat(32.16364864430221) * t449 * t198 + f64x8::splat(96.49094593290663) * t454 * t446;
            let t458 = t457 * t201;
            let t459 = t458 * t228;
            let t461 = f64x8::splat(1.0) / t226 / t225;
            let t462 = t202 * t461;
            let t463 = t210 * t210;
            let t464 = f64x8::splat(1.0) / t463;
            let t465 = t205 * t464;
            let t466 = t215 * t28;
            let t467 = t465 * t466;
            let t469 = t217 * t140 * t219;
            let t470 = t449 * t205;
            let t472 = t23 * t453;
            let t473 = t472 * t446;
            let t476 = f64x8::splat(3.0) * t206 * t473 - t207 * t470;
            let t479 = t469 * t6 * t476 * t209;
            let t483 = f64x8::splat(1.0) / t9 / t263;
            let t484 = t31 * t483;
            let t487 = f64x8::splat(0.06402563160609462) * t212 * t484 * t222;
            let t488 = t212 * t466;
            let t489 = t198 * t140;
            let t490 = t219 * t6;
            let t492 = t489 * t490 * t446;
            let t495 = -f64x8::splat(0.02743955640261198) * t467 * t479 - t487 - f64x8::splat(0.05487911280522396) * t488 * t492;
            let t498 = t459 + t462 * t495 / f64x8::splat(4.0);
            let t499 = t196 * t498;
            let t500 = f64x8::splat(1.0) / t230;
            let t503 = t361 + t380 + t383 - t387 + t390 + t423 + t425 - t430 - t435 + f64x8::splat(0.093273) * t436 * t446 + f64x8::splat(0.031091) * t499 * t500;
            let t504 = t503 * t109;
            let t505 = t504 * t236;
            let t506 = f64x8::splat(0.01995706950310635) * t505;
            let t507 = t234 * t108;
            let t508 = t236 * t321;
            let t509 = t507 * t508;
            let t510 = f64x8::splat(0.05987120850931904) * t509;
            let t511 = t325 * t128;
            let t512 = t511 * t333;
            let t513 = t235 * t512;
            let t514 = f64x8::splat(0.05987120850931904) * t513;
            let t515 = t117 * t338;
            let t516 = t515 * t352;
            let t517 = t235 * t516;
            let t518 = f64x8::splat(0.01995706950310635) * t517;
            let tvrho0 = t133 + t238 + t8 * (t276 - t293 + t304 + t358 + t506 + t510 - t514 - t518);
            acc_vrho_0 = tvrho0;
            let t521 = -t51 - t279;
            let t524 = ((t54).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t57 * t521));
            let t525 = -t521;
            let t528 = ((t61).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t62 * t525));
            let t529 = t524 + t528;
            let t530 = t277 * t529;
            let t531 = t530 * t291;
            let t532 = f64x8::splat(2.363) * t531;
            let t534 = f64x8::splat(12.0) * t296 + f64x8::splat(12.0) * t300;
            let t535 = t534 * t131;
            let t536 = t72 * t535;
            let t537 = t521 / f64x8::splat(2.0);
            let t538 = t90 * t537;
            let t541 = v_rho1 * v_rho1;
            let t543 = f64x8::splat(1.0) / t95 / t541;
            let t544 = v_tau1 * t543;
            let t547 = -t537;
            let t548 = t101 * t547;
            let t551 = f64x8::splat(10.0) / f64x8::splat(3.0) * t87 * t538 - f64x8::splat(10.0) / f64x8::splat(3.0) * t544 * t102 + f64x8::splat(10.0) / f64x8::splat(3.0) * t98 * t548 + t320;
            let t552 = t128 * t551;
            let t553 = t305 * t552;
            let t556 = t551 * t29 * t27;
            let t558 = f64x8::splat(5.0) / f64x8::splat(18.0) * t556 - t332;
            let t559 = t128 * t558;
            let t560 = t326 * t559;
            let t570 = f64x8::splat(0.01995706950310635) * t305 * t551 - f64x8::splat(0.01995706950310635) * t326 * t558 + f64x8::splat(0.0002655230821012116) * t344 * t551 - f64x8::splat(0.0002655230821012116) * t349 * t558;
            let t571 = t338 * t570;
            let t572 = t118 * t571;
            let t574 = -f64x8::splat(0.05987120850931904) * t553 + f64x8::splat(0.05987120850931904) * t560 + f64x8::splat(0.01995706950310635) * t572;
            let t575 = t82 * t574;
            let t576 = t72 * t575;
            let t577 = t529 * t68;
            let t578 = t577 * t183;
            let t579 = t155 * t578;
            let t581 = f64x8::splat(0.0197516734986138) * t577 * t181;
            let t584 = ((t54).select(f64x8::splat(0.0), f64x8::splat(2.0) / f64x8::splat(3.0) * t437 * t521));
            let t587 = ((t61).select(f64x8::splat(0.0), f64x8::splat(2.0) / f64x8::splat(3.0) * t441 * t525));
            let t589 = t584 / f64x8::splat(2.0) + t587 / f64x8::splat(2.0);
            let t592 = t361 + t380 - t383 - t387 + t579 + t423 + t581 - t430 - t435;
            let t597 = -f64x8::splat(32.16364864430221) * t592 * t198 + f64x8::splat(96.49094593290663) * t454 * t589;
            let t598 = t597 * t201;
            let t600 = t592 * t205;
            let t602 = t472 * t589;
            let t605 = f64x8::splat(3.0) * t206 * t602 - t207 * t600;
            let t608 = t469 * t6 * t605 * t209;
            let t612 = t489 * t490 * t589;
            let t615 = -f64x8::splat(0.02743955640261198) * t467 * t608 - t487 - f64x8::splat(0.05487911280522396) * t488 * t612;
            let t618 = t598 * t228 + t462 * t615 / f64x8::splat(4.0);
            let t619 = t196 * t618;
            let t622 = t361 + t380 - t383 - t387 + t579 + t423 + t581 - t430 - t435 + f64x8::splat(0.093273) * t436 * t589 + f64x8::splat(0.031091) * t619 * t500;
            let t623 = t622 * t109;
            let t624 = t623 * t236;
            let t625 = f64x8::splat(0.01995706950310635) * t624;
            let t626 = t236 * t551;
            let t627 = t507 * t626;
            let t628 = f64x8::splat(0.05987120850931904) * t627;
            let t629 = t511 * t558;
            let t630 = t235 * t629;
            let t631 = f64x8::splat(0.05987120850931904) * t630;
            let t632 = t515 * t570;
            let t633 = t235 * t632;
            let t634 = f64x8::splat(0.01995706950310635) * t633;
            let tvrho1 = t133 + t238 + t8 * (t276 - t532 + t536 + t576 + t625 + t628 - t631 - t634);
            acc_vrho_1 = tvrho1;
            let t637 = t27 * t29;
            let t638 = t260 * t637;
            let t639 = t36 * t271;
            let t640 = t71 * t82;
            let t641 = t640 * t131;
            let t643 = t638 * t639 * t641;
            let t644 = f64x8::splat(0.00015243824895787514) * t643;
            let t645 = t128 * t36;
            let t646 = t305 * t645;
            let t648 = t326 * t128;
            let t649 = t29 * t36;
            let t650 = t27 * t649;
            let t651 = t648 * t650;
            let t653 = t305 * t36;
            let t655 = t326 * t22;
            let t656 = t262 * t36;
            let t657 = t655 * t656;
            let t659 = t344 * t36;
            let t661 = t349 * t22;
            let t662 = t661 * t656;
            let t664 = -f64x8::splat(0.004989267375776587) * t653 + f64x8::splat(0.0011364442355935559) * t657 - f64x8::splat(6.63807705253029e-05) * t659 + f64x8::splat(1.5120064397430106e-05) * t662;
            let t665 = t338 * t664;
            let t666 = t118 * t665;
            let t668 = f64x8::splat(0.01496780212732976) * t646 - f64x8::splat(0.0034093327067806676) * t651 + f64x8::splat(0.01995706950310635) * t666;
            let t669 = t82 * t668;
            let t670 = t72 * t669;
            let t671 = t194 * t202;
            let t672 = t671 * t461;
            let t673 = t214 * t28;
            let t674 = t212 * t673;
            let t675 = t672 * t674;
            let t676 = t500 * t109;
            let t677 = t676 * t236;
            let t678 = t221 * t677;
            let t679 = t675 * t678;
            let t680 = f64x8::splat(4.256459989329784e-06) * t679;
            let t681 = t236 * t36;
            let t682 = t507 * t681;
            let t683 = f64x8::splat(0.01496780212732976) * t682;
            let t684 = t235 * t511;
            let t685 = t684 * t650;
            let t686 = f64x8::splat(0.0034093327067806676) * t685;
            let t687 = t515 * t664;
            let t689 = f64x8::splat(0.01995706950310635) * t235 * t687;
            let tvsigma0 = t8 * (t644 + t670 + t680 - t683 + t686 - t689);
            acc_vsigma_0 = tvsigma0;
            let t691 = f64x8::splat(0.0003048764979157503) * t643;
            let t698 = -f64x8::splat(0.009978534751553175) * t653 + f64x8::splat(0.0022728884711871117) * t657 - f64x8::splat(0.0001327615410506058) * t659 + f64x8::splat(3.0240128794860212e-05) * t662;
            let t699 = t338 * t698;
            let t700 = t118 * t699;
            let t702 = f64x8::splat(0.02993560425465952) * t646 - f64x8::splat(0.006818665413561335) * t651 + f64x8::splat(0.01995706950310635) * t700;
            let t703 = t82 * t702;
            let t704 = t72 * t703;
            let t705 = f64x8::splat(8.512919978659568e-06) * t679;
            let t706 = f64x8::splat(0.02993560425465952) * t682;
            let t707 = f64x8::splat(0.006818665413561335) * t685;
            let t708 = t515 * t698;
            let t710 = f64x8::splat(0.01995706950310635) * t235 * t708;
            let tvsigma1 = t8 * (t691 + t704 + t705 - t706 + t707 - t710);
            acc_vsigma_1 = tvsigma1;
            let tvsigma2 = tvsigma0;
            acc_vsigma_2 = tvsigma2;
            let tvlapl0 = f64x8::splat(0.0);
            acc_vlapl_0 = tvlapl0;
            let tvlapl1 = f64x8::splat(0.0);
            acc_vlapl_1 = tvlapl1;
            let t712 = t128 * t86;
            let t713 = t712 * t91;
            let t714 = t305 * t713;
            let t716 = t326 * t712;
            let t717 = t91 * t29;
            let t718 = t717 * t27;
            let t719 = t716 * t718;
            let t721 = t86 * t91;
            let t732 = f64x8::splat(0.0399141390062127) * t305 * t721 - f64x8::splat(0.011087260835059082) * t326 * t86 * t718 + f64x8::splat(0.0005310461642024232) * t344 * t721 - f64x8::splat(0.00014751282338956202) * t349 * t86 * t718;
            let t733 = t338 * t732;
            let t734 = t118 * t733;
            let t736 = -f64x8::splat(0.11974241701863808) * t714 + f64x8::splat(0.033261782505177244) * t719 + f64x8::splat(0.01995706950310635) * t734;
            let t737 = t82 * t736;
            let t738 = t72 * t737;
            let t739 = t507 * t117;
            let t741 = f64x8::splat(0.11974241701863808) * t739 * t713;
            let t742 = t721 * t637;
            let t744 = f64x8::splat(0.033261782505177244) * t684 * t742;
            let t745 = t515 * t732;
            let t747 = f64x8::splat(0.01995706950310635) * t235 * t745;
            let tvtau0 = t8 * (t738 + t741 - t744 - t747);
            acc_vtau_0 = tvtau0;
            let t749 = t128 * t97;
            let t750 = t749 * t102;
            let t751 = t305 * t750;
            let t753 = t326 * t749;
            let t754 = t102 * t29;
            let t755 = t754 * t27;
            let t756 = t753 * t755;
            let t758 = t97 * t102;
            let t769 = f64x8::splat(0.0399141390062127) * t305 * t758 - f64x8::splat(0.011087260835059082) * t326 * t97 * t755 + f64x8::splat(0.0005310461642024232) * t344 * t758 - f64x8::splat(0.00014751282338956202) * t349 * t97 * t755;
            let t770 = t338 * t769;
            let t771 = t118 * t770;
            let t773 = -f64x8::splat(0.11974241701863808) * t751 + f64x8::splat(0.033261782505177244) * t756 + f64x8::splat(0.01995706950310635) * t771;
            let t774 = t82 * t773;
            let t775 = t72 * t774;
            let t777 = f64x8::splat(0.11974241701863808) * t739 * t750;
            let t778 = t758 * t637;
            let t780 = f64x8::splat(0.033261782505177244) * t684 * t778;
            let t781 = t515 * t769;
            let t783 = f64x8::splat(0.01995706950310635) * t235 * t781;
            let tvtau1 = t8 * (t775 + t777 - t780 - t783);
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
