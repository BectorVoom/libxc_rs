//! MGGA_XC_ZLP fxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_xc_zlp.c`
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
pub fn mgga_xc_zlp_fxc_unpol(
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
            let t2 = f64x8::splat(M_CBRT3);
            let t4 = (simd::cbrt(f64x8::splat(1.0) / f64x8::splat(M_PI)));
            let t5 = t2 * t4;
            let t6 = f64x8::splat(M_CBRT4);
            let t7 = t6 * t6;
            let t10 = v_rho * v_rho;
            let t11 = (simd::cbrt(v_rho));
            let t12 = t11 * t11;
            let t14 = f64x8::splat(1.0) / t12 / t10;
            let t17 = f64x8::splat(1.0) / t12 / v_rho;
            let t24 = f64x8::splat(0.207108) * t5 * t7 + f64x8::splat(0.005387725) * t5 * t7 * (-v_lapl * t17 / f64x8::splat(8.0) + v_sigma * t14 / f64x8::splat(8.0));
            let t25 = f64x8::splat(1.0) / t11;
            let t27 = f64x8::splat(1.0) + f64x8::splat(488.4942506669168) * t25;
            let t28 = (simd::ln(t27));
            let t31 = f64x8::splat(1.0) - f64x8::splat(0.002047107) * t28 * t11;
            let t33 = t2 * t2;
            let t34 = t24 * t31 * t33;
            let t35 = f64x8::splat(1.0) / t4;
            let t36 = t35 * t6;
            let t37 = t36 * t11;
            let t38 = t34 * t37;
            let tzk0 = -t38 / f64x8::splat(3.0);
            acc_zk = tzk0;
            let t41 = t11 * v_rho;
            let t42 = t10 * v_rho;
            let t44 = f64x8::splat(1.0) / t12 / t42;
            let t49 = -v_sigma * t44 / f64x8::splat(3.0) + f64x8::splat(5.0) / f64x8::splat(24.0) * v_lapl * t14;
            let t50 = t41 * t49;
            let t53 = t41 * t24;
            let t55 = f64x8::splat(1.0) / t27;
            let t58 = f64x8::splat(1.0) / t12;
            let t61 = f64x8::splat(0.3333333333333333) / v_rho * t55 - f64x8::splat(0.000682369) * t28 * t58;
            let t64 = t33 * t35 * t6;
            let tvrho0 = -f64x8::splat(4.0) / f64x8::splat(9.0) * t38 - f64x8::splat(0.0215509) * t50 * t31 - t53 * t61 * t64 / f64x8::splat(3.0);
            acc_vrho = tvrho0;
            let t67 = f64x8::splat(1.0) / t41;
            let t68 = t67 * t31;
            let tvsigma0 = -f64x8::splat(0.0026938625) * t68;
            acc_vsigma = tvsigma0;
            let tvlapl0 = f64x8::splat(0.0026938625) * t25 * t31;
            acc_vlapl = tvlapl0;
            let tvtau0 = f64x8::splat(0.0);
            acc_vtau = tvtau0;
            let t71 = t49 * t31;
            let t75 = t24 * t61 * t33;
            let t78 = t36 * t58;
            let t81 = t10 * t10;
            let t83 = f64x8::splat(1.0) / t12 / t81;
            let t88 = f64x8::splat(11.0) / f64x8::splat(9.0) * v_sigma * t83 - f64x8::splat(5.0) / f64x8::splat(9.0) * v_lapl * t44;
            let t89 = t41 * t88;
            let t98 = f64x8::splat(1.0) / t11 / t10;
            let t99 = t27 * t27;
            let t100 = f64x8::splat(1.0) / t99;
            let t105 = -f64x8::splat(0.2222222222222222) / t10 * t55 + f64x8::splat(54.277138962990755) * t98 * t100 + f64x8::splat(0.00045491266666666667) * t28 * t17;
            let tv2rho20 = -f64x8::splat(0.057469066666666666) * t71 * t11 - f64x8::splat(8.0) / f64x8::splat(9.0) * t75 * t37 - f64x8::splat(4.0) / f64x8::splat(27.0) * t34 * t78 - f64x8::splat(0.0215509) * t89 * t31 - f64x8::splat(0.0431018) * t50 * t61 - t53 * t105 * t64 / f64x8::splat(3.0);
            acc_v2rho2 = tv2rho20;
            let t109 = t98 * t31;
            let t111 = t67 * t61;
            let tv2rhosigma0 = f64x8::splat(0.0035918166666666666) * t109 - f64x8::splat(0.0026938625) * t111;
            acc_v2rhosigma = tv2rhosigma0;
            let tv2rholapl0 = -f64x8::splat(0.0008979541666666666) * t68 + f64x8::splat(0.0026938625) * t25 * t61;
            acc_v2rholapl = tv2rholapl0;
            let tv2rhotau0 = f64x8::splat(0.0);
            acc_v2rhotau = tv2rhotau0;
            let tv2sigma20 = f64x8::splat(0.0);
            acc_v2sigma2 = tv2sigma20;
            let tv2sigmalapl0 = f64x8::splat(0.0);
            acc_v2sigmalapl = tv2sigmalapl0;
            let tv2sigmatau0 = f64x8::splat(0.0);
            acc_v2sigmatau = tv2sigmatau0;
            let tv2lapl20 = f64x8::splat(0.0);
            acc_v2lapl2 = tv2lapl20;
            let tv2lapltau0 = f64x8::splat(0.0);
            acc_v2lapltau = tv2lapltau0;
            let tv2tau20 = f64x8::splat(0.0);
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
