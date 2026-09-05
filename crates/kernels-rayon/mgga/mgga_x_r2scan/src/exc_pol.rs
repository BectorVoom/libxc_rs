//! MGGA_X_R2SCAN exc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_r2scan.c`
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
pub fn mgga_x_r2scan_exc_pol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    param_c1: f64,
    param_c2: f64,
    param_d: f64,
    param_dp2: f64,
    param_eta: f64,
    param_k1: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_c1 = f64x8::splat(param_c1);
    let param_c2 = f64x8::splat(param_c2);
    let param_d = f64x8::splat(param_d);
    let param_dp2 = f64x8::splat(param_dp2);
    let param_eta = f64x8::splat(param_eta);
    let param_k1 = f64x8::splat(param_k1);
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
            let t27 = t6 * t26;
            let t28 = (simd::cbrt(t7));
            let t30 = f64x8::splat(20.0) / f64x8::splat(27.0) + f64x8::splat(5.0) / f64x8::splat(3.0) * param_eta;
            let t31 = f64x8::splat(M_CBRT6);
            let t32 = t31 * t31;
            let t33 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t34 = (simd::cbrt(t33));
            let t35 = t34 * t33;
            let t36 = f64x8::splat(1.0) / t35;
            let t37 = t32 * t36;
            let t38 = v_sigma0 * v_sigma0;
            let t39 = v_rho0 * v_rho0;
            let t40 = t39 * t39;
            let t41 = t40 * v_rho0;
            let t42 = (simd::cbrt(v_rho0));
            let t44 = f64x8::splat(1.0) / t42 / t41;
            let t45 = t38 * t44;
            let t46 = param_dp2 * param_dp2;
            let t47 = t46 * t46;
            let t48 = f64x8::splat(1.0) / t47;
            let t52 = (simd::exp(-t37 * t45 * t48 / f64x8::splat(576.0)));
            let t56 = (-f64x8::splat(0.162742215233874) * t30 * t52 + f64x8::splat(10.0) / f64x8::splat(81.0)) * t31;
            let t57 = t34 * t34;
            let t58 = f64x8::splat(1.0) / t57;
            let t59 = t58 * v_sigma0;
            let t60 = t42 * t42;
            let t61 = t60 * t39;
            let t62 = f64x8::splat(1.0) / t61;
            let t66 = param_k1 + t56 * t59 * t62 / f64x8::splat(24.0);
            let t70 = param_k1 * (f64x8::splat(1.0) - param_k1 / t66);
            let t71 = t60 * v_rho0;
            let t72 = f64x8::splat(1.0) / t71;
            let t74 = v_sigma0 * t62;
            let t76 = v_tau0 * t72 - t74 / f64x8::splat(8.0);
            let t78 = f64x8::splat(3.0) / f64x8::splat(10.0) * t32 * t57;
            let t79 = param_eta * v_sigma0;
            let t82 = t78 + t79 * t62 / f64x8::splat(8.0);
            let t83 = f64x8::splat(1.0) / t82;
            let t84 = t76 * t83;
            let t85 = (t84).simd_le(f64x8::splat(0.0));
            let t86 = (f64x8::splat(0.0)).simd_lt(t84);
            let t87 = ((t86).select(f64x8::splat(0.0), t84));
            let t88 = param_c1 * t87;
            let t89 = f64x8::splat(1.0) - t87;
            let t90 = f64x8::splat(1.0) / t89;
            let t92 = (simd::exp(-t88 * t90));
            let t93 = (t84).simd_le(f64x8::splat(2.5));
            let t94 = (f64x8::splat(2.5)).simd_lt(t84);
            let t95 = ((t94).select(f64x8::splat(2.5), t84));
            let t97 = t95 * t95;
            let t99 = t97 * t95;
            let t101 = t97 * t97;
            let t103 = t101 * t95;
            let t105 = t101 * t97;
            let t110 = ((t94).select(t84, f64x8::splat(2.5)));
            let t111 = f64x8::splat(1.0) - t110;
            let t114 = (simd::exp(param_c2 / t111));
            let t116 = ((t85).select(t92, (t93).select(f64x8::splat(1.0) - f64x8::splat(0.667) * t95 - f64x8::splat(0.4445555) * t97 - f64x8::splat(0.663086601049) * t99 + f64x8::splat(1.45129704449) * t101 - f64x8::splat(0.887998041597) * t103 + f64x8::splat(0.234528941479) * t105 - f64x8::splat(0.023185843322) * t101 * t99, -param_d * t114)));
            let t117 = f64x8::splat(0.174) - t70;
            let t119 = t116 * t117 + t70 + f64x8::splat(1.0);
            let t120 = t28 * t119;
            let t121 = ((f64x8::splat(3.0)).sqrt());
            let t122 = f64x8::splat(1.0) / t34;
            let t123 = t32 * t122;
            let t124 = ((v_sigma0).sqrt());
            let t125 = t42 * v_rho0;
            let t126 = f64x8::splat(1.0) / t125;
            let t128 = t123 * t124 * t126;
            let t129 = ((t128).sqrt());
            let t133 = (simd::exp(-f64x8::splat(9.8958) * t121 / t129));
            let t134 = f64x8::splat(1.0) - t133;
            let t135 = t120 * t134;
            let t138 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t27 * t135));
            let t139 = (v_rho1).simd_le(dens_threshold);
            let t140 = -t17;
            let t142 = ((t15).select(t12, (t11).select(t16, t140 * t8)));
            let t143 = f64x8::splat(1.0) + t142;
            let t144 = (t143).simd_le(zeta_threshold);
            let t145 = (simd::cbrt(t143));
            let t147 = ((t144).select(t23, t145 * t143));
            let t148 = t6 * t147;
            let t149 = v_sigma2 * v_sigma2;
            let t150 = v_rho1 * v_rho1;
            let t151 = t150 * t150;
            let t152 = t151 * v_rho1;
            let t153 = (simd::cbrt(v_rho1));
            let t155 = f64x8::splat(1.0) / t153 / t152;
            let t156 = t149 * t155;
            let t160 = (simd::exp(-t37 * t156 * t48 / f64x8::splat(576.0)));
            let t164 = (-f64x8::splat(0.162742215233874) * t30 * t160 + f64x8::splat(10.0) / f64x8::splat(81.0)) * t31;
            let t165 = t58 * v_sigma2;
            let t166 = t153 * t153;
            let t167 = t166 * t150;
            let t168 = f64x8::splat(1.0) / t167;
            let t172 = param_k1 + t164 * t165 * t168 / f64x8::splat(24.0);
            let t176 = param_k1 * (f64x8::splat(1.0) - param_k1 / t172);
            let t177 = t166 * v_rho1;
            let t178 = f64x8::splat(1.0) / t177;
            let t180 = v_sigma2 * t168;
            let t182 = v_tau1 * t178 - t180 / f64x8::splat(8.0);
            let t183 = param_eta * v_sigma2;
            let t186 = t78 + t183 * t168 / f64x8::splat(8.0);
            let t187 = f64x8::splat(1.0) / t186;
            let t188 = t182 * t187;
            let t189 = (t188).simd_le(f64x8::splat(0.0));
            let t190 = (f64x8::splat(0.0)).simd_lt(t188);
            let t191 = ((t190).select(f64x8::splat(0.0), t188));
            let t192 = param_c1 * t191;
            let t193 = f64x8::splat(1.0) - t191;
            let t194 = f64x8::splat(1.0) / t193;
            let t196 = (simd::exp(-t192 * t194));
            let t197 = (t188).simd_le(f64x8::splat(2.5));
            let t198 = (f64x8::splat(2.5)).simd_lt(t188);
            let t199 = ((t198).select(f64x8::splat(2.5), t188));
            let t201 = t199 * t199;
            let t203 = t201 * t199;
            let t205 = t201 * t201;
            let t207 = t205 * t199;
            let t209 = t205 * t201;
            let t214 = ((t198).select(t188, f64x8::splat(2.5)));
            let t215 = f64x8::splat(1.0) - t214;
            let t218 = (simd::exp(param_c2 / t215));
            let t220 = ((t189).select(t196, (t197).select(f64x8::splat(1.0) - f64x8::splat(0.667) * t199 - f64x8::splat(0.4445555) * t201 - f64x8::splat(0.663086601049) * t203 + f64x8::splat(1.45129704449) * t205 - f64x8::splat(0.887998041597) * t207 + f64x8::splat(0.234528941479) * t209 - f64x8::splat(0.023185843322) * t205 * t203, -param_d * t218)));
            let t221 = f64x8::splat(0.174) - t176;
            let t223 = t220 * t221 + t176 + f64x8::splat(1.0);
            let t224 = t28 * t223;
            let t225 = ((v_sigma2).sqrt());
            let t226 = t153 * v_rho1;
            let t227 = f64x8::splat(1.0) / t226;
            let t229 = t123 * t225 * t227;
            let t230 = ((t229).sqrt());
            let t234 = (simd::exp(-f64x8::splat(9.8958) * t121 / t230));
            let t235 = f64x8::splat(1.0) - t234;
            let t236 = t224 * t235;
            let t239 = ((t139).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t148 * t236));
            let tzk0 = t138 + t239;
            acc_zk = tzk0;
        }
        store_add(zk, ip, m, acc_zk);
        ip += 8;
    }
}
