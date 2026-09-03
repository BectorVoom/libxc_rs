//! MGGA_X_MBEEF exc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_mbeef.c`
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

#[allow(unused_variables, non_snake_case)]
pub fn mgga_x_mbeef_exc_unpol(
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
        let v_rho = load(rho, ip, np);
        let v_sigma = load(sigma, ip, np);
        let v_lapl = load(lapl, ip, np);
        let v_tau = load(tau, ip, np);
        let mut acc_zk = V_ZERO;
        {
            let t3 = (v_rho / f64x8::splat(2.0)).simd_le(dens_threshold);
            let t4 = f64x8::splat(M_CBRT3);
            let t5 = f64x8::splat(M_CBRTPI);
            let t7 = t4 / t5;
            let t8 = (f64x8::splat(1.0)).simd_le(zeta_threshold);
            let t9 = zeta_threshold - f64x8::splat(1.0);
            let t11 = ((t8).select(t9, (t8).select(-t9, f64x8::splat(0.0))));
            let t12 = t11 + f64x8::splat(1.0);
            let t14 = (simd::cbrt(zeta_threshold));
            let t16 = (simd::cbrt(t12));
            let t18 = (((t12).simd_le(zeta_threshold)).select(t14 * zeta_threshold, t16 * t12));
            let t19 = (simd::cbrt(v_rho));
            let t20 = t18 * t19;
            let t21 = f64x8::splat(M_CBRT6);
            let t22 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t23 = (simd::cbrt(t22));
            let t24 = t23 * t23;
            let t25 = f64x8::splat(1.0) / t24;
            let t26 = t21 * t25;
            let t27 = t26 * v_sigma;
            let t28 = f64x8::splat(M_CBRT2);
            let t29 = t28 * t28;
            let t30 = v_rho * v_rho;
            let t31 = t19 * t19;
            let t33 = f64x8::splat(1.0) / t31 / t30;
            let t34 = t29 * t33;
            let t35 = v_sigma * t29;
            let t36 = t35 * t33;
            let t39 = f64x8::splat(6.5124) + t26 * t36 / f64x8::splat(24.0);
            let t40 = f64x8::splat(1.0) / t39;
            let t41 = t34 * t40;
            let t42 = t27 * t41;
            let t44 = v_tau * t29;
            let t46 = f64x8::splat(1.0) / t31 / v_rho;
            let t52 = f64x8::splat(5.0) / f64x8::splat(9.0) * (t44 * t46 - t36 / f64x8::splat(8.0)) * t21 * t25;
            let t53 = (f64x8::splat(10000.0)).simd_le(t52);
            let t54 = (f64x8::splat(10000.0)).simd_lt(t52);
            let t55 = ((t54).select(t52, f64x8::splat(10000.0)));
            let t56 = t55 * t55;
            let t59 = t56 * t55;
            let t60 = f64x8::splat(1.0) / t59;
            let t61 = t56 * t56;
            let t62 = f64x8::splat(1.0) / t61;
            let t65 = ((t54).select(f64x8::splat(10000.0), t52));
            let t66 = t65 * t65;
            let t67 = f64x8::splat(1.0) - t66;
            let t68 = t67 * t67;
            let t69 = t68 * t67;
            let t70 = t66 * t65;
            let t71 = f64x8::splat(1.0) + t70;
            let t73 = t70 * t71 + f64x8::splat(1.0);
            let t74 = f64x8::splat(1.0) / t73;
            let t76 = ((t53).select(f64x8::splat(1.0) - f64x8::splat(3.0) / t56 - t60 + f64x8::splat(3.0) * t62, -t69 * t74));
            let t77 = t76 * t76;
            let t78 = t77 * t76;
            let t79 = t77 * t77;
            let t80 = t79 * t78;
            let t83 = t42 / f64x8::splat(12.0) - f64x8::splat(1.0);
            let t84 = t83 * t83;
            let t85 = t84 * t83;
            let t87 = t84 * t84;
            let t88 = t87 * t84;
            let t91 = t87 * t85;
            let t93 = t87 * t83;
            let t97 = f64x8::splat(429.0) / f64x8::splat(16.0) * t91 - f64x8::splat(693.0) / f64x8::splat(16.0) * t93 + f64x8::splat(315.0) / f64x8::splat(16.0) * t85 - f64x8::splat(35.0) / f64x8::splat(192.0) * t42 + f64x8::splat(35.0) / f64x8::splat(16.0);
            let t99 = t79 * t76;
            let t103 = f64x8::splat(429.0) / f64x8::splat(16.0) * t80 - f64x8::splat(693.0) / f64x8::splat(16.0) * t99 + f64x8::splat(315.0) / f64x8::splat(16.0) * t78 - f64x8::splat(35.0) / f64x8::splat(16.0) * t76;
            let t106 = t79 * t77;
            let t110 = -f64x8::splat(5.0) / f64x8::splat(16.0) + f64x8::splat(231.0) / f64x8::splat(16.0) * t106 - f64x8::splat(315.0) / f64x8::splat(16.0) * t79 + f64x8::splat(105.0) / f64x8::splat(16.0) * t77;
            let t116 = f64x8::splat(63.0) / f64x8::splat(8.0) * t99 - f64x8::splat(35.0) / f64x8::splat(4.0) * t78 + f64x8::splat(15.0) / f64x8::splat(8.0) * t76;
            let t121 = f64x8::splat(3.0) / f64x8::splat(8.0) + f64x8::splat(35.0) / f64x8::splat(8.0) * t79 - f64x8::splat(15.0) / f64x8::splat(4.0) * t77;
            let t126 = f64x8::splat(5.0) / f64x8::splat(2.0) * t78 - f64x8::splat(3.0) / f64x8::splat(2.0) * t76;
            let t130 = -f64x8::splat(1.0) / f64x8::splat(2.0) + f64x8::splat(3.0) / f64x8::splat(2.0) * t77;
            let t133 = t97 * t76;
            let t139 = f64x8::splat(63.0) / f64x8::splat(8.0) * t93 - f64x8::splat(35.0) / f64x8::splat(4.0) * t85 + f64x8::splat(5.0) / f64x8::splat(32.0) * t42 - f64x8::splat(15.0) / f64x8::splat(8.0);
            let t146 = -f64x8::splat(0.013022208355989584) * t42 + f64x8::splat(1.9735677658125e-05) * t80 + f64x8::splat(0.497944638409375) * t85 + f64x8::splat(0.080024660533125) * t88 - f64x8::splat(0.004373652639371875) * t76 + f64x8::splat(8.88525527e-09) * t97 * t103 - f64x8::splat(7.74224962e-09) * t97 * t110 - f64x8::splat(3.38128188e-08) * t97 * t116 + f64x8::splat(5.54588743e-08) * t97 * t121 + f64x8::splat(5.05920757e-08) * t97 * t126 - f64x8::splat(2.7652468e-07) * t97 * t130 + f64x8::splat(0.00940675747) * t133 - f64x8::splat(0.138056183978125) * t87 - f64x8::splat(1.38472194e-08) * t139 * t110 - f64x8::splat(3.76702959e-08) * t139 * t116 + f64x8::splat(1.62238741e-07) * t139 * t121;
            let t151 = t139 * t76;
            let t155 = f64x8::splat(3.0) / f64x8::splat(8.0) + f64x8::splat(35.0) / f64x8::splat(8.0) * t87 - f64x8::splat(15.0) / f64x8::splat(4.0) * t84;
            let t168 = t155 * t76;
            let t172 = f64x8::splat(5.0) / f64x8::splat(2.0) * t85 - t42 / f64x8::splat(8.0) + f64x8::splat(3.0) / f64x8::splat(2.0);
            let t185 = -f64x8::splat(0.00896771404) * t139 * t126 - f64x8::splat(0.0188495102) * t139 * t130 - f64x8::splat(0.00884148272) * t151 - f64x8::splat(4.93824365e-09) * t155 * t103 + f64x8::splat(9.12223751e-09) * t155 * t110 + f64x8::splat(2.09603871e-08) * t155 * t116 - f64x8::splat(7.90811707e-08) * t155 * t121 + f64x8::splat(0.00631891628) * t155 * t126 - f64x8::splat(0.0182911291) * t155 * t130 + f64x8::splat(0.0162638575) * t168 + f64x8::splat(6.74910119e-09) * t172 * t103 - f64x8::splat(2.16860568e-08) * t172 * t110 + f64x8::splat(0.000896739466) * t172 * t116 + f64x8::splat(0.00339308972) * t172 * t121 - f64x8::splat(0.00845508103) * t172 * t126 + f64x8::splat(0.0280678872) * t172 * t130;
            let t187 = t172 * t76;
            let t190 = -f64x8::splat(1.0) / f64x8::splat(2.0) + f64x8::splat(3.0) / f64x8::splat(2.0) * t84;
            let t206 = t190 * t76;
            let t218 = -f64x8::splat(0.0182177954) * t187 - f64x8::splat(2.23014657e-09) * t190 * t103 - f64x8::splat(0.395061199588125) * t93 - f64x8::splat(0.000945883103563125) * t99 + f64x8::splat(0.004646102821846875) * t78 + f64x8::splat(6.68980219e-09) * t190 * t110 - f64x8::splat(0.00035104103) * t190 * t116 + f64x8::splat(0.00182906057) * t190 * t121 + f64x8::splat(0.00293253041) * t190 * t126 - f64x8::splat(0.0150103636) * t190 * t130 - f64x8::splat(0.043464346) * t206 - f64x8::splat(9.40351563e-06) * t83 * t103 - f64x8::splat(5.14204676e-05) * t83 * t110 + f64x8::splat(0.000822139896) * t83 * t116 + f64x8::splat(0.00119130546) * t83 * t121 - f64x8::splat(0.00303347141) * t83 * t126;
            let t221 = t83 * t76;
            let t226 = -f64x8::splat(5.0) / f64x8::splat(16.0) + f64x8::splat(231.0) / f64x8::splat(16.0) * t88 - f64x8::splat(315.0) / f64x8::splat(16.0) * t87 + f64x8::splat(105.0) / f64x8::splat(16.0) * t84;
            let t239 = t226 * t76;
            let t248 = f64x8::splat(1.3805672252189969) - f64x8::splat(0.00879090772) * t83 * t130 + f64x8::splat(0.100339208) * t221 - f64x8::splat(6.91592964e-09) * t226 * t103 + f64x8::splat(6.94482484e-09) * t226 * t110 + f64x8::splat(2.36391411e-08) * t226 * t116 - f64x8::splat(4.16393106e-08) * t226 * t121 - f64x8::splat(2.65114646e-08) * t226 * t126 + f64x8::splat(1.69805915e-07) * t226 * t130 - f64x8::splat(0.00957417512) * t239 + f64x8::splat(8.50272392e-09) * t139 * t103 + f64x8::splat(0.106025815520625) * t91 - f64x8::splat(8.0008813355625e-05) * t106 + f64x8::splat(0.003020715669803125) * t79 + f64x8::splat(0.007031826877565625) * t77 - f64x8::splat(0.092294814328125) * t84;
            let t250 = t146 + t185 + t218 + t248;
            let t254 = ((t3).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t7 * t20 * t250));
            let tzk0 = f64x8::splat(2.0) * t254;
            acc_zk = tzk0;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        ip += 8;
    }
}
