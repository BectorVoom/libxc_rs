//! MGGA_X_BR89_EXPLICIT exc unpol kernel — explicit SIMD (bit-exact).
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

#[allow(unused_variables, non_snake_case)]
pub fn mgga_x_br89_explicit_exc_unpol(
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
        let v_rho = load(rho, ip, np);
        let v_sigma = load(sigma, ip, np);
        let v_lapl = load(lapl, ip, np);
        let v_tau = load(tau, ip, np);
        let mut acc_zk = V_ZERO;
        {
            let t3 = (v_rho / f64x8::splat(2.0)).simd_le(dens_threshold);
            let t4 = (f64x8::splat(1.0)).simd_le(zeta_threshold);
            let t5 = zeta_threshold - f64x8::splat(1.0);
            let t7 = ((t4).select(t5, (t4).select(-t5, f64x8::splat(0.0))));
            let t8 = f64x8::splat(1.0) + t7;
            let t10 = (simd::cbrt(zeta_threshold));
            let t12 = (simd::cbrt(t8));
            let t14 = (((t8).simd_le(zeta_threshold)).select(t10 * zeta_threshold, t12 * t8));
            let t15 = (simd::cbrt(v_rho));
            let t16 = t14 * t15;
            let t18 = (simd::cbrt(f64x8::splat(1.0) / f64x8::splat(M_PI)));
            let t19 = f64x8::splat(1.0) / t18;
            let t20 = t16 * t19;
            let t21 = f64x8::splat(M_CBRT4);
            let t22 = f64x8::splat(M_CBRTPI);
            let t23 = t22 * t22;
            let t24 = f64x8::splat(M_CBRT2);
            let t25 = t24 * t24;
            let t26 = t15 * t15;
            let t28 = f64x8::splat(1.0) / t26 / v_rho;
            let t31 = param_gamma * v_tau;
            let t34 = param_gamma * v_sigma;
            let t35 = v_rho * v_rho;
            let t37 = f64x8::splat(1.0) / t26 / t35;
            let t41 = ((v_lapl * t28 / f64x8::splat(2.0) - f64x8::splat(2.0) * t31 * t28 + t34 * t37 / f64x8::splat(4.0)).abs());
            let t44 = (t25 * t41 / f64x8::splat(3.0)).simd_lt(f64x8::splat(5e-13));
            let t45 = v_lapl * t25;
            let t48 = t25 * t28;
            let t51 = t25 * t37;
            let t54 = t45 * t28 / f64x8::splat(6.0) - f64x8::splat(2.0) / f64x8::splat(3.0) * t31 * t48 + t34 * t51 / f64x8::splat(12.0);
            let t55 = (f64x8::splat(0.0)).simd_lt(t54);
            let t56 = ((t55).select(f64x8::splat(5e-13), -f64x8::splat(5e-13)));
            let t57 = ((t44).select(t56, t54));
            let t60 = f64x8::splat(2.0) / f64x8::splat(3.0) * t23 / t57;
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
            let t114 = t21 * t113;
            let t115 = (simd::exp(-t111));
            let t117 = f64x8::splat(1.0) + t111 / f64x8::splat(2.0);
            let t118 = t115 * t117;
            let t119 = f64x8::splat(1.0) - t118;
            let t120 = f64x8::splat(1.0) / t111;
            let t121 = t119 * t120;
            let t122 = t114 * t121;
            let t125 = ((t3).select(f64x8::splat(0.0), -t20 * t122 / f64x8::splat(4.0)));
            let tzk0 = f64x8::splat(2.0) * t125;
            acc_zk = tzk0;
        }
        store_add(zk, ip, m, acc_zk);
        ip += 8;
    }
}
