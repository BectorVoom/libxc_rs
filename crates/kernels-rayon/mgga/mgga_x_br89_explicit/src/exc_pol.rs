//! MGGA_X_BR89_EXPLICIT exc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_br89_explicit.c`
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
pub fn mgga_x_br89_explicit_exc_pol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    param_gamma: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_gamma = f64x8::splat(param_gamma);
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
            let t3 = v_rho0 + v_rho1;
            let t4 = f64x8::splat(1.0) / t3;
            let t7 = (f64x8::splat(2.0) * v_rho0 * t4).simd_le(zeta_threshold);
            let t8 = zeta_threshold - f64x8::splat(1.0);
            let t11 = (f64x8::splat(2.0) * v_rho1 * t4).simd_le(zeta_threshold);
            let t12 = -t8;
            let t13 = v_rho0 - v_rho1;
            let t15 = ((t7).select(t8, (t11).select(t12, t13 * t4)));
            let t16 = f64x8::splat(1.0) + t15;
            let t17 = (t16).simd_le(zeta_threshold);
            let t18 = (simd::cbrt(zeta_threshold));
            let t19 = t18 * zeta_threshold;
            let t20 = (simd::cbrt(t16));
            let t22 = ((t17).select(t19, t20 * t16));
            let t23 = (simd::cbrt(t3));
            let t24 = t22 * t23;
            let t26 = (simd::cbrt(f64x8::splat(1.0) / f64x8::splat(M_PI)));
            let t27 = f64x8::splat(1.0) / t26;
            let t28 = t24 * t27;
            let t29 = f64x8::splat(M_CBRT4);
            let t30 = f64x8::splat(M_CBRTPI);
            let t31 = t30 * t30;
            let t32 = (simd::cbrt(v_rho0));
            let t33 = t32 * t32;
            let t35 = f64x8::splat(1.0) / t33 / v_rho0;
            let t36 = v_lapl0 * t35;
            let t38 = v_tau0 * param_gamma;
            let t39 = t38 * t35;
            let t41 = param_gamma * v_sigma0;
            let t42 = v_rho0 * v_rho0;
            let t44 = f64x8::splat(1.0) / t33 / t42;
            let t45 = t41 * t44;
            let t48 = ((t36 / f64x8::splat(2.0) - f64x8::splat(2.0) * t39 + t45 / f64x8::splat(4.0)).abs());
            let t50 = (t48 / f64x8::splat(3.0)).simd_lt(f64x8::splat(5e-13));
            let t54 = t36 / f64x8::splat(6.0) - f64x8::splat(2.0) / f64x8::splat(3.0) * t39 + t45 / f64x8::splat(12.0);
            let t55 = (f64x8::splat(0.0)).simd_lt(t54);
            let t56 = ((t55).select(f64x8::splat(5e-13), -f64x8::splat(5e-13)));
            let t57 = ((t50).select(t56, t54));
            let t60 = f64x8::splat(2.0) / f64x8::splat(3.0) * t31 / t57;
            let t61 = (t60).simd_le(f64x8::splat(0.0));
            let t62 = (-f64x8::splat(5e-13)).simd_lt(t60);
            let t63 = ((t62).select(-f64x8::splat(5e-13), t60));
            let t65 = f64x8::splat(1.525525181200953) * t63 + f64x8::splat(0.4576575543602858);
            let t66 = (simd::atan(t65));
            let t67 = -t66 + f64x8::splat(0.4292036732051034);
            let t69 = t63 * t63;
            let t71 = t69 * t63;
            let t73 = t69 * t69;
            let t75 = t73 * t63;
            let t77 = f64x8::splat(0.7566445420735584) - f64x8::splat(2.636397787137096) * t63 + f64x8::splat(5.474515996423288) * t69 - f64x8::splat(12.65730812710829) * t71 + f64x8::splat(4.125058472512136) * t73 - f64x8::splat(30.42513395716384) * t75;
            let t78 = t67 * t77;
            let t84 = f64x8::splat(0.4771976183772063) - f64x8::splat(1.779981349455627) * t63 + f64x8::splat(3.843384186230215) * t69 - f64x8::splat(9.591205088051849) * t71 + f64x8::splat(2.173018028591672) * t73 - f64x8::splat(30.42513385160366) * t75;
            let t85 = f64x8::splat(1.0) / t84;
            let t87 = (f64x8::splat(5e-13)).simd_lt(t60);
            let t88 = ((t87).select(t60, f64x8::splat(5e-13)));
            let t90 = (simd::ln(f64x8::splat(1.0) / (f64x8::splat(2.085749716493756) * t88) + ((((f64x8::splat(1.0) / (f64x8::splat(2.085749716493756) * t88)) * (f64x8::splat(1.0) / (f64x8::splat(2.085749716493756) * t88))) + f64x8::splat(1.0)).sqrt())));
            let t91 = t90 + f64x8::splat(2.0);
            let t93 = t88 * t88;
            let t95 = t93 * t88;
            let t97 = t93 * t93;
            let t99 = t97 * t88;
            let t101 = f64x8::splat(4.435009886795587e-05) + f64x8::splat(0.5812865360445791) * t88 + f64x8::splat(66.7427645159406) * t93 + f64x8::splat(434.2678089722977) * t95 + f64x8::splat(824.7765766052239) * t97 + f64x8::splat(1657.965273158212) * t99;
            let t102 = t91 * t101;
            let t108 = f64x8::splat(3.347285060926091e-05) + f64x8::splat(0.4791793102397135) * t88 + f64x8::splat(62.39226833857424) * t93 + f64x8::splat(463.1481642793812) * t95 + f64x8::splat(785.2360350104029) * t97 + f64x8::splat(1657.962968223273) * t99;
            let t109 = f64x8::splat(1.0) / t108;
            let t111 = ((t61).select(t78 * t85, t102 * t109));
            let t113 = (simd::exp(t111 / f64x8::splat(3.0)));
            let t114 = t29 * t113;
            let t115 = (simd::exp(-t111));
            let t117 = f64x8::splat(1.0) + t111 / f64x8::splat(2.0);
            let t118 = t115 * t117;
            let t119 = f64x8::splat(1.0) - t118;
            let t120 = f64x8::splat(1.0) / t111;
            let t121 = t119 * t120;
            let t122 = t114 * t121;
            let t125 = ((t2).select(f64x8::splat(0.0), -t28 * t122 / f64x8::splat(4.0)));
            let t126 = (v_rho1).simd_le(dens_threshold);
            let t127 = -t13;
            let t129 = ((t11).select(t8, (t7).select(t12, t127 * t4)));
            let t130 = f64x8::splat(1.0) + t129;
            let t131 = (t130).simd_le(zeta_threshold);
            let t132 = (simd::cbrt(t130));
            let t134 = ((t131).select(t19, t132 * t130));
            let t135 = t134 * t23;
            let t136 = t135 * t27;
            let t137 = (simd::cbrt(v_rho1));
            let t138 = t137 * t137;
            let t140 = f64x8::splat(1.0) / t138 / v_rho1;
            let t141 = v_lapl1 * t140;
            let t143 = v_tau1 * param_gamma;
            let t144 = t143 * t140;
            let t146 = param_gamma * v_sigma2;
            let t147 = v_rho1 * v_rho1;
            let t149 = f64x8::splat(1.0) / t138 / t147;
            let t150 = t146 * t149;
            let t153 = ((t141 / f64x8::splat(2.0) - f64x8::splat(2.0) * t144 + t150 / f64x8::splat(4.0)).abs());
            let t155 = (t153 / f64x8::splat(3.0)).simd_lt(f64x8::splat(5e-13));
            let t159 = t141 / f64x8::splat(6.0) - f64x8::splat(2.0) / f64x8::splat(3.0) * t144 + t150 / f64x8::splat(12.0);
            let t160 = (f64x8::splat(0.0)).simd_lt(t159);
            let t161 = ((t160).select(f64x8::splat(5e-13), -f64x8::splat(5e-13)));
            let t162 = ((t155).select(t161, t159));
            let t165 = f64x8::splat(2.0) / f64x8::splat(3.0) * t31 / t162;
            let t166 = (t165).simd_le(f64x8::splat(0.0));
            let t167 = (-f64x8::splat(5e-13)).simd_lt(t165);
            let t168 = ((t167).select(-f64x8::splat(5e-13), t165));
            let t170 = f64x8::splat(1.525525181200953) * t168 + f64x8::splat(0.4576575543602858);
            let t171 = (simd::atan(t170));
            let t172 = -t171 + f64x8::splat(0.4292036732051034);
            let t174 = t168 * t168;
            let t176 = t174 * t168;
            let t178 = t174 * t174;
            let t180 = t178 * t168;
            let t182 = f64x8::splat(0.7566445420735584) - f64x8::splat(2.636397787137096) * t168 + f64x8::splat(5.474515996423288) * t174 - f64x8::splat(12.65730812710829) * t176 + f64x8::splat(4.125058472512136) * t178 - f64x8::splat(30.42513395716384) * t180;
            let t183 = t172 * t182;
            let t189 = f64x8::splat(0.4771976183772063) - f64x8::splat(1.779981349455627) * t168 + f64x8::splat(3.843384186230215) * t174 - f64x8::splat(9.591205088051849) * t176 + f64x8::splat(2.173018028591672) * t178 - f64x8::splat(30.42513385160366) * t180;
            let t190 = f64x8::splat(1.0) / t189;
            let t192 = (f64x8::splat(5e-13)).simd_lt(t165);
            let t193 = ((t192).select(t165, f64x8::splat(5e-13)));
            let t195 = (simd::ln(f64x8::splat(1.0) / (f64x8::splat(2.085749716493756) * t193) + ((((f64x8::splat(1.0) / (f64x8::splat(2.085749716493756) * t193)) * (f64x8::splat(1.0) / (f64x8::splat(2.085749716493756) * t193))) + f64x8::splat(1.0)).sqrt())));
            let t196 = t195 + f64x8::splat(2.0);
            let t198 = t193 * t193;
            let t200 = t198 * t193;
            let t202 = t198 * t198;
            let t204 = t202 * t193;
            let t206 = f64x8::splat(4.435009886795587e-05) + f64x8::splat(0.5812865360445791) * t193 + f64x8::splat(66.7427645159406) * t198 + f64x8::splat(434.2678089722977) * t200 + f64x8::splat(824.7765766052239) * t202 + f64x8::splat(1657.965273158212) * t204;
            let t207 = t196 * t206;
            let t213 = f64x8::splat(3.347285060926091e-05) + f64x8::splat(0.4791793102397135) * t193 + f64x8::splat(62.39226833857424) * t198 + f64x8::splat(463.1481642793812) * t200 + f64x8::splat(785.2360350104029) * t202 + f64x8::splat(1657.962968223273) * t204;
            let t214 = f64x8::splat(1.0) / t213;
            let t216 = ((t166).select(t183 * t190, t207 * t214));
            let t218 = (simd::exp(t216 / f64x8::splat(3.0)));
            let t219 = t29 * t218;
            let t220 = (simd::exp(-t216));
            let t222 = f64x8::splat(1.0) + t216 / f64x8::splat(2.0);
            let t223 = t220 * t222;
            let t224 = f64x8::splat(1.0) - t223;
            let t225 = f64x8::splat(1.0) / t216;
            let t226 = t224 * t225;
            let t227 = t219 * t226;
            let t230 = ((t126).select(f64x8::splat(0.0), -t136 * t227 / f64x8::splat(4.0)));
            let tzk0 = t125 + t230;
            acc_zk = tzk0;
        }
        store_add(zk, ip, m, acc_zk);
        ip += 8;
    }
}
