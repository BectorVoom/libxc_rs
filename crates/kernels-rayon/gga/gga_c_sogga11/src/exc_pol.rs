//! GGA_C_SOGGA11 exc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_sogga11.c`
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
pub fn gga_c_sogga11_exc_pol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    param_sogga11_a_1: f64,
    param_sogga11_a_2: f64,
    param_sogga11_a_3: f64,
    param_sogga11_a_4: f64,
    param_sogga11_a_5: f64,
    param_sogga11_b_1: f64,
    param_sogga11_b_2: f64,
    param_sogga11_b_3: f64,
    param_sogga11_b_4: f64,
    param_sogga11_b_5: f64,
    param_sogga11_a_0: f64,
    param_sogga11_b_0: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_sogga11_a_1 = f64x8::splat(param_sogga11_a_1);
    let param_sogga11_a_2 = f64x8::splat(param_sogga11_a_2);
    let param_sogga11_a_3 = f64x8::splat(param_sogga11_a_3);
    let param_sogga11_a_4 = f64x8::splat(param_sogga11_a_4);
    let param_sogga11_a_5 = f64x8::splat(param_sogga11_a_5);
    let param_sogga11_b_1 = f64x8::splat(param_sogga11_b_1);
    let param_sogga11_b_2 = f64x8::splat(param_sogga11_b_2);
    let param_sogga11_b_3 = f64x8::splat(param_sogga11_b_3);
    let param_sogga11_b_4 = f64x8::splat(param_sogga11_b_4);
    let param_sogga11_b_5 = f64x8::splat(param_sogga11_b_5);
    let param_sogga11_a_0 = f64x8::splat(param_sogga11_a_0);
    let param_sogga11_b_0 = f64x8::splat(param_sogga11_b_0);
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
        {
            let t1 = f64x8::splat(M_CBRT3);
            let t2 = f64x8::splat(1.0) / f64x8::splat(M_PI);
            let t3 = (simd::cbrt(t2));
            let t4 = t1 * t3;
            let t5 = f64x8::splat(M_CBRT4);
            let t6 = t5 * t5;
            let t7 = v_rho0 + v_rho1;
            let t8 = (simd::cbrt(t7));
            let t11 = t4 * t6 / t8;
            let t13 = f64x8::splat(1.0) + f64x8::splat(0.053425) * t11;
            let t14 = ((t11).sqrt());
            let t17 = ((t11) * (t11).sqrt());
            let t19 = t1 * t1;
            let t20 = t3 * t3;
            let t21 = t19 * t20;
            let t22 = t8 * t8;
            let t25 = t21 * t5 / t22;
            let t27 = f64x8::splat(3.79785) * t14 + f64x8::splat(0.8969) * t11 + f64x8::splat(0.204775) * t17 + f64x8::splat(0.123235) * t25;
            let t30 = f64x8::splat(1.0) + f64x8::splat(16.081979498692537) / t27;
            let t31 = (simd::ln(t30));
            let t33 = f64x8::splat(0.0621814) * t13 * t31;
            let t34 = v_rho0 - v_rho1;
            let t35 = t34 * t34;
            let t36 = t35 * t35;
            let t37 = t7 * t7;
            let t38 = t37 * t37;
            let t39 = f64x8::splat(1.0) / t38;
            let t40 = t36 * t39;
            let t41 = f64x8::splat(1.0) / t7;
            let t42 = t34 * t41;
            let t43 = f64x8::splat(1.0) + t42;
            let t44 = (t43).simd_le(zeta_threshold);
            let t45 = (simd::cbrt(zeta_threshold));
            let t46 = t45 * zeta_threshold;
            let t47 = (simd::cbrt(t43));
            let t48 = t47 * t43;
            let t49 = ((t44).select(t46, t48));
            let t50 = f64x8::splat(1.0) - t42;
            let t51 = (t50).simd_le(zeta_threshold);
            let t52 = (simd::cbrt(t50));
            let t53 = t52 * t50;
            let t54 = ((t51).select(t46, t53));
            let t55 = t49 + t54 - f64x8::splat(2.0);
            let t56 = f64x8::splat(M_CBRT2);
            let t59 = f64x8::splat(1.0) / (f64x8::splat(2.0) * t56 - f64x8::splat(2.0));
            let t60 = t55 * t59;
            let t62 = f64x8::splat(1.0) + f64x8::splat(0.05137) * t11;
            let t67 = f64x8::splat(7.05945) * t14 + f64x8::splat(1.549425) * t11 + f64x8::splat(0.420775) * t17 + f64x8::splat(0.1562925) * t25;
            let t70 = f64x8::splat(1.0) + f64x8::splat(32.16395899738507) / t67;
            let t71 = (simd::ln(t70));
            let t75 = f64x8::splat(1.0) + f64x8::splat(0.0278125) * t11;
            let t80 = f64x8::splat(5.1785) * t14 + f64x8::splat(0.905775) * t11 + f64x8::splat(0.1100325) * t17 + f64x8::splat(0.1241775) * t25;
            let t83 = f64x8::splat(1.0) + f64x8::splat(29.608749977793437) / t80;
            let t84 = (simd::ln(t83));
            let t85 = t75 * t84;
            let t87 = -f64x8::splat(0.0310907) * t62 * t71 + t33 - f64x8::splat(0.0197516734986138) * t85;
            let t88 = t60 * t87;
            let t92 = -t33 + t40 * t88 + f64x8::splat(0.0197516734986138) * t60 * t85;
            let t94 = param_sogga11_a_1;
            let t95 = t45 * t45;
            let t96 = t47 * t47;
            let t97 = ((t44).select(t95, t96));
            let t98 = t52 * t52;
            let t99 = ((t51).select(t95, t98));
            let t101 = t97 / f64x8::splat(2.0) + t99 / f64x8::splat(2.0);
            let t102 = t56 * t101;
            let t104 = v_sigma0 + f64x8::splat(2.0) * v_sigma1 + v_sigma2;
            let t106 = f64x8::splat(1.0) / t8 / t37;
            let t107 = t104 * t106;
            let t108 = t102 * t107;
            let t109 = f64x8::splat(1.0) / t3;
            let t110 = t19 * t109;
            let t111 = f64x8::splat(1.0) / t92;
            let t112 = t5 * t111;
            let t113 = t110 * t112;
            let t115 = f64x8::splat(0.0006950658458333333) * t108 * t113;
            let t116 = f64x8::splat(1.0) - t115;
            let t118 = f64x8::splat(1.0) - f64x8::splat(1.0) / t116;
            let t120 = param_sogga11_a_2;
            let t121 = t118 * t118;
            let t123 = param_sogga11_a_3;
            let t124 = t121 * t118;
            let t126 = param_sogga11_a_4;
            let t127 = t121 * t121;
            let t129 = param_sogga11_a_5;
            let t133 = param_sogga11_b_1;
            let t134 = (simd::exp(t115));
            let t135 = f64x8::splat(1.0) - t134;
            let t137 = param_sogga11_b_2;
            let t138 = t135 * t135;
            let t140 = param_sogga11_b_3;
            let t141 = t138 * t135;
            let t143 = param_sogga11_b_4;
            let t144 = t138 * t138;
            let t146 = param_sogga11_b_5;
            let t149 = t129 * t127 * t118 + t146 * t144 * t135 + t94 * t118 + t120 * t121 + t123 * t124 + t126 * t127 + t133 * t135 + t137 * t138 + t140 * t141 + t143 * t144 + param_sogga11_a_0 + param_sogga11_b_0;
            let tzk0 = t92 * t149;
            acc_zk = tzk0;
        }
        store_add(zk, ip, m, acc_zk);
        ip += 8;
    }
}
