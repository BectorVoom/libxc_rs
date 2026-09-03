//! LDA_X_REL vxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_x_rel.c`
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
pub fn lda_x_rel_vxc_unpol(
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
        let v_rho = load(rho, ip, np);
        let mut acc_zk = V_ZERO;
        let mut acc_vrho = V_ZERO;
        {
            let t2 = (v_rho / f64x8::splat(2.0)).simd_le(dens_threshold);
            let t3 = f64x8::splat(M_CBRT3);
            let t4 = f64x8::splat(M_CBRTPI);
            let t6 = t3 / t4;
            let t8 = (simd::cbrt(zeta_threshold));
            let t10 = (((f64x8::splat(1.0)).simd_le(zeta_threshold)).select(t8 * zeta_threshold, f64x8::splat(1.0)));
            let t11 = (simd::cbrt(v_rho));
            let t15 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t10 * t11));
            let t16 = (simd::cbrt(f64x8::splat(9.0)));
            let t17 = t16 * t16;
            let t18 = t17 * t3;
            let t19 = f64x8::splat(1.0) / f64x8::splat(M_PI);
            let t20 = (simd::cbrt(t19));
            let t21 = t20 * t20;
            let t22 = f64x8::splat(1.0) / t21;
            let t23 = t11 * t11;
            let t27 = f64x8::splat(1.0) + f64x8::splat(3.8075239991386495e-05) * t18 * t22 * t23;
            let t28 = ((t27).sqrt());
            let t29 = t28 * t17;
            let t30 = t3 * t20;
            let t35 = t3 * t3;
            let t36 = t16 * t35;
            let t37 = f64x8::splat(1.0) / t20;
            let t41 = (simd::ln(f64x8::splat(0.0035625477770544352) * t36 * t37 * t11 + ((((f64x8::splat(0.0035625477770544352) * t36 * t37 * t11) * (f64x8::splat(0.0035625477770544352) * t36 * t37 * t11)) + f64x8::splat(1.0)).sqrt())));
            let t42 = t41 * t16;
            let t43 = t35 * t21;
            let t44 = f64x8::splat(1.0) / t23;
            let t48 = f64x8::splat(10.396221848752237) * t29 * t30 / t11 - f64x8::splat(972.7328585562606) * t42 * t43 * t44;
            let t49 = t48 * t48;
            let t51 = f64x8::splat(1.0) - f64x8::splat(1.5) * t49;
            let tzk0 = f64x8::splat(2.0) * t15 * t51;
            acc_zk = tzk0;
            let t56 = ((t2).select(f64x8::splat(0.0), -t6 * t10 * t44 / f64x8::splat(8.0)));
            let t57 = v_rho * t56;
            let t60 = v_rho * t15;
            let t61 = f64x8::splat(1.0) / t28;
            let t62 = t61 * t16;
            let t63 = t35 * t37;
            let t68 = f64x8::splat(1.0) / t11 / v_rho;
            let t69 = t30 * t68;
            let t72 = t61 * t17;
            let t76 = f64x8::splat(1.0) / t23 / v_rho;
            let t80 = f64x8::splat(0.0011875159256848119) * t62 * t63 * t44 - f64x8::splat(3.4654072829174125) * t29 * t69 - f64x8::splat(3.4654072829174125) * t72 * t69 + f64x8::splat(648.4885723708404) * t42 * t43 * t76;
            let t81 = t48 * t80;
            let tvrho0 = tzk0 + f64x8::splat(2.0) * t57 * t51 - f64x8::splat(6.0) * t60 * t81;
            acc_vrho = tvrho0;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_vrho.into(); vrho[ip..ip + m].copy_from_slice(&a[..m]); }
        ip += 8;
    }
}
