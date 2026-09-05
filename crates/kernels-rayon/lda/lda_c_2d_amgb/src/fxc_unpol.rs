//! LDA_C_2D_AMGB fxc unpol kernel — explicit SIMD (bit-exact).
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

#[allow(unused_variables, non_snake_case)]
pub fn lda_c_2d_amgb_fxc_unpol(
    rho: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    v2rho2: &mut [f64],
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
        let mut acc_v2rho2 = V_ZERO;
        {
            let t1 = ((v_rho).sqrt());
            let t2 = f64x8::splat(1.0) / t1;
            let t4 = f64x8::splat(1.0) / v_rho;
            let t7 = f64x8::splat(1.0) / t1 / v_rho;
            let t9 = f64x8::splat(0.04869723403850762) * t2 + f64x8::splat(0.018219548589342285) * t4 + f64x8::splat(0.000603947002028882) * t7;
            let t11 = ((f64x8::splat(M_PI)).sqrt());
            let t12 = f64x8::splat(1.0) / t11;
            let t13 = t12 * t2;
            let t14 = ((t13) * (t13).sqrt());
            let t18 = f64x8::splat(0.5654308006315614) * t2 - f64x8::splat(0.02069) * t14 + f64x8::splat(0.10821581200590331) * t4 + f64x8::splat(0.00313738702352666) * t7;
            let t20 = f64x8::splat(1.0) + f64x8::splat(1.0) / t18;
            let t21 = (simd::ln(t20));
            let t22 = t9 * t21;
            let t24 = (simd::exp(-f64x8::splat(0.7552241765370266) * t2));
            let t26 = f64x8::splat(M_SQRT2);
            let t27 = (t24 - f64x8::splat(1.0)) * t26;
            let t30 = ((zeta_threshold).sqrt());
            let t32 = (((f64x8::splat(1.0)).simd_le(zeta_threshold)).select(t30 * zeta_threshold, f64x8::splat(1.0)));
            let t33 = t32 - f64x8::splat(1.0);
            let t36 = f64x8::splat(4.0) / f64x8::splat(3.0) * t27 * t12 * t1 * t33;
            let tzk0 = -f64x8::splat(0.1925) + t22 - t36;
            acc_zk = tzk0;
            let t38 = v_rho * v_rho;
            let t39 = f64x8::splat(1.0) / t38;
            let t42 = f64x8::splat(1.0) / t1 / t38;
            let t44 = -f64x8::splat(0.02434861701925381) * t7 - f64x8::splat(0.018219548589342285) * t39 - f64x8::splat(0.000905920503043323) * t42;
            let t45 = t44 * t21;
            let t46 = t18 * t18;
            let t47 = f64x8::splat(1.0) / t46;
            let t48 = t9 * t47;
            let t50 = ((t13).sqrt());
            let t51 = t50 * t12;
            let t56 = -f64x8::splat(0.2827154003157807) * t7 + f64x8::splat(0.0155175) * t51 * t7 - f64x8::splat(0.10821581200590331) * t39 - f64x8::splat(0.00470608053528999) * t42;
            let t57 = f64x8::splat(1.0) / t20;
            let t58 = t56 * t57;
            let t59 = t48 * t58;
            let t61 = t26 * t33;
            let t62 = t4 * t24 * t61;
            let t65 = t27 * t13 * t33;
            let tvrho0 = -f64x8::splat(0.1925) + t22 - t36 + v_rho * (t45 - t59 - f64x8::splat(0.2840597424304148) * t62 - f64x8::splat(2.0) / f64x8::splat(3.0) * t65);
            acc_vrho = tvrho0;
            let t74 = t38 * v_rho;
            let t75 = f64x8::splat(1.0) / t74;
            let t78 = f64x8::splat(1.0) / t1 / t74;
            let t80 = f64x8::splat(0.036522925528880715) * t42 + f64x8::splat(0.03643909717868457) * t75 + f64x8::splat(0.0022648012576083074) * t78;
            let t81 = t80 * t21;
            let t82 = t44 * t47;
            let t83 = t82 * t58;
            let t85 = t46 * t18;
            let t86 = f64x8::splat(1.0) / t85;
            let t87 = t9 * t86;
            let t88 = t56 * t56;
            let t89 = t88 * t57;
            let t90 = t87 * t89;
            let t93 = f64x8::splat(1.0)/((t13).sqrt());
            let t95 = t93 / f64x8::splat(M_PI);
            let t102 = f64x8::splat(0.424073100473671) * t42 - f64x8::splat(0.003879375) * t95 * t75 - f64x8::splat(0.02327625) * t51 * t42 + f64x8::splat(0.21643162401180663) * t75 + f64x8::splat(0.011765201338224974) * t78;
            let t103 = t102 * t57;
            let t104 = t48 * t103;
            let t105 = t46 * t46;
            let t106 = f64x8::splat(1.0) / t105;
            let t107 = t9 * t106;
            let t108 = t20 * t20;
            let t109 = f64x8::splat(1.0) / t108;
            let t110 = t88 * t109;
            let t111 = t107 * t110;
            let t113 = t39 * t24 * t61;
            let t116 = t42 * t24 * t61;
            let t120 = t27 * t12 * t7 * t33;
            let tv2rho20 = f64x8::splat(2.0) * t45 - f64x8::splat(2.0) * t59 - f64x8::splat(0.5681194848608296) * t62 - f64x8::splat(4.0) / f64x8::splat(3.0) * t65 + v_rho * (t81 - f64x8::splat(2.0) * t83 + f64x8::splat(2.0) * t90 - t104 - t111 + f64x8::splat(0.1420298712152074) * t113 - f64x8::splat(0.10726439253216494) * t116 + t120 / f64x8::splat(3.0));
            acc_v2rho2 = tv2rho20;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_vrho.into(); vrho[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v2rho2.into(); v2rho2[ip..ip + m].copy_from_slice(&a[..m]); }
        ip += 8;
    }
}
