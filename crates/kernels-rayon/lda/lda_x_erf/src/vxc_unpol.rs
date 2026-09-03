//! LDA_X_ERF vxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_x_erf.c`
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
pub fn lda_x_erf_vxc_unpol(
    rho: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
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
        let v_rho = load(rho, ip, np);
        let mut acc_zk = V_ZERO;
        let mut acc_vrho = V_ZERO;
        {
            let t1 = f64x8::splat(M_CBRT3);
            let t3 = (simd::cbrt(f64x8::splat(1.0) / f64x8::splat(M_PI)));
            let t5 = f64x8::splat(M_CBRT4);
            let t6 = t5 * t5;
            let t7 = t1 * t3 * t6;
            let t8 = f64x8::splat(M_CBRT2);
            let t9 = t8 * t8;
            let t10 = (f64x8::splat(1.0)).simd_le(zeta_threshold);
            let t11 = (simd::cbrt(zeta_threshold));
            let t13 = ((t10).select(t11 * zeta_threshold, f64x8::splat(1.0)));
            let t14 = t9 * t13;
            let t15 = (simd::cbrt(v_rho));
            let t16 = (simd::cbrt(f64x8::splat(9.0)));
            let t17 = t16 * t16;
            let t18 = t3 * t3;
            let t20 = t17 * t18 * param_hyb_omega_0;
            let t23 = ((t10).select(t11, f64x8::splat(1.0)));
            let t24 = f64x8::splat(1.0) / t23;
            let t27 = t20 * t1 / t15 * t24 / f64x8::splat(18.0);
            let t28 = (f64x8::splat(1.35)).simd_le(t27);
            let t29 = (f64x8::splat(1.35)).simd_lt(t27);
            let t30 = ((t29).select(t27, f64x8::splat(1.35)));
            let t31 = t30 * t30;
            let t34 = t31 * t31;
            let t35 = f64x8::splat(1.0) / t34;
            let t37 = t34 * t31;
            let t38 = f64x8::splat(1.0) / t37;
            let t40 = t34 * t34;
            let t41 = f64x8::splat(1.0) / t40;
            let t44 = f64x8::splat(1.0) / t40 / t31;
            let t47 = f64x8::splat(1.0) / t40 / t34;
            let t50 = f64x8::splat(1.0) / t40 / t37;
            let t52 = t40 * t40;
            let t53 = f64x8::splat(1.0) / t52;
            let t56 = ((t29).select(f64x8::splat(1.35), t27));
            let t57 = ((f64x8::splat(M_PI)).sqrt());
            let t58 = f64x8::splat(1.0) / t56;
            let t60 = (simd::erf(t58 / f64x8::splat(2.0)));
            let t62 = t56 * t56;
            let t63 = f64x8::splat(1.0) / t62;
            let t65 = (simd::exp(-t63 / f64x8::splat(4.0)));
            let t66 = t65 - f64x8::splat(1.0);
            let t69 = t65 - f64x8::splat(3.0) / f64x8::splat(2.0) - f64x8::splat(2.0) * t62 * t66;
            let t72 = f64x8::splat(2.0) * t56 * t69 + t57 * t60;
            let t76 = ((t28).select(f64x8::splat(1.0) / t31 / f64x8::splat(36.0) - t35 / f64x8::splat(960.0) + t38 / f64x8::splat(26880.0) - t41 / f64x8::splat(829440.0) + t44 / f64x8::splat(28385280.0) - t47 / f64x8::splat(1073479680.0) + t50 / f64x8::splat(44590694400.0) - t53 / f64x8::splat(2021444812800.0), f64x8::splat(1.0) - f64x8::splat(8.0) / f64x8::splat(3.0) * t56 * t72));
            let t79 = t7 * t14 * t15 * t76;
            let tzk0 = -f64x8::splat(3.0) / f64x8::splat(16.0) * t79;
            acc_zk = tzk0;
            let t82 = t15 * v_rho;
            let t84 = t82 * t1 * t3;
            let t85 = t6 * t9;
            let t86 = t31 * t30;
            let t87 = f64x8::splat(1.0) / t86;
            let t92 = t20 * t1 / t82 * t24 / f64x8::splat(54.0);
            let t93 = ((t29).select(-t92, f64x8::splat(0.0)));
            let t96 = t34 * t30;
            let t97 = f64x8::splat(1.0) / t96;
            let t100 = t34 * t86;
            let t101 = f64x8::splat(1.0) / t100;
            let t105 = f64x8::splat(1.0) / t40 / t30;
            let t109 = f64x8::splat(1.0) / t40 / t86;
            let t113 = f64x8::splat(1.0) / t40 / t96;
            let t117 = f64x8::splat(1.0) / t40 / t100;
            let t121 = f64x8::splat(1.0) / t52 / t30;
            let t125 = ((t29).select(f64x8::splat(0.0), -t92));
            let t127 = t65 * t63;
            let t131 = t62 * t56;
            let t132 = f64x8::splat(1.0) / t131;
            let t136 = t56 * t66;
            let t141 = t132 * t125 * t65 / f64x8::splat(2.0) - f64x8::splat(4.0) * t136 * t125 - t58 * t125 * t65;
            let t144 = -t127 * t125 + f64x8::splat(2.0) * t125 * t69 + f64x8::splat(2.0) * t56 * t141;
            let t148 = ((t28).select(-t87 * t93 / f64x8::splat(18.0) + t97 * t93 / f64x8::splat(240.0) - t101 * t93 / f64x8::splat(4480.0) + t105 * t93 / f64x8::splat(103680.0) - t109 * t93 / f64x8::splat(2838528.0) + t113 * t93 / f64x8::splat(89456640.0) - t117 * t93 / f64x8::splat(3185049600.0) + t121 * t93 / f64x8::splat(126340300800.0), -f64x8::splat(8.0) / f64x8::splat(3.0) * t125 * t72 - f64x8::splat(8.0) / f64x8::splat(3.0) * t56 * t144));
            let tvrho0 = -t79 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(16.0) * t84 * t85 * t13 * t148;
            acc_vrho = tvrho0;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_vrho.into(); vrho[ip..ip + m].copy_from_slice(&a[..m]); }
        ip += 8;
    }
}
