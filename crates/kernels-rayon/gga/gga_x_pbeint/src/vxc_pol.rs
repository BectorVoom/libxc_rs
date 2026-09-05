//! GGA_X_PBEINT vxc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_pbeint.c`
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
pub fn gga_x_pbeint_vxc_pol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    param_muPBE: f64,
    param_muGE: f64,
    param_alpha: f64,
    param_kappa: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_muPBE = f64x8::splat(param_muPBE);
    let param_muGE = f64x8::splat(param_muGE);
    let param_alpha = f64x8::splat(param_alpha);
    let param_kappa = f64x8::splat(param_kappa);
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
            let t26 = (simd::cbrt(t6));
            let t27 = t25 * t26;
            let t28 = param_muPBE - param_muGE;
            let t30 = f64x8::splat(M_CBRT6);
            let t31 = t28 * param_alpha * t30;
            let t32 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t33 = (simd::cbrt(t32));
            let t34 = t33 * t33;
            let t35 = f64x8::splat(1.0) / t34;
            let t36 = t35 * v_sigma0;
            let t37 = v_rho0 * v_rho0;
            let t38 = (simd::cbrt(v_rho0));
            let t39 = t38 * t38;
            let t41 = f64x8::splat(1.0) / t39 / t37;
            let t42 = param_alpha * t30;
            let t43 = t36 * t41;
            let t46 = f64x8::splat(1.0) + t42 * t43 / f64x8::splat(24.0);
            let t47 = f64x8::splat(1.0) / t46;
            let t53 = (param_muGE + t31 * t36 * t41 * t47 / f64x8::splat(24.0)) * t30;
            let t56 = param_kappa + t53 * t43 / f64x8::splat(24.0);
            let t61 = f64x8::splat(1.0) + param_kappa * (f64x8::splat(1.0) - param_kappa / t56);
            let t65 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t27 * t61));
            let t66 = (v_rho1).simd_le(dens_threshold);
            let t67 = -t16;
            let t69 = ((t14).select(t11, (t10).select(t15, t67 * t7)));
            let t70 = f64x8::splat(1.0) + t69;
            let t71 = (t70).simd_le(zeta_threshold);
            let t72 = (simd::cbrt(t70));
            let t74 = ((t71).select(t22, t72 * t70));
            let t75 = t74 * t26;
            let t76 = t35 * v_sigma2;
            let t77 = v_rho1 * v_rho1;
            let t78 = (simd::cbrt(v_rho1));
            let t79 = t78 * t78;
            let t81 = f64x8::splat(1.0) / t79 / t77;
            let t82 = t76 * t81;
            let t85 = f64x8::splat(1.0) + t42 * t82 / f64x8::splat(24.0);
            let t86 = f64x8::splat(1.0) / t85;
            let t92 = (param_muGE + t31 * t76 * t81 * t86 / f64x8::splat(24.0)) * t30;
            let t95 = param_kappa + t92 * t82 / f64x8::splat(24.0);
            let t100 = f64x8::splat(1.0) + param_kappa * (f64x8::splat(1.0) - param_kappa / t95);
            let t104 = ((t66).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t75 * t100));
            let tzk0 = t65 + t104;
            acc_zk = tzk0;
            let t105 = t6 * t6;
            let t106 = f64x8::splat(1.0) / t105;
            let t107 = t16 * t106;
            let t109 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), t7 - t107)));
            let t112 = ((t20).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t109));
            let t113 = t112 * t26;
            let t117 = t26 * t26;
            let t118 = f64x8::splat(1.0) / t117;
            let t119 = t25 * t118;
            let t122 = t5 * t119 * t61 / f64x8::splat(8.0);
            let t123 = t5 * t25;
            let t124 = param_kappa * param_kappa;
            let t125 = t26 * t124;
            let t126 = t56 * t56;
            let t127 = f64x8::splat(1.0) / t126;
            let t128 = t37 * v_rho0;
            let t130 = f64x8::splat(1.0) / t39 / t128;
            let t135 = param_alpha * param_alpha;
            let t137 = t30 * t30;
            let t138 = t28 * t135 * t137;
            let t140 = f64x8::splat(1.0) / t33 / t32;
            let t141 = v_sigma0 * v_sigma0;
            let t142 = t140 * t141;
            let t143 = t37 * t37;
            let t144 = t143 * t37;
            let t146 = f64x8::splat(1.0) / t38 / t144;
            let t147 = t46 * t46;
            let t148 = f64x8::splat(1.0) / t147;
            let t154 = (-t31 * t36 * t130 * t47 / f64x8::splat(9.0) + t138 * t142 * t146 * t148 / f64x8::splat(216.0)) * t30;
            let t157 = t36 * t130;
            let t160 = t154 * t43 / f64x8::splat(24.0) - t53 * t157 / f64x8::splat(9.0);
            let t161 = t127 * t160;
            let t162 = t125 * t161;
            let t166 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t113 * t61 - t122 - f64x8::splat(3.0) / f64x8::splat(8.0) * t123 * t162));
            let t167 = t67 * t106;
            let t169 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), -t7 - t167)));
            let t172 = ((t71).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t72 * t169));
            let t173 = t172 * t26;
            let t177 = t74 * t118;
            let t180 = t5 * t177 * t100 / f64x8::splat(8.0);
            let t182 = ((t66).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t173 * t100 - t180));
            let tvrho0 = t65 + t104 + t6 * (t166 + t182);
            acc_vrho_0 = tvrho0;
            let t186 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), -t7 - t107)));
            let t189 = ((t20).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t186));
            let t190 = t189 * t26;
            let t195 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t190 * t61 - t122));
            let t197 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), t7 - t167)));
            let t200 = ((t71).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t72 * t197));
            let t201 = t200 * t26;
            let t205 = t5 * t74;
            let t206 = t95 * t95;
            let t207 = f64x8::splat(1.0) / t206;
            let t208 = t77 * v_rho1;
            let t210 = f64x8::splat(1.0) / t79 / t208;
            let t215 = v_sigma2 * v_sigma2;
            let t216 = t140 * t215;
            let t217 = t77 * t77;
            let t218 = t217 * t77;
            let t220 = f64x8::splat(1.0) / t78 / t218;
            let t221 = t85 * t85;
            let t222 = f64x8::splat(1.0) / t221;
            let t228 = (-t31 * t76 * t210 * t86 / f64x8::splat(9.0) + t138 * t216 * t220 * t222 / f64x8::splat(216.0)) * t30;
            let t231 = t76 * t210;
            let t234 = t228 * t82 / f64x8::splat(24.0) - t92 * t231 / f64x8::splat(9.0);
            let t235 = t207 * t234;
            let t236 = t125 * t235;
            let t240 = ((t66).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t201 * t100 - t180 - f64x8::splat(3.0) / f64x8::splat(8.0) * t205 * t236));
            let tvrho1 = t65 + t104 + t6 * (t195 + t240);
            acc_vrho_1 = tvrho1;
            let t243 = t35 * t41;
            let t248 = t143 * v_rho0;
            let t250 = f64x8::splat(1.0) / t38 / t248;
            let t256 = (t31 * t243 * t47 / f64x8::splat(24.0) - t138 * t140 * v_sigma0 * t250 * t148 / f64x8::splat(576.0)) * t30;
            let t260 = t53 * t243 / f64x8::splat(24.0) + t256 * t43 / f64x8::splat(24.0);
            let t261 = t127 * t260;
            let t262 = t125 * t261;
            let t265 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t123 * t262));
            let tvsigma0 = t6 * t265;
            acc_vsigma_0 = tvsigma0;
            let tvsigma1 = f64x8::splat(0.0);
            acc_vsigma_1 = tvsigma1;
            let t266 = t35 * t81;
            let t271 = t217 * v_rho1;
            let t273 = f64x8::splat(1.0) / t78 / t271;
            let t279 = (t31 * t266 * t86 / f64x8::splat(24.0) - t138 * t140 * v_sigma2 * t273 * t222 / f64x8::splat(576.0)) * t30;
            let t283 = t92 * t266 / f64x8::splat(24.0) + t279 * t82 / f64x8::splat(24.0);
            let t284 = t207 * t283;
            let t285 = t125 * t284;
            let t288 = ((t66).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t205 * t285));
            let tvsigma2 = t6 * t288;
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
