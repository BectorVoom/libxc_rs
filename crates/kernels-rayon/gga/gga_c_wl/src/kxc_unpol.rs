//! GGA_C_WL kxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_wl.c`
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
pub fn gga_c_wl_kxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    v2rho2: &mut [f64],
    v2rhosigma: &mut [f64],
    v2sigma2: &mut [f64],
    v3rho3: &mut [f64],
    v3rho2sigma: &mut [f64],
    v3rhosigma2: &mut [f64],
    v3sigma3: &mut [f64],
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
        let mut acc_v2rho2 = V_ZERO;
        let mut acc_v2rhosigma = V_ZERO;
        let mut acc_v2sigma2 = V_ZERO;
        let mut acc_v3rho3 = V_ZERO;
        let mut acc_v3rho2sigma = V_ZERO;
        let mut acc_v3rhosigma2 = V_ZERO;
        let mut acc_v3sigma3 = V_ZERO;
        {
            let t1 = ((v_sigma).sqrt());
            let t2 = (simd::cbrt(v_rho));
            let t4 = f64x8::splat(1.0) / t2 / v_rho;
            let t5 = t1 * t4;
            let t7 = -f64x8::splat(0.7486) + f64x8::splat(0.06001) * t5;
            let t8 = f64x8::splat(M_CBRT2);
            let t9 = t1 * t8;
            let t12 = f64x8::splat(M_CBRT3);
            let t14 = (simd::cbrt(f64x8::splat(1.0) / f64x8::splat(M_PI)));
            let t15 = t12 * t14;
            let t16 = f64x8::splat(M_CBRT4);
            let t17 = t16 * t16;
            let t18 = f64x8::splat(1.0) / t2;
            let t22 = f64x8::splat(3.60073) + f64x8::splat(1.8) * t9 * t4 + t15 * t17 * t18 / f64x8::splat(4.0);
            let t23 = f64x8::splat(1.0) / t22;
            let tzk0 = t7 * t23;
            acc_zk = tzk0;
            let t26 = v_rho * t7;
            let t27 = t22 * t22;
            let t28 = f64x8::splat(1.0) / t27;
            let t29 = v_rho * v_rho;
            let t31 = f64x8::splat(1.0) / t2 / t29;
            let t37 = -f64x8::splat(2.4) * t9 * t31 - t15 * t17 * t4 / f64x8::splat(12.0);
            let t38 = t28 * t37;
            let tvrho0 = tzk0 - f64x8::splat(0.08001333333333334) * t5 * t23 - t26 * t38;
            acc_vrho = tvrho0;
            let t40 = f64x8::splat(1.0) / t1;
            let t41 = t18 * t40;
            let t44 = t18 * t7;
            let t46 = t28 * t40 * t8;
            let tvsigma0 = f64x8::splat(0.030005) * t41 * t23 - f64x8::splat(0.9) * t44 * t46;
            acc_vsigma = tvsigma0;
            let t49 = t1 * t31;
            let t52 = t7 * t28;
            let t58 = f64x8::splat(1.0) / t27 / t22;
            let t59 = t37 * t37;
            let t60 = t58 * t59;
            let t63 = t29 * v_rho;
            let t65 = f64x8::splat(1.0) / t2 / t63;
            let t71 = f64x8::splat(5.6) * t9 * t65 + t15 * t17 * t31 / f64x8::splat(9.0);
            let t72 = t28 * t71;
            let tv2rho20 = f64x8::splat(0.02667111111111111) * t49 * t23 - f64x8::splat(2.0) * t52 * t37 + f64x8::splat(0.16002666666666668) * t5 * t38 + f64x8::splat(2.0) * t26 * t60 - t26 * t72;
            acc_v2rho2 = tv2rho20;
            let t74 = t4 * t40;
            let t79 = t4 * t7;
            let t82 = t2 * t2;
            let t84 = f64x8::splat(1.0) / t82 / t29;
            let t88 = t44 * t58;
            let t89 = t40 * t8;
            let t90 = t89 * t37;
            let tv2rhosigma0 = -f64x8::splat(0.010001666666666667) * t74 * t23 - f64x8::splat(0.030005) * t41 * t38 + f64x8::splat(0.3) * t79 * t46 + f64x8::splat(0.072012) * t84 * t28 * t8 + f64x8::splat(1.8) * t88 * t90;
            acc_v2rhosigma = tv2rhosigma0;
            let t94 = f64x8::splat(1.0) / t1 / v_sigma;
            let t95 = t18 * t94;
            let t99 = f64x8::splat(1.0) / t82 / v_rho;
            let t100 = f64x8::splat(1.0) / v_sigma;
            let t101 = t99 * t100;
            let t102 = t28 * t8;
            let t105 = t99 * t7;
            let t107 = t8 * t8;
            let t108 = t58 * t100 * t107;
            let t112 = t28 * t94 * t8;
            let tv2sigma20 = -f64x8::splat(0.0150025) * t95 * t23 - f64x8::splat(0.054009) * t101 * t102 + f64x8::splat(1.62) * t105 * t108 + f64x8::splat(0.45) * t44 * t112;
            acc_v2sigma2 = tv2sigma20;
            let t115 = t1 * t65;
            let t120 = t7 * t58;
            let t129 = t27 * t27;
            let t130 = f64x8::splat(1.0) / t129;
            let t131 = t59 * t37;
            let t132 = t130 * t131;
            let t135 = t58 * t37;
            let t136 = t135 * t71;
            let t139 = t29 * t29;
            let t141 = f64x8::splat(1.0) / t2 / t139;
            let t147 = -f64x8::splat(18.666666666666668) * t9 * t141 - f64x8::splat(7.0) / f64x8::splat(27.0) * t15 * t17 * t65;
            let t148 = t28 * t147;
            let tv3rho30 = -f64x8::splat(0.06223259259259259) * t115 * t23 - f64x8::splat(0.08001333333333334) * t49 * t38 + f64x8::splat(6.0) * t120 * t59 - f64x8::splat(3.0) * t52 * t71 - f64x8::splat(0.48008) * t5 * t60 + f64x8::splat(0.24004) * t5 * t72 - f64x8::splat(6.0) * t26 * t132 + f64x8::splat(6.0) * t26 * t136 - t26 * t148;
            acc_v3rho3 = tv3rho30;
            let t150 = t31 * t40;
            let t159 = t31 * t7;
            let t163 = f64x8::splat(1.0) / t82 / t63;
            let t167 = t79 * t58;
            let t170 = t84 * t58;
            let t171 = t8 * t37;
            let t174 = t44 * t130;
            let t175 = t89 * t59;
            let t178 = t89 * t71;
            let tv3rho2sigma0 = f64x8::splat(0.013335555555555555) * t150 * t23 + f64x8::splat(0.020003333333333335) * t74 * t38 + f64x8::splat(0.06001) * t41 * t60 - f64x8::splat(0.030005) * t41 * t72 - f64x8::splat(0.4) * t159 * t46 - f64x8::splat(0.216036) * t163 * t28 * t8 - f64x8::splat(1.2) * t167 * t90 - f64x8::splat(0.288048) * t170 * t171 - f64x8::splat(5.4) * t174 * t175 + f64x8::splat(1.8) * t88 * t178;
            acc_v3rho2sigma = tv3rho2sigma0;
            let t181 = t4 * t94;
            let t186 = t84 * t100;
            let t189 = t58 * t8;
            let t190 = t189 * t37;
            let t193 = t84 * t7;
            let t196 = f64x8::splat(1.0) / t139;
            let t197 = t196 * t40;
            let t198 = t58 * t107;
            let t201 = t105 * t130;
            let t202 = t100 * t107;
            let t203 = t202 * t37;
            let t208 = t94 * t8;
            let t209 = t208 * t37;
            let tv3rhosigma20 = f64x8::splat(0.005000833333333334) * t181 * t23 + f64x8::splat(0.0150025) * t95 * t38 + f64x8::splat(0.054009) * t186 * t102 + f64x8::splat(0.108018) * t101 * t190 - f64x8::splat(2.7) * t193 * t108 - f64x8::splat(0.1296216) * t197 * t198 - f64x8::splat(4.86) * t201 * t203 - f64x8::splat(0.15) * t79 * t112 - f64x8::splat(0.9) * t88 * t209;
            acc_v3rhosigma2 = tv3rhosigma20;
            let t212 = v_sigma * v_sigma;
            let t214 = f64x8::splat(1.0) / t1 / t212;
            let t215 = t18 * t214;
            let t218 = f64x8::splat(1.0) / t212;
            let t219 = t99 * t218;
            let t222 = f64x8::splat(1.0) / t63;
            let t223 = t222 * t94;
            let t226 = t222 * t7;
            let t227 = t130 * t94;
            let t231 = t58 * t218 * t107;
            let t235 = t28 * t214 * t8;
            let tv3sigma30 = f64x8::splat(0.02250375) * t215 * t23 + f64x8::splat(0.0810135) * t219 * t102 + f64x8::splat(0.1458243) * t223 * t198 - f64x8::splat(8.748) * t226 * t227 - f64x8::splat(2.43) * t105 * t231 - f64x8::splat(0.675) * t44 * t235;
            acc_v3sigma3 = tv3sigma30;
        }
        store_add(zk, ip, m, acc_zk);
        store_add(vrho, ip, m, acc_vrho);
        store_add(vsigma, ip, m, acc_vsigma);
        store_add(v2rho2, ip, m, acc_v2rho2);
        store_add(v2rhosigma, ip, m, acc_v2rhosigma);
        store_add(v2sigma2, ip, m, acc_v2sigma2);
        store_add(v3rho3, ip, m, acc_v3rho3);
        store_add(v3rho2sigma, ip, m, acc_v3rho2sigma);
        store_add(v3rhosigma2, ip, m, acc_v3rhosigma2);
        store_add(v3sigma3, ip, m, acc_v3sigma3);
        ip += 8;
    }
}
