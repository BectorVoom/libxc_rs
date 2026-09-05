//! MGGA_X_TPSS exc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_tpss.c`
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
pub fn mgga_x_tpss_exc_pol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    param_BLOC_a: f64,
    param_BLOC_b: f64,
    param_b: f64,
    param_c: f64,
    param_e: f64,
    param_kappa: f64,
    param_mu: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_BLOC_a = f64x8::splat(param_BLOC_a);
    let param_BLOC_b = f64x8::splat(param_BLOC_b);
    let param_b = f64x8::splat(param_b);
    let param_c = f64x8::splat(param_c);
    let param_e = f64x8::splat(param_e);
    let param_kappa = f64x8::splat(param_kappa);
    let param_mu = f64x8::splat(param_mu);
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
            let t31 = f64x8::splat(1.0) / v_tau0;
            let t33 = v_sigma0 * t29 * t31 / f64x8::splat(8.0);
            let t34 = param_BLOC_b * v_sigma0;
            let t38 = param_BLOC_a + t34 * t29 * t31 / f64x8::splat(8.0);
            let t39 = (simd::pow(t33, t38));
            let t40 = param_c * t39;
            let t41 = v_sigma0 * v_sigma0;
            let t42 = v_rho0 * v_rho0;
            let t43 = f64x8::splat(1.0) / t42;
            let t44 = t41 * t43;
            let t45 = v_tau0 * v_tau0;
            let t46 = f64x8::splat(1.0) / t45;
            let t47 = t44 * t46;
            let t49 = f64x8::splat(1.0) + t47 / f64x8::splat(64.0);
            let t50 = t49 * t49;
            let t51 = f64x8::splat(1.0) / t50;
            let t54 = f64x8::splat(M_CBRT6);
            let t55 = (f64x8::splat(10.0) / f64x8::splat(81.0) + t40 * t51) * t54;
            let t56 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t57 = (simd::cbrt(t56));
            let t58 = t57 * t57;
            let t59 = f64x8::splat(1.0) / t58;
            let t60 = t59 * v_sigma0;
            let t61 = (simd::cbrt(v_rho0));
            let t62 = t61 * t61;
            let t64 = f64x8::splat(1.0) / t62 / t42;
            let t65 = t60 * t64;
            let t69 = f64x8::splat(1.0) / t62 / v_rho0;
            let t71 = v_sigma0 * t64;
            let t73 = v_tau0 * t69 - t71 / f64x8::splat(8.0);
            let t77 = f64x8::splat(5.0) / f64x8::splat(9.0) * t73 * t54 * t59 - f64x8::splat(1.0);
            let t78 = param_b * t73;
            let t79 = t54 * t59;
            let t80 = t79 * t77;
            let t83 = f64x8::splat(5.0) * t78 * t80 + f64x8::splat(9.0);
            let t84 = ((t83).sqrt());
            let t85 = f64x8::splat(1.0) / t84;
            let t90 = f64x8::splat(27.0) / f64x8::splat(20.0) * t77 * t85 + t79 * t71 / f64x8::splat(36.0);
            let t91 = t90 * t90;
            let t94 = t54 * t54;
            let t96 = f64x8::splat(1.0) / t57 / t56;
            let t97 = t94 * t96;
            let t98 = t42 * t42;
            let t99 = t98 * v_rho0;
            let t101 = f64x8::splat(1.0) / t61 / t99;
            let t105 = f64x8::splat(50.0) * t97 * t41 * t101 + f64x8::splat(162.0) * t47;
            let t106 = ((t105).sqrt());
            let t110 = f64x8::splat(1.0) / param_kappa * t94;
            let t111 = t96 * t41;
            let t115 = ((param_e).sqrt());
            let t116 = t115 * t41;
            let t117 = t43 * t46;
            let t120 = param_e * param_mu;
            let t121 = t56 * t56;
            let t122 = f64x8::splat(1.0) / t121;
            let t123 = t41 * v_sigma0;
            let t124 = t122 * t123;
            let t125 = t98 * t98;
            let t126 = f64x8::splat(1.0) / t125;
            let t130 = t55 * t65 / f64x8::splat(24.0) + f64x8::splat(146.0) / f64x8::splat(2025.0) * t91 - f64x8::splat(73.0) / f64x8::splat(97200.0) * t90 * t106 + f64x8::splat(25.0) / f64x8::splat(944784.0) * t110 * t111 * t101 + t116 * t117 / f64x8::splat(720.0) + t120 * t124 * t126 / f64x8::splat(2304.0);
            let t131 = t115 * t54;
            let t134 = f64x8::splat(1.0) + t131 * t65 / f64x8::splat(24.0);
            let t135 = t134 * t134;
            let t136 = f64x8::splat(1.0) / t135;
            let t138 = t130 * t136 + param_kappa;
            let t143 = f64x8::splat(1.0) + param_kappa * (f64x8::splat(1.0) - param_kappa / t138);
            let t147 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t28 * t143));
            let t148 = (v_rho1).simd_le(dens_threshold);
            let t149 = -t17;
            let t151 = ((t15).select(t12, (t11).select(t16, t149 * t8)));
            let t152 = f64x8::splat(1.0) + t151;
            let t153 = (t152).simd_le(zeta_threshold);
            let t154 = (simd::cbrt(t152));
            let t156 = ((t153).select(t23, t154 * t152));
            let t157 = t156 * t27;
            let t158 = f64x8::splat(1.0) / v_rho1;
            let t160 = f64x8::splat(1.0) / v_tau1;
            let t162 = v_sigma2 * t158 * t160 / f64x8::splat(8.0);
            let t163 = param_BLOC_b * v_sigma2;
            let t167 = param_BLOC_a + t163 * t158 * t160 / f64x8::splat(8.0);
            let t168 = (simd::pow(t162, t167));
            let t169 = param_c * t168;
            let t170 = v_sigma2 * v_sigma2;
            let t171 = v_rho1 * v_rho1;
            let t172 = f64x8::splat(1.0) / t171;
            let t173 = t170 * t172;
            let t174 = v_tau1 * v_tau1;
            let t175 = f64x8::splat(1.0) / t174;
            let t176 = t173 * t175;
            let t178 = f64x8::splat(1.0) + t176 / f64x8::splat(64.0);
            let t179 = t178 * t178;
            let t180 = f64x8::splat(1.0) / t179;
            let t183 = (f64x8::splat(10.0) / f64x8::splat(81.0) + t169 * t180) * t54;
            let t184 = t59 * v_sigma2;
            let t185 = (simd::cbrt(v_rho1));
            let t186 = t185 * t185;
            let t188 = f64x8::splat(1.0) / t186 / t171;
            let t189 = t184 * t188;
            let t193 = f64x8::splat(1.0) / t186 / v_rho1;
            let t195 = v_sigma2 * t188;
            let t197 = v_tau1 * t193 - t195 / f64x8::splat(8.0);
            let t201 = f64x8::splat(5.0) / f64x8::splat(9.0) * t197 * t54 * t59 - f64x8::splat(1.0);
            let t202 = param_b * t197;
            let t203 = t79 * t201;
            let t206 = f64x8::splat(5.0) * t202 * t203 + f64x8::splat(9.0);
            let t207 = ((t206).sqrt());
            let t208 = f64x8::splat(1.0) / t207;
            let t213 = f64x8::splat(27.0) / f64x8::splat(20.0) * t201 * t208 + t79 * t195 / f64x8::splat(36.0);
            let t214 = t213 * t213;
            let t217 = t171 * t171;
            let t218 = t217 * v_rho1;
            let t220 = f64x8::splat(1.0) / t185 / t218;
            let t224 = f64x8::splat(50.0) * t97 * t170 * t220 + f64x8::splat(162.0) * t176;
            let t225 = ((t224).sqrt());
            let t228 = t96 * t170;
            let t232 = t115 * t170;
            let t233 = t172 * t175;
            let t236 = t170 * v_sigma2;
            let t237 = t122 * t236;
            let t238 = t217 * t217;
            let t239 = f64x8::splat(1.0) / t238;
            let t243 = t183 * t189 / f64x8::splat(24.0) + f64x8::splat(146.0) / f64x8::splat(2025.0) * t214 - f64x8::splat(73.0) / f64x8::splat(97200.0) * t213 * t225 + f64x8::splat(25.0) / f64x8::splat(944784.0) * t110 * t228 * t220 + t232 * t233 / f64x8::splat(720.0) + t120 * t237 * t239 / f64x8::splat(2304.0);
            let t246 = f64x8::splat(1.0) + t131 * t189 / f64x8::splat(24.0);
            let t247 = t246 * t246;
            let t248 = f64x8::splat(1.0) / t247;
            let t250 = t243 * t248 + param_kappa;
            let t255 = f64x8::splat(1.0) + param_kappa * (f64x8::splat(1.0) - param_kappa / t250);
            let t259 = ((t148).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t157 * t255));
            let tzk0 = t147 + t259;
            acc_zk = tzk0;
        }
        store_add(zk, ip, m, acc_zk);
        ip += 8;
    }
}
