//! MGGA_X_TH fxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_th.c`
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
pub fn mgga_x_th_fxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    vlapl: &mut [f64],
    vtau: &mut [f64],
    v2rho2: &mut [f64],
    v2rhosigma: &mut [f64],
    v2rholapl: &mut [f64],
    v2rhotau: &mut [f64],
    v2sigma2: &mut [f64],
    v2sigmalapl: &mut [f64],
    v2sigmatau: &mut [f64],
    v2lapl2: &mut [f64],
    v2lapltau: &mut [f64],
    v2tau2: &mut [f64],
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
        let mut acc_vrho = V_ZERO;
        let mut acc_vsigma = V_ZERO;
        let mut acc_vlapl = V_ZERO;
        let mut acc_vtau = V_ZERO;
        let mut acc_v2rho2 = V_ZERO;
        let mut acc_v2rhosigma = V_ZERO;
        let mut acc_v2rholapl = V_ZERO;
        let mut acc_v2rhotau = V_ZERO;
        let mut acc_v2sigma2 = V_ZERO;
        let mut acc_v2sigmalapl = V_ZERO;
        let mut acc_v2sigmatau = V_ZERO;
        let mut acc_v2lapl2 = V_ZERO;
        let mut acc_v2lapltau = V_ZERO;
        let mut acc_v2tau2 = V_ZERO;
        {
            let t3 = (v_rho / f64x8::splat(2.0)).simd_le(dens_threshold);
            let t4 = f64x8::splat(M_CBRTPI);
            let t5 = t4 * t4;
            let t6 = (f64x8::splat(1.0)).simd_le(zeta_threshold);
            let t7 = zeta_threshold - f64x8::splat(1.0);
            let t9 = ((t6).select(t7, (t6).select(-t7, f64x8::splat(0.0))));
            let t10 = f64x8::splat(1.0) + t9;
            let t12 = (simd::cbrt(zeta_threshold));
            let t14 = (simd::cbrt(t10));
            let t16 = (((t10).simd_le(zeta_threshold)).select(t12 * zeta_threshold, t14 * t10));
            let t17 = t5 * t16;
            let t18 = v_rho * v_rho;
            let t19 = f64x8::splat(1.0) / v_tau;
            let t22 = f64x8::splat(M_CBRT2);
            let t23 = f64x8::splat(1.0) / v_rho;
            let t30 = (simd::cbrt(f64x8::splat(1.0) / f64x8::splat(M_PI)));
            let t31 = f64x8::splat(1.0) / t30;
            let t32 = f64x8::splat(M_CBRT4);
            let t33 = t31 * t32;
            let t34 = t22 * (f64x8::splat(1.0) + f64x8::splat(7.0) / f64x8::splat(216.0) * v_sigma * t23 * t19) * t33;
            let t37 = ((t3).select(f64x8::splat(0.0), -f64x8::splat(27.0) / f64x8::splat(160.0) * t17 * t18 * t19 * t34));
            let tzk0 = f64x8::splat(2.0) * t37;
            acc_zk = tzk0;
            let t42 = v_tau * v_tau;
            let t43 = f64x8::splat(1.0) / t42;
            let t44 = t17 * t43;
            let t46 = t22 * v_sigma * t33;
            let t50 = ((t3).select(f64x8::splat(0.0), -f64x8::splat(27.0) / f64x8::splat(80.0) * t17 * v_rho * t19 * t34 + f64x8::splat(7.0) / f64x8::splat(1280.0) * t44 * t46));
            let tvrho0 = f64x8::splat(2.0) * v_rho * t50 + f64x8::splat(2.0) * t37;
            acc_vrho = tvrho0;
            let t53 = t17 * v_rho;
            let t58 = ((t3).select(f64x8::splat(0.0), -f64x8::splat(7.0) / f64x8::splat(1280.0) * t53 * t43 * t22 * t33));
            let tvsigma0 = f64x8::splat(2.0) * v_rho * t58;
            acc_vsigma = tvsigma0;
            let tvlapl0 = f64x8::splat(0.0);
            acc_vlapl = tvlapl0;
            let t65 = f64x8::splat(1.0) / t42 / v_tau;
            let t67 = t17 * v_rho * t65;
            let t71 = ((t3).select(f64x8::splat(0.0), f64x8::splat(27.0) / f64x8::splat(160.0) * t17 * t18 * t43 * t34 + f64x8::splat(7.0) / f64x8::splat(1280.0) * t67 * t46));
            let tvtau0 = f64x8::splat(2.0) * v_rho * t71;
            acc_vtau = tvtau0;
            let t82 = ((t3).select(f64x8::splat(0.0), -f64x8::splat(27.0) / f64x8::splat(80.0) * t17 * t19 * t34 + f64x8::splat(7.0) / f64x8::splat(640.0) * t17 * t23 * t43 * t46));
            let tv2rho20 = f64x8::splat(2.0) * v_rho * t82 + f64x8::splat(4.0) * t50;
            acc_v2rho2 = tv2rho20;
            let t86 = t22 * t31 * t32;
            let t89 = ((t3).select(f64x8::splat(0.0), -f64x8::splat(7.0) / f64x8::splat(1280.0) * t44 * t86));
            let tv2rhosigma0 = f64x8::splat(2.0) * v_rho * t89 + f64x8::splat(2.0) * t58;
            acc_v2rhosigma = tv2rhosigma0;
            let tv2rholapl0 = f64x8::splat(0.0);
            acc_v2rholapl = tv2rholapl0;
            let t96 = ((t3).select(f64x8::splat(0.0), f64x8::splat(27.0) / f64x8::splat(80.0) * t17 * v_rho * t43 * t34));
            let tv2rhotau0 = f64x8::splat(2.0) * v_rho * t96 + f64x8::splat(2.0) * t71;
            acc_v2rhotau = tv2rhotau0;
            let tv2sigma20 = f64x8::splat(0.0);
            acc_v2sigma2 = tv2sigma20;
            let tv2sigmalapl0 = f64x8::splat(0.0);
            acc_v2sigmalapl = tv2sigmalapl0;
            let t103 = ((t3).select(f64x8::splat(0.0), f64x8::splat(7.0) / f64x8::splat(640.0) * t53 * t65 * t22 * t33));
            let tv2sigmatau0 = f64x8::splat(2.0) * v_rho * t103;
            acc_v2sigmatau = tv2sigmatau0;
            let tv2lapl20 = f64x8::splat(0.0);
            acc_v2lapl2 = tv2lapl20;
            let tv2lapltau0 = f64x8::splat(0.0);
            acc_v2lapltau = tv2lapltau0;
            let t109 = t42 * t42;
            let t110 = f64x8::splat(1.0) / t109;
            let t112 = t17 * v_rho * t110;
            let t116 = ((t3).select(f64x8::splat(0.0), -f64x8::splat(27.0) / f64x8::splat(80.0) * t17 * t18 * t65 * t34 - f64x8::splat(7.0) / f64x8::splat(320.0) * t112 * t46));
            let tv2tau20 = f64x8::splat(2.0) * v_rho * t116;
            acc_v2tau2 = tv2tau20;
        }
        store_add(zk, ip, m, acc_zk);
        store_add(vrho, ip, m, acc_vrho);
        store_add(vsigma, ip, m, acc_vsigma);
        store_add(vlapl, ip, m, acc_vlapl);
        store_add(vtau, ip, m, acc_vtau);
        store_add(v2rho2, ip, m, acc_v2rho2);
        store_add(v2rhosigma, ip, m, acc_v2rhosigma);
        store_add(v2rholapl, ip, m, acc_v2rholapl);
        store_add(v2rhotau, ip, m, acc_v2rhotau);
        store_add(v2sigma2, ip, m, acc_v2sigma2);
        store_add(v2sigmalapl, ip, m, acc_v2sigmalapl);
        store_add(v2sigmatau, ip, m, acc_v2sigmatau);
        store_add(v2lapl2, ip, m, acc_v2lapl2);
        store_add(v2lapltau, ip, m, acc_v2lapltau);
        store_add(v2tau2, ip, m, acc_v2tau2);
        ip += 8;
    }
}
