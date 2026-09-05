//! MGGA_X_GX vxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_gx.c`
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
pub fn mgga_x_gx_vxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    vlapl: &mut [f64],
    vtau: &mut [f64],
    param_c0: f64,
    param_c1: f64,
    param_alphainf: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_c0 = f64x8::splat(param_c0);
    let param_c1 = f64x8::splat(param_c1);
    let param_alphainf = f64x8::splat(param_alphainf);
    let dens_threshold = f64x8::splat(dens_threshold);
    let zeta_threshold = f64x8::splat(zeta_threshold);
    let mut ip = 0usize;
    while ip < np {
        let m = (np - ip).min(8);
        let v_rho = load(rho, ip, np);
        let v_sigma = load(sigma, ip, np);
        let v_lapl = load(lapl, ip, np);
        let v_tau = load(tau, ip, np);
        let mut acc_zk = V_ZERO;
        let mut acc_vrho = V_ZERO;
        let mut acc_vsigma = V_ZERO;
        let mut acc_vlapl = V_ZERO;
        let mut acc_vtau = V_ZERO;
        {
            let t3 = (v_rho / f64x8::splat(2.0)).simd_le(dens_threshold);
            let t4 = f64x8::splat(M_CBRT3);
            let t5 = f64x8::splat(M_CBRTPI);
            let t7 = t4 / t5;
            let t8 = (f64x8::splat(1.0)).simd_le(zeta_threshold);
            let t9 = zeta_threshold - f64x8::splat(1.0);
            let t11 = ((t8).select(t9, (t8).select(-t9, f64x8::splat(0.0))));
            let t12 = f64x8::splat(1.0) + t11;
            let t14 = (simd::cbrt(zeta_threshold));
            let t16 = (simd::cbrt(t12));
            let t18 = (((t12).simd_le(zeta_threshold)).select(t14 * zeta_threshold, t16 * t12));
            let t19 = (simd::cbrt(v_rho));
            let t20 = t18 * t19;
            let t21 = f64x8::splat(M_CBRT2);
            let t22 = t4 * t4;
            let t24 = f64x8::splat(M_CBRT4);
            let t26 = f64x8::splat(8.0) / f64x8::splat(27.0) * t21 * t22 * t24;
            let t27 = t21 * t21;
            let t28 = v_tau * t27;
            let t29 = t19 * t19;
            let t31 = f64x8::splat(1.0) / t29 / v_rho;
            let t33 = v_sigma * t27;
            let t34 = v_rho * v_rho;
            let t36 = f64x8::splat(1.0) / t29 / t34;
            let t39 = t28 * t31 - t33 * t36 / f64x8::splat(8.0);
            let t40 = f64x8::splat(M_CBRT6);
            let t42 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t43 = (simd::cbrt(t42));
            let t44 = t43 * t43;
            let t45 = f64x8::splat(1.0) / t44;
            let t46 = t39 * t40 * t45;
            let t48 = t40 * t45;
            let t51 = param_c0 + f64x8::splat(5.0) / f64x8::splat(9.0) * param_c1 * t39 * t48;
            let t52 = param_c0 + param_c1 - f64x8::splat(1.0);
            let t56 = f64x8::splat(1.0) + f64x8::splat(5.0) / f64x8::splat(9.0) * t52 * t39 * t48;
            let t57 = f64x8::splat(1.0) / t56;
            let t59 = f64x8::splat(1.0) - t26;
            let t60 = t51 * t57 * t59;
            let t63 = t26 + f64x8::splat(5.0) / f64x8::splat(9.0) * t46 * t60;
            let t64 = f64x8::splat(5.0) / f64x8::splat(9.0) * t46;
            let t65 = f64x8::splat(1.0) - t64;
            let t66 = ((t65).simd_ge(V_ZERO).select(V_ONE, V_ZERO));
            let t68 = f64x8::splat(1.0) - param_alphainf;
            let t69 = t68 * t65;
            let t70 = f64x8::splat(1.0) + t64;
            let t71 = f64x8::splat(1.0) / t70;
            let t73 = t69 * t71 + f64x8::splat(1.0);
            let t74 = -t65;
            let t75 = ((t74).simd_ge(V_ZERO).select(V_ONE, V_ZERO));
            let t77 = t63 * t66 + t73 * t75;
            let t81 = ((t3).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t7 * t20 * t77));
            let tzk0 = f64x8::splat(2.0) * t81;
            acc_zk = tzk0;
            let t83 = t18 / t29;
            let t89 = t34 * v_rho;
            let t91 = f64x8::splat(1.0) / t29 / t89;
            let t94 = -f64x8::splat(5.0) / f64x8::splat(3.0) * t28 * t36 + t33 * t91 / f64x8::splat(3.0);
            let t96 = t94 * t40 * t45;
            let t99 = t40 * t40;
            let t100 = t39 * t99;
            let t102 = f64x8::splat(1.0) / t43 / t42;
            let t103 = t100 * t102;
            let t105 = t57 * t59;
            let t106 = param_c1 * t94 * t105;
            let t109 = t102 * t51;
            let t110 = t100 * t109;
            let t111 = t56 * t56;
            let t112 = f64x8::splat(1.0) / t111;
            let t113 = t112 * t59;
            let t115 = t113 * t52 * t94;
            let t118 = f64x8::splat(5.0) / f64x8::splat(9.0) * t96 * t60 + f64x8::splat(25.0) / f64x8::splat(81.0) * t103 * t106 - f64x8::splat(25.0) / f64x8::splat(81.0) * t110 * t115;
            let t120 = f64x8::splat(0.0);
            let t121 = t63 * t120;
            let t125 = t48 * t71;
            let t127 = t70 * t70;
            let t128 = f64x8::splat(1.0) / t127;
            let t129 = t69 * t128;
            let t132 = -f64x8::splat(5.0) / f64x8::splat(9.0) * t68 * t94 * t125 - f64x8::splat(5.0) / f64x8::splat(9.0) * t129 * t96;
            let t134 = t73 * t120;
            let t137 = t118 * t66 - f64x8::splat(5.0) / f64x8::splat(9.0) * t121 * t96 + t132 * t75 + f64x8::splat(5.0) / f64x8::splat(9.0) * t134 * t96;
            let t142 = ((t3).select(f64x8::splat(0.0), -t7 * t83 * t77 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t7 * t20 * t137));
            let tvrho0 = f64x8::splat(2.0) * v_rho * t142 + f64x8::splat(2.0) * t81;
            acc_vrho = tvrho0;
            let t145 = t27 * t36;
            let t148 = t45 * t51 * t105;
            let t149 = t145 * t40 * t148;
            let t151 = t102 * param_c1;
            let t152 = t100 * t151;
            let t154 = t152 * t145 * t105;
            let t156 = t52 * t27;
            let t159 = t110 * t113 * t156 * t36;
            let t161 = -f64x8::splat(5.0) / f64x8::splat(72.0) * t149 - f64x8::splat(25.0) / f64x8::splat(648.0) * t154 + f64x8::splat(25.0) / f64x8::splat(648.0) * t159;
            let t163 = t121 * t27;
            let t165 = t36 * t40 * t45;
            let t166 = t163 * t165;
            let t168 = t68 * t27;
            let t169 = t168 * t36;
            let t170 = t169 * t125;
            let t172 = t129 * t145 * t48;
            let t174 = f64x8::splat(5.0) / f64x8::splat(72.0) * t170 + f64x8::splat(5.0) / f64x8::splat(72.0) * t172;
            let t176 = t134 * t27;
            let t177 = t176 * t165;
            let t179 = t161 * t66 + f64x8::splat(5.0) / f64x8::splat(72.0) * t166 + t174 * t75 - f64x8::splat(5.0) / f64x8::splat(72.0) * t177;
            let t183 = ((t3).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t7 * t20 * t179));
            let tvsigma0 = f64x8::splat(2.0) * v_rho * t183;
            acc_vsigma = tvsigma0;
            let tvlapl0 = f64x8::splat(0.0);
            acc_vlapl = tvlapl0;
            let t185 = t27 * t31;
            let t196 = f64x8::splat(5.0) / f64x8::splat(9.0) * t185 * t40 * t148 + f64x8::splat(25.0) / f64x8::splat(81.0) * t152 * t185 * t105 - f64x8::splat(25.0) / f64x8::splat(81.0) * t110 * t113 * t156 * t31;
            let t199 = t31 * t40 * t45;
            let t202 = t168 * t31;
            let t207 = -f64x8::splat(5.0) / f64x8::splat(9.0) * t129 * t185 * t48 - f64x8::splat(5.0) / f64x8::splat(9.0) * t202 * t125;
            let t211 = t196 * t66 - f64x8::splat(5.0) / f64x8::splat(9.0) * t163 * t199 + t207 * t75 + f64x8::splat(5.0) / f64x8::splat(9.0) * t176 * t199;
            let t215 = ((t3).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t7 * t20 * t211));
            let tvtau0 = f64x8::splat(2.0) * v_rho * t215;
            acc_vtau = tvtau0;
        }
        store_add(zk, ip, m, acc_zk);
        store_add(vrho, ip, m, acc_vrho);
        store_add(vsigma, ip, m, acc_vsigma);
        store_add(vlapl, ip, m, acc_vlapl);
        store_add(vtau, ip, m, acc_vtau);
        ip += 8;
    }
}
