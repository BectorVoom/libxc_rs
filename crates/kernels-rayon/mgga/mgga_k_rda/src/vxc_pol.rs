//! MGGA_K_RDA vxc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_k_rda.c`
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
pub fn mgga_k_rda_vxc_pol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    vlapl: &mut [f64],
    vtau: &mut [f64],
    param_A0: f64,
    param_A1: f64,
    param_A2: f64,
    param_A3: f64,
    param_a: f64,
    param_b: f64,
    param_beta1: f64,
    param_beta2: f64,
    param_beta3: f64,
    param_c: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_A0 = f64x8::splat(param_A0);
    let param_A1 = f64x8::splat(param_A1);
    let param_A2 = f64x8::splat(param_A2);
    let param_A3 = f64x8::splat(param_A3);
    let param_a = f64x8::splat(param_a);
    let param_b = f64x8::splat(param_b);
    let param_beta1 = f64x8::splat(param_beta1);
    let param_beta2 = f64x8::splat(param_beta2);
    let param_beta3 = f64x8::splat(param_beta3);
    let param_c = f64x8::splat(param_c);
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
            let t4 = t3 * t3;
            let t5 = f64x8::splat(M_CBRTPI);
            let t7 = t4 * t5 * f64x8::splat(M_PI);
            let t8 = v_rho0 + v_rho1;
            let t9 = f64x8::splat(1.0) / t8;
            let t12 = (f64x8::splat(2.0) * v_rho0 * t9).simd_le(zeta_threshold);
            let t13 = zeta_threshold - f64x8::splat(1.0);
            let t16 = (f64x8::splat(2.0) * v_rho1 * t9).simd_le(zeta_threshold);
            let t17 = -t13;
            let t18 = v_rho0 - v_rho1;
            let t20 = ((t12).select(t13, (t16).select(t17, t18 * t9)));
            let t21 = f64x8::splat(1.0) + t20;
            let t22 = (t21).simd_le(zeta_threshold);
            let t23 = (simd::cbrt(zeta_threshold));
            let t24 = t23 * t23;
            let t25 = t24 * zeta_threshold;
            let t26 = (simd::cbrt(t21));
            let t27 = t26 * t26;
            let t29 = ((t22).select(t25, t27 * t21));
            let t30 = (simd::cbrt(t8));
            let t31 = t30 * t30;
            let t32 = t29 * t31;
            let t33 = f64x8::splat(M_CBRT6);
            let t34 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t35 = (simd::cbrt(t34));
            let t36 = t35 * t35;
            let t37 = f64x8::splat(1.0) / t36;
            let t38 = t33 * t37;
            let t39 = v_rho0 * v_rho0;
            let t40 = (simd::cbrt(v_rho0));
            let t41 = t40 * t40;
            let t43 = f64x8::splat(1.0) / t41 / t39;
            let t45 = t38 * v_sigma0 * t43;
            let t47 = t33 * t33;
            let t49 = f64x8::splat(1.0) / t35 / t34;
            let t50 = t47 * t49;
            let t51 = v_sigma0 * v_sigma0;
            let t52 = t39 * t39;
            let t53 = t52 * v_rho0;
            let t55 = f64x8::splat(1.0) / t40 / t53;
            let t57 = t50 * t51 * t55;
            let t58 = param_a * t47;
            let t59 = v_lapl0 * v_lapl0;
            let t60 = t49 * t59;
            let t61 = t39 * v_rho0;
            let t63 = f64x8::splat(1.0) / t40 / t61;
            let t64 = t60 * t63;
            let t66 = t58 * t64 + t57;
            let t68 = ((t66).sqrt());
            let t71 = f64x8::splat(1.0) + param_beta1 * t68 / f64x8::splat(24.0);
            let t72 = t71 * t71;
            let t73 = f64x8::splat(1.0) / t72;
            let t76 = param_b * t47;
            let t78 = t76 * t64 + t57;
            let t79 = t78 * t78;
            let t81 = ((t78).sqrt());
            let t84 = f64x8::splat(1.0) + param_beta2 * t81 / f64x8::splat(24.0);
            let t85 = t84 * t84;
            let t86 = t85 * t85;
            let t87 = f64x8::splat(1.0) / t86;
            let t90 = param_c * t33;
            let t91 = t37 * v_lapl0;
            let t93 = f64x8::splat(1.0) / t41 / v_rho0;
            let t97 = t90 * t91 * t93 / f64x8::splat(24.0) + t45 / f64x8::splat(24.0);
            let t98 = param_A3 * t97;
            let t100 = param_beta3 * t97 + f64x8::splat(1.0);
            let t101 = f64x8::splat(1.0) / t100;
            let t103 = f64x8::splat(5.0) / f64x8::splat(72.0) * t45 + param_A0 + param_A1 * t66 * t73 / f64x8::splat(576.0) + param_A2 * t79 * t87 / f64x8::splat(331776.0) + t98 * t101;
            let t107 = ((t2).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t7 * t32 * t103));
            let t108 = (v_rho1).simd_le(dens_threshold);
            let t109 = -t18;
            let t111 = ((t16).select(t13, (t12).select(t17, t109 * t9)));
            let t112 = f64x8::splat(1.0) + t111;
            let t113 = (t112).simd_le(zeta_threshold);
            let t114 = (simd::cbrt(t112));
            let t115 = t114 * t114;
            let t117 = ((t113).select(t25, t115 * t112));
            let t118 = t117 * t31;
            let t119 = v_rho1 * v_rho1;
            let t120 = (simd::cbrt(v_rho1));
            let t121 = t120 * t120;
            let t123 = f64x8::splat(1.0) / t121 / t119;
            let t125 = t38 * v_sigma2 * t123;
            let t127 = v_sigma2 * v_sigma2;
            let t128 = t119 * t119;
            let t129 = t128 * v_rho1;
            let t131 = f64x8::splat(1.0) / t120 / t129;
            let t133 = t50 * t127 * t131;
            let t134 = v_lapl1 * v_lapl1;
            let t135 = t49 * t134;
            let t136 = t119 * v_rho1;
            let t138 = f64x8::splat(1.0) / t120 / t136;
            let t139 = t135 * t138;
            let t141 = t58 * t139 + t133;
            let t143 = ((t141).sqrt());
            let t146 = f64x8::splat(1.0) + param_beta1 * t143 / f64x8::splat(24.0);
            let t147 = t146 * t146;
            let t148 = f64x8::splat(1.0) / t147;
            let t152 = t76 * t139 + t133;
            let t153 = t152 * t152;
            let t155 = ((t152).sqrt());
            let t158 = f64x8::splat(1.0) + param_beta2 * t155 / f64x8::splat(24.0);
            let t159 = t158 * t158;
            let t160 = t159 * t159;
            let t161 = f64x8::splat(1.0) / t160;
            let t164 = t37 * v_lapl1;
            let t166 = f64x8::splat(1.0) / t121 / v_rho1;
            let t170 = t90 * t164 * t166 / f64x8::splat(24.0) + t125 / f64x8::splat(24.0);
            let t171 = param_A3 * t170;
            let t173 = param_beta3 * t170 + f64x8::splat(1.0);
            let t174 = f64x8::splat(1.0) / t173;
            let t176 = f64x8::splat(5.0) / f64x8::splat(72.0) * t125 + param_A0 + param_A1 * t141 * t148 / f64x8::splat(576.0) + param_A2 * t153 * t161 / f64x8::splat(331776.0) + t171 * t174;
            let t180 = ((t108).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t7 * t118 * t176));
            let tzk0 = t107 + t180;
            acc_zk = tzk0;
            let t181 = t8 * t8;
            let t182 = f64x8::splat(1.0) / t181;
            let t183 = t18 * t182;
            let t185 = ((t12).select(f64x8::splat(0.0), (t16).select(f64x8::splat(0.0), t9 - t183)));
            let t188 = ((t22).select(f64x8::splat(0.0), f64x8::splat(5.0) / f64x8::splat(3.0) * t27 * t185));
            let t189 = t188 * t31;
            let t193 = f64x8::splat(1.0) / t30;
            let t194 = t29 * t193;
            let t197 = t7 * t194 * t103 / f64x8::splat(10.0);
            let t199 = f64x8::splat(1.0) / t41 / t61;
            let t201 = t38 * v_sigma0 * t199;
            let t203 = t52 * t39;
            let t205 = f64x8::splat(1.0) / t40 / t203;
            let t208 = f64x8::splat(16.0) / f64x8::splat(3.0) * t50 * t51 * t205;
            let t210 = f64x8::splat(1.0) / t40 / t52;
            let t211 = t60 * t210;
            let t214 = -t208 - f64x8::splat(10.0) / f64x8::splat(3.0) * t58 * t211;
            let t218 = param_A1 * t68;
            let t220 = f64x8::splat(1.0) / t72 / t71;
            let t221 = t220 * param_beta1;
            let t225 = param_A2 * t78;
            let t228 = -t208 - f64x8::splat(10.0) / f64x8::splat(3.0) * t76 * t211;
            let t232 = t81 * t78;
            let t233 = param_A2 * t232;
            let t235 = f64x8::splat(1.0) / t86 / t84;
            let t236 = t235 * param_beta2;
            let t244 = -t201 / f64x8::splat(9.0) - f64x8::splat(5.0) / f64x8::splat(72.0) * t90 * t91 * t43;
            let t245 = param_A3 * t244;
            let t247 = t100 * t100;
            let t248 = f64x8::splat(1.0) / t247;
            let t249 = t248 * param_beta3;
            let t250 = t249 * t244;
            let t252 = -f64x8::splat(5.0) / f64x8::splat(27.0) * t201 + param_A1 * t214 * t73 / f64x8::splat(576.0) - t218 * t221 * t214 / f64x8::splat(13824.0) + t225 * t87 * t228 / f64x8::splat(165888.0) - t233 * t236 * t228 / f64x8::splat(3981312.0) + t245 * t101 - t98 * t250;
            let t257 = ((t2).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t7 * t189 * t103 + t197 + f64x8::splat(3.0) / f64x8::splat(20.0) * t7 * t32 * t252));
            let t258 = t109 * t182;
            let t260 = ((t16).select(f64x8::splat(0.0), (t12).select(f64x8::splat(0.0), -t9 - t258)));
            let t263 = ((t113).select(f64x8::splat(0.0), f64x8::splat(5.0) / f64x8::splat(3.0) * t115 * t260));
            let t264 = t263 * t31;
            let t268 = t117 * t193;
            let t271 = t7 * t268 * t176 / f64x8::splat(10.0);
            let t273 = ((t108).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t7 * t264 * t176 + t271));
            let tvrho0 = t107 + t180 + t8 * (t257 + t273);
            acc_vrho_0 = tvrho0;
            let t277 = ((t12).select(f64x8::splat(0.0), (t16).select(f64x8::splat(0.0), -t9 - t183)));
            let t280 = ((t22).select(f64x8::splat(0.0), f64x8::splat(5.0) / f64x8::splat(3.0) * t27 * t277));
            let t281 = t280 * t31;
            let t286 = ((t2).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t7 * t281 * t103 + t197));
            let t288 = ((t16).select(f64x8::splat(0.0), (t12).select(f64x8::splat(0.0), t9 - t258)));
            let t291 = ((t113).select(f64x8::splat(0.0), f64x8::splat(5.0) / f64x8::splat(3.0) * t115 * t288));
            let t292 = t291 * t31;
            let t297 = f64x8::splat(1.0) / t121 / t136;
            let t299 = t38 * v_sigma2 * t297;
            let t301 = t128 * t119;
            let t303 = f64x8::splat(1.0) / t120 / t301;
            let t306 = f64x8::splat(16.0) / f64x8::splat(3.0) * t50 * t127 * t303;
            let t308 = f64x8::splat(1.0) / t120 / t128;
            let t309 = t135 * t308;
            let t312 = -t306 - f64x8::splat(10.0) / f64x8::splat(3.0) * t58 * t309;
            let t316 = param_A1 * t143;
            let t318 = f64x8::splat(1.0) / t147 / t146;
            let t319 = t318 * param_beta1;
            let t323 = param_A2 * t152;
            let t326 = -t306 - f64x8::splat(10.0) / f64x8::splat(3.0) * t76 * t309;
            let t330 = t155 * t152;
            let t331 = param_A2 * t330;
            let t333 = f64x8::splat(1.0) / t160 / t158;
            let t334 = t333 * param_beta2;
            let t342 = -t299 / f64x8::splat(9.0) - f64x8::splat(5.0) / f64x8::splat(72.0) * t90 * t164 * t123;
            let t343 = param_A3 * t342;
            let t345 = t173 * t173;
            let t346 = f64x8::splat(1.0) / t345;
            let t347 = t346 * param_beta3;
            let t348 = t347 * t342;
            let t350 = -f64x8::splat(5.0) / f64x8::splat(27.0) * t299 + param_A1 * t312 * t148 / f64x8::splat(576.0) - t316 * t319 * t312 / f64x8::splat(13824.0) + t323 * t161 * t326 / f64x8::splat(165888.0) - t331 * t334 * t326 / f64x8::splat(3981312.0) + t343 * t174 - t171 * t348;
            let t355 = ((t108).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t7 * t292 * t176 + t271 + f64x8::splat(3.0) / f64x8::splat(20.0) * t7 * t118 * t350));
            let tvrho1 = t107 + t180 + t8 * (t286 + t355);
            acc_vrho_1 = tvrho1;
            let t360 = param_A1 * t47;
            let t361 = t360 * t49;
            let t362 = v_sigma0 * t55;
            let t366 = t218 * t221;
            let t367 = t50 * t362;
            let t370 = t225 * t87;
            let t373 = t233 * t236;
            let t376 = param_A3 * t33;
            let t377 = t37 * t43;
            let t378 = t377 * t101;
            let t381 = t98 * t248;
            let t382 = param_beta3 * t33;
            let t386 = f64x8::splat(5.0) / f64x8::splat(72.0) * t38 * t43 + t361 * t362 * t73 / f64x8::splat(288.0) - t366 * t367 / f64x8::splat(6912.0) + t370 * t367 / f64x8::splat(82944.0) - t373 * t367 / f64x8::splat(1990656.0) + t376 * t378 / f64x8::splat(24.0) - t381 * t382 * t377 / f64x8::splat(24.0);
            let t390 = ((t2).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t7 * t32 * t386));
            let tvsigma0 = t8 * t390;
            acc_vsigma_0 = tvsigma0;
            let tvsigma1 = f64x8::splat(0.0);
            acc_vsigma_1 = tvsigma1;
            let t393 = v_sigma2 * t131;
            let t397 = t316 * t319;
            let t398 = t50 * t393;
            let t401 = t323 * t161;
            let t404 = t331 * t334;
            let t407 = t37 * t123;
            let t408 = t407 * t174;
            let t411 = t171 * t346;
            let t415 = f64x8::splat(5.0) / f64x8::splat(72.0) * t38 * t123 + t361 * t393 * t148 / f64x8::splat(288.0) - t397 * t398 / f64x8::splat(6912.0) + t401 * t398 / f64x8::splat(82944.0) - t404 * t398 / f64x8::splat(1990656.0) + t376 * t408 / f64x8::splat(24.0) - t411 * t382 * t407 / f64x8::splat(24.0);
            let t419 = ((t108).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t7 * t118 * t415));
            let tvsigma2 = t8 * t419;
            acc_vsigma_2 = tvsigma2;
            let t420 = param_A1 * param_a;
            let t421 = t420 * t47;
            let t422 = t49 * v_lapl0;
            let t427 = t422 * t63;
            let t431 = t87 * param_b;
            let t432 = t225 * t431;
            let t433 = v_lapl0 * t63;
            let t434 = t50 * t433;
            let t440 = param_A3 * param_c;
            let t441 = t440 * t33;
            let t442 = t37 * t93;
            let t446 = t98 * t249;
            let t447 = t90 * t442;
            let t450 = t421 * t422 * t63 * t73 / f64x8::splat(288.0) - t366 * t58 * t427 / f64x8::splat(6912.0) + t432 * t434 / f64x8::splat(82944.0) - t373 * t76 * t427 / f64x8::splat(1990656.0) + t441 * t442 * t101 / f64x8::splat(24.0) - t446 * t447 / f64x8::splat(24.0);
            let t454 = ((t2).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t7 * t32 * t450));
            let tvlapl0 = t8 * t454;
            acc_vlapl_0 = tvlapl0;
            let t455 = t49 * v_lapl1;
            let t460 = t455 * t138;
            let t464 = t161 * param_b;
            let t465 = t323 * t464;
            let t466 = v_lapl1 * t138;
            let t467 = t50 * t466;
            let t473 = t37 * t166;
            let t477 = t171 * t347;
            let t478 = t90 * t473;
            let t481 = t421 * t455 * t138 * t148 / f64x8::splat(288.0) - t397 * t58 * t460 / f64x8::splat(6912.0) + t465 * t467 / f64x8::splat(82944.0) - t404 * t76 * t460 / f64x8::splat(1990656.0) + t441 * t473 * t174 / f64x8::splat(24.0) - t477 * t478 / f64x8::splat(24.0);
            let t485 = ((t108).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t7 * t118 * t481));
            let tvlapl1 = t8 * t485;
            acc_vlapl_1 = tvlapl1;
            let tvtau0 = f64x8::splat(0.0);
            acc_vtau_0 = tvtau0;
            let tvtau1 = f64x8::splat(0.0);
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
