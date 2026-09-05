//! GGA_X_HJS_B88_V2 exc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_hjs_b88_v2.c`
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
pub fn gga_x_hjs_b88_v2_exc_unpol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    param_a_0: f64,
    param_a_1: f64,
    param_a_2: f64,
    param_a_3: f64,
    param_a_4: f64,
    param_a_5: f64,
    param_b_0: f64,
    param_b_1: f64,
    param_b_2: f64,
    param_b_3: f64,
    param_b_4: f64,
    param_b_5: f64,
    param_b_6: f64,
    param_b_7: f64,
    param_b_8: f64,
    param_hyb_omega_0: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_a_0 = f64x8::splat(param_a_0);
    let param_a_1 = f64x8::splat(param_a_1);
    let param_a_2 = f64x8::splat(param_a_2);
    let param_a_3 = f64x8::splat(param_a_3);
    let param_a_4 = f64x8::splat(param_a_4);
    let param_a_5 = f64x8::splat(param_a_5);
    let param_b_0 = f64x8::splat(param_b_0);
    let param_b_1 = f64x8::splat(param_b_1);
    let param_b_2 = f64x8::splat(param_b_2);
    let param_b_3 = f64x8::splat(param_b_3);
    let param_b_4 = f64x8::splat(param_b_4);
    let param_b_5 = f64x8::splat(param_b_5);
    let param_b_6 = f64x8::splat(param_b_6);
    let param_b_7 = f64x8::splat(param_b_7);
    let param_b_8 = f64x8::splat(param_b_8);
    let param_hyb_omega_0 = f64x8::splat(param_hyb_omega_0);
    let dens_threshold = f64x8::splat(dens_threshold);
    let zeta_threshold = f64x8::splat(zeta_threshold);
    let mut ip = 0usize;
    while ip < np {
        let m = (np - ip).min(8);
        let v_rho = load(rho, ip, np);
        let v_sigma = load(sigma, ip, np);
        let mut acc_zk = V_ZERO;
        {
            let t2 = (v_rho / f64x8::splat(2.0)).simd_le(dens_threshold);
            let t3 = f64x8::splat(M_CBRT3);
            let t4 = f64x8::splat(M_CBRTPI);
            let t6 = t3 / t4;
            let t7 = (f64x8::splat(1.0)).simd_le(zeta_threshold);
            let t8 = zeta_threshold - f64x8::splat(1.0);
            let t10 = ((t7).select(t8, (t7).select(-t8, f64x8::splat(0.0))));
            let t11 = f64x8::splat(1.0) + t10;
            let t12 = (t11).simd_le(zeta_threshold);
            let t13 = (simd::cbrt(zeta_threshold));
            let t15 = (simd::cbrt(t11));
            let t17 = ((t12).select(t13 * zeta_threshold, t15 * t11));
            let t18 = (simd::cbrt(v_rho));
            let t19 = t17 * t18;
            let t20 = t3 * t3;
            let t21 = param_hyb_omega_0 * t20;
            let t22 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t23 = (simd::cbrt(t22));
            let t24 = f64x8::splat(1.0) / t23;
            let t25 = t21 * t24;
            let t26 = ((t12).select(t13, t15));
            let t27 = f64x8::splat(1.0) / t26;
            let t28 = f64x8::splat(1.0) / t18;
            let t29 = t27 * t28;
            let t30 = f64x8::splat(M_CBRT6);
            let t31 = t30 * t30;
            let t32 = t31 * t24;
            let t33 = ((v_sigma).sqrt());
            let t34 = f64x8::splat(M_CBRT2);
            let t35 = t33 * t34;
            let t37 = f64x8::splat(1.0) / t18 / v_rho;
            let t41 = (simd::exp(-t32 * t35 * t37 / f64x8::splat(12.0)));
            let t42 = (simd::exp(f64x8::splat(20.0)));
            let t44 = f64x8::splat(1.0) / (t42 - f64x8::splat(1.0));
            let t45 = t41 + t44;
            let t49 = (simd::ln(t45 / (f64x8::splat(1.0) + t44)));
            let t50 = t49 * t49;
            let t51 = param_a_0;
            let t53 = param_a_1;
            let t54 = t50 * t49;
            let t56 = param_a_2;
            let t57 = t50 * t50;
            let t59 = param_a_3;
            let t60 = t57 * t49;
            let t62 = param_a_4;
            let t63 = t57 * t50;
            let t65 = param_a_5;
            let t66 = t57 * t54;
            let t68 = t50 * t51 - t53 * t54 + t56 * t57 - t59 * t60 + t62 * t63 - t65 * t66;
            let t69 = t50 * t68;
            let t70 = param_b_0;
            let t72 = param_b_1;
            let t74 = param_b_2;
            let t76 = param_b_3;
            let t78 = param_b_4;
            let t80 = param_b_5;
            let t82 = param_b_6;
            let t84 = param_b_7;
            let t85 = t57 * t57;
            let t87 = param_b_8;
            let t90 = -t49 * t85 * t87 - t49 * t70 + t50 * t72 - t54 * t74 + t57 * t76 - t60 * t78 + t63 * t80 - t66 * t82 + t84 * t85 + f64x8::splat(1.0);
            let t91 = f64x8::splat(1.0) / t90;
            let t92 = t69 * t91;
            let t93 = (f64x8::splat(1e-10)).simd_lt(t92);
            let t94 = ((t93).select(t92, f64x8::splat(1e-10)));
            let t95 = param_hyb_omega_0 * param_hyb_omega_0;
            let t96 = t95 * t3;
            let t97 = t23 * t23;
            let t98 = f64x8::splat(1.0) / t97;
            let t99 = t26 * t26;
            let t101 = t98 / t99;
            let t102 = t18 * t18;
            let t103 = f64x8::splat(1.0) / t102;
            let t105 = t96 * t101 * t103;
            let t107 = f64x8::splat(0.60965) + t94 + t105 / f64x8::splat(3.0);
            let t108 = ((t107).sqrt());
            let t109 = f64x8::splat(1.0) / t108;
            let t111 = t25 * t29 * t109;
            let t113 = f64x8::splat(1.0) - t111 / f64x8::splat(3.0);
            let t114 = f64x8::splat(0.60965) + t94;
            let t115 = f64x8::splat(1.0) / t114;
            let t119 = f64x8::splat(1.0) + t50 / f64x8::splat(4.0);
            let t120 = f64x8::splat(1.0) / t119;
            let t124 = f64x8::splat(1.0) + f64x8::splat(0.3121563353845126) * t50 * t120 + f64x8::splat(4.21411052769092) * t94;
            let t126 = f64x8::splat(1.0) / t22;
            let t127 = t95 * param_hyb_omega_0 * t126;
            let t129 = f64x8::splat(1.0) / t99 / t26;
            let t130 = f64x8::splat(1.0) / v_rho;
            let t131 = t129 * t130;
            let t133 = f64x8::splat(1.0) / t108 / t107;
            let t135 = t127 * t131 * t133;
            let t137 = f64x8::splat(2.0) - t111 + t135 / f64x8::splat(3.0);
            let t138 = t124 * t137;
            let t139 = t114 * t114;
            let t140 = f64x8::splat(1.0) / t139;
            let t146 = t139 * t114;
            let t148 = ((t114).sqrt());
            let t149 = t148 * t146;
            let t150 = ((f64x8::splat(M_PI)).sqrt());
            let t152 = ((t94).sqrt());
            let t155 = (f64x8::splat(0.0)).simd_lt(f64x8::splat(0.7572109999) + t94);
            let t157 = ((t155).select(f64x8::splat(0.757211) + t94, f64x8::splat(1e-10)));
            let t158 = ((t157).sqrt());
            let t160 = f64x8::splat(4.0) / f64x8::splat(5.0) * t150 + f64x8::splat(12.0) / f64x8::splat(5.0) * t152 - f64x8::splat(12.0) / f64x8::splat(5.0) * t158;
            let t162 = f64x8::splat(0.0474596) * t124 * t114 + f64x8::splat(0.028363733333333332) * t139 - f64x8::splat(0.9086532) * t146 - t149 * t160;
            let t165 = t95 * t95;
            let t167 = t165 * param_hyb_omega_0 * t3;
            let t169 = f64x8::splat(1.0) / t97 / t22;
            let t170 = t167 * t169;
            let t171 = t99 * t99;
            let t173 = f64x8::splat(1.0) / t171 / t26;
            let t175 = f64x8::splat(1.0) / t102 / v_rho;
            let t176 = t173 * t175;
            let t177 = t107 * t107;
            let t179 = f64x8::splat(1.0) / t108 / t177;
            let t183 = f64x8::splat(8.0) - f64x8::splat(5.0) * t111 + f64x8::splat(10.0) / f64x8::splat(3.0) * t135 - t170 * t176 * t179 / f64x8::splat(3.0);
            let t184 = t162 * t183;
            let t185 = f64x8::splat(1.0) / t146;
            let t189 = f64x8::splat(3.0) * t105;
            let t190 = f64x8::splat(9.0) * t94 + t189;
            let t191 = ((t190).sqrt());
            let t193 = f64x8::splat(9.0) * t157 + t189;
            let t194 = ((t193).sqrt());
            let t196 = t191 / f64x8::splat(3.0) - t194 / f64x8::splat(3.0);
            let t200 = t24 * t27;
            let t202 = t21 * t200 * t28;
            let t204 = t202 / f64x8::splat(3.0) + t191 / f64x8::splat(3.0);
            let t206 = t202 / f64x8::splat(3.0) + t108;
            let t207 = f64x8::splat(1.0) / t206;
            let t209 = (simd::ln(t204 * t207));
            let t213 = t202 / f64x8::splat(3.0) + t194 / f64x8::splat(3.0);
            let t215 = (simd::ln(t213 * t207));
            let t218 = f64x8::splat(0.757211) + f64x8::splat(0.04727288888888889) * t113 * t115 + f64x8::splat(0.026366444444444446) * t138 * t140 - t184 * t185 / f64x8::splat(9.0) + f64x8::splat(2.0) / f64x8::splat(3.0) * t25 * t29 * t196 + f64x8::splat(2.0) * t94 * t209 - f64x8::splat(2.0) * t157 * t215;
            let t222 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t218));
            let tzk0 = f64x8::splat(2.0) * t222;
            acc_zk = tzk0;
        }
        store_add(zk, ip, m, acc_zk);
        ip += 8;
    }
}
