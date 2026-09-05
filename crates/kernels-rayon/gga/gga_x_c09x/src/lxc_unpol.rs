//! GGA_X_C09X lxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_c09x.c`
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
pub fn gga_x_c09x_lxc_unpol(
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
    v4rho4: &mut [f64],
    v4rho3sigma: &mut [f64],
    v4rho2sigma2: &mut [f64],
    v4rhosigma3: &mut [f64],
    v4sigma4: &mut [f64],
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
        let mut acc_v4rho4 = V_ZERO;
        let mut acc_v4rho3sigma = V_ZERO;
        let mut acc_v4rho2sigma2 = V_ZERO;
        let mut acc_v4rhosigma3 = V_ZERO;
        let mut acc_v4sigma4 = V_ZERO;
        {
            let t2 = (v_rho / f64x8::splat(2.0)).simd_le(dens_threshold);
            let t3 = f64x8::splat(M_CBRT3);
            let t4 = f64x8::splat(M_CBRTPI);
            let t6 = t3 / t4;
            let t7 = (f64x8::splat(1.0)).simd_le(zeta_threshold);
            let t8 = zeta_threshold - f64x8::splat(1.0);
            let t10 = ((t7).select(t8, (t7).select(-t8, f64x8::splat(0.0))));
            let t11 = f64x8::splat(1.0) + t10;
            let t13 = (simd::cbrt(zeta_threshold));
            let t15 = (simd::cbrt(t11));
            let t17 = (((t11).simd_le(zeta_threshold)).select(t13 * zeta_threshold, t15 * t11));
            let t18 = (simd::cbrt(v_rho));
            let t19 = t17 * t18;
            let t20 = f64x8::splat(M_CBRT6);
            let t21 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t22 = (simd::cbrt(t21));
            let t23 = t22 * t22;
            let t24 = f64x8::splat(1.0) / t23;
            let t25 = t20 * t24;
            let t26 = t25 * v_sigma;
            let t27 = f64x8::splat(M_CBRT2);
            let t28 = t27 * t27;
            let t29 = v_rho * v_rho;
            let t30 = t18 * t18;
            let t32 = f64x8::splat(1.0) / t30 / t29;
            let t33 = t28 * t32;
            let t34 = v_sigma * t28;
            let t36 = t25 * t34 * t32;
            let t38 = (simd::exp(-f64x8::splat(0.0020125) * t36));
            let t39 = t33 * t38;
            let t43 = (simd::exp(-f64x8::splat(0.00100625) * t36));
            let t45 = f64x8::splat(2.245) + f64x8::splat(0.0025708333333333334) * t26 * t39 - f64x8::splat(1.245) * t43;
            let t49 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t45));
            let tzk0 = f64x8::splat(2.0) * t49;
            acc_zk = tzk0;
            let t51 = t17 / t30;
            let t55 = t29 * v_rho;
            let t57 = f64x8::splat(1.0) / t30 / t55;
            let t58 = t28 * t57;
            let t59 = t58 * t38;
            let t62 = t20 * t20;
            let t64 = f64x8::splat(1.0) / t22 / t21;
            let t65 = t62 * t64;
            let t66 = v_sigma * v_sigma;
            let t67 = t65 * t66;
            let t68 = t29 * t29;
            let t69 = t68 * t29;
            let t71 = f64x8::splat(1.0) / t18 / t69;
            let t72 = t27 * t71;
            let t73 = t72 * t38;
            let t76 = t58 * t43;
            let t79 = -f64x8::splat(0.006855555555555556) * t26 * t59 + f64x8::splat(2.7593611111111112e-05) * t67 * t73 - f64x8::splat(0.00334075) * t26 * t76;
            let t84 = ((t2).select(f64x8::splat(0.0), -t6 * t51 * t45 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t79));
            let tvrho0 = f64x8::splat(2.0) * v_rho * t84 + f64x8::splat(2.0) * t49;
            acc_vrho = tvrho0;
            let t90 = t68 * v_rho;
            let t93 = t27 / t18 / t90;
            let t94 = t93 * t38;
            let t100 = f64x8::splat(0.0025708333333333334) * t25 * t39 - f64x8::splat(1.0347604166666667e-05) * t65 * v_sigma * t94 + f64x8::splat(0.00125278125) * t25 * t33 * t43;
            let t104 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t100));
            let tvsigma0 = f64x8::splat(2.0) * v_rho * t104;
            acc_vsigma = tvsigma0;
            let t109 = t17 / t30 / v_rho;
            let t118 = t28 / t30 / t68;
            let t119 = t118 * t38;
            let t122 = t68 * t55;
            let t124 = f64x8::splat(1.0) / t18 / t122;
            let t125 = t27 * t124;
            let t126 = t125 * t38;
            let t129 = t66 * v_sigma;
            let t130 = t68 * t68;
            let t131 = t130 * t29;
            let t132 = f64x8::splat(1.0) / t131;
            let t136 = t118 * t43;
            let t139 = t125 * t43;
            let t142 = f64x8::splat(0.025137037037037038) * t26 * t119 - f64x8::splat(0.0002483425) * t67 * t126 + f64x8::splat(1.824294361740067e-08) * t129 * t132 * t38 + f64x8::splat(0.012249416666666667) * t26 * t136 - f64x8::splat(1.792869166666667e-05) * t67 * t139;
            let t147 = ((t2).select(f64x8::splat(0.0), t6 * t109 * t45 / f64x8::splat(12.0) - t6 * t51 * t79 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t142));
            let tv2rho20 = f64x8::splat(2.0) * v_rho * t147 + f64x8::splat(4.0) * t84;
            acc_v2rho2 = tv2rho20;
            let t155 = t65 * t27;
            let t156 = t71 * v_sigma;
            let t160 = t130 * v_rho;
            let t161 = f64x8::splat(1.0) / t160;
            let t170 = -f64x8::splat(0.006855555555555556) * t25 * t59 + f64x8::splat(8.278083333333333e-05) * t155 * t156 * t38 - f64x8::splat(6.841103856525251e-09) * t66 * t161 * t38 - f64x8::splat(0.00334075) * t25 * t76 + f64x8::splat(6.723259375e-06) * t155 * t156 * t43;
            let t175 = ((t2).select(f64x8::splat(0.0), -t6 * t51 * t100 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t170));
            let tv2rhosigma0 = f64x8::splat(2.0) * v_rho * t175 + f64x8::splat(2.0) * t104;
            acc_v2rhosigma = tv2rhosigma0;
            let t180 = f64x8::splat(1.0) / t130;
            let t187 = -f64x8::splat(2.0695208333333333e-05) * t65 * t94 + f64x8::splat(2.565413946196969e-09) * v_sigma * t180 * t38 - f64x8::splat(2.521222265625e-06) * t65 * t93 * t43;
            let t191 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t187));
            let tv2sigma20 = f64x8::splat(2.0) * v_rho * t191;
            acc_v2sigma2 = tv2sigma20;
            let t194 = t17 * t32;
            let t206 = t28 / t30 / t90;
            let t207 = t206 * t38;
            let t211 = f64x8::splat(1.0) / t18 / t130;
            let t212 = t27 * t211;
            let t216 = t130 * t55;
            let t217 = f64x8::splat(1.0) / t216;
            let t218 = t129 * t217;
            let t221 = t66 * t66;
            let t222 = t130 * t90;
            let t224 = f64x8::splat(1.0) / t30 / t222;
            let t227 = t24 * t28;
            let t228 = t227 * t38;
            let t231 = t206 * t43;
            let t239 = -f64x8::splat(0.11730617283950617) * t26 * t207 + f64x8::splat(0.0020909825308641976) * t67 * t212 * t38 - f64x8::splat(3.4661592873061273e-07) * t218 * t38 + f64x8::splat(9.79037974133836e-11) * t221 * t224 * t20 * t228 - f64x8::splat(0.05716394444444445) * t26 * t231 + f64x8::splat(0.00019721560833333332) * t67 * t212 * t43 - f64x8::splat(5.926591302090563e-09) * t218 * t43;
            let t244 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(5.0) / f64x8::splat(36.0) * t6 * t194 * t45 + t6 * t109 * t79 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t51 * t142 - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t239));
            let tv3rho30 = f64x8::splat(2.0) * v_rho * t244 + f64x8::splat(6.0) * t147;
            acc_v3rho3 = tv3rho30;
            let t256 = t124 * v_sigma;
            let t260 = t132 * t66;
            let t263 = t130 * t68;
            let t265 = f64x8::splat(1.0) / t30 / t263;
            let t277 = f64x8::splat(0.025137037037037038) * t25 * t119 - f64x8::splat(0.0005978615740740741) * t155 * t256 * t38 + f64x8::splat(1.1629876556092927e-07) * t260 * t38 - f64x8::splat(3.671392403001885e-11) * t129 * t265 * t20 * t228 + f64x8::splat(0.012249416666666667) * t25 * t136 - f64x8::splat(6.0509334375e-05) * t155 * t256 * t43 + f64x8::splat(2.222471738283961e-09) * t260 * t43;
            let t282 = ((t2).select(f64x8::splat(0.0), t6 * t109 * t100 / f64x8::splat(12.0) - t6 * t51 * t170 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t277));
            let tv3rho2sigma0 = f64x8::splat(2.0) * v_rho * t282 + f64x8::splat(4.0) * t175;
            acc_v3rho2sigma = tv3rho2sigma0;
            let t290 = t161 * v_sigma;
            let t294 = f64x8::splat(1.0) / t30 / t216;
            let t304 = f64x8::splat(0.00011037444444444445) * t65 * t73 - f64x8::splat(3.420551928262626e-08) * t290 * t38 + f64x8::splat(1.3767721511257068e-11) * t66 * t294 * t20 * t228 + f64x8::splat(1.344651875e-05) * t65 * t72 * t43 - f64x8::splat(8.334269018564854e-10) * t290 * t43;
            let t309 = ((t2).select(f64x8::splat(0.0), -t6 * t51 * t187 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t304));
            let tv3rhosigma20 = f64x8::splat(2.0) * v_rho * t309 + f64x8::splat(2.0) * t191;
            acc_v3rhosigma2 = tv3rhosigma20;
            let t315 = f64x8::splat(1.0) / t30 / t131;
            let t322 = f64x8::splat(7.696241838590908e-09) * t180 * t38 - f64x8::splat(5.1628955667214e-12) * v_sigma * t315 * t20 * t228 + f64x8::splat(3.12535088196182e-10) * t180 * t43;
            let t326 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t322));
            let tv3sigma30 = f64x8::splat(2.0) * v_rho * t326;
            acc_v3sigma3 = tv3sigma30;
            let t344 = t28 / t30 / t69;
            let t350 = t27 / t18 / t160;
            let t355 = t129 / t263;
            let t358 = t130 * t69;
            let t362 = t221 / t30 / t358 * t20;
            let t366 = t130 * t130;
            let t373 = t64 * t27 * t38;
            let t384 = t227 * t43;
            let t392 = ((t2).select(f64x8::splat(0.0), f64x8::splat(10.0) / f64x8::splat(27.0) * t6 * t17 * t57 * t45 - f64x8::splat(5.0) / f64x8::splat(9.0) * t6 * t194 * t79 + t6 * t109 * t142 / f64x8::splat(2.0) - t6 * t51 * t239 / f64x8::splat(2.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * (f64x8::splat(0.6647349794238683) * t26 * t344 * t38 - f64x8::splat(0.018683940679012346) * t67 * t350 * t38 + f64x8::splat(5.1951849434886575e-06) * t355 * t38 - f64x8::splat(3.198190715503864e-09) * t362 * t228 + f64x8::splat(1.0508340922369839e-12) * t221 * v_sigma / t18 / t366 / v_rho * t62 * t373 + f64x8::splat(0.3239290185185185) * t26 * t344 * t43 - f64x8::splat(0.0019502432379629629) * t67 * t350 * t43 + f64x8::splat(1.303850086459924e-07) * t355 * t43 - f64x8::splat(1.590301999394301e-11) * t362 * t384)));
            let tv4rho40 = f64x8::splat(2.0) * v_rho * t392 + f64x8::splat(8.0) * t244;
            acc_v4rho4 = tv4rho40;
            let t407 = t211 * v_sigma;
            let t411 = t217 * t66;
            let t415 = t224 * t129 * t20;
            let t438 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(5.0) / f64x8::splat(36.0) * t6 * t194 * t100 + t6 * t109 * t170 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t51 * t277 - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * (-f64x8::splat(0.11730617283950617) * t25 * t207 + f64x8::splat(0.004654122407407407) * t155 * t407 * t38 - f64x8::splat(1.5582514339863073e-06) * t411 * t38 + f64x8::splat(1.0891797462238925e-09) * t415 * t228 - f64x8::splat(3.94062784588869e-13) * t221 / t18 / t366 * t62 * t373 - f64x8::splat(0.05716394444444445) * t25 * t231 + f64x8::splat(0.0005094736548611111) * t155 * t407 * t43 - f64x8::splat(4.222696302739526e-08) * t411 * t43 + f64x8::splat(5.963632497728629e-12) * t415 * t384)));
            let tv4rho3sigma0 = f64x8::splat(2.0) * v_rho * t438 + f64x8::splat(6.0) * t282;
            acc_v4rho3sigma = tv4rho3sigma0;
            let t450 = t132 * v_sigma;
            let t454 = t265 * t66 * t20;
            let t475 = ((t2).select(f64x8::splat(0.0), t6 * t109 * t187 / f64x8::splat(12.0) - t6 * t51 * t304 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * (-f64x8::splat(0.0006990381481481482) * t65 * t126 + f64x8::splat(3.8082144801323897e-07) * t450 * t38 - f64x8::splat(3.441930377814267e-10) * t454 * t228 + f64x8::splat(1.4777354422082585e-13) * t129 / t18 / t130 / t122 * t62 * t373 - f64x8::splat(8.516128541666667e-05) * t65 * t139 + f64x8::splat(1.194578559327629e-08) * t450 * t43 - f64x8::splat(2.236362186648236e-12) * t454 * t384)));
            let tv4rho2sigma20 = f64x8::splat(2.0) * v_rho * t475 + f64x8::splat(4.0) * t309;
            acc_v4rho2sigma2 = tv4rho2sigma20;
            let t484 = t294 * t20 * t24;
            let t504 = ((t2).select(f64x8::splat(0.0), -t6 * t51 * t322 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * (-f64x8::splat(6.156993470872726e-08) * t161 * t38 + f64x8::splat(9.637405057879947e-11) * t484 * t34 * t38 - f64x8::splat(5.5415079082809697e-14) * t66 / t18 / t358 * t62 * t373 - f64x8::splat(2.500280705569456e-09) * t161 * t43 + f64x8::splat(8.386358199930884e-13) * t484 * t34 * t43)));
            let tv4rhosigma30 = f64x8::splat(2.0) * v_rho * t504 + f64x8::splat(2.0) * t326;
            acc_v4rhosigma3 = tv4rhosigma30;
            let t507 = t315 * t20;
            let t522 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * (-f64x8::splat(2.06515822668856e-11) * t507 * t228 + f64x8::splat(2.0780654656053638e-14) * v_sigma / t18 / t222 * t62 * t373 - f64x8::splat(3.1448843249740816e-13) * t507 * t384)));
            let tv4sigma40 = f64x8::splat(2.0) * v_rho * t522;
            acc_v4sigma4 = tv4sigma40;
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
        store_add(v4rho4, ip, m, acc_v4rho4);
        store_add(v4rho3sigma, ip, m, acc_v4rho3sigma);
        store_add(v4rho2sigma2, ip, m, acc_v4rho2sigma2);
        store_add(v4rhosigma3, ip, m, acc_v4rhosigma3);
        store_add(v4sigma4, ip, m, acc_v4sigma4);
        ip += 8;
    }
}
