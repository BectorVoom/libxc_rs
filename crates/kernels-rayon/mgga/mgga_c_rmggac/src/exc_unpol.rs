//! MGGA_C_RMGGAC exc unpol kernel — explicit SIMD (bit-exact).
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

#[allow(unused_variables, non_snake_case)]
pub fn mgga_c_rmggac_exc_unpol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
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
            let t2 = f64x8::splat(M_CBRT3);
            let t3 = f64x8::splat(1.0) / f64x8::splat(M_PI);
            let t4 = (simd::cbrt(t3));
            let t5 = t2 * t4;
            let t6 = f64x8::splat(M_CBRT4);
            let t7 = t6 * t6;
            let t8 = (simd::cbrt(v_rho));
            let t11 = t5 * t7 / t8;
            let t12 = ((t11).sqrt());
            let t15 = f64x8::splat(1.0) + f64x8::splat(0.04445) * t12 + f64x8::splat(0.03138525) * t11;
            let t16 = f64x8::splat(1.0) / t15;
            let t19 = (simd::exp(f64x8::splat(1.0) * t16));
            let t20 = t19 - f64x8::splat(1.0);
            let t21 = f64x8::splat(M_CBRT6);
            let t22 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t23 = (simd::cbrt(t22));
            let t24 = t23 * t23;
            let t25 = f64x8::splat(1.0) / t24;
            let t26 = t21 * t25;
            let t27 = f64x8::splat(M_CBRT2);
            let t28 = t27 * t27;
            let t29 = t28 * v_sigma;
            let t30 = v_rho * v_rho;
            let t31 = t8 * t8;
            let t33 = f64x8::splat(1.0) / t31 / t30;
            let t35 = t26 * t29 * t33;
            let t37 = f64x8::splat(1.0) + f64x8::splat(0.02133764210437636) * t35;
            let t38 = ((t37).sqrt().sqrt());
            let t40 = f64x8::splat(1.0) - f64x8::splat(1.0) / t38;
            let t42 = t20 * t40 + f64x8::splat(1.0);
            let t43 = (simd::ln(t42));
            let t46 = t27 - f64x8::splat(1.0);
            let t47 = (f64x8::splat(1.0)).simd_le(zeta_threshold);
            let t48 = (simd::cbrt(zeta_threshold));
            let t50 = ((t47).select(t48 * zeta_threshold, f64x8::splat(1.0)));
            let t52 = f64x8::splat(2.0) * t50 - f64x8::splat(2.0);
            let t55 = f64x8::splat(1.0) / t46 / f64x8::splat(2.0);
            let t58 = f64x8::splat(1.0) - f64x8::splat(2.363) * t46 * t52 * t55;
            let t59 = (-f64x8::splat(0.0285764) * t16 + f64x8::splat(0.0285764) * t43) * t58;
            let t61 = f64x8::splat(1.0) / t31 / v_rho;
            let t66 = f64x8::splat(2.0) * v_tau * t61 - v_sigma * t33 / f64x8::splat(4.0);
            let t67 = t66 * t66;
            let t68 = t67 * t66;
            let t73 = f64x8::splat(0.08) + f64x8::splat(5.0) / f64x8::splat(18.0) * t66 * t28 * t26 + f64x8::splat(0.0125) * t35;
            let t74 = t73 * t73;
            let t75 = t74 * t73;
            let t76 = f64x8::splat(1.0) / t75;
            let t77 = t68 * t76;
            let t79 = t67 * t67;
            let t80 = t79 * t67;
            let t81 = t74 * t74;
            let t83 = f64x8::splat(1.0) / t81 / t74;
            let t86 = f64x8::splat(1.0) + f64x8::splat(0.006652356501035449) * t77 + f64x8::splat(4.42538470168686e-05) * t80 * t83;
            let t87 = f64x8::splat(1.0) / t86;
            let t88 = t77 * t87;
            let t90 = f64x8::splat(1.0) - f64x8::splat(0.01995706950310635) * t88;
            let t91 = t59 * t90;
            let t93 = f64x8::splat(1.0) + f64x8::splat(0.053425) * t11;
            let t96 = ((t11) * (t11).sqrt());
            let t98 = t2 * t2;
            let t99 = t4 * t4;
            let t100 = t98 * t99;
            let t103 = t100 * t6 / t31;
            let t105 = f64x8::splat(3.79785) * t12 + f64x8::splat(0.8969) * t11 + f64x8::splat(0.204775) * t96 + f64x8::splat(0.123235) * t103;
            let t108 = f64x8::splat(1.0) + f64x8::splat(16.081979498692537) / t105;
            let t109 = (simd::ln(t108));
            let t111 = f64x8::splat(0.0621814) * t93 * t109;
            let t112 = t52 * t55;
            let t114 = f64x8::splat(1.0) + f64x8::splat(0.0278125) * t11;
            let t119 = f64x8::splat(5.1785) * t12 + f64x8::splat(0.905775) * t11 + f64x8::splat(0.1100325) * t96 + f64x8::splat(0.1241775) * t103;
            let t122 = f64x8::splat(1.0) + f64x8::splat(29.608749977793437) / t119;
            let t123 = (simd::ln(t122));
            let t126 = f64x8::splat(0.0197516734986138) * t112 * t114 * t123;
            let t127 = t48 * t48;
            let t128 = ((t47).select(t127, f64x8::splat(1.0)));
            let t129 = t128 * t128;
            let t130 = t129 * t128;
            let t131 = -t111 + t126;
            let t132 = f64x8::splat(1.0) / t130;
            let t135 = (simd::exp(-f64x8::splat(32.16364864430221) * t131 * t132));
            let t136 = t135 - f64x8::splat(1.0);
            let t137 = (simd::ln(f64x8::splat(2.0)));
            let t138 = f64x8::splat(1.0) - t137;
            let t139 = f64x8::splat(1.0) / t138;
            let t143 = (simd::exp(-t131 * t139 * t22 * t132));
            let t144 = t143 - f64x8::splat(1.0);
            let t145 = f64x8::splat(1.0) / t144;
            let t146 = t139 * t145;
            let t148 = f64x8::splat(1.0) / t8 / t30;
            let t151 = f64x8::splat(1.0) / t129;
            let t153 = f64x8::splat(1.0) / t4;
            let t154 = t98 * t153;
            let t155 = t154 * t6;
            let t156 = t27 * t151 * t155;
            let t159 = f64x8::splat(1.0) + f64x8::splat(0.02743955640261198) * t146 * v_sigma * t148 * t156;
            let t160 = ((t159).sqrt().sqrt());
            let t162 = f64x8::splat(1.0) - f64x8::splat(1.0) / t160;
            let t164 = t136 * t162 + f64x8::splat(1.0);
            let t165 = (simd::ln(t164));
            let t168 = -t111 + t126 + f64x8::splat(0.031091) * t130 * t165;
            let t169 = t168 * t68;
            let t170 = t76 * t87;
            let t172 = f64x8::splat(0.01995706950310635) * t169 * t170;
            let tzk0 = t91 + t172;
            acc_zk = tzk0;
        }
        store_add(zk, ip, m, acc_zk);
        ip += 8;
    }
}
