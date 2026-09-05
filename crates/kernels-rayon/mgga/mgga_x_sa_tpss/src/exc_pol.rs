//! MGGA_X_SA_TPSS exc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_sa_tpss.c`
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
pub fn mgga_x_sa_tpss_exc_pol(
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
            let t29 = ((f64x8::splat(5.0)).sqrt());
            let t30 = f64x8::splat(M_PI) * t29;
            let t31 = (simd::cbrt(v_rho0));
            let t32 = t31 * t31;
            let t34 = f64x8::splat(1.0) / t32 / v_rho0;
            let t36 = v_rho0 * v_rho0;
            let t38 = f64x8::splat(1.0) / t32 / t36;
            let t39 = v_sigma0 * t38;
            let t41 = v_tau0 * t34 - t39 / f64x8::splat(8.0);
            let t42 = f64x8::splat(M_CBRT6);
            let t43 = t41 * t42;
            let t44 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t45 = (simd::cbrt(t44));
            let t46 = t45 * t45;
            let t47 = f64x8::splat(1.0) / t46;
            let t48 = t43 * t47;
            let t50 = f64x8::splat(5.0) * t48 + f64x8::splat(9.0);
            let t51 = ((t50).sqrt());
            let t52 = f64x8::splat(5.0) / f64x8::splat(9.0) * t48;
            let t53 = t52 + f64x8::splat(0.348);
            let t54 = (simd::ln(t53));
            let t55 = f64x8::splat(2.413) + t54;
            let t56 = ((t55).sqrt());
            let t57 = f64x8::splat(1.0) / t56;
            let t58 = t51 * t57;
            let t59 = t30 * t58;
            let t61 = v_sigma0 * v_sigma0;
            let t62 = f64x8::splat(1.0) / t36;
            let t63 = t61 * t62;
            let t64 = v_tau0 * v_tau0;
            let t65 = f64x8::splat(1.0) / t64;
            let t66 = t63 * t65;
            let t68 = f64x8::splat(1.0) + t66 / f64x8::splat(64.0);
            let t69 = t68 * t68;
            let t70 = f64x8::splat(1.0) / t69;
            let t71 = t65 * t70;
            let t75 = (f64x8::splat(10.0) / f64x8::splat(81.0) + f64x8::splat(0.02485875) * t63 * t71) * t42;
            let t76 = t47 * v_sigma0;
            let t77 = t76 * t38;
            let t80 = t52 - f64x8::splat(1.0);
            let t81 = t47 * t80;
            let t84 = f64x8::splat(1.0) + f64x8::splat(0.2222222222222222) * t43 * t81;
            let t85 = ((t84).sqrt());
            let t86 = f64x8::splat(1.0) / t85;
            let t89 = t42 * t47;
            let t90 = t89 * t39;
            let t92 = f64x8::splat(9.0) / f64x8::splat(20.0) * t80 * t86 + t90 / f64x8::splat(36.0);
            let t93 = t92 * t92;
            let t96 = t42 * t42;
            let t98 = f64x8::splat(1.0) / t45 / t44;
            let t99 = t96 * t98;
            let t100 = t36 * t36;
            let t101 = t100 * v_rho0;
            let t103 = f64x8::splat(1.0) / t31 / t101;
            let t104 = t61 * t103;
            let t105 = t99 * t104;
            let t107 = f64x8::splat(162.0) * t66 + f64x8::splat(50.0) * t105;
            let t108 = ((t107).sqrt());
            let t112 = f64x8::splat(1.0) / f64x8::splat(M_PI) * t29;
            let t113 = f64x8::splat(1.0) / t51;
            let t115 = t112 * t113 * t56;
            let t119 = t61 * v_sigma0;
            let t120 = t100 * t100;
            let t121 = f64x8::splat(1.0) / t120;
            let t124 = t75 * t77 / f64x8::splat(24.0) + f64x8::splat(146.0) / f64x8::splat(2025.0) * t93 - f64x8::splat(73.0) / f64x8::splat(97200.0) * t92 * t108 + f64x8::splat(25.0) / f64x8::splat(209952.0) * t115 * t105 + f64x8::splat(0.0017218861679299947) * t66 + f64x8::splat(1.5033019185692233e-06) * t119 * t121;
            let t126 = f64x8::splat(1.0) + f64x8::splat(0.05165658503789984) * t90;
            let t127 = t126 * t126;
            let t128 = f64x8::splat(1.0) / t127;
            let t130 = f64x8::splat(2.0) / f64x8::splat(45.0) * t59 + t124 * t128;
            let t131 = f64x8::splat(1.0) / t130;
            let t135 = f64x8::splat(1.0) - f64x8::splat(2.0) / f64x8::splat(45.0) * t30 * t58 * t131;
            let t139 = f64x8::splat(1.0) + f64x8::splat(2.0) / f64x8::splat(45.0) * t30 * t58 * t135;
            let t143 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t28 * t139));
            let t144 = (v_rho1).simd_le(dens_threshold);
            let t145 = -t17;
            let t147 = ((t15).select(t12, (t11).select(t16, t145 * t8)));
            let t148 = f64x8::splat(1.0) + t147;
            let t149 = (t148).simd_le(zeta_threshold);
            let t150 = (simd::cbrt(t148));
            let t152 = ((t149).select(t23, t150 * t148));
            let t153 = t152 * t27;
            let t154 = (simd::cbrt(v_rho1));
            let t155 = t154 * t154;
            let t157 = f64x8::splat(1.0) / t155 / v_rho1;
            let t159 = v_rho1 * v_rho1;
            let t161 = f64x8::splat(1.0) / t155 / t159;
            let t162 = v_sigma2 * t161;
            let t164 = v_tau1 * t157 - t162 / f64x8::splat(8.0);
            let t165 = t164 * t42;
            let t166 = t165 * t47;
            let t168 = f64x8::splat(5.0) * t166 + f64x8::splat(9.0);
            let t169 = ((t168).sqrt());
            let t170 = f64x8::splat(5.0) / f64x8::splat(9.0) * t166;
            let t171 = t170 + f64x8::splat(0.348);
            let t172 = (simd::ln(t171));
            let t173 = f64x8::splat(2.413) + t172;
            let t174 = ((t173).sqrt());
            let t175 = f64x8::splat(1.0) / t174;
            let t176 = t169 * t175;
            let t177 = t30 * t176;
            let t179 = v_sigma2 * v_sigma2;
            let t180 = f64x8::splat(1.0) / t159;
            let t181 = t179 * t180;
            let t182 = v_tau1 * v_tau1;
            let t183 = f64x8::splat(1.0) / t182;
            let t184 = t181 * t183;
            let t186 = f64x8::splat(1.0) + t184 / f64x8::splat(64.0);
            let t187 = t186 * t186;
            let t188 = f64x8::splat(1.0) / t187;
            let t189 = t183 * t188;
            let t193 = (f64x8::splat(10.0) / f64x8::splat(81.0) + f64x8::splat(0.02485875) * t181 * t189) * t42;
            let t194 = t47 * v_sigma2;
            let t195 = t194 * t161;
            let t198 = t170 - f64x8::splat(1.0);
            let t199 = t47 * t198;
            let t202 = f64x8::splat(1.0) + f64x8::splat(0.2222222222222222) * t165 * t199;
            let t203 = ((t202).sqrt());
            let t204 = f64x8::splat(1.0) / t203;
            let t207 = t89 * t162;
            let t209 = f64x8::splat(9.0) / f64x8::splat(20.0) * t198 * t204 + t207 / f64x8::splat(36.0);
            let t210 = t209 * t209;
            let t213 = t159 * t159;
            let t214 = t213 * v_rho1;
            let t216 = f64x8::splat(1.0) / t154 / t214;
            let t217 = t179 * t216;
            let t218 = t99 * t217;
            let t220 = f64x8::splat(162.0) * t184 + f64x8::splat(50.0) * t218;
            let t221 = ((t220).sqrt());
            let t224 = f64x8::splat(1.0) / t169;
            let t226 = t112 * t224 * t174;
            let t230 = t179 * v_sigma2;
            let t231 = t213 * t213;
            let t232 = f64x8::splat(1.0) / t231;
            let t235 = t193 * t195 / f64x8::splat(24.0) + f64x8::splat(146.0) / f64x8::splat(2025.0) * t210 - f64x8::splat(73.0) / f64x8::splat(97200.0) * t209 * t221 + f64x8::splat(25.0) / f64x8::splat(209952.0) * t226 * t218 + f64x8::splat(0.0017218861679299947) * t184 + f64x8::splat(1.5033019185692233e-06) * t230 * t232;
            let t237 = f64x8::splat(1.0) + f64x8::splat(0.05165658503789984) * t207;
            let t238 = t237 * t237;
            let t239 = f64x8::splat(1.0) / t238;
            let t241 = f64x8::splat(2.0) / f64x8::splat(45.0) * t177 + t235 * t239;
            let t242 = f64x8::splat(1.0) / t241;
            let t246 = f64x8::splat(1.0) - f64x8::splat(2.0) / f64x8::splat(45.0) * t30 * t176 * t242;
            let t250 = f64x8::splat(1.0) + f64x8::splat(2.0) / f64x8::splat(45.0) * t30 * t176 * t246;
            let t254 = ((t144).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t153 * t250));
            let tzk0 = t143 + t254;
            acc_zk = tzk0;
        }
        store_add(zk, ip, m, acc_zk);
        ip += 8;
    }
}
