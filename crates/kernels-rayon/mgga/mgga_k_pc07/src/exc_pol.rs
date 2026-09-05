//! MGGA_K_PC07 exc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_k_pc07.c`
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
pub fn mgga_k_pc07_exc_pol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    param_a: f64,
    param_b: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_a = f64x8::splat(param_a);
    let param_b = f64x8::splat(param_b);
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
            let t46 = f64x8::splat(5.0) / f64x8::splat(72.0) * t45;
            let t49 = f64x8::splat(1.0) / t41 / v_rho0;
            let t53 = t33 * t33;
            let t55 = f64x8::splat(1.0) / t35 / t34;
            let t56 = t53 * t55;
            let t57 = v_lapl0 * v_lapl0;
            let t58 = t39 * v_rho0;
            let t60 = f64x8::splat(1.0) / t40 / t58;
            let t63 = t56 * t57 * t60 / f64x8::splat(5832.0);
            let t64 = t39 * t39;
            let t66 = f64x8::splat(1.0) / t40 / t64;
            let t67 = v_sigma0 * t66;
            let t70 = t56 * t67 * v_lapl0 / f64x8::splat(5184.0);
            let t71 = v_sigma0 * v_sigma0;
            let t72 = t64 * v_rho0;
            let t74 = f64x8::splat(1.0) / t40 / t72;
            let t77 = t56 * t71 * t74 / f64x8::splat(17496.0);
            let t78 = f64x8::splat(1.0) + f64x8::splat(5.0) / f64x8::splat(648.0) * t45 + f64x8::splat(5.0) / f64x8::splat(54.0) * t38 * v_lapl0 * t49 + t63 - t70 + t77;
            let t79 = t63 - t70 + t77;
            let t80 = t79 * t79;
            let t81 = f64x8::splat(1.0) + t46;
            let t82 = t81 * t81;
            let t83 = f64x8::splat(1.0) / t82;
            let t85 = t80 * t83 + f64x8::splat(1.0);
            let t86 = ((t85).sqrt());
            let t87 = f64x8::splat(1.0) / t86;
            let t89 = t78 * t87 - t46;
            let t90 = param_a / f64x8::splat(40.0);
            let t91 = (t89).simd_le(t90);
            let t92 = f64x8::splat(39.0) / f64x8::splat(40.0) * param_a;
            let t93 = (t92).simd_le(t89);
            let t94 = param_a * param_b;
            let t95 = (t89).simd_lt(t90);
            let t96 = ((t95).select(t90, t89));
            let t97 = (t96).simd_lt(t92);
            let t98 = ((t97).select(t96, t92));
            let t99 = f64x8::splat(1.0) / t98;
            let t101 = (simd::exp(-t94 * t99));
            let t102 = param_a - t98;
            let t105 = (simd::exp(-param_a / t102));
            let t106 = f64x8::splat(1.0) + t105;
            let t107 = (simd::pow(t106, param_b));
            let t108 = t101 * t107;
            let t110 = (simd::exp(-param_a * t99));
            let t111 = t110 + t105;
            let t112 = (simd::pow(t111, param_b));
            let t113 = f64x8::splat(1.0) / t112;
            let t114 = t108 * t113;
            let t115 = ((t91).select(f64x8::splat(0.0), (t93).select(f64x8::splat(1.0), t114)));
            let t117 = t115 * t89 + t46;
            let t121 = ((t2).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t7 * t32 * t117));
            let t122 = (v_rho1).simd_le(dens_threshold);
            let t123 = -t18;
            let t125 = ((t16).select(t13, (t12).select(t17, t123 * t9)));
            let t126 = f64x8::splat(1.0) + t125;
            let t127 = (t126).simd_le(zeta_threshold);
            let t128 = (simd::cbrt(t126));
            let t129 = t128 * t128;
            let t131 = ((t127).select(t25, t129 * t126));
            let t132 = t131 * t31;
            let t133 = v_rho1 * v_rho1;
            let t134 = (simd::cbrt(v_rho1));
            let t135 = t134 * t134;
            let t137 = f64x8::splat(1.0) / t135 / t133;
            let t139 = t38 * v_sigma2 * t137;
            let t140 = f64x8::splat(5.0) / f64x8::splat(72.0) * t139;
            let t143 = f64x8::splat(1.0) / t135 / v_rho1;
            let t147 = v_lapl1 * v_lapl1;
            let t148 = t133 * v_rho1;
            let t150 = f64x8::splat(1.0) / t134 / t148;
            let t153 = t56 * t147 * t150 / f64x8::splat(5832.0);
            let t154 = t133 * t133;
            let t156 = f64x8::splat(1.0) / t134 / t154;
            let t157 = v_sigma2 * t156;
            let t160 = t56 * t157 * v_lapl1 / f64x8::splat(5184.0);
            let t161 = v_sigma2 * v_sigma2;
            let t162 = t154 * v_rho1;
            let t164 = f64x8::splat(1.0) / t134 / t162;
            let t167 = t56 * t161 * t164 / f64x8::splat(17496.0);
            let t168 = f64x8::splat(1.0) + f64x8::splat(5.0) / f64x8::splat(648.0) * t139 + f64x8::splat(5.0) / f64x8::splat(54.0) * t38 * v_lapl1 * t143 + t153 - t160 + t167;
            let t169 = t153 - t160 + t167;
            let t170 = t169 * t169;
            let t171 = f64x8::splat(1.0) + t140;
            let t172 = t171 * t171;
            let t173 = f64x8::splat(1.0) / t172;
            let t175 = t170 * t173 + f64x8::splat(1.0);
            let t176 = ((t175).sqrt());
            let t177 = f64x8::splat(1.0) / t176;
            let t179 = t168 * t177 - t140;
            let t180 = (t179).simd_le(t90);
            let t181 = (t92).simd_le(t179);
            let t182 = (t179).simd_lt(t90);
            let t183 = ((t182).select(t90, t179));
            let t184 = (t183).simd_lt(t92);
            let t185 = ((t184).select(t183, t92));
            let t186 = f64x8::splat(1.0) / t185;
            let t188 = (simd::exp(-t94 * t186));
            let t189 = param_a - t185;
            let t192 = (simd::exp(-param_a / t189));
            let t193 = f64x8::splat(1.0) + t192;
            let t194 = (simd::pow(t193, param_b));
            let t195 = t188 * t194;
            let t197 = (simd::exp(-param_a * t186));
            let t198 = t197 + t192;
            let t199 = (simd::pow(t198, param_b));
            let t200 = f64x8::splat(1.0) / t199;
            let t201 = t195 * t200;
            let t202 = ((t180).select(f64x8::splat(0.0), (t181).select(f64x8::splat(1.0), t201)));
            let t204 = t179 * t202 + t140;
            let t208 = ((t122).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t7 * t132 * t204));
            let tzk0 = t121 + t208;
            acc_zk = tzk0;
        }
        store_add(zk, ip, m, acc_zk);
        ip += 8;
    }
}
