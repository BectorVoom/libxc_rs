//! GGA_C_OP_G96 vxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_op_g96.c`
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
pub fn gga_c_op_g96_vxc_unpol(
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
            let t19 = f64x8::splat(1.0) / t18;
            let t20 = t16 * t19;
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
            let t33 = ((v_sigma).sqrt());
            let t34 = t33 * t23;
            let t35 = (simd::cbrt(v_rho));
            let t37 = f64x8::splat(1.0) / t35 / v_rho;
            let t38 = t34 * t37;
            let t39 = ((t38).sqrt());
            let t40 = t39 * t38;
            let t44 = f64x8::splat(1.0) + f64x8::splat(2.0) / f64x8::splat(1233.0) * t20 * t21 * t40;
            let t45 = f64x8::splat(1.0) / t44;
            let t49 = ((t14).select(f64x8::splat(0.0), t22 * t23 * t31 * t45 / f64x8::splat(9.0)));
            let t53 = (t25 * v_rho / f64x8::splat(2.0)).simd_le(dens_threshold);
            let t54 = ((t26).select(t5, (t24).select(t6, -t7)));
            let t55 = f64x8::splat(1.0) + t54;
            let t56 = t55 * v_rho;
            let t57 = (simd::cbrt(t56));
            let t58 = f64x8::splat(1.0) / t57;
            let t63 = ((t53).select(f64x8::splat(0.0), t22 * t23 * t58 * t45 / f64x8::splat(9.0)));
            let t64 = t49 + t63;
            let t65 = (t64).simd_eq(f64x8::splat(0.0));
            let t66 = ((t65).select(f64x8::splat(f64::EPSILON), t64));
            let t69 = f64x8::splat(3.59628532) / t66 + f64x8::splat(0.5764);
            let t70 = t66 * t66;
            let t71 = t70 * t70;
            let t72 = f64x8::splat(1.0) / t71;
            let t74 = t70 * t66;
            let t75 = f64x8::splat(1.0) / t74;
            let t77 = f64x8::splat(1.0) / t70;
            let t79 = f64x8::splat(31.220719919544194) * t72 + f64x8::splat(14.903739892213245) * t75 + f64x8::splat(1.778517305052) * t77;
            let t80 = f64x8::splat(1.0) / t79;
            let tzk0 = ((t4).select(f64x8::splat(0.0), -f64x8::splat(0.25) * t10 * t69 * t80));
            acc_zk = tzk0;
            let t84 = t9 * t69;
            let t88 = f64x8::splat(1.0) / t30 / t29;
            let t94 = t18 * t18;
            let t95 = f64x8::splat(1.0) / t94;
            let t96 = t15 * t95;
            let t97 = t21 * t21;
            let t98 = t23 * t23;
            let t99 = t97 * t98;
            let t100 = t96 * t99;
            let t101 = t44 * t44;
            let t102 = f64x8::splat(1.0) / t101;
            let t103 = t31 * t102;
            let t104 = t39 * t33;
            let t105 = v_rho * v_rho;
            let t107 = f64x8::splat(1.0) / t35 / t105;
            let t108 = t104 * t107;
            let t113 = ((t14).select(f64x8::splat(0.0), -t22 * t23 * t88 * t45 * t28 / f64x8::splat(27.0) + f64x8::splat(4.0) / f64x8::splat(3699.0) * t100 * t103 * t108));
            let t115 = f64x8::splat(1.0) / t57 / t56;
            let t121 = t58 * t102;
            let t126 = ((t53).select(f64x8::splat(0.0), -t22 * t23 * t115 * t45 * t55 / f64x8::splat(27.0) + f64x8::splat(4.0) / f64x8::splat(3699.0) * t100 * t121 * t108));
            let t128 = ((t65).select(f64x8::splat(0.0), t113 + t126));
            let t133 = t79 * t79;
            let t134 = f64x8::splat(1.0) / t133;
            let t135 = t69 * t134;
            let t137 = f64x8::splat(1.0) / t71 / t66;
            let t138 = t137 * t128;
            let t140 = t72 * t128;
            let t144 = -f64x8::splat(124.88287967817678) * t138 - f64x8::splat(44.711219676639736) * t140 - f64x8::splat(3.557034610104) * t75 * t128;
            let t149 = ((t4).select(f64x8::splat(0.0), -f64x8::splat(0.25) * t84 * t80 + f64x8::splat(0.89907133) * t10 * t77 * t128 * t80 + f64x8::splat(0.25) * t10 * t135 * t144));
            let tvrho0 = v_rho * t149 + tzk0;
            acc_vrho = tvrho0;
            let t151 = f64x8::splat(1.0) / t33;
            let t152 = t39 * t151;
            let t153 = t152 * t37;
            let t157 = ((t14).select(f64x8::splat(0.0), -t100 * t103 * t153 / f64x8::splat(2466.0)));
            let t161 = ((t53).select(f64x8::splat(0.0), -t100 * t121 * t153 / f64x8::splat(2466.0)));
            let t163 = ((t65).select(f64x8::splat(0.0), t157 + t161));
            let t168 = t137 * t163;
            let t170 = t72 * t163;
            let t172 = t75 * t163;
            let t174 = -f64x8::splat(124.88287967817678) * t168 - f64x8::splat(44.711219676639736) * t170 - f64x8::splat(3.557034610104) * t172;
            let t179 = ((t4).select(f64x8::splat(0.0), f64x8::splat(0.89907133) * t10 * t77 * t163 * t80 + f64x8::splat(0.25) * t10 * t135 * t174));
            let tvsigma0 = v_rho * t179;
            acc_vsigma = tvsigma0;
        }
        store_add(zk, ip, m, acc_zk);
        store_add(vrho, ip, m, acc_vrho);
        store_add(vsigma, ip, m, acc_vsigma);
        ip += 8;
    }
}
