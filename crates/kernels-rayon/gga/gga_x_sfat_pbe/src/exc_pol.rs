//! GGA_X_SFAT_PBE exc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_sfat_pbe.c`
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
pub fn gga_x_sfat_pbe_exc_pol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    param_hyb_omega_0: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_hyb_omega_0 = f64x8::splat(param_hyb_omega_0);
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
            let t1 = (v_rho0).simd_le(dens_threshold);
            let t2 = f64x8::splat(M_CBRT3);
            let t3 = f64x8::splat(M_CBRTPI);
            let t5 = f64x8::splat(1.0) / t3 * t2;
            let t6 = v_rho0 + v_rho1;
            let t7 = f64x8::splat(1.0) / t6;
            let t10 = (f64x8::splat(2.0) * t7 * v_rho0).simd_le(zeta_threshold);
            let t11 = zeta_threshold - f64x8::splat(1.0);
            let t14 = (f64x8::splat(2.0) * t7 * v_rho1).simd_le(zeta_threshold);
            let t15 = -t11;
            let t16 = v_rho0 - v_rho1;
            let t18 = ((t10).select(t11, (t14).select(t15, t7 * t16)));
            let t19 = f64x8::splat(1.0) + t18;
            let t20 = (t19).simd_le(zeta_threshold);
            let t21 = (simd::cbrt(zeta_threshold));
            let t22 = t21 * zeta_threshold;
            let t23 = (simd::cbrt(t19));
            let t25 = ((t20).select(t22, t23 * t19));
            let t26 = t25 * t5;
            let t27 = (simd::cbrt(t6));
            let t28 = t2 * t2;
            let t29 = t28 * f64x8::splat(M_PI);
            let t30 = f64x8::splat(1.0) / f64x8::splat(M_PI);
            let t31 = (simd::cbrt(t30));
            let t32 = f64x8::splat(1.0) / t31;
            let t33 = f64x8::splat(M_CBRT4);
            let t34 = t33 * t32;
            let t35 = f64x8::splat(M_CBRT6);
            let t36 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t37 = (simd::cbrt(t36));
            let t38 = t37 * t37;
            let t39 = f64x8::splat(1.0) / t38;
            let t40 = t39 * t35;
            let t41 = v_rho0 * v_rho0;
            let t42 = (simd::cbrt(v_rho0));
            let t43 = t42 * t42;
            let t45 = f64x8::splat(1.0) / t43 / t41;
            let t49 = f64x8::splat(0.804) + f64x8::splat(0.009146457198521547) * t45 * v_sigma0 * t40;
            let t52 = f64x8::splat(1.804) - f64x8::splat(0.646416) / t49;
            let t55 = f64x8::splat(1.0) / t52 * t34 * t29;
            let t56 = ((t55).sqrt());
            let t58 = f64x8::splat(1.0) / t56 * param_hyb_omega_0;
            let t59 = f64x8::splat(M_CBRT2);
            let t60 = t6 * t19;
            let t61 = (simd::cbrt(t60));
            let t62 = f64x8::splat(1.0) / t61;
            let t63 = t62 * t59;
            let t65 = t63 * t58 / f64x8::splat(2.0);
            let t66 = (f64x8::splat(1.92)).simd_le(t65);
            let t67 = (f64x8::splat(1.92)).simd_lt(t65);
            let t68 = ((t67).select(t65, f64x8::splat(1.92)));
            let t69 = t68 * t68;
            let t70 = t69 * t69;
            let t71 = f64x8::splat(1.0) / t70;
            let t73 = t70 * t69;
            let t74 = f64x8::splat(1.0) / t73;
            let t76 = t70 * t70;
            let t77 = f64x8::splat(1.0) / t76;
            let t79 = t76 * t69;
            let t80 = f64x8::splat(1.0) / t79;
            let t82 = t76 * t70;
            let t83 = f64x8::splat(1.0) / t82;
            let t85 = t76 * t73;
            let t86 = f64x8::splat(1.0) / t85;
            let t88 = t76 * t76;
            let t89 = f64x8::splat(1.0) / t88;
            let t92 = f64x8::splat(1.0) / t88 / t69;
            let t95 = f64x8::splat(1.0) / t88 / t70;
            let t98 = f64x8::splat(1.0) / t88 / t73;
            let t101 = f64x8::splat(1.0) / t88 / t76;
            let t104 = f64x8::splat(1.0) / t88 / t79;
            let t107 = f64x8::splat(1.0) / t88 / t82;
            let t110 = f64x8::splat(1.0) / t88 / t85;
            let t112 = t88 * t88;
            let t113 = f64x8::splat(1.0) / t112;
            let t116 = f64x8::splat(1.0) / t112 / t69;
            let t119 = f64x8::splat(1.0) / t112 / t70;
            let t123 = -t71 / f64x8::splat(30.0) + t74 / f64x8::splat(70.0) - t77 / f64x8::splat(135.0) + t80 / f64x8::splat(231.0) - t83 / f64x8::splat(364.0) + t86 / f64x8::splat(540.0) - t89 / f64x8::splat(765.0) + t92 / f64x8::splat(1045.0) - t95 / f64x8::splat(1386.0) + t98 / f64x8::splat(1794.0) - t101 / f64x8::splat(2275.0) + t104 / f64x8::splat(2835.0) - t107 / f64x8::splat(3480.0) + t110 / f64x8::splat(4216.0) - t113 / f64x8::splat(5049.0) + t116 / f64x8::splat(5985.0) - t119 / f64x8::splat(7030.0) + f64x8::splat(1.0) / t69 / f64x8::splat(9.0);
            let t124 = ((t67).select(f64x8::splat(1.92), t65));
            let t125 = (simd::atan2(f64x8::splat(1.0), t124));
            let t126 = t124 * t124;
            let t127 = t126 + f64x8::splat(3.0);
            let t128 = f64x8::splat(1.0) / t126;
            let t129 = f64x8::splat(1.0) + t128;
            let t130 = (simd::ln(t129));
            let t132 = -t130 * t127 + f64x8::splat(1.0);
            let t135 = t125 + t132 * t124 / f64x8::splat(4.0);
            let t139 = ((t66).select(t123, f64x8::splat(1.0) - f64x8::splat(8.0) / f64x8::splat(3.0) * t135 * t124));
            let t140 = t139 * t27;
            let t141 = t52 * t140;
            let t144 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t141 * t26));
            let t145 = (v_rho1).simd_le(dens_threshold);
            let t146 = -t16;
            let t148 = ((t14).select(t11, (t10).select(t15, t7 * t146)));
            let t149 = f64x8::splat(1.0) + t148;
            let t150 = (t149).simd_le(zeta_threshold);
            let t151 = (simd::cbrt(t149));
            let t153 = ((t150).select(t22, t151 * t149));
            let t154 = t153 * t5;
            let t155 = v_rho1 * v_rho1;
            let t156 = (simd::cbrt(v_rho1));
            let t157 = t156 * t156;
            let t159 = f64x8::splat(1.0) / t157 / t155;
            let t163 = f64x8::splat(0.804) + f64x8::splat(0.009146457198521547) * t159 * v_sigma2 * t40;
            let t166 = f64x8::splat(1.804) - f64x8::splat(0.646416) / t163;
            let t169 = f64x8::splat(1.0) / t166 * t34 * t29;
            let t170 = ((t169).sqrt());
            let t172 = f64x8::splat(1.0) / t170 * param_hyb_omega_0;
            let t173 = t6 * t149;
            let t174 = (simd::cbrt(t173));
            let t175 = f64x8::splat(1.0) / t174;
            let t176 = t175 * t59;
            let t178 = t176 * t172 / f64x8::splat(2.0);
            let t179 = (f64x8::splat(1.92)).simd_le(t178);
            let t180 = (f64x8::splat(1.92)).simd_lt(t178);
            let t181 = ((t180).select(t178, f64x8::splat(1.92)));
            let t182 = t181 * t181;
            let t183 = t182 * t182;
            let t184 = f64x8::splat(1.0) / t183;
            let t186 = t183 * t182;
            let t187 = f64x8::splat(1.0) / t186;
            let t189 = t183 * t183;
            let t190 = f64x8::splat(1.0) / t189;
            let t192 = t189 * t182;
            let t193 = f64x8::splat(1.0) / t192;
            let t195 = t189 * t183;
            let t196 = f64x8::splat(1.0) / t195;
            let t198 = t189 * t186;
            let t199 = f64x8::splat(1.0) / t198;
            let t201 = t189 * t189;
            let t202 = f64x8::splat(1.0) / t201;
            let t205 = f64x8::splat(1.0) / t201 / t182;
            let t208 = f64x8::splat(1.0) / t201 / t183;
            let t211 = f64x8::splat(1.0) / t201 / t186;
            let t214 = f64x8::splat(1.0) / t201 / t189;
            let t217 = f64x8::splat(1.0) / t201 / t192;
            let t220 = f64x8::splat(1.0) / t201 / t195;
            let t223 = f64x8::splat(1.0) / t201 / t198;
            let t225 = t201 * t201;
            let t226 = f64x8::splat(1.0) / t225;
            let t229 = f64x8::splat(1.0) / t225 / t182;
            let t232 = f64x8::splat(1.0) / t225 / t183;
            let t236 = -t184 / f64x8::splat(30.0) + t187 / f64x8::splat(70.0) - t190 / f64x8::splat(135.0) + t193 / f64x8::splat(231.0) - t196 / f64x8::splat(364.0) + t199 / f64x8::splat(540.0) - t202 / f64x8::splat(765.0) + t205 / f64x8::splat(1045.0) - t208 / f64x8::splat(1386.0) + t211 / f64x8::splat(1794.0) - t214 / f64x8::splat(2275.0) + t217 / f64x8::splat(2835.0) - t220 / f64x8::splat(3480.0) + t223 / f64x8::splat(4216.0) - t226 / f64x8::splat(5049.0) + t229 / f64x8::splat(5985.0) - t232 / f64x8::splat(7030.0) + f64x8::splat(1.0) / t182 / f64x8::splat(9.0);
            let t237 = ((t180).select(f64x8::splat(1.92), t178));
            let t238 = (simd::atan2(f64x8::splat(1.0), t237));
            let t239 = t237 * t237;
            let t240 = t239 + f64x8::splat(3.0);
            let t241 = f64x8::splat(1.0) / t239;
            let t242 = f64x8::splat(1.0) + t241;
            let t243 = (simd::ln(t242));
            let t245 = -t243 * t240 + f64x8::splat(1.0);
            let t248 = t238 + t245 * t237 / f64x8::splat(4.0);
            let t252 = ((t179).select(t236, f64x8::splat(1.0) - f64x8::splat(8.0) / f64x8::splat(3.0) * t248 * t237));
            let t253 = t252 * t27;
            let t254 = t166 * t253;
            let t257 = ((t145).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t254 * t154));
            let tzk0 = t144 + t257;
            acc_zk = tzk0;
        }
        store_add(zk, ip, m, acc_zk);
        ip += 8;
    }
}
