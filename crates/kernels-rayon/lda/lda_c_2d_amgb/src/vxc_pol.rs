//! LDA_C_2D_AMGB vxc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_c_2d_amgb.c`
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
pub fn lda_c_2d_amgb_vxc_pol(
    rho: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
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
        let mut acc_zk = V_ZERO;
        let mut acc_vrho_0 = V_ZERO;
        let mut acc_vrho_1 = V_ZERO;
        {
            let t1 = v_rho0 + v_rho1;
            let t2 = ((t1).sqrt());
            let t3 = f64x8::splat(1.0) / t2;
            let t5 = f64x8::splat(1.0) / t1;
            let t8 = f64x8::splat(1.0) / t2 / t1;
            let t10 = f64x8::splat(0.04869723403850762) * t3 + f64x8::splat(0.018219548589342285) * t5 + f64x8::splat(0.000603947002028882) * t8;
            let t12 = ((f64x8::splat(M_PI)).sqrt());
            let t13 = f64x8::splat(1.0) / t12;
            let t14 = t13 * t3;
            let t15 = ((t14) * (t14).sqrt());
            let t19 = f64x8::splat(0.5654308006315614) * t3 - f64x8::splat(0.02069) * t15 + f64x8::splat(0.10821581200590331) * t5 + f64x8::splat(0.00313738702352666) * t8;
            let t21 = f64x8::splat(1.0) + f64x8::splat(1.0) / t19;
            let t22 = (simd::ln(t21));
            let t23 = t10 * t22;
            let t27 = -f64x8::splat(0.01914859446561085) * t3 - f64x8::splat(0.0024406887987971425) * t5 - f64x8::splat(1.643337945467037e-05) * t8;
            let t31 = f64x8::splat(0.2331795548802877) * t3 + f64x8::splat(0.021277965468762) * t5 + f64x8::splat(0.0001400599965454174) * t8;
            let t33 = f64x8::splat(1.0) + f64x8::splat(1.0) / t31;
            let t34 = (simd::ln(t33));
            let t36 = f64x8::splat(0.117331) + t27 * t34;
            let t37 = v_rho0 - v_rho1;
            let t38 = t37 * t37;
            let t39 = t36 * t38;
            let t40 = t1 * t1;
            let t41 = f64x8::splat(1.0) / t40;
            let t42 = t39 * t41;
            let t46 = -f64x8::splat(0.020927484222536923) * t3 + f64x8::splat(0.005208122695761946) * t5 - f64x8::splat(0.0048916627893863685) * t8;
            let t49 = f64x8::splat(0.8035757880366529) * t3 + f64x8::splat(0.2088776021566591) * t8;
            let t51 = f64x8::splat(1.0) + f64x8::splat(1.0) / t49;
            let t52 = (simd::ln(t51));
            let t54 = f64x8::splat(0.0234188) + t46 * t52;
            let t55 = t38 * t38;
            let t56 = t54 * t55;
            let t57 = t40 * t40;
            let t58 = f64x8::splat(1.0) / t57;
            let t59 = t56 * t58;
            let t61 = (simd::exp(-f64x8::splat(0.7552241765370266) * t3));
            let t63 = f64x8::splat(M_SQRT2);
            let t64 = (t61 - f64x8::splat(1.0)) * t63;
            let t65 = t13 * t2;
            let t66 = t37 * t5;
            let t67 = f64x8::splat(1.0) + t66;
            let t68 = (t67).simd_le(zeta_threshold);
            let t69 = ((zeta_threshold).sqrt());
            let t70 = t69 * zeta_threshold;
            let t71 = ((t67).sqrt());
            let t72 = t71 * t67;
            let t73 = ((t68).select(t70, t72));
            let t75 = f64x8::splat(1.0) - t66;
            let t76 = (t75).simd_le(zeta_threshold);
            let t77 = ((t75).sqrt());
            let t78 = t77 * t75;
            let t79 = ((t76).select(t70, t78));
            let t85 = t73 / f64x8::splat(2.0) + t79 / f64x8::splat(2.0) - f64x8::splat(1.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t38 * t41 - f64x8::splat(3.0) / f64x8::splat(128.0) * t55 * t58;
            let t88 = f64x8::splat(4.0) / f64x8::splat(3.0) * t64 * t65 * t85;
            let tzk0 = -f64x8::splat(0.1925) + t23 + t42 + t59 - t88;
            acc_zk = tzk0;
            let t92 = f64x8::splat(1.0) / t2 / t40;
            let t94 = -f64x8::splat(0.02434861701925381) * t8 - f64x8::splat(0.018219548589342285) * t41 - f64x8::splat(0.000905920503043323) * t92;
            let t95 = t94 * t22;
            let t96 = t19 * t19;
            let t97 = f64x8::splat(1.0) / t96;
            let t98 = t10 * t97;
            let t100 = ((t14).sqrt());
            let t101 = t100 * t13;
            let t106 = -f64x8::splat(0.2827154003157807) * t8 + f64x8::splat(0.0155175) * t101 * t8 - f64x8::splat(0.10821581200590331) * t41 - f64x8::splat(0.00470608053528999) * t92;
            let t107 = f64x8::splat(1.0) / t21;
            let t108 = t106 * t107;
            let t109 = t98 * t108;
            let t113 = f64x8::splat(0.009574297232805425) * t8 + f64x8::splat(0.0024406887987971425) * t41 + f64x8::splat(2.4650069182005552e-05) * t92;
            let t115 = t31 * t31;
            let t116 = f64x8::splat(1.0) / t115;
            let t117 = t27 * t116;
            let t121 = -f64x8::splat(0.11658977744014384) * t8 - f64x8::splat(0.021277965468762) * t41 - f64x8::splat(0.0002100899948181261) * t92;
            let t122 = f64x8::splat(1.0) / t33;
            let t123 = t121 * t122;
            let t125 = t113 * t34 - t117 * t123;
            let t126 = t125 * t38;
            let t127 = t126 * t41;
            let t128 = t36 * t37;
            let t129 = t128 * t41;
            let t130 = f64x8::splat(2.0) * t129;
            let t131 = t40 * t1;
            let t132 = f64x8::splat(1.0) / t131;
            let t133 = t39 * t132;
            let t134 = f64x8::splat(2.0) * t133;
            let t138 = f64x8::splat(0.010463742111268461) * t8 - f64x8::splat(0.005208122695761946) * t41 + f64x8::splat(0.007337494184079552) * t92;
            let t140 = t49 * t49;
            let t141 = f64x8::splat(1.0) / t140;
            let t142 = t46 * t141;
            let t145 = -f64x8::splat(0.40178789401832643) * t8 - f64x8::splat(0.31331640323498866) * t92;
            let t146 = f64x8::splat(1.0) / t51;
            let t147 = t145 * t146;
            let t149 = t138 * t52 - t142 * t147;
            let t150 = t149 * t55;
            let t151 = t150 * t58;
            let t152 = t38 * t37;
            let t153 = t54 * t152;
            let t154 = t153 * t58;
            let t155 = f64x8::splat(4.0) * t154;
            let t156 = t57 * t1;
            let t157 = f64x8::splat(1.0) / t156;
            let t158 = t56 * t157;
            let t159 = f64x8::splat(4.0) * t158;
            let t160 = t5 * t61;
            let t161 = t63 * t85;
            let t162 = t160 * t161;
            let t163 = f64x8::splat(0.2840597424304148) * t162;
            let t165 = t64 * t14 * t85;
            let t166 = f64x8::splat(2.0) / f64x8::splat(3.0) * t165;
            let t167 = t37 * t41;
            let t168 = t5 - t167;
            let t171 = ((t68).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(2.0) * t71 * t168));
            let t173 = -t168;
            let t176 = ((t76).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(2.0) * t77 * t173));
            let t178 = f64x8::splat(3.0) / f64x8::splat(4.0) * t167;
            let t180 = f64x8::splat(3.0) / f64x8::splat(4.0) * t38 * t132;
            let t182 = f64x8::splat(3.0) / f64x8::splat(32.0) * t152 * t58;
            let t184 = f64x8::splat(3.0) / f64x8::splat(32.0) * t55 * t157;
            let t185 = t171 / f64x8::splat(2.0) + t176 / f64x8::splat(2.0) - t178 + t180 - t182 + t184;
            let t187 = t64 * t65 * t185;
            let t188 = f64x8::splat(4.0) / f64x8::splat(3.0) * t187;
            let t189 = t95 - t109 + t127 + t130 - t134 + t151 + t155 - t159 - t163 - t166 - t188;
            let tvrho0 = -f64x8::splat(0.1925) + t23 + t42 + t59 - t88 + t1 * t189;
            acc_vrho_0 = tvrho0;
            let t191 = -t5 - t167;
            let t194 = ((t68).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(2.0) * t71 * t191));
            let t196 = -t191;
            let t199 = ((t76).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(2.0) * t77 * t196));
            let t201 = t194 / f64x8::splat(2.0) + t199 / f64x8::splat(2.0) + t178 + t180 + t182 + t184;
            let t203 = t64 * t65 * t201;
            let t204 = f64x8::splat(4.0) / f64x8::splat(3.0) * t203;
            let t205 = t95 - t109 + t127 - t130 - t134 + t151 - t155 - t159 - t163 - t166 - t204;
            let tvrho1 = -f64x8::splat(0.1925) + t23 + t42 + t59 - t88 + t1 * t205;
            acc_vrho_1 = tvrho1;
        }
        store_add(zk, ip, m, acc_zk);
        store_strided(vrho, ip, m, 2, 0, acc_vrho_0);
        store_strided(vrho, ip, m, 2, 1, acc_vrho_1);
        ip += 8;
    }
}
