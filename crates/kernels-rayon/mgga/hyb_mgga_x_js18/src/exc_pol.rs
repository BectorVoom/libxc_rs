//! HYB_MGGA_X_JS18 exc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/hyb_mgga_x_js18.c`
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
pub fn hyb_mgga_x_js18_exc_pol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    param_hyb_omega_0: f64,
    param_hyb_coeff_0: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_hyb_omega_0 = f64x8::splat(param_hyb_omega_0);
    let param_hyb_coeff_0 = f64x8::splat(param_hyb_coeff_0);
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
            let t18 = t17 * t8;
            let t19 = ((t11).select(t12, (t15).select(t16, t18)));
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
            let t33 = t30 * t31 / f64x8::splat(8.0);
            let t34 = (t33).simd_lt(f64x8::splat(1.0));
            let t35 = ((t34).select(t33, f64x8::splat(1.0)));
            let t36 = t35 * t35;
            let t37 = t36 * t35;
            let t39 = t36 + f64x8::splat(3.0) * t37;
            let t40 = f64x8::splat(1.0) + t37;
            let t41 = t40 * t40;
            let t42 = f64x8::splat(1.0) / t41;
            let t43 = t39 * t42;
            let t44 = (simd::cbrt(f64x8::splat(9.0)));
            let t45 = t44 * t44;
            let t47 = (simd::cbrt(f64x8::splat(1.0) / f64x8::splat(M_PI)));
            let t48 = t47 * t47;
            let t49 = t45 * t48;
            let t50 = t49 * param_hyb_omega_0;
            let t51 = f64x8::splat(1.0) / t27;
            let t52 = t3 * t51;
            let t53 = f64x8::splat(M_CBRT6);
            let t54 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t55 = (simd::cbrt(t54));
            let t56 = t55 * t55;
            let t57 = f64x8::splat(1.0) / t56;
            let t58 = t53 * t57;
            let t59 = v_rho0 * v_rho0;
            let t60 = (simd::cbrt(v_rho0));
            let t61 = t60 * t60;
            let t63 = f64x8::splat(1.0) / t61 / t59;
            let t64 = v_sigma0 * t63;
            let t65 = t58 * t64;
            let t67 = t53 * t53;
            let t69 = f64x8::splat(1.0) / t55 / t54;
            let t70 = t67 * t69;
            let t71 = v_sigma0 * v_sigma0;
            let t72 = t59 * t59;
            let t73 = t72 * v_rho0;
            let t75 = f64x8::splat(1.0) / t60 / t73;
            let t79 = f64x8::splat(1.0) + f64x8::splat(0.1504548888888889) * t65 + f64x8::splat(0.002689949046226295) * t70 * t71 * t75;
            let t80 = (simd::pow(t79, f64x8::splat(1.0) / f64x8::splat(10.0)));
            let t81 = f64x8::splat(1.0) / t80;
            let t83 = (f64x8::splat(1.0) + t18).simd_le(zeta_threshold);
            let t85 = (f64x8::splat(1.0) - t18).simd_le(zeta_threshold);
            let t86 = ((t83).select(t12, (t85).select(t16, t18)));
            let t87 = f64x8::splat(1.0) + t86;
            let t88 = (t87).simd_le(zeta_threshold);
            let t89 = (simd::cbrt(t87));
            let t90 = ((t88).select(t22, t89));
            let t91 = f64x8::splat(1.0) / t90;
            let t92 = t81 * t91;
            let t95 = t50 * t52 * t92 / f64x8::splat(18.0);
            let t96 = (t95).simd_lt(f64x8::splat(1e-10));
            let t97 = ((t96).select(f64x8::splat(1e-10), t95));
            let t98 = (f64x8::splat(1.35)).simd_le(t97);
            let t99 = (f64x8::splat(1.35)).simd_lt(t97);
            let t100 = ((t99).select(t97, f64x8::splat(1.35)));
            let t101 = t100 * t100;
            let t104 = t101 * t101;
            let t105 = f64x8::splat(1.0) / t104;
            let t107 = t104 * t101;
            let t108 = f64x8::splat(1.0) / t107;
            let t110 = t104 * t104;
            let t111 = f64x8::splat(1.0) / t110;
            let t114 = f64x8::splat(1.0) / t110 / t101;
            let t117 = f64x8::splat(1.0) / t110 / t104;
            let t120 = f64x8::splat(1.0) / t110 / t107;
            let t122 = t110 * t110;
            let t123 = f64x8::splat(1.0) / t122;
            let t126 = ((t99).select(f64x8::splat(1.35), t97));
            let t127 = ((f64x8::splat(M_PI)).sqrt());
            let t128 = f64x8::splat(1.0) / t126;
            let t130 = (simd::erf(t128 / f64x8::splat(2.0)));
            let t132 = t126 * t126;
            let t133 = f64x8::splat(1.0) / t132;
            let t135 = (simd::exp(-t133 / f64x8::splat(4.0)));
            let t136 = t135 - f64x8::splat(1.0);
            let t139 = t135 - f64x8::splat(3.0) / f64x8::splat(2.0) - f64x8::splat(2.0) * t132 * t136;
            let t142 = f64x8::splat(2.0) * t126 * t139 + t127 * t130;
            let t146 = ((t98).select(f64x8::splat(1.0) / t101 / f64x8::splat(36.0) - t105 / f64x8::splat(960.0) + t108 / f64x8::splat(26880.0) - t111 / f64x8::splat(829440.0) + t114 / f64x8::splat(28385280.0) - t117 / f64x8::splat(1073479680.0) + t120 / f64x8::splat(44590694400.0) - t123 / f64x8::splat(2021444812800.0), f64x8::splat(1.0) - f64x8::splat(8.0) / f64x8::splat(3.0) * t126 * t142));
            let t147 = (simd::pow(t79, f64x8::splat(1.0) / f64x8::splat(5.0)));
            let t148 = f64x8::splat(1.0) / t147;
            let t150 = (f64x8::splat(0.27)).simd_le(t97);
            let t151 = (f64x8::splat(0.27)).simd_lt(t97);
            let t152 = ((t151).select(t97, f64x8::splat(0.27)));
            let t153 = t152 * t152;
            let t154 = t153 * t153;
            let t155 = t154 * t154;
            let t156 = t155 * t154;
            let t157 = t155 * t155;
            let t158 = t157 * t157;
            let t160 = f64x8::splat(1.0) / t158 / t156;
            let t162 = t154 * t153;
            let t163 = t155 * t162;
            let t165 = f64x8::splat(1.0) / t158 / t163;
            let t169 = f64x8::splat(1.0) / t162;
            let t171 = f64x8::splat(1.0) / t155;
            let t173 = t155 * t153;
            let t174 = f64x8::splat(1.0) / t173;
            let t176 = f64x8::splat(1.0) / t156;
            let t178 = f64x8::splat(1.0) / t163;
            let t180 = f64x8::splat(1.0) / t157;
            let t182 = t157 * t153;
            let t183 = f64x8::splat(1.0) / t182;
            let t186 = f64x8::splat(1.0) / t157 / t154;
            let t188 = t160 / f64x8::splat(3.3929038000650147e+37) - t165 / f64x8::splat(3.511556992918352e+39) + f64x8::splat(3.0) / f64x8::splat(2240.0) / t154 - t169 / f64x8::splat(11520.0) + f64x8::splat(3.0) / f64x8::splat(788480.0) * t171 - t174 / f64x8::splat(7454720.0) + t176 / f64x8::splat(247726080.0) - t178 / f64x8::splat(9358540800.0) + t180 / f64x8::splat(394474291200.0) - t183 / f64x8::splat(18311911833600.0) + t186 / f64x8::splat(927028425523200.0);
            let t190 = f64x8::splat(1.0) / t157 / t162;
            let t193 = f64x8::splat(1.0) / t157 / t155;
            let t196 = f64x8::splat(1.0) / t157 / t173;
            let t199 = f64x8::splat(1.0) / t157 / t156;
            let t202 = f64x8::splat(1.0) / t157 / t163;
            let t204 = f64x8::splat(1.0) / t158;
            let t207 = f64x8::splat(1.0) / t158 / t153;
            let t210 = f64x8::splat(1.0) / t158 / t154;
            let t213 = f64x8::splat(1.0) / t158 / t162;
            let t216 = f64x8::splat(1.0) / t158 / t155;
            let t219 = f64x8::splat(1.0) / t158 / t173;
            let t221 = -t190 / f64x8::splat(5.0785035485184e+16) + t193 / f64x8::splat(2.991700272218112e+18) - t196 / f64x8::splat(1.88514051721003e+20) + t199 / f64x8::splat(1.2648942844388573e+22) - t202 / f64x8::splat(9.002316741416457e+23) + t204 / f64x8::splat(6.772652029299977e+25) - t207 / f64x8::splat(5.36974553751641e+27) + t210 / f64x8::splat(4.474731034888079e+29) - t213 / f64x8::splat(3.909716563474291e+31) + t216 / f64x8::splat(3.5738523369945735e+33) - t219 / f64x8::splat(3.410951160703658e+35);
            let t223 = ((t151).select(f64x8::splat(0.27), t97));
            let t224 = t223 * t223;
            let t226 = t224 * t224;
            let t227 = f64x8::splat(64.0) * t226;
            let t228 = f64x8::splat(20.0) * t224 - t227;
            let t231 = (simd::exp(-f64x8::splat(1.0) / t224 / f64x8::splat(4.0)));
            let t235 = f64x8::splat(1.0) / t223;
            let t237 = (simd::erf(t235 / f64x8::splat(2.0)));
            let t240 = f64x8::splat(10.0) * t127 * t223 * t237 + t228 * t231 - f64x8::splat(36.0) * t224 + t227 - f64x8::splat(3.0);
            let t244 = ((t150).select(t188 + t221, f64x8::splat(24.0) * t224 * t240 + f64x8::splat(1.0)));
            let t246 = f64x8::splat(1.0) / t61 / v_rho0;
            let t247 = v_tau0 * t246;
            let t248 = f64x8::splat(0.14554132) * t247;
            let t249 = t67 * t56;
            let t250 = f64x8::splat(0.043662396) * t249;
            let t252 = -t248 + t250 + f64x8::splat(0.04229627833333333) * t64;
            let t253 = t244 * t252;
            let t254 = t147 * t147;
            let t255 = f64x8::splat(1.0) / t254;
            let t256 = t58 * t255;
            let t259 = (f64x8::splat(0.32)).simd_le(t97);
            let t260 = (f64x8::splat(0.32)).simd_lt(t97);
            let t261 = ((t260).select(t97, f64x8::splat(0.32)));
            let t262 = t261 * t261;
            let t263 = t262 * t262;
            let t266 = t263 * t262;
            let t267 = f64x8::splat(1.0) / t266;
            let t269 = t263 * t263;
            let t270 = f64x8::splat(1.0) / t269;
            let t272 = t269 * t262;
            let t273 = f64x8::splat(1.0) / t272;
            let t275 = t269 * t263;
            let t276 = f64x8::splat(1.0) / t275;
            let t278 = t269 * t266;
            let t279 = f64x8::splat(1.0) / t278;
            let t281 = t269 * t269;
            let t282 = f64x8::splat(1.0) / t281;
            let t285 = f64x8::splat(1.0) / t281 / t262;
            let t288 = f64x8::splat(1.0) / t281 / t263;
            let t291 = f64x8::splat(1.0) / t281 / t266;
            let t294 = f64x8::splat(1.0) / t281 / t269;
            let t297 = f64x8::splat(1.0) / t281 / t272;
            let t300 = f64x8::splat(1.0) / t281 / t275;
            let t303 = f64x8::splat(1.0) / t281 / t278;
            let t305 = t281 * t281;
            let t306 = f64x8::splat(1.0) / t305;
            let t309 = f64x8::splat(1.0) / t305 / t262;
            let t312 = f64x8::splat(1.0) / t305 / t263;
            let t315 = f64x8::splat(1.0) / t305 / t266;
            let t317 = f64x8::splat(3.0) / f64x8::splat(7840.0) / t263 - t267 / f64x8::splat(56448.0) + f64x8::splat(5.0) / f64x8::splat(8515584.0) * t270 - t273 / f64x8::splat(61501440.0) + t276 / f64x8::splat(2530344960.0) - t279 / f64x8::splat(115811942400.0) + t282 / f64x8::splat(5811921223680.0) - t285 / f64x8::splat(316612955602944.0) + t288 / f64x8::splat(1.85827061661696e+16) - t291 / f64x8::splat(1.168055816159232e+18) + t294 / f64x8::splat(7.824446865801216e+19) - t297 / f64x8::splat(5.562511054710453e+21) + t300 / f64x8::splat(4.181740504354862e+23) - t303 / f64x8::splat(3.3139778504339334e+25) + t306 / f64x8::splat(2.7608516801793436e+27) - t309 / f64x8::splat(2.4119107039344544e+29) + t312 / f64x8::splat(2.2046293272414373e+31) - t315 / f64x8::splat(2.1042094544618633e+33);
            let t318 = ((t260).select(f64x8::splat(0.32), t97));
            let t320 = t318 * t318;
            let t321 = t320 * t318;
            let t323 = t320 * t320;
            let t324 = t323 * t318;
            let t326 = t323 * t321;
            let t328 = t323 * t323;
            let t329 = t328 * t318;
            let t331 = -f64x8::splat(8.0) * t318 + f64x8::splat(256.0) * t321 - f64x8::splat(576.0) * t324 + f64x8::splat(3840.0) * t326 - f64x8::splat(122880.0) * t329;
            let t332 = f64x8::splat(1.0) / t320;
            let t334 = (simd::exp(-t332 / f64x8::splat(4.0)));
            let t338 = t323 * t320;
            let t340 = -f64x8::splat(35.0) + f64x8::splat(224.0) * t320 - f64x8::splat(1440.0) * t323 + f64x8::splat(5120.0) * t338;
            let t344 = -f64x8::splat(2.0) + f64x8::splat(60.0) * t320;
            let t346 = f64x8::splat(1.0) / t318;
            let t348 = (simd::erf(t346 / f64x8::splat(2.0)));
            let t351 = f64x8::splat(2.0) * t127 * t344 * t348 + f64x8::splat(24.0) * t321 * t340 + t331 * t334;
            let t355 = ((t259).select(t317, f64x8::splat(1.0) + f64x8::splat(8.0) / f64x8::splat(7.0) * t318 * t351));
            let t356 = t355 * t53;
            let t357 = t356 * t57;
            let t358 = t64 * t255;
            let t361 = t146 * t148 + f64x8::splat(35.0) / f64x8::splat(81.0) * t253 * t256 + f64x8::splat(0.026329605555555555) * t357 * t358;
            let t363 = f64x8::splat(1.0) - t43;
            let t366 = t50 * t52 * t91 / f64x8::splat(18.0);
            let t367 = (f64x8::splat(1.35)).simd_le(t366);
            let t368 = (f64x8::splat(1.35)).simd_lt(t366);
            let t369 = ((t368).select(t366, f64x8::splat(1.35)));
            let t370 = t369 * t369;
            let t373 = t370 * t370;
            let t374 = f64x8::splat(1.0) / t373;
            let t376 = t373 * t370;
            let t377 = f64x8::splat(1.0) / t376;
            let t379 = t373 * t373;
            let t380 = f64x8::splat(1.0) / t379;
            let t383 = f64x8::splat(1.0) / t379 / t370;
            let t386 = f64x8::splat(1.0) / t379 / t373;
            let t389 = f64x8::splat(1.0) / t379 / t376;
            let t391 = t379 * t379;
            let t392 = f64x8::splat(1.0) / t391;
            let t395 = ((t368).select(f64x8::splat(1.35), t366));
            let t396 = f64x8::splat(1.0) / t395;
            let t398 = (simd::erf(t396 / f64x8::splat(2.0)));
            let t400 = t395 * t395;
            let t401 = f64x8::splat(1.0) / t400;
            let t403 = (simd::exp(-t401 / f64x8::splat(4.0)));
            let t404 = t403 - f64x8::splat(1.0);
            let t407 = t403 - f64x8::splat(3.0) / f64x8::splat(2.0) - f64x8::splat(2.0) * t400 * t404;
            let t410 = t127 * t398 + f64x8::splat(2.0) * t395 * t407;
            let t414 = ((t367).select(f64x8::splat(1.0) / t370 / f64x8::splat(36.0) - t374 / f64x8::splat(960.0) + t377 / f64x8::splat(26880.0) - t380 / f64x8::splat(829440.0) + t383 / f64x8::splat(28385280.0) - t386 / f64x8::splat(1073479680.0) + t389 / f64x8::splat(44590694400.0) - t392 / f64x8::splat(2021444812800.0), f64x8::splat(1.0) - f64x8::splat(8.0) / f64x8::splat(3.0) * t395 * t410));
            let t415 = t363 * t414;
            let t418 = (f64x8::splat(10.0) / f64x8::splat(81.0) + f64x8::splat(25.0) / f64x8::splat(8748.0) * t65) * t53;
            let t419 = t57 * v_sigma0;
            let t429 = (t247 - t64 / f64x8::splat(8.0)) * t53 * t57 / f64x8::splat(4.0) - f64x8::splat(9.0) / f64x8::splat(20.0) + t65 / f64x8::splat(36.0);
            let t430 = t429 * t429;
            let t432 = t429 * t35;
            let t433 = f64x8::splat(1.0) - t35;
            let t436 = f64x8::splat(1.0) + f64x8::splat(5.0) / f64x8::splat(12.0) * t418 * t419 * t63 + f64x8::splat(292.0) / f64x8::splat(405.0) * t430 - f64x8::splat(146.0) / f64x8::splat(135.0) * t432 * t433;
            let t437 = (simd::pow(t436, f64x8::splat(1.0) / f64x8::splat(10.0)));
            let t442 = f64x8::splat(0.256337604) * t249;
            let t448 = f64x8::splat(1.0) + f64x8::splat(0.06394332777777778) * t65 - f64x8::splat(5.0) / f64x8::splat(9.0) * (t248 + t442 + f64x8::splat(0.011867481666666667) * t64) * t53 * t57;
            let t451 = t148 + f64x8::splat(7.0) / f64x8::splat(9.0) * t448 * t255;
            let t454 = -param_hyb_coeff_0 * (t361 * t43 + t415 * t437) + t43 * t451 + t363 * t437;
            let t458 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t28 * t454));
            let t459 = (v_rho1).simd_le(dens_threshold);
            let t460 = -t17;
            let t462 = ((t15).select(t12, (t11).select(t16, t460 * t8)));
            let t463 = f64x8::splat(1.0) + t462;
            let t464 = (t463).simd_le(zeta_threshold);
            let t465 = (simd::cbrt(t463));
            let t467 = ((t464).select(t23, t465 * t463));
            let t468 = t467 * t27;
            let t469 = f64x8::splat(1.0) / v_rho1;
            let t470 = v_sigma2 * t469;
            let t471 = f64x8::splat(1.0) / v_tau1;
            let t473 = t470 * t471 / f64x8::splat(8.0);
            let t474 = (t473).simd_lt(f64x8::splat(1.0));
            let t475 = ((t474).select(t473, f64x8::splat(1.0)));
            let t476 = t475 * t475;
            let t477 = t476 * t475;
            let t479 = t476 + f64x8::splat(3.0) * t477;
            let t480 = f64x8::splat(1.0) + t477;
            let t481 = t480 * t480;
            let t482 = f64x8::splat(1.0) / t481;
            let t483 = t479 * t482;
            let t484 = v_rho1 * v_rho1;
            let t485 = (simd::cbrt(v_rho1));
            let t486 = t485 * t485;
            let t488 = f64x8::splat(1.0) / t486 / t484;
            let t489 = v_sigma2 * t488;
            let t490 = t58 * t489;
            let t492 = v_sigma2 * v_sigma2;
            let t493 = t484 * t484;
            let t494 = t493 * v_rho1;
            let t496 = f64x8::splat(1.0) / t485 / t494;
            let t500 = f64x8::splat(1.0) + f64x8::splat(0.1504548888888889) * t490 + f64x8::splat(0.002689949046226295) * t70 * t492 * t496;
            let t501 = (simd::pow(t500, f64x8::splat(1.0) / f64x8::splat(10.0)));
            let t502 = f64x8::splat(1.0) / t501;
            let t503 = ((t85).select(t12, (t83).select(t16, -t18)));
            let t504 = f64x8::splat(1.0) + t503;
            let t505 = (t504).simd_le(zeta_threshold);
            let t506 = (simd::cbrt(t504));
            let t507 = ((t505).select(t22, t506));
            let t508 = f64x8::splat(1.0) / t507;
            let t509 = t502 * t508;
            let t512 = t50 * t52 * t509 / f64x8::splat(18.0);
            let t513 = (t512).simd_lt(f64x8::splat(1e-10));
            let t514 = ((t513).select(f64x8::splat(1e-10), t512));
            let t515 = (f64x8::splat(1.35)).simd_le(t514);
            let t516 = (f64x8::splat(1.35)).simd_lt(t514);
            let t517 = ((t516).select(t514, f64x8::splat(1.35)));
            let t518 = t517 * t517;
            let t521 = t518 * t518;
            let t522 = f64x8::splat(1.0) / t521;
            let t524 = t521 * t518;
            let t525 = f64x8::splat(1.0) / t524;
            let t527 = t521 * t521;
            let t528 = f64x8::splat(1.0) / t527;
            let t531 = f64x8::splat(1.0) / t527 / t518;
            let t534 = f64x8::splat(1.0) / t527 / t521;
            let t537 = f64x8::splat(1.0) / t527 / t524;
            let t539 = t527 * t527;
            let t540 = f64x8::splat(1.0) / t539;
            let t543 = ((t516).select(f64x8::splat(1.35), t514));
            let t544 = f64x8::splat(1.0) / t543;
            let t546 = (simd::erf(t544 / f64x8::splat(2.0)));
            let t548 = t543 * t543;
            let t549 = f64x8::splat(1.0) / t548;
            let t551 = (simd::exp(-t549 / f64x8::splat(4.0)));
            let t552 = t551 - f64x8::splat(1.0);
            let t555 = t551 - f64x8::splat(3.0) / f64x8::splat(2.0) - f64x8::splat(2.0) * t548 * t552;
            let t558 = t127 * t546 + f64x8::splat(2.0) * t543 * t555;
            let t562 = ((t515).select(f64x8::splat(1.0) / t518 / f64x8::splat(36.0) - t522 / f64x8::splat(960.0) + t525 / f64x8::splat(26880.0) - t528 / f64x8::splat(829440.0) + t531 / f64x8::splat(28385280.0) - t534 / f64x8::splat(1073479680.0) + t537 / f64x8::splat(44590694400.0) - t540 / f64x8::splat(2021444812800.0), f64x8::splat(1.0) - f64x8::splat(8.0) / f64x8::splat(3.0) * t543 * t558));
            let t563 = (simd::pow(t500, f64x8::splat(1.0) / f64x8::splat(5.0)));
            let t564 = f64x8::splat(1.0) / t563;
            let t566 = (f64x8::splat(0.27)).simd_le(t514);
            let t567 = (f64x8::splat(0.27)).simd_lt(t514);
            let t568 = ((t567).select(t514, f64x8::splat(0.27)));
            let t569 = t568 * t568;
            let t570 = t569 * t569;
            let t571 = t570 * t570;
            let t572 = t571 * t570;
            let t573 = t571 * t571;
            let t574 = t573 * t573;
            let t576 = f64x8::splat(1.0) / t574 / t572;
            let t578 = t570 * t569;
            let t579 = t571 * t578;
            let t581 = f64x8::splat(1.0) / t574 / t579;
            let t585 = f64x8::splat(1.0) / t578;
            let t587 = f64x8::splat(1.0) / t571;
            let t589 = t571 * t569;
            let t590 = f64x8::splat(1.0) / t589;
            let t592 = f64x8::splat(1.0) / t572;
            let t594 = f64x8::splat(1.0) / t579;
            let t596 = f64x8::splat(1.0) / t573;
            let t598 = t573 * t569;
            let t599 = f64x8::splat(1.0) / t598;
            let t602 = f64x8::splat(1.0) / t573 / t570;
            let t604 = t576 / f64x8::splat(3.3929038000650147e+37) - t581 / f64x8::splat(3.511556992918352e+39) + f64x8::splat(3.0) / f64x8::splat(2240.0) / t570 - t585 / f64x8::splat(11520.0) + f64x8::splat(3.0) / f64x8::splat(788480.0) * t587 - t590 / f64x8::splat(7454720.0) + t592 / f64x8::splat(247726080.0) - t594 / f64x8::splat(9358540800.0) + t596 / f64x8::splat(394474291200.0) - t599 / f64x8::splat(18311911833600.0) + t602 / f64x8::splat(927028425523200.0);
            let t606 = f64x8::splat(1.0) / t573 / t578;
            let t609 = f64x8::splat(1.0) / t573 / t571;
            let t612 = f64x8::splat(1.0) / t573 / t589;
            let t615 = f64x8::splat(1.0) / t573 / t572;
            let t618 = f64x8::splat(1.0) / t573 / t579;
            let t620 = f64x8::splat(1.0) / t574;
            let t623 = f64x8::splat(1.0) / t574 / t569;
            let t626 = f64x8::splat(1.0) / t574 / t570;
            let t629 = f64x8::splat(1.0) / t574 / t578;
            let t632 = f64x8::splat(1.0) / t574 / t571;
            let t635 = f64x8::splat(1.0) / t574 / t589;
            let t637 = -t606 / f64x8::splat(5.0785035485184e+16) + t609 / f64x8::splat(2.991700272218112e+18) - t612 / f64x8::splat(1.88514051721003e+20) + t615 / f64x8::splat(1.2648942844388573e+22) - t618 / f64x8::splat(9.002316741416457e+23) + t620 / f64x8::splat(6.772652029299977e+25) - t623 / f64x8::splat(5.36974553751641e+27) + t626 / f64x8::splat(4.474731034888079e+29) - t629 / f64x8::splat(3.909716563474291e+31) + t632 / f64x8::splat(3.5738523369945735e+33) - t635 / f64x8::splat(3.410951160703658e+35);
            let t639 = ((t567).select(f64x8::splat(0.27), t514));
            let t640 = t639 * t639;
            let t642 = t640 * t640;
            let t643 = f64x8::splat(64.0) * t642;
            let t644 = f64x8::splat(20.0) * t640 - t643;
            let t647 = (simd::exp(-f64x8::splat(1.0) / t640 / f64x8::splat(4.0)));
            let t651 = f64x8::splat(1.0) / t639;
            let t653 = (simd::erf(t651 / f64x8::splat(2.0)));
            let t656 = f64x8::splat(10.0) * t127 * t639 * t653 + t644 * t647 - f64x8::splat(36.0) * t640 + t643 - f64x8::splat(3.0);
            let t660 = ((t566).select(t604 + t637, f64x8::splat(24.0) * t640 * t656 + f64x8::splat(1.0)));
            let t662 = f64x8::splat(1.0) / t486 / v_rho1;
            let t663 = v_tau1 * t662;
            let t664 = f64x8::splat(0.14554132) * t663;
            let t666 = -t664 + t250 + f64x8::splat(0.04229627833333333) * t489;
            let t667 = t660 * t666;
            let t668 = t563 * t563;
            let t669 = f64x8::splat(1.0) / t668;
            let t670 = t58 * t669;
            let t673 = (f64x8::splat(0.32)).simd_le(t514);
            let t674 = (f64x8::splat(0.32)).simd_lt(t514);
            let t675 = ((t674).select(t514, f64x8::splat(0.32)));
            let t676 = t675 * t675;
            let t677 = t676 * t676;
            let t680 = t677 * t676;
            let t681 = f64x8::splat(1.0) / t680;
            let t683 = t677 * t677;
            let t684 = f64x8::splat(1.0) / t683;
            let t686 = t683 * t676;
            let t687 = f64x8::splat(1.0) / t686;
            let t689 = t683 * t677;
            let t690 = f64x8::splat(1.0) / t689;
            let t692 = t683 * t680;
            let t693 = f64x8::splat(1.0) / t692;
            let t695 = t683 * t683;
            let t696 = f64x8::splat(1.0) / t695;
            let t699 = f64x8::splat(1.0) / t695 / t676;
            let t702 = f64x8::splat(1.0) / t695 / t677;
            let t705 = f64x8::splat(1.0) / t695 / t680;
            let t708 = f64x8::splat(1.0) / t695 / t683;
            let t711 = f64x8::splat(1.0) / t695 / t686;
            let t714 = f64x8::splat(1.0) / t695 / t689;
            let t717 = f64x8::splat(1.0) / t695 / t692;
            let t719 = t695 * t695;
            let t720 = f64x8::splat(1.0) / t719;
            let t723 = f64x8::splat(1.0) / t719 / t676;
            let t726 = f64x8::splat(1.0) / t719 / t677;
            let t729 = f64x8::splat(1.0) / t719 / t680;
            let t731 = f64x8::splat(3.0) / f64x8::splat(7840.0) / t677 - t681 / f64x8::splat(56448.0) + f64x8::splat(5.0) / f64x8::splat(8515584.0) * t684 - t687 / f64x8::splat(61501440.0) + t690 / f64x8::splat(2530344960.0) - t693 / f64x8::splat(115811942400.0) + t696 / f64x8::splat(5811921223680.0) - t699 / f64x8::splat(316612955602944.0) + t702 / f64x8::splat(1.85827061661696e+16) - t705 / f64x8::splat(1.168055816159232e+18) + t708 / f64x8::splat(7.824446865801216e+19) - t711 / f64x8::splat(5.562511054710453e+21) + t714 / f64x8::splat(4.181740504354862e+23) - t717 / f64x8::splat(3.3139778504339334e+25) + t720 / f64x8::splat(2.7608516801793436e+27) - t723 / f64x8::splat(2.4119107039344544e+29) + t726 / f64x8::splat(2.2046293272414373e+31) - t729 / f64x8::splat(2.1042094544618633e+33);
            let t732 = ((t674).select(f64x8::splat(0.32), t514));
            let t734 = t732 * t732;
            let t735 = t734 * t732;
            let t737 = t734 * t734;
            let t738 = t737 * t732;
            let t740 = t737 * t735;
            let t742 = t737 * t737;
            let t743 = t742 * t732;
            let t745 = -f64x8::splat(8.0) * t732 + f64x8::splat(256.0) * t735 - f64x8::splat(576.0) * t738 + f64x8::splat(3840.0) * t740 - f64x8::splat(122880.0) * t743;
            let t746 = f64x8::splat(1.0) / t734;
            let t748 = (simd::exp(-t746 / f64x8::splat(4.0)));
            let t752 = t737 * t734;
            let t754 = -f64x8::splat(35.0) + f64x8::splat(224.0) * t734 - f64x8::splat(1440.0) * t737 + f64x8::splat(5120.0) * t752;
            let t758 = -f64x8::splat(2.0) + f64x8::splat(60.0) * t734;
            let t760 = f64x8::splat(1.0) / t732;
            let t762 = (simd::erf(t760 / f64x8::splat(2.0)));
            let t765 = f64x8::splat(2.0) * t127 * t758 * t762 + f64x8::splat(24.0) * t735 * t754 + t745 * t748;
            let t769 = ((t673).select(t731, f64x8::splat(1.0) + f64x8::splat(8.0) / f64x8::splat(7.0) * t732 * t765));
            let t770 = t769 * t53;
            let t771 = t770 * t57;
            let t772 = t489 * t669;
            let t775 = t562 * t564 + f64x8::splat(35.0) / f64x8::splat(81.0) * t667 * t670 + f64x8::splat(0.026329605555555555) * t771 * t772;
            let t777 = f64x8::splat(1.0) - t483;
            let t780 = t50 * t52 * t508 / f64x8::splat(18.0);
            let t781 = (f64x8::splat(1.35)).simd_le(t780);
            let t782 = (f64x8::splat(1.35)).simd_lt(t780);
            let t783 = ((t782).select(t780, f64x8::splat(1.35)));
            let t784 = t783 * t783;
            let t787 = t784 * t784;
            let t788 = f64x8::splat(1.0) / t787;
            let t790 = t787 * t784;
            let t791 = f64x8::splat(1.0) / t790;
            let t793 = t787 * t787;
            let t794 = f64x8::splat(1.0) / t793;
            let t797 = f64x8::splat(1.0) / t793 / t784;
            let t800 = f64x8::splat(1.0) / t793 / t787;
            let t803 = f64x8::splat(1.0) / t793 / t790;
            let t805 = t793 * t793;
            let t806 = f64x8::splat(1.0) / t805;
            let t809 = ((t782).select(f64x8::splat(1.35), t780));
            let t810 = f64x8::splat(1.0) / t809;
            let t812 = (simd::erf(t810 / f64x8::splat(2.0)));
            let t814 = t809 * t809;
            let t815 = f64x8::splat(1.0) / t814;
            let t817 = (simd::exp(-t815 / f64x8::splat(4.0)));
            let t818 = t817 - f64x8::splat(1.0);
            let t821 = t817 - f64x8::splat(3.0) / f64x8::splat(2.0) - f64x8::splat(2.0) * t814 * t818;
            let t824 = t127 * t812 + f64x8::splat(2.0) * t809 * t821;
            let t828 = ((t781).select(f64x8::splat(1.0) / t784 / f64x8::splat(36.0) - t788 / f64x8::splat(960.0) + t791 / f64x8::splat(26880.0) - t794 / f64x8::splat(829440.0) + t797 / f64x8::splat(28385280.0) - t800 / f64x8::splat(1073479680.0) + t803 / f64x8::splat(44590694400.0) - t806 / f64x8::splat(2021444812800.0), f64x8::splat(1.0) - f64x8::splat(8.0) / f64x8::splat(3.0) * t809 * t824));
            let t829 = t777 * t828;
            let t832 = (f64x8::splat(10.0) / f64x8::splat(81.0) + f64x8::splat(25.0) / f64x8::splat(8748.0) * t490) * t53;
            let t833 = t57 * v_sigma2;
            let t843 = (t663 - t489 / f64x8::splat(8.0)) * t53 * t57 / f64x8::splat(4.0) - f64x8::splat(9.0) / f64x8::splat(20.0) + t490 / f64x8::splat(36.0);
            let t844 = t843 * t843;
            let t846 = t843 * t475;
            let t847 = f64x8::splat(1.0) - t475;
            let t850 = f64x8::splat(1.0) + f64x8::splat(5.0) / f64x8::splat(12.0) * t832 * t833 * t488 + f64x8::splat(292.0) / f64x8::splat(405.0) * t844 - f64x8::splat(146.0) / f64x8::splat(135.0) * t846 * t847;
            let t851 = (simd::pow(t850, f64x8::splat(1.0) / f64x8::splat(10.0)));
            let t861 = f64x8::splat(1.0) + f64x8::splat(0.06394332777777778) * t490 - f64x8::splat(5.0) / f64x8::splat(9.0) * (t664 + t442 + f64x8::splat(0.011867481666666667) * t489) * t53 * t57;
            let t864 = t564 + f64x8::splat(7.0) / f64x8::splat(9.0) * t861 * t669;
            let t867 = -param_hyb_coeff_0 * (t483 * t775 + t829 * t851) + t483 * t864 + t777 * t851;
            let t871 = ((t459).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t468 * t867));
            let tzk0 = t458 + t871;
            acc_zk = tzk0;
        }
        store_add(zk, ip, m, acc_zk);
        ip += 8;
    }
}
