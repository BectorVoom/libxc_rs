//! GGA_C_OP_B88 vxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_op_b88.c`
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
pub fn gga_c_op_b88_vxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
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
        let mut acc_zk = V_ZERO;
        let mut acc_vrho = V_ZERO;
        let mut acc_vsigma = V_ZERO;
        {
            let t1 = (f64x8::splat(1.0)).simd_le(zeta_threshold);
            let t4 = (t1) | ((v_rho / f64x8::splat(2.0)).simd_le(dens_threshold));
            let t5 = zeta_threshold - f64x8::splat(1.0);
            let t6 = -t5;
            let t7 = ((t1).select(t5, (t1).select(t6, f64x8::splat(0.0))));
            let t8 = t7 * t7;
            let t9 = f64x8::splat(1.0) - t8;
            let t10 = t9 * v_rho;
            let t11 = f64x8::splat(1.0) + t7;
            let t14 = (t11 * v_rho / f64x8::splat(2.0)).simd_le(dens_threshold);
            let t15 = f64x8::splat(M_CBRT3);
            let t16 = t15 * t15;
            let t18 = (simd::cbrt(f64x8::splat(1.0) / f64x8::splat(M_PI)));
            let t20 = t16 / t18;
            let t21 = f64x8::splat(M_CBRT4);
            let t22 = t20 * t21;
            let t23 = f64x8::splat(M_CBRT2);
            let t24 = (t11).simd_le(zeta_threshold);
            let t25 = f64x8::splat(1.0) - t7;
            let t26 = (t25).simd_le(zeta_threshold);
            let t27 = ((t24).select(t5, (t26).select(t6, t7)));
            let t28 = f64x8::splat(1.0) + t27;
            let t29 = t28 * v_rho;
            let t30 = (simd::cbrt(t29));
            let t31 = f64x8::splat(1.0) / t30;
            let t32 = t23 * t31;
            let t33 = t23 * t23;
            let t34 = v_sigma * t33;
            let t35 = v_rho * v_rho;
            let t36 = (simd::cbrt(v_rho));
            let t37 = t36 * t36;
            let t39 = f64x8::splat(1.0) / t37 / t35;
            let t40 = ((v_sigma).sqrt());
            let t41 = t40 * t23;
            let t43 = f64x8::splat(1.0) / t36 / v_rho;
            let t45 = (simd::ln(t41 * t43 + ((((t41 * t43) * (t41 * t43)) + f64x8::splat(1.0)).sqrt())));
            let t46 = t43 * t45;
            let t49 = f64x8::splat(1.0) + f64x8::splat(0.0252) * t41 * t46;
            let t50 = f64x8::splat(1.0) / t49;
            let t55 = f64x8::splat(1.0) + f64x8::splat(0.0009333333333333333) * t22 * t34 * t39 * t50;
            let t56 = f64x8::splat(1.0) / t55;
            let t60 = ((t14).select(f64x8::splat(0.0), t22 * t32 * t56 / f64x8::splat(9.0)));
            let t64 = (t25 * v_rho / f64x8::splat(2.0)).simd_le(dens_threshold);
            let t65 = ((t26).select(t5, (t24).select(t6, -t7)));
            let t66 = f64x8::splat(1.0) + t65;
            let t67 = t66 * v_rho;
            let t68 = (simd::cbrt(t67));
            let t69 = f64x8::splat(1.0) / t68;
            let t70 = t23 * t69;
            let t74 = ((t64).select(f64x8::splat(0.0), t22 * t70 * t56 / f64x8::splat(9.0)));
            let t75 = t60 + t74;
            let t76 = (t75).simd_eq(f64x8::splat(0.0));
            let t77 = ((t76).select(f64x8::splat(f64::EPSILON), t75));
            let t80 = f64x8::splat(3.6011538) / t77 + f64x8::splat(0.5764);
            let t81 = t77 * t77;
            let t82 = t81 * t81;
            let t83 = f64x8::splat(1.0) / t82;
            let t85 = t81 * t77;
            let t86 = f64x8::splat(1.0) / t85;
            let t88 = f64x8::splat(1.0) / t81;
            let t90 = f64x8::splat(31.390124030721) * t83 + f64x8::splat(14.9643497914092) * t86 + f64x8::splat(1.7833359087) * t88;
            let t91 = f64x8::splat(1.0) / t90;
            let tzk0 = ((t4).select(f64x8::splat(0.0), -f64x8::splat(0.25) * t10 * t80 * t91));
            acc_zk = tzk0;
            let t95 = t9 * t80;
            let t99 = f64x8::splat(1.0) / t30 / t29;
            let t105 = t55 * t55;
            let t106 = f64x8::splat(1.0) / t105;
            let t107 = t35 * v_rho;
            let t109 = f64x8::splat(1.0) / t37 / t107;
            let t114 = t21 * v_sigma;
            let t115 = t20 * t114;
            let t116 = t33 * t39;
            let t117 = t49 * t49;
            let t118 = f64x8::splat(1.0) / t117;
            let t121 = f64x8::splat(1.0) / t36 / t35 * t45;
            let t125 = t34 * t39 + f64x8::splat(1.0);
            let t126 = ((t125).sqrt());
            let t127 = f64x8::splat(1.0) / t126;
            let t128 = t109 * t127;
            let t131 = -f64x8::splat(0.0336) * t41 * t121 - f64x8::splat(0.0336) * t34 * t128;
            let t132 = t118 * t131;
            let t133 = t116 * t132;
            let t136 = -f64x8::splat(0.002488888888888889) * t22 * t34 * t109 * t50 - f64x8::splat(0.0009333333333333333) * t115 * t133;
            let t137 = t106 * t136;
            let t142 = ((t14).select(f64x8::splat(0.0), -t22 * t23 * t99 * t56 * t28 / f64x8::splat(27.0) - t22 * t32 * t137 / f64x8::splat(9.0)));
            let t144 = f64x8::splat(1.0) / t68 / t67;
            let t154 = ((t64).select(f64x8::splat(0.0), -t22 * t23 * t144 * t56 * t66 / f64x8::splat(27.0) - t22 * t70 * t137 / f64x8::splat(9.0)));
            let t156 = ((t76).select(f64x8::splat(0.0), t142 + t154));
            let t161 = t90 * t90;
            let t162 = f64x8::splat(1.0) / t161;
            let t163 = t80 * t162;
            let t165 = f64x8::splat(1.0) / t82 / t77;
            let t166 = t165 * t156;
            let t168 = t83 * t156;
            let t172 = -f64x8::splat(125.560496122884) * t166 - f64x8::splat(44.8930493742276) * t168 - f64x8::splat(3.5666718174) * t86 * t156;
            let t177 = ((t4).select(f64x8::splat(0.0), -f64x8::splat(0.25) * t95 * t91 + f64x8::splat(0.90028845) * t10 * t88 * t156 * t91 + f64x8::splat(0.25) * t10 * t163 * t172));
            let tvrho0 = v_rho * t177 + tzk0;
            acc_vrho = tvrho0;
            let t183 = f64x8::splat(1.0) / t40 * t23;
            let t188 = f64x8::splat(0.0126) * t183 * t46 + f64x8::splat(0.0126) * t116 * t127;
            let t189 = t118 * t188;
            let t190 = t116 * t189;
            let t193 = f64x8::splat(0.0009333333333333333) * t22 * t116 * t50 - f64x8::splat(0.0009333333333333333) * t115 * t190;
            let t194 = t106 * t193;
            let t198 = ((t14).select(f64x8::splat(0.0), -t22 * t32 * t194 / f64x8::splat(9.0)));
            let t202 = ((t64).select(f64x8::splat(0.0), -t22 * t70 * t194 / f64x8::splat(9.0)));
            let t204 = ((t76).select(f64x8::splat(0.0), t198 + t202));
            let t209 = t165 * t204;
            let t211 = t83 * t204;
            let t213 = t86 * t204;
            let t215 = -f64x8::splat(125.560496122884) * t209 - f64x8::splat(44.8930493742276) * t211 - f64x8::splat(3.5666718174) * t213;
            let t220 = ((t4).select(f64x8::splat(0.0), f64x8::splat(0.90028845) * t10 * t88 * t204 * t91 + f64x8::splat(0.25) * t10 * t163 * t215));
            let tvsigma0 = v_rho * t220;
            acc_vsigma = tvsigma0;
        }
        store_add(zk, ip, m, acc_zk);
        store_add(vrho, ip, m, acc_vrho);
        store_add(vsigma, ip, m, acc_vsigma);
        ip += 8;
    }
}
