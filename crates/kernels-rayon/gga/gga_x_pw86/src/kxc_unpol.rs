//! GGA_X_PW86 kxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_pw86.c`
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
pub fn gga_x_pw86_kxc_unpol(
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
    param_aa: f64,
    param_bb: f64,
    param_cc: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_aa = f64x8::splat(param_aa);
    let param_bb = f64x8::splat(param_bb);
    let param_cc = f64x8::splat(param_cc);
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
            let t21 = param_aa * t20;
            let t22 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t23 = (simd::cbrt(t22));
            let t24 = t23 * t23;
            let t25 = f64x8::splat(1.0) / t24;
            let t26 = t21 * t25;
            let t27 = f64x8::splat(M_CBRT2);
            let t28 = t27 * t27;
            let t29 = v_sigma * t28;
            let t30 = v_rho * v_rho;
            let t31 = t18 * t18;
            let t33 = f64x8::splat(1.0) / t31 / t30;
            let t37 = t20 * t20;
            let t38 = param_bb * t37;
            let t40 = f64x8::splat(1.0) / t23 / t22;
            let t41 = t38 * t40;
            let t42 = v_sigma * v_sigma;
            let t43 = t42 * t27;
            let t44 = t30 * t30;
            let t45 = t44 * v_rho;
            let t47 = f64x8::splat(1.0) / t18 / t45;
            let t51 = t22 * t22;
            let t53 = param_cc / t51;
            let t54 = t42 * v_sigma;
            let t55 = t44 * t44;
            let t56 = f64x8::splat(1.0) / t55;
            let t60 = f64x8::splat(1.0) + t26 * t29 * t33 / f64x8::splat(24.0) + t41 * t43 * t47 / f64x8::splat(288.0) + t53 * t54 * t56 / f64x8::splat(576.0);
            let t61 = (simd::pow(t60, f64x8::splat(1.0) / f64x8::splat(15.0)));
            let t65 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t61));
            let tzk0 = f64x8::splat(2.0) * t65;
            acc_zk = tzk0;
            let t66 = f64x8::splat(1.0) / t31;
            let t71 = t6 * t17;
            let t72 = t61 * t61;
            let t73 = t72 * t72;
            let t75 = t73 * t73;
            let t76 = t75 * t73 * t72;
            let t77 = f64x8::splat(1.0) / t76;
            let t78 = t18 * t77;
            let t79 = t30 * v_rho;
            let t81 = f64x8::splat(1.0) / t31 / t79;
            let t85 = t44 * t30;
            let t87 = f64x8::splat(1.0) / t18 / t85;
            let t91 = t55 * v_rho;
            let t92 = f64x8::splat(1.0) / t91;
            let t96 = -t26 * t29 * t81 / f64x8::splat(9.0) - t41 * t43 * t87 / f64x8::splat(54.0) - t53 * t54 * t92 / f64x8::splat(72.0);
            let t101 = ((t2).select(f64x8::splat(0.0), -t6 * t17 * t66 * t61 / f64x8::splat(8.0) - t71 * t78 * t96 / f64x8::splat(40.0)));
            let tvrho0 = f64x8::splat(2.0) * v_rho * t101 + f64x8::splat(2.0) * t65;
            acc_vrho = tvrho0;
            let t104 = t25 * t28;
            let t108 = v_sigma * t27;
            let t115 = t21 * t104 * t33 / f64x8::splat(24.0) + t41 * t108 * t47 / f64x8::splat(144.0) + t53 * t42 * t56 / f64x8::splat(192.0);
            let t119 = ((t2).select(f64x8::splat(0.0), -t71 * t78 * t115 / f64x8::splat(40.0)));
            let tvsigma0 = f64x8::splat(2.0) * v_rho * t119;
            acc_vsigma = tvsigma0;
            let t123 = f64x8::splat(1.0) / t31 / v_rho;
            let t128 = t66 * t77;
            let t133 = f64x8::splat(1.0) / t76 / t60;
            let t134 = t18 * t133;
            let t135 = t96 * t96;
            let t140 = f64x8::splat(1.0) / t31 / t44;
            let t144 = t44 * t79;
            let t146 = f64x8::splat(1.0) / t18 / t144;
            let t151 = f64x8::splat(1.0) / t55 / t30;
            let t155 = f64x8::splat(11.0) / f64x8::splat(27.0) * t26 * t29 * t140 + f64x8::splat(19.0) / f64x8::splat(162.0) * t41 * t43 * t146 + t53 * t54 * t151 / f64x8::splat(8.0);
            let t160 = ((t2).select(f64x8::splat(0.0), t6 * t17 * t123 * t61 / f64x8::splat(12.0) - t71 * t128 * t96 / f64x8::splat(60.0) + f64x8::splat(7.0) / f64x8::splat(300.0) * t71 * t134 * t135 - t71 * t78 * t155 / f64x8::splat(40.0)));
            let tv2rho20 = f64x8::splat(2.0) * v_rho * t160 + f64x8::splat(4.0) * t101;
            acc_v2rho2 = tv2rho20;
            let t166 = t115 * t96;
            let t179 = -t21 * t104 * t81 / f64x8::splat(9.0) - t41 * t108 * t87 / f64x8::splat(27.0) - t53 * t42 * t92 / f64x8::splat(24.0);
            let t184 = ((t2).select(f64x8::splat(0.0), -t71 * t128 * t115 / f64x8::splat(120.0) + f64x8::splat(7.0) / f64x8::splat(300.0) * t71 * t134 * t166 - t71 * t78 * t179 / f64x8::splat(40.0)));
            let tv2rhosigma0 = f64x8::splat(2.0) * v_rho * t184 + f64x8::splat(2.0) * t119;
            acc_v2rhosigma = tv2rhosigma0;
            let t187 = t115 * t115;
            let t191 = t40 * t27;
            let t198 = t38 * t191 * t47 / f64x8::splat(144.0) + t53 * v_sigma * t56 / f64x8::splat(96.0);
            let t203 = ((t2).select(f64x8::splat(0.0), f64x8::splat(7.0) / f64x8::splat(300.0) * t71 * t134 * t187 - t71 * t78 * t198 / f64x8::splat(40.0)));
            let tv2sigma20 = f64x8::splat(2.0) * v_rho * t203;
            acc_v2sigma2 = tv2sigma20;
            let t210 = t123 * t77;
            let t214 = t66 * t133;
            let t221 = t60 * t60;
            let t223 = f64x8::splat(1.0) / t76 / t221;
            let t224 = t18 * t223;
            let t225 = t135 * t96;
            let t229 = t96 * t155;
            let t234 = f64x8::splat(1.0) / t31 / t45;
            let t239 = f64x8::splat(1.0) / t18 / t55;
            let t244 = f64x8::splat(1.0) / t55 / t79;
            let t248 = -f64x8::splat(154.0) / f64x8::splat(81.0) * t26 * t29 * t234 - f64x8::splat(209.0) / f64x8::splat(243.0) * t41 * t43 * t239 - f64x8::splat(5.0) / f64x8::splat(4.0) * t53 * t54 * t244;
            let t253 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(5.0) / f64x8::splat(36.0) * t6 * t17 * t33 * t61 + t71 * t210 * t96 / f64x8::splat(60.0) + f64x8::splat(7.0) / f64x8::splat(300.0) * t71 * t214 * t135 - t71 * t128 * t155 / f64x8::splat(40.0) - f64x8::splat(203.0) / f64x8::splat(4500.0) * t71 * t224 * t225 + f64x8::splat(7.0) / f64x8::splat(100.0) * t71 * t134 * t229 - t71 * t78 * t248 / f64x8::splat(40.0)));
            let tv3rho30 = f64x8::splat(2.0) * v_rho * t253 + f64x8::splat(6.0) * t160;
            acc_v3rho3 = tv3rho30;
            let t266 = t115 * t135;
            let t270 = t179 * t96;
            let t274 = t115 * t155;
            let t287 = f64x8::splat(11.0) / f64x8::splat(27.0) * t21 * t104 * t140 + f64x8::splat(19.0) / f64x8::splat(81.0) * t41 * t108 * t146 + f64x8::splat(3.0) / f64x8::splat(8.0) * t53 * t42 * t151;
            let t292 = ((t2).select(f64x8::splat(0.0), t71 * t210 * t115 / f64x8::splat(180.0) + f64x8::splat(7.0) / f64x8::splat(450.0) * t71 * t214 * t166 - t71 * t128 * t179 / f64x8::splat(60.0) - f64x8::splat(203.0) / f64x8::splat(4500.0) * t71 * t224 * t266 + f64x8::splat(7.0) / f64x8::splat(150.0) * t71 * t134 * t270 + f64x8::splat(7.0) / f64x8::splat(300.0) * t71 * t134 * t274 - t71 * t78 * t287 / f64x8::splat(40.0)));
            let tv3rho2sigma0 = f64x8::splat(2.0) * v_rho * t292 + f64x8::splat(4.0) * t184;
            acc_v3rho2sigma = tv3rho2sigma0;
            let t298 = t187 * t96;
            let t302 = t115 * t179;
            let t309 = t198 * t96;
            let t319 = -t38 * t191 * t87 / f64x8::splat(27.0) - t53 * v_sigma * t92 / f64x8::splat(12.0);
            let t324 = ((t2).select(f64x8::splat(0.0), f64x8::splat(7.0) / f64x8::splat(900.0) * t71 * t214 * t187 - f64x8::splat(203.0) / f64x8::splat(4500.0) * t71 * t224 * t298 + f64x8::splat(7.0) / f64x8::splat(150.0) * t71 * t134 * t302 - t71 * t128 * t198 / f64x8::splat(120.0) + f64x8::splat(7.0) / f64x8::splat(300.0) * t71 * t134 * t309 - t71 * t78 * t319 / f64x8::splat(40.0)));
            let tv3rhosigma20 = f64x8::splat(2.0) * v_rho * t324 + f64x8::splat(2.0) * t203;
            acc_v3rhosigma2 = tv3rhosigma20;
            let t327 = t187 * t115;
            let t331 = t115 * t198;
            let t338 = t3 / t4 / t51 * t17;
            let t340 = f64x8::splat(1.0) / t31 / t144;
            let t346 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(203.0) / f64x8::splat(4500.0) * t71 * t224 * t327 + f64x8::splat(7.0) / f64x8::splat(100.0) * t71 * t134 * t331 - t338 * t340 * t77 * param_cc / f64x8::splat(3840.0)));
            let tv3sigma30 = f64x8::splat(2.0) * v_rho * t346;
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
