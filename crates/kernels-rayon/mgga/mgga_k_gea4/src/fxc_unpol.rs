//! MGGA_K_GEA4 fxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_k_gea4.c`
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
pub fn mgga_k_gea4_fxc_unpol(
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
            let t4 = f64x8::splat(M_CBRT3);
            let t5 = t4 * t4;
            let t6 = f64x8::splat(M_CBRTPI);
            let t8 = t5 * t6 * f64x8::splat(M_PI);
            let t9 = (f64x8::splat(1.0)).simd_le(zeta_threshold);
            let t10 = zeta_threshold - f64x8::splat(1.0);
            let t12 = ((t9).select(t10, (t9).select(-t10, f64x8::splat(0.0))));
            let t13 = f64x8::splat(1.0) + t12;
            let t15 = (simd::cbrt(zeta_threshold));
            let t16 = t15 * t15;
            let t18 = (simd::cbrt(t13));
            let t19 = t18 * t18;
            let t21 = (((t13).simd_le(zeta_threshold)).select(t16 * zeta_threshold, t19 * t13));
            let t22 = (simd::cbrt(v_rho));
            let t23 = t22 * t22;
            let t24 = t21 * t23;
            let t25 = f64x8::splat(M_CBRT6);
            let t26 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t27 = (simd::cbrt(t26));
            let t28 = t27 * t27;
            let t30 = t25 / t28;
            let t31 = f64x8::splat(M_CBRT2);
            let t32 = t31 * t31;
            let t33 = v_sigma * t32;
            let t34 = v_rho * v_rho;
            let t36 = f64x8::splat(1.0) / t23 / t34;
            let t40 = v_lapl * t32;
            let t42 = f64x8::splat(1.0) / t23 / v_rho;
            let t46 = t25 * t25;
            let t48 = f64x8::splat(1.0) / t27 / t26;
            let t49 = t46 * t48;
            let t50 = v_lapl * v_lapl;
            let t51 = t50 * t31;
            let t52 = t34 * v_rho;
            let t54 = f64x8::splat(1.0) / t22 / t52;
            let t58 = t49 * v_sigma;
            let t59 = t34 * t34;
            let t61 = f64x8::splat(1.0) / t22 / t59;
            let t63 = t31 * t61 * v_lapl;
            let t66 = v_sigma * v_sigma;
            let t67 = t66 * t31;
            let t68 = t59 * v_rho;
            let t70 = f64x8::splat(1.0) / t22 / t68;
            let t74 = f64x8::splat(1.0) + f64x8::splat(5.0) / f64x8::splat(648.0) * t30 * t33 * t36 + f64x8::splat(5.0) / f64x8::splat(54.0) * t30 * t40 * t42 + t49 * t51 * t54 / f64x8::splat(2916.0) - t58 * t63 / f64x8::splat(2592.0) + t49 * t67 * t70 / f64x8::splat(8748.0);
            let t78 = ((t3).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t8 * t24 * t74));
            let tzk0 = f64x8::splat(2.0) * t78;
            acc_zk = tzk0;
            let t80 = t21 / t22;
            let t85 = f64x8::splat(1.0) / t23 / t52;
            let t96 = t31 * t70 * v_lapl;
            let t99 = t59 * t34;
            let t101 = f64x8::splat(1.0) / t22 / t99;
            let t105 = -f64x8::splat(5.0) / f64x8::splat(243.0) * t30 * t33 * t85 - f64x8::splat(25.0) / f64x8::splat(162.0) * t30 * t40 * t36 - f64x8::splat(5.0) / f64x8::splat(4374.0) * t49 * t51 * t61 + f64x8::splat(13.0) / f64x8::splat(7776.0) * t58 * t96 - f64x8::splat(4.0) / f64x8::splat(6561.0) * t49 * t67 * t101;
            let t110 = ((t3).select(f64x8::splat(0.0), t8 * t80 * t74 / f64x8::splat(10.0) + f64x8::splat(3.0) / f64x8::splat(20.0) * t8 * t24 * t105));
            let tvrho0 = f64x8::splat(2.0) * v_rho * t110 + f64x8::splat(2.0) * t78;
            acc_vrho = tvrho0;
            let t114 = t30 * t32 * t36;
            let t116 = t49 * t63;
            let t118 = v_sigma * t31;
            let t120 = t49 * t118 * t70;
            let t122 = f64x8::splat(5.0) / f64x8::splat(648.0) * t114 - t116 / f64x8::splat(2592.0) + t120 / f64x8::splat(4374.0);
            let t126 = ((t3).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t8 * t24 * t122));
            let tvsigma0 = f64x8::splat(2.0) * v_rho * t126;
            acc_vsigma = tvsigma0;
            let t138 = f64x8::splat(5.0) / f64x8::splat(54.0) * t30 * t32 * t42 + t49 * v_lapl * t31 * t54 / f64x8::splat(1458.0) - t49 * t118 * t61 / f64x8::splat(2592.0);
            let t142 = ((t3).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t8 * t24 * t138));
            let tvlapl0 = f64x8::splat(2.0) * v_rho * t142;
            acc_vlapl = tvlapl0;
            let tvtau0 = f64x8::splat(0.0);
            acc_vtau = tvtau0;
            let t147 = t21 / t22 / v_rho;
            let t155 = f64x8::splat(1.0) / t23 / t59;
            let t166 = t31 * t101 * v_lapl;
            let t171 = f64x8::splat(1.0) / t22 / t59 / t52;
            let t175 = f64x8::splat(55.0) / f64x8::splat(729.0) * t30 * t33 * t155 + f64x8::splat(100.0) / f64x8::splat(243.0) * t30 * t40 * t85 + f64x8::splat(65.0) / f64x8::splat(13122.0) * t49 * t51 * t70 - f64x8::splat(13.0) / f64x8::splat(1458.0) * t58 * t166 + f64x8::splat(76.0) / f64x8::splat(19683.0) * t49 * t67 * t171;
            let t180 = ((t3).select(f64x8::splat(0.0), -t8 * t147 * t74 / f64x8::splat(30.0) + t8 * t80 * t105 / f64x8::splat(5.0) + f64x8::splat(3.0) / f64x8::splat(20.0) * t8 * t24 * t175));
            let tv2rho20 = f64x8::splat(2.0) * v_rho * t180 + f64x8::splat(4.0) * t110;
            acc_v2rho2 = tv2rho20;
            let t187 = t30 * t32 * t85;
            let t189 = t49 * t96;
            let t192 = t49 * t118 * t101;
            let t194 = -f64x8::splat(5.0) / f64x8::splat(243.0) * t187 + f64x8::splat(13.0) / f64x8::splat(7776.0) * t189 - f64x8::splat(8.0) / f64x8::splat(6561.0) * t192;
            let t199 = ((t3).select(f64x8::splat(0.0), t8 * t80 * t122 / f64x8::splat(10.0) + f64x8::splat(3.0) / f64x8::splat(20.0) * t8 * t24 * t194));
            let tv2rhosigma0 = f64x8::splat(2.0) * v_rho * t199 + f64x8::splat(2.0) * t126;
            acc_v2rhosigma = tv2rhosigma0;
            let t208 = -f64x8::splat(25.0) / f64x8::splat(162.0) * t114 - f64x8::splat(5.0) / f64x8::splat(2187.0) * t116 + f64x8::splat(13.0) / f64x8::splat(7776.0) * t120;
            let t213 = ((t3).select(f64x8::splat(0.0), t8 * t80 * t138 / f64x8::splat(10.0) + f64x8::splat(3.0) / f64x8::splat(20.0) * t8 * t24 * t208));
            let tv2rholapl0 = f64x8::splat(2.0) * v_rho * t213 + f64x8::splat(2.0) * t142;
            acc_v2rholapl = tv2rholapl0;
            let tv2rhotau0 = f64x8::splat(0.0);
            acc_v2rhotau = tv2rhotau0;
            let t216 = t8 * t21;
            let t218 = t48 * t31;
            let t220 = t216 * t155 * t46 * t218;
            let t222 = ((t3).select(f64x8::splat(0.0), t220 / f64x8::splat(29160.0)));
            let tv2sigma20 = f64x8::splat(2.0) * v_rho * t222;
            acc_v2sigma2 = tv2sigma20;
            let t226 = t216 * t85 * t46 * t218;
            let t228 = ((t3).select(f64x8::splat(0.0), -t226 / f64x8::splat(17280.0)));
            let tv2sigmalapl0 = f64x8::splat(2.0) * v_rho * t228;
            acc_v2sigmalapl = tv2sigmalapl0;
            let tv2sigmatau0 = f64x8::splat(0.0);
            acc_v2sigmatau = tv2sigmatau0;
            let t234 = ((t3).select(f64x8::splat(0.0), t216 * t36 * t46 * t218 / f64x8::splat(9720.0)));
            let tv2lapl20 = f64x8::splat(2.0) * v_rho * t234;
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
