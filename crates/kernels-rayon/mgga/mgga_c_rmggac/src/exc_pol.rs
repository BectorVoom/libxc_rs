//! MGGA_C_RMGGAC exc pol kernel — explicit SIMD (bit-exact).
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
pub fn mgga_c_rmggac_exc_pol(
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
            let t2 = f64x8::splat(M_CBRT3);
            let t3 = f64x8::splat(1.0) / f64x8::splat(M_PI);
            let t4 = (simd::cbrt(t3));
            let t5 = t2 * t4;
            let t6 = f64x8::splat(M_CBRT4);
            let t7 = t6 * t6;
            let t8 = v_rho0 + v_rho1;
            let t9 = (simd::cbrt(t8));
            let t12 = t5 * t7 / t9;
            let t13 = ((t12).sqrt());
            let t16 = f64x8::splat(1.0) + f64x8::splat(0.04445) * t13 + f64x8::splat(0.03138525) * t12;
            let t17 = f64x8::splat(1.0) / t16;
            let t20 = (simd::exp(f64x8::splat(1.0) * t17));
            let t21 = t20 - f64x8::splat(1.0);
            let t22 = f64x8::splat(M_CBRT6);
            let t23 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t24 = (simd::cbrt(t23));
            let t25 = t24 * t24;
            let t26 = f64x8::splat(1.0) / t25;
            let t27 = t22 * t26;
            let t28 = f64x8::splat(M_CBRT2);
            let t29 = t28 * t28;
            let t31 = v_sigma0 + f64x8::splat(2.0) * v_sigma1 + v_sigma2;
            let t32 = t29 * t31;
            let t33 = t8 * t8;
            let t34 = t9 * t9;
            let t36 = f64x8::splat(1.0) / t34 / t33;
            let t38 = t27 * t32 * t36;
            let t40 = f64x8::splat(1.0) + f64x8::splat(0.02133764210437636) * t38;
            let t41 = ((t40).sqrt().sqrt());
            let t43 = f64x8::splat(1.0) - f64x8::splat(1.0) / t41;
            let t45 = t21 * t43 + f64x8::splat(1.0);
            let t46 = (simd::ln(t45));
            let t48 = -f64x8::splat(0.0285764) * t17 + f64x8::splat(0.0285764) * t46;
            let t49 = t28 - f64x8::splat(1.0);
            let t50 = v_rho0 - v_rho1;
            let t51 = f64x8::splat(1.0) / t8;
            let t52 = t50 * t51;
            let t53 = f64x8::splat(1.0) + t52;
            let t54 = (t53).simd_le(zeta_threshold);
            let t55 = (simd::cbrt(zeta_threshold));
            let t56 = t55 * zeta_threshold;
            let t57 = (simd::cbrt(t53));
            let t58 = t57 * t53;
            let t59 = ((t54).select(t56, t58));
            let t60 = f64x8::splat(1.0) - t52;
            let t61 = (t60).simd_le(zeta_threshold);
            let t62 = (simd::cbrt(t60));
            let t63 = t62 * t60;
            let t64 = ((t61).select(t56, t63));
            let t65 = t59 + t64 - f64x8::splat(2.0);
            let t68 = f64x8::splat(1.0) / t49 / f64x8::splat(2.0);
            let t71 = f64x8::splat(1.0) - f64x8::splat(2.363) * t49 * t65 * t68;
            let t72 = t48 * t71;
            let t73 = t50 * t50;
            let t74 = t73 * t73;
            let t75 = t74 * t74;
            let t76 = t75 * t74;
            let t77 = t33 * t33;
            let t78 = t77 * t77;
            let t79 = t78 * t77;
            let t80 = f64x8::splat(1.0) / t79;
            let t82 = -t76 * t80 + f64x8::splat(1.0);
            let t83 = (simd::cbrt(v_rho0));
            let t84 = t83 * t83;
            let t86 = f64x8::splat(1.0) / t84 / v_rho0;
            let t87 = v_tau0 * t86;
            let t88 = t53 / f64x8::splat(2.0);
            let t89 = (simd::cbrt(t88));
            let t90 = t89 * t89;
            let t91 = t90 * t88;
            let t94 = (simd::cbrt(v_rho1));
            let t95 = t94 * t94;
            let t97 = f64x8::splat(1.0) / t95 / v_rho1;
            let t98 = v_tau1 * t97;
            let t99 = t60 / f64x8::splat(2.0);
            let t100 = (simd::cbrt(t99));
            let t101 = t100 * t100;
            let t102 = t101 * t99;
            let t107 = f64x8::splat(2.0) * t87 * t91 + f64x8::splat(2.0) * t98 * t102 - t31 * t36 / f64x8::splat(4.0);
            let t108 = t107 * t107;
            let t109 = t108 * t107;
            let t114 = f64x8::splat(0.08) + f64x8::splat(5.0) / f64x8::splat(18.0) * t107 * t29 * t27 + f64x8::splat(0.0125) * t38;
            let t115 = t114 * t114;
            let t116 = t115 * t114;
            let t117 = f64x8::splat(1.0) / t116;
            let t118 = t109 * t117;
            let t120 = t108 * t108;
            let t121 = t120 * t108;
            let t122 = t115 * t115;
            let t124 = f64x8::splat(1.0) / t122 / t115;
            let t127 = f64x8::splat(1.0) + f64x8::splat(0.006652356501035449) * t118 + f64x8::splat(4.42538470168686e-05) * t121 * t124;
            let t128 = f64x8::splat(1.0) / t127;
            let t129 = t118 * t128;
            let t131 = f64x8::splat(1.0) - f64x8::splat(0.01995706950310635) * t129;
            let t132 = t82 * t131;
            let t133 = t72 * t132;
            let t135 = f64x8::splat(1.0) + f64x8::splat(0.053425) * t12;
            let t138 = ((t12) * (t12).sqrt());
            let t140 = t2 * t2;
            let t141 = t4 * t4;
            let t142 = t140 * t141;
            let t145 = t142 * t6 / t34;
            let t147 = f64x8::splat(3.79785) * t13 + f64x8::splat(0.8969) * t12 + f64x8::splat(0.204775) * t138 + f64x8::splat(0.123235) * t145;
            let t150 = f64x8::splat(1.0) + f64x8::splat(16.081979498692537) / t147;
            let t151 = (simd::ln(t150));
            let t153 = f64x8::splat(0.0621814) * t135 * t151;
            let t154 = f64x8::splat(1.0) / t77;
            let t155 = t74 * t154;
            let t156 = t65 * t68;
            let t158 = f64x8::splat(1.0) + f64x8::splat(0.05137) * t12;
            let t163 = f64x8::splat(7.05945) * t13 + f64x8::splat(1.549425) * t12 + f64x8::splat(0.420775) * t138 + f64x8::splat(0.1562925) * t145;
            let t166 = f64x8::splat(1.0) + f64x8::splat(32.16395899738507) / t163;
            let t167 = (simd::ln(t166));
            let t171 = f64x8::splat(1.0) + f64x8::splat(0.0278125) * t12;
            let t176 = f64x8::splat(5.1785) * t13 + f64x8::splat(0.905775) * t12 + f64x8::splat(0.1100325) * t138 + f64x8::splat(0.1241775) * t145;
            let t179 = f64x8::splat(1.0) + f64x8::splat(29.608749977793437) / t176;
            let t180 = (simd::ln(t179));
            let t181 = t171 * t180;
            let t183 = -f64x8::splat(0.0310907) * t158 * t167 + t153 - f64x8::splat(0.0197516734986138) * t181;
            let t184 = t156 * t183;
            let t185 = t155 * t184;
            let t187 = f64x8::splat(0.0197516734986138) * t156 * t181;
            let t188 = t55 * t55;
            let t189 = t57 * t57;
            let t190 = ((t54).select(t188, t189));
            let t191 = t62 * t62;
            let t192 = ((t61).select(t188, t191));
            let t194 = t190 / f64x8::splat(2.0) + t192 / f64x8::splat(2.0);
            let t195 = t194 * t194;
            let t196 = t195 * t194;
            let t197 = -t153 + t185 + t187;
            let t198 = f64x8::splat(1.0) / t196;
            let t201 = (simd::exp(-f64x8::splat(32.16364864430221) * t197 * t198));
            let t202 = t201 - f64x8::splat(1.0);
            let t203 = (simd::ln(f64x8::splat(2.0)));
            let t204 = f64x8::splat(1.0) - t203;
            let t205 = f64x8::splat(1.0) / t204;
            let t206 = t197 * t205;
            let t207 = t23 * t198;
            let t209 = (simd::exp(-t206 * t207));
            let t210 = t209 - f64x8::splat(1.0);
            let t211 = f64x8::splat(1.0) / t210;
            let t212 = t205 * t211;
            let t214 = f64x8::splat(1.0) / t9 / t33;
            let t215 = t31 * t214;
            let t217 = f64x8::splat(1.0) / t195;
            let t219 = f64x8::splat(1.0) / t4;
            let t220 = t140 * t219;
            let t221 = t220 * t6;
            let t222 = t28 * t217 * t221;
            let t225 = f64x8::splat(1.0) + f64x8::splat(0.02743955640261198) * t212 * t215 * t222;
            let t226 = ((t225).sqrt().sqrt());
            let t228 = f64x8::splat(1.0) - f64x8::splat(1.0) / t226;
            let t230 = t202 * t228 + f64x8::splat(1.0);
            let t231 = (simd::ln(t230));
            let t234 = -t153 + t185 + t187 + f64x8::splat(0.031091) * t196 * t231;
            let t235 = t234 * t109;
            let t236 = t117 * t128;
            let t238 = f64x8::splat(0.01995706950310635) * t235 * t236;
            let tzk0 = t133 + t238;
            acc_zk = tzk0;
        }
        store_add(zk, ip, m, acc_zk);
        ip += 8;
    }
}
