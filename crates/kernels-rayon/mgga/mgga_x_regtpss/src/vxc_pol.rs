//! MGGA_X_REGTPSS vxc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_regtpss.c`
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
pub fn mgga_x_regtpss_vxc_pol(
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
            let t29 = f64x8::splat(1.0) / v_rho0;
            let t30 = v_sigma0 * t29;
            let t31 = f64x8::splat(1.0) / v_tau0;
            let t32 = t30 * t31;
            let t33 = ((t32) * (t32) * (t32));
            let t34 = v_sigma0 * v_sigma0;
            let t35 = v_rho0 * v_rho0;
            let t36 = f64x8::splat(1.0) / t35;
            let t37 = t34 * t36;
            let t38 = v_tau0 * v_tau0;
            let t39 = f64x8::splat(1.0) / t38;
            let t40 = t37 * t39;
            let t42 = f64x8::splat(1.0) + t40 / f64x8::splat(64.0);
            let t43 = t42 * t42;
            let t44 = f64x8::splat(1.0) / t43;
            let t48 = f64x8::splat(M_CBRT6);
            let t49 = (f64x8::splat(10.0) / f64x8::splat(81.0) + f64x8::splat(0.0045938270703125) * t33 * t44) * t48;
            let t50 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t51 = (simd::cbrt(t50));
            let t52 = t51 * t51;
            let t53 = f64x8::splat(1.0) / t52;
            let t54 = t53 * v_sigma0;
            let t55 = (simd::cbrt(v_rho0));
            let t56 = t55 * t55;
            let t58 = f64x8::splat(1.0) / t56 / t35;
            let t59 = t54 * t58;
            let t63 = f64x8::splat(1.0) / t56 / v_rho0;
            let t65 = v_sigma0 * t58;
            let t67 = v_tau0 * t63 - t65 / f64x8::splat(8.0);
            let t68 = t67 * t48;
            let t69 = t68 * t53;
            let t71 = f64x8::splat(5.0) / f64x8::splat(9.0) * t69 - f64x8::splat(1.0);
            let t72 = t53 * t71;
            let t75 = f64x8::splat(1.0) + f64x8::splat(0.2222222222222222) * t68 * t72;
            let t76 = ((t75).sqrt());
            let t77 = f64x8::splat(1.0) / t76;
            let t80 = t48 * t53;
            let t81 = t80 * t65;
            let t82 = t81 / f64x8::splat(36.0);
            let t83 = f64x8::splat(9.0) / f64x8::splat(20.0) * t71 * t77 + t82;
            let t84 = t83 * t83;
            let t87 = t48 * t48;
            let t89 = f64x8::splat(1.0) / t51 / t50;
            let t90 = t87 * t89;
            let t91 = t35 * t35;
            let t92 = t91 * v_rho0;
            let t94 = f64x8::splat(1.0) / t55 / t92;
            let t96 = t90 * t34 * t94;
            let t97 = f64x8::splat(50.0) * t96;
            let t98 = f64x8::splat(162.0) * t40 + t97;
            let t99 = ((t98).sqrt());
            let t102 = f64x8::splat(3.291178445357254e-05) * t96;
            let t104 = t34 * v_sigma0;
            let t105 = t91 * t91;
            let t106 = f64x8::splat(1.0) / t105;
            let t108 = f64x8::splat(1.3522126526770064e-06) * t104 * t106;
            let t109 = t49 * t59 / f64x8::splat(24.0) + f64x8::splat(146.0) / f64x8::splat(2025.0) * t84 - f64x8::splat(73.0) / f64x8::splat(97200.0) * t83 * t99 + t102 + f64x8::splat(0.0020448759451792767) * t40 + t108;
            let t111 = f64x8::splat(1.0) + f64x8::splat(0.06134627835537829) * t81;
            let t112 = t111 * t111;
            let t113 = f64x8::splat(1.0) / t112;
            let t115 = f64x8::splat(0.804) + t109 * t113;
            let t117 = f64x8::splat(0.646416) / t115;
            let t118 = -t71;
            let t119 = t118 * t118;
            let t120 = t119 * t118;
            let t121 = t67 * t67;
            let t122 = t121 * t87;
            let t123 = t122 * t89;
            let t125 = f64x8::splat(1.0) + f64x8::splat(0.6714891975308642) * t123;
            let t126 = ((t125).sqrt());
            let t128 = f64x8::splat(1.0) / t126 / t125;
            let t129 = t120 * t128;
            let t131 = (simd::exp(-t81 / f64x8::splat(8.0)));
            let t133 = -f64x8::splat(0.45) + t82;
            let t134 = t133 * t133;
            let t136 = f64x8::splat(10368.0) + t97;
            let t137 = ((t136).sqrt());
            let t140 = f64x8::splat(0.029644443963477367) * t81 + f64x8::splat(146.0) / f64x8::splat(2025.0) * t134 - f64x8::splat(73.0) / f64x8::splat(97200.0) * t133 * t137 + t102 + f64x8::splat(0.1308720604914737) + t108;
            let t142 = f64x8::splat(0.804) + t140 * t113;
            let t145 = -f64x8::splat(0.646416) / t142 + t117;
            let t146 = t131 * t145;
            let t148 = f64x8::splat(1.804) - t117 + t129 * t146;
            let t152 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t28 * t148));
            let t153 = (v_rho1).simd_le(dens_threshold);
            let t154 = -t17;
            let t156 = ((t15).select(t12, (t11).select(t16, t154 * t8)));
            let t157 = f64x8::splat(1.0) + t156;
            let t158 = (t157).simd_le(zeta_threshold);
            let t159 = (simd::cbrt(t157));
            let t161 = ((t158).select(t23, t159 * t157));
            let t162 = t161 * t27;
            let t163 = f64x8::splat(1.0) / v_rho1;
            let t164 = v_sigma2 * t163;
            let t165 = f64x8::splat(1.0) / v_tau1;
            let t166 = t164 * t165;
            let t167 = ((t166) * (t166) * (t166));
            let t168 = v_sigma2 * v_sigma2;
            let t169 = v_rho1 * v_rho1;
            let t170 = f64x8::splat(1.0) / t169;
            let t171 = t168 * t170;
            let t172 = v_tau1 * v_tau1;
            let t173 = f64x8::splat(1.0) / t172;
            let t174 = t171 * t173;
            let t176 = f64x8::splat(1.0) + t174 / f64x8::splat(64.0);
            let t177 = t176 * t176;
            let t178 = f64x8::splat(1.0) / t177;
            let t182 = (f64x8::splat(10.0) / f64x8::splat(81.0) + f64x8::splat(0.0045938270703125) * t167 * t178) * t48;
            let t183 = t53 * v_sigma2;
            let t184 = (simd::cbrt(v_rho1));
            let t185 = t184 * t184;
            let t187 = f64x8::splat(1.0) / t185 / t169;
            let t188 = t183 * t187;
            let t192 = f64x8::splat(1.0) / t185 / v_rho1;
            let t194 = v_sigma2 * t187;
            let t196 = v_tau1 * t192 - t194 / f64x8::splat(8.0);
            let t197 = t196 * t48;
            let t198 = t197 * t53;
            let t200 = f64x8::splat(5.0) / f64x8::splat(9.0) * t198 - f64x8::splat(1.0);
            let t201 = t53 * t200;
            let t204 = f64x8::splat(1.0) + f64x8::splat(0.2222222222222222) * t197 * t201;
            let t205 = ((t204).sqrt());
            let t206 = f64x8::splat(1.0) / t205;
            let t209 = t80 * t194;
            let t210 = t209 / f64x8::splat(36.0);
            let t211 = f64x8::splat(9.0) / f64x8::splat(20.0) * t200 * t206 + t210;
            let t212 = t211 * t211;
            let t215 = t169 * t169;
            let t216 = t215 * v_rho1;
            let t218 = f64x8::splat(1.0) / t184 / t216;
            let t220 = t90 * t168 * t218;
            let t221 = f64x8::splat(50.0) * t220;
            let t222 = f64x8::splat(162.0) * t174 + t221;
            let t223 = ((t222).sqrt());
            let t226 = f64x8::splat(3.291178445357254e-05) * t220;
            let t228 = t168 * v_sigma2;
            let t229 = t215 * t215;
            let t230 = f64x8::splat(1.0) / t229;
            let t232 = f64x8::splat(1.3522126526770064e-06) * t228 * t230;
            let t233 = t182 * t188 / f64x8::splat(24.0) + f64x8::splat(146.0) / f64x8::splat(2025.0) * t212 - f64x8::splat(73.0) / f64x8::splat(97200.0) * t211 * t223 + t226 + f64x8::splat(0.0020448759451792767) * t174 + t232;
            let t235 = f64x8::splat(1.0) + f64x8::splat(0.06134627835537829) * t209;
            let t236 = t235 * t235;
            let t237 = f64x8::splat(1.0) / t236;
            let t239 = f64x8::splat(0.804) + t233 * t237;
            let t241 = f64x8::splat(0.646416) / t239;
            let t242 = -t200;
            let t243 = t242 * t242;
            let t244 = t243 * t242;
            let t245 = t196 * t196;
            let t246 = t245 * t87;
            let t247 = t246 * t89;
            let t249 = f64x8::splat(1.0) + f64x8::splat(0.6714891975308642) * t247;
            let t250 = ((t249).sqrt());
            let t252 = f64x8::splat(1.0) / t250 / t249;
            let t253 = t244 * t252;
            let t255 = (simd::exp(-t209 / f64x8::splat(8.0)));
            let t257 = -f64x8::splat(0.45) + t210;
            let t258 = t257 * t257;
            let t260 = f64x8::splat(10368.0) + t221;
            let t261 = ((t260).sqrt());
            let t264 = f64x8::splat(0.029644443963477367) * t209 + f64x8::splat(146.0) / f64x8::splat(2025.0) * t258 - f64x8::splat(73.0) / f64x8::splat(97200.0) * t257 * t261 + t226 + f64x8::splat(0.1308720604914737) + t232;
            let t266 = f64x8::splat(0.804) + t264 * t237;
            let t269 = -f64x8::splat(0.646416) / t266 + t241;
            let t270 = t255 * t269;
            let t272 = f64x8::splat(1.804) - t241 + t253 * t270;
            let t276 = ((t153).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t162 * t272));
            let tzk0 = t152 + t276;
            acc_zk = tzk0;
            let t277 = t7 * t7;
            let t278 = f64x8::splat(1.0) / t277;
            let t279 = t17 * t278;
            let t281 = ((t11).select(f64x8::splat(0.0), (t15).select(f64x8::splat(0.0), t8 - t279)));
            let t284 = ((t21).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t24 * t281));
            let t285 = t284 * t27;
            let t289 = t27 * t27;
            let t290 = f64x8::splat(1.0) / t289;
            let t291 = t26 * t290;
            let t294 = t6 * t291 * t148 / f64x8::splat(8.0);
            let t295 = t115 * t115;
            let t296 = f64x8::splat(1.0) / t295;
            let t297 = ((t32) * (t32));
            let t298 = t297 * t44;
            let t299 = v_sigma0 * t36;
            let t304 = f64x8::splat(1.0) / t43 / t42;
            let t305 = t33 * t304;
            let t306 = t35 * v_rho0;
            let t307 = f64x8::splat(1.0) / t306;
            let t308 = t34 * t307;
            let t309 = t308 * t39;
            let t313 = (-f64x8::splat(0.0137814812109375) * t298 * t299 * t31 + f64x8::splat(0.00028711419189453123) * t305 * t309) * t48;
            let t317 = f64x8::splat(1.0) / t56 / t306;
            let t318 = t54 * t317;
            let t323 = v_sigma0 * t317;
            let t325 = -f64x8::splat(5.0) / f64x8::splat(3.0) * v_tau0 * t58 + t323 / f64x8::splat(3.0);
            let t326 = t325 * t48;
            let t327 = t53 * t77;
            let t331 = f64x8::splat(1.0) / t76 / t75;
            let t332 = t71 * t331;
            let t335 = t67 * t87;
            let t336 = t89 * t325;
            let t337 = t335 * t336;
            let t339 = f64x8::splat(0.2222222222222222) * t326 * t72 + f64x8::splat(0.12345679012345678) * t337;
            let t342 = t80 * t323;
            let t344 = t326 * t327 / f64x8::splat(4.0) - f64x8::splat(9.0) / f64x8::splat(40.0) * t332 * t339 - f64x8::splat(2.0) / f64x8::splat(27.0) * t342;
            let t349 = f64x8::splat(1.0) / t99;
            let t350 = t83 * t349;
            let t352 = t91 * t35;
            let t354 = f64x8::splat(1.0) / t55 / t352;
            let t356 = t90 * t34 * t354;
            let t358 = -f64x8::splat(324.0) * t309 - f64x8::splat(800.0) / f64x8::splat(3.0) * t356;
            let t361 = f64x8::splat(0.00017552951708572022) * t356;
            let t363 = t105 * v_rho0;
            let t364 = f64x8::splat(1.0) / t363;
            let t366 = f64x8::splat(1.0817701221416051e-05) * t104 * t364;
            let t367 = t313 * t59 / f64x8::splat(24.0) - t49 * t318 / f64x8::splat(9.0) + f64x8::splat(292.0) / f64x8::splat(2025.0) * t83 * t344 - f64x8::splat(73.0) / f64x8::splat(97200.0) * t344 * t99 - f64x8::splat(73.0) / f64x8::splat(194400.0) * t350 * t358 - t361 - f64x8::splat(0.004089751890358553) * t309 - t366;
            let t369 = t112 * t111;
            let t370 = f64x8::splat(1.0) / t369;
            let t371 = t109 * t370;
            let t372 = t371 * t48;
            let t375 = t367 * t113 + f64x8::splat(0.32718015122868427) * t372 * t318;
            let t377 = f64x8::splat(0.646416) * t296 * t375;
            let t378 = t119 * t128;
            let t379 = t378 * t131;
            let t380 = t145 * t325;
            let t384 = t125 * t125;
            let t386 = f64x8::splat(1.0) / t126 / t384;
            let t387 = t120 * t386;
            let t388 = t387 * t146;
            let t391 = t129 * t80;
            let t395 = t142 * t142;
            let t396 = f64x8::splat(1.0) / t395;
            let t398 = t133 * t48;
            let t404 = f64x8::splat(1.0) / t137;
            let t405 = t133 * t404;
            let t406 = t405 * t87;
            let t407 = t89 * t34;
            let t411 = -f64x8::splat(0.07905185056927298) * t342 - f64x8::splat(584.0) / f64x8::splat(54675.0) * t398 * t318 + f64x8::splat(73.0) / f64x8::splat(1312200.0) * t80 * t323 * t137 + f64x8::splat(73.0) / f64x8::splat(729.0) * t406 * t407 * t354 - t361 - t366;
            let t413 = t140 * t370;
            let t414 = t413 * t48;
            let t417 = t411 * t113 + f64x8::splat(0.32718015122868427) * t414 * t318;
            let t420 = f64x8::splat(0.646416) * t396 * t417 - t377;
            let t421 = t131 * t420;
            let t423 = t377 - f64x8::splat(5.0) / f64x8::splat(3.0) * t379 * t380 * t80 - f64x8::splat(2.0144675925925926) * t388 * t337 + t391 * t323 * t146 / f64x8::splat(3.0) + t129 * t421;
            let t428 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t285 * t148 - t294 - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t28 * t423));
            let t429 = t154 * t278;
            let t431 = ((t15).select(f64x8::splat(0.0), (t11).select(f64x8::splat(0.0), -t8 - t429)));
            let t434 = ((t158).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t159 * t431));
            let t435 = t434 * t27;
            let t439 = t161 * t290;
            let t442 = t6 * t439 * t272 / f64x8::splat(8.0);
            let t444 = ((t153).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t435 * t272 - t442));
            let tvrho0 = t152 + t276 + t7 * (t428 + t444);
            acc_vrho_0 = tvrho0;
            let t448 = ((t11).select(f64x8::splat(0.0), (t15).select(f64x8::splat(0.0), -t8 - t279)));
            let t451 = ((t21).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t24 * t448));
            let t452 = t451 * t27;
            let t457 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t452 * t148 - t294));
            let t459 = ((t15).select(f64x8::splat(0.0), (t11).select(f64x8::splat(0.0), t8 - t429)));
            let t462 = ((t158).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t159 * t459));
            let t463 = t462 * t27;
            let t467 = t239 * t239;
            let t468 = f64x8::splat(1.0) / t467;
            let t469 = ((t166) * (t166));
            let t470 = t469 * t178;
            let t471 = v_sigma2 * t170;
            let t476 = f64x8::splat(1.0) / t177 / t176;
            let t477 = t167 * t476;
            let t478 = t169 * v_rho1;
            let t479 = f64x8::splat(1.0) / t478;
            let t480 = t168 * t479;
            let t481 = t480 * t173;
            let t485 = (-f64x8::splat(0.0137814812109375) * t470 * t471 * t165 + f64x8::splat(0.00028711419189453123) * t477 * t481) * t48;
            let t489 = f64x8::splat(1.0) / t185 / t478;
            let t490 = t183 * t489;
            let t495 = v_sigma2 * t489;
            let t497 = -f64x8::splat(5.0) / f64x8::splat(3.0) * v_tau1 * t187 + t495 / f64x8::splat(3.0);
            let t498 = t497 * t48;
            let t499 = t53 * t206;
            let t503 = f64x8::splat(1.0) / t205 / t204;
            let t504 = t200 * t503;
            let t507 = t196 * t87;
            let t508 = t89 * t497;
            let t509 = t507 * t508;
            let t511 = f64x8::splat(0.2222222222222222) * t498 * t201 + f64x8::splat(0.12345679012345678) * t509;
            let t514 = t80 * t495;
            let t516 = t498 * t499 / f64x8::splat(4.0) - f64x8::splat(9.0) / f64x8::splat(40.0) * t504 * t511 - f64x8::splat(2.0) / f64x8::splat(27.0) * t514;
            let t521 = f64x8::splat(1.0) / t223;
            let t522 = t211 * t521;
            let t524 = t215 * t169;
            let t526 = f64x8::splat(1.0) / t184 / t524;
            let t528 = t90 * t168 * t526;
            let t530 = -f64x8::splat(324.0) * t481 - f64x8::splat(800.0) / f64x8::splat(3.0) * t528;
            let t533 = f64x8::splat(0.00017552951708572022) * t528;
            let t535 = t229 * v_rho1;
            let t536 = f64x8::splat(1.0) / t535;
            let t538 = f64x8::splat(1.0817701221416051e-05) * t228 * t536;
            let t539 = t485 * t188 / f64x8::splat(24.0) - t182 * t490 / f64x8::splat(9.0) + f64x8::splat(292.0) / f64x8::splat(2025.0) * t211 * t516 - f64x8::splat(73.0) / f64x8::splat(97200.0) * t516 * t223 - f64x8::splat(73.0) / f64x8::splat(194400.0) * t522 * t530 - t533 - f64x8::splat(0.004089751890358553) * t481 - t538;
            let t541 = t236 * t235;
            let t542 = f64x8::splat(1.0) / t541;
            let t543 = t233 * t542;
            let t544 = t543 * t48;
            let t547 = t539 * t237 + f64x8::splat(0.32718015122868427) * t544 * t490;
            let t549 = f64x8::splat(0.646416) * t468 * t547;
            let t550 = t243 * t252;
            let t551 = t550 * t255;
            let t552 = t269 * t497;
            let t553 = t552 * t80;
            let t556 = t249 * t249;
            let t558 = f64x8::splat(1.0) / t250 / t556;
            let t559 = t244 * t558;
            let t560 = t559 * t270;
            let t563 = t253 * t80;
            let t567 = t266 * t266;
            let t568 = f64x8::splat(1.0) / t567;
            let t570 = t257 * t48;
            let t576 = f64x8::splat(1.0) / t261;
            let t577 = t257 * t576;
            let t578 = t577 * t87;
            let t579 = t89 * t168;
            let t583 = -f64x8::splat(0.07905185056927298) * t514 - f64x8::splat(584.0) / f64x8::splat(54675.0) * t570 * t490 + f64x8::splat(73.0) / f64x8::splat(1312200.0) * t80 * t495 * t261 + f64x8::splat(73.0) / f64x8::splat(729.0) * t578 * t579 * t526 - t533 - t538;
            let t585 = t264 * t542;
            let t586 = t585 * t48;
            let t589 = t583 * t237 + f64x8::splat(0.32718015122868427) * t586 * t490;
            let t592 = f64x8::splat(0.646416) * t568 * t589 - t549;
            let t593 = t255 * t592;
            let t595 = t549 - f64x8::splat(5.0) / f64x8::splat(3.0) * t551 * t553 - f64x8::splat(2.0144675925925926) * t560 * t509 + t563 * t495 * t270 / f64x8::splat(3.0) + t253 * t593;
            let t600 = ((t153).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t463 * t272 - t442 - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t162 * t595));
            let tvrho1 = t152 + t276 + t7 * (t457 + t600);
            acc_vrho_1 = tvrho1;
            let t606 = t299 * t39;
            let t610 = (f64x8::splat(0.0137814812109375) * t298 * t29 * t31 - f64x8::splat(0.00028711419189453123) * t305 * t606) * t48;
            let t613 = t53 * t58;
            let t616 = t58 * t48;
            let t617 = t616 * t327;
            let t619 = t616 * t72;
            let t621 = t89 * t58;
            let t622 = t335 * t621;
            let t624 = -f64x8::splat(0.027777777777777776) * t619 - f64x8::splat(0.015432098765432098) * t622;
            let t627 = t616 * t53;
            let t629 = -t617 / f64x8::splat(32.0) - f64x8::splat(9.0) / f64x8::splat(40.0) * t332 * t624 + t627 / f64x8::splat(36.0);
            let t635 = v_sigma0 * t94;
            let t636 = t90 * t635;
            let t638 = f64x8::splat(324.0) * t606 + f64x8::splat(100.0) * t636;
            let t641 = f64x8::splat(6.582356890714508e-05) * t636;
            let t644 = f64x8::splat(4.056637958031019e-06) * t34 * t106;
            let t645 = t610 * t59 / f64x8::splat(24.0) + t49 * t613 / f64x8::splat(24.0) + f64x8::splat(292.0) / f64x8::splat(2025.0) * t83 * t629 - f64x8::splat(73.0) / f64x8::splat(97200.0) * t629 * t99 - f64x8::splat(73.0) / f64x8::splat(194400.0) * t350 * t638 + t641 + f64x8::splat(0.004089751890358553) * t606 + t644;
            let t649 = t645 * t113 - f64x8::splat(0.12269255671075659) * t371 * t627;
            let t651 = f64x8::splat(0.646416) * t296 * t649;
            let t652 = t145 * t58;
            let t654 = t379 * t652 * t80;
            let t656 = t388 * t622;
            let t658 = t129 * t58;
            let t659 = t80 * t146;
            let t666 = t53 * t137;
            let t669 = t89 * v_sigma0;
            let t673 = f64x8::splat(0.029644443963477367) * t627 + f64x8::splat(73.0) / f64x8::splat(18225.0) * t133 * t58 * t80 - f64x8::splat(73.0) / f64x8::splat(3499200.0) * t616 * t666 - f64x8::splat(73.0) / f64x8::splat(1944.0) * t406 * t669 * t94 + t641 + t644;
            let t677 = t673 * t113 - f64x8::splat(0.12269255671075659) * t413 * t627;
            let t680 = f64x8::splat(0.646416) * t396 * t677 - t651;
            let t681 = t131 * t680;
            let t683 = t651 + f64x8::splat(5.0) / f64x8::splat(24.0) * t654 + f64x8::splat(0.25180844907407407) * t656 - t658 * t659 / f64x8::splat(8.0) + t129 * t681;
            let t687 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t28 * t683));
            let tvsigma0 = t7 * t687;
            acc_vsigma_0 = tvsigma0;
            let tvsigma1 = f64x8::splat(0.0);
            acc_vsigma_1 = tvsigma1;
            let t691 = t471 * t173;
            let t695 = (f64x8::splat(0.0137814812109375) * t470 * t163 * t165 - f64x8::splat(0.00028711419189453123) * t477 * t691) * t48;
            let t698 = t53 * t187;
            let t701 = t187 * t48;
            let t702 = t701 * t499;
            let t704 = t701 * t201;
            let t706 = t89 * t187;
            let t707 = t507 * t706;
            let t709 = -f64x8::splat(0.027777777777777776) * t704 - f64x8::splat(0.015432098765432098) * t707;
            let t712 = t701 * t53;
            let t714 = -t702 / f64x8::splat(32.0) - f64x8::splat(9.0) / f64x8::splat(40.0) * t504 * t709 + t712 / f64x8::splat(36.0);
            let t720 = v_sigma2 * t218;
            let t721 = t90 * t720;
            let t723 = f64x8::splat(324.0) * t691 + f64x8::splat(100.0) * t721;
            let t726 = f64x8::splat(6.582356890714508e-05) * t721;
            let t729 = f64x8::splat(4.056637958031019e-06) * t168 * t230;
            let t730 = t695 * t188 / f64x8::splat(24.0) + t182 * t698 / f64x8::splat(24.0) + f64x8::splat(292.0) / f64x8::splat(2025.0) * t211 * t714 - f64x8::splat(73.0) / f64x8::splat(97200.0) * t714 * t223 - f64x8::splat(73.0) / f64x8::splat(194400.0) * t522 * t723 + t726 + f64x8::splat(0.004089751890358553) * t691 + t729;
            let t734 = t730 * t237 - f64x8::splat(0.12269255671075659) * t543 * t712;
            let t736 = f64x8::splat(0.646416) * t468 * t734;
            let t737 = t269 * t187;
            let t739 = t551 * t737 * t80;
            let t741 = t560 * t707;
            let t743 = t253 * t187;
            let t744 = t80 * t270;
            let t751 = t53 * t261;
            let t754 = t89 * v_sigma2;
            let t758 = f64x8::splat(0.029644443963477367) * t712 + f64x8::splat(73.0) / f64x8::splat(18225.0) * t257 * t187 * t80 - f64x8::splat(73.0) / f64x8::splat(3499200.0) * t701 * t751 - f64x8::splat(73.0) / f64x8::splat(1944.0) * t578 * t754 * t218 + t726 + t729;
            let t762 = t758 * t237 - f64x8::splat(0.12269255671075659) * t585 * t712;
            let t765 = f64x8::splat(0.646416) * t568 * t762 - t736;
            let t766 = t255 * t765;
            let t768 = t736 + f64x8::splat(5.0) / f64x8::splat(24.0) * t739 + f64x8::splat(0.25180844907407407) * t741 - t743 * t744 / f64x8::splat(8.0) + t253 * t766;
            let t772 = ((t153).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t162 * t768));
            let tvsigma2 = t7 * t772;
            acc_vsigma_2 = tvsigma2;
            let tvlapl0 = f64x8::splat(0.0);
            acc_vlapl_0 = tvlapl0;
            let tvlapl1 = f64x8::splat(0.0);
            acc_vlapl_1 = tvlapl1;
            let t776 = t38 * v_tau0;
            let t777 = f64x8::splat(1.0) / t776;
            let t778 = t37 * t777;
            let t782 = (-f64x8::splat(0.0137814812109375) * t298 * t30 * t39 + f64x8::splat(0.00028711419189453123) * t305 * t778) * t48;
            let t785 = t63 * t48;
            let t790 = t89 * t63;
            let t791 = t335 * t790;
            let t793 = f64x8::splat(0.2222222222222222) * t785 * t72 + f64x8::splat(0.12345679012345678) * t791;
            let t796 = t785 * t327 / f64x8::splat(4.0) - f64x8::splat(9.0) / f64x8::splat(40.0) * t332 * t793;
            let t804 = t782 * t59 / f64x8::splat(24.0) + f64x8::splat(292.0) / f64x8::splat(2025.0) * t83 * t796 - f64x8::splat(73.0) / f64x8::splat(97200.0) * t796 * t99 + f64x8::splat(73.0) / f64x8::splat(600.0) * t350 * t778 - f64x8::splat(0.004089751890358553) * t778;
            let t805 = t296 * t804;
            let t806 = t805 * t113;
            let t808 = t145 * t63;
            let t814 = t129 * t131;
            let t817 = f64x8::splat(0.646416) * t806 - f64x8::splat(5.0) / f64x8::splat(3.0) * t379 * t808 * t80 - f64x8::splat(2.0144675925925926) * t388 * t791 - f64x8::splat(0.646416) * t814 * t806;
            let t821 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t28 * t817));
            let tvtau0 = t7 * t821;
            acc_vtau_0 = tvtau0;
            let t825 = t172 * v_tau1;
            let t826 = f64x8::splat(1.0) / t825;
            let t827 = t171 * t826;
            let t831 = (-f64x8::splat(0.0137814812109375) * t470 * t164 * t173 + f64x8::splat(0.00028711419189453123) * t477 * t827) * t48;
            let t834 = t192 * t48;
            let t839 = t89 * t192;
            let t840 = t507 * t839;
            let t842 = f64x8::splat(0.2222222222222222) * t834 * t201 + f64x8::splat(0.12345679012345678) * t840;
            let t845 = t834 * t499 / f64x8::splat(4.0) - f64x8::splat(9.0) / f64x8::splat(40.0) * t504 * t842;
            let t853 = t831 * t188 / f64x8::splat(24.0) + f64x8::splat(292.0) / f64x8::splat(2025.0) * t211 * t845 - f64x8::splat(73.0) / f64x8::splat(97200.0) * t845 * t223 + f64x8::splat(73.0) / f64x8::splat(600.0) * t522 * t827 - f64x8::splat(0.004089751890358553) * t827;
            let t854 = t468 * t853;
            let t855 = t854 * t237;
            let t857 = t269 * t192;
            let t863 = t253 * t255;
            let t866 = f64x8::splat(0.646416) * t855 - f64x8::splat(5.0) / f64x8::splat(3.0) * t551 * t857 * t80 - f64x8::splat(2.0144675925925926) * t560 * t840 - f64x8::splat(0.646416) * t863 * t855;
            let t870 = ((t153).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t162 * t866));
            let tvtau1 = t7 * t870;
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
