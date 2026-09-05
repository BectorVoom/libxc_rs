//! GGA_X_CHACHIYO fxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_chachiyo.c`
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
pub fn gga_x_chachiyo_fxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    v2rho2: &mut [f64],
    v2rhosigma: &mut [f64],
    v2sigma2: &mut [f64],
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
        {
            let t2 = (v_rho / f64x8::splat(2.0)).simd_le(dens_threshold);
            let t3 = f64x8::splat(M_CBRT3);
            let t4 = f64x8::splat(M_CBRTPI);
            let t7 = (f64x8::splat(1.0)).simd_le(zeta_threshold);
            let t8 = zeta_threshold - f64x8::splat(1.0);
            let t10 = ((t7).select(t8, (t7).select(-t8, f64x8::splat(0.0))));
            let t11 = t10 + f64x8::splat(1.0);
            let t13 = (simd::cbrt(zeta_threshold));
            let t15 = (simd::cbrt(t11));
            let t17 = (((t11).simd_le(zeta_threshold)).select(t13 * zeta_threshold, t15 * t11));
            let t18 = t3 / t4 * t17;
            let t19 = (simd::cbrt(v_rho));
            let t20 = t4 * t4;
            let t21 = t3 * t20;
            let t22 = v_rho * v_rho;
            let t23 = t19 * t19;
            let t25 = f64x8::splat(1.0) / t23 / t22;
            let t29 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t30 = t3 * t3;
            let t31 = t30 * t4;
            let t32 = ((v_sigma).sqrt());
            let t34 = f64x8::splat(1.0) / t19 / v_rho;
            let t36 = t31 * t32 * t34;
            let t38 = f64x8::splat(2.0) / f64x8::splat(27.0) * t36 + f64x8::splat(1.0);
            let t39 = (simd::ln(t38));
            let t41 = f64x8::splat(4.0) / f64x8::splat(81.0) * t21 * v_sigma * t25 + t29 * t39;
            let t44 = f64x8::splat(2.0) / f64x8::splat(9.0) * t36 + t29;
            let t45 = f64x8::splat(1.0) / t44;
            let t46 = f64x8::splat(1.0) / t39;
            let t47 = t45 * t46;
            let t51 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t18 * t19 * t41 * t47));
            let tzk0 = f64x8::splat(2.0) * t51;
            acc_zk = tzk0;
            let t52 = f64x8::splat(1.0) / t23;
            let t57 = t22 * v_rho;
            let t59 = f64x8::splat(1.0) / t23 / t57;
            let t64 = t4 * t29 * t30;
            let t66 = f64x8::splat(1.0) / t19 / t22;
            let t68 = f64x8::splat(1.0) / t38;
            let t72 = -f64x8::splat(32.0) / f64x8::splat(243.0) * t21 * v_sigma * t59 - f64x8::splat(8.0) / f64x8::splat(81.0) * t64 * t32 * t66 * t68;
            let t78 = t17 / t22;
            let t79 = t78 * t41;
            let t80 = t44 * t44;
            let t81 = f64x8::splat(1.0) / t80;
            let t82 = t81 * t46;
            let t83 = t82 * t32;
            let t86 = t39 * t39;
            let t87 = f64x8::splat(1.0) / t86;
            let t88 = t45 * t87;
            let t90 = t88 * t32 * t68;
            let t94 = ((t2).select(f64x8::splat(0.0), -t18 * t52 * t41 * t47 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t18 * t19 * t72 * t47 - t79 * t83 / f64x8::splat(3.0) - t79 * t90 / f64x8::splat(9.0)));
            let tvrho0 = f64x8::splat(2.0) * v_rho * t94 + f64x8::splat(2.0) * t51;
            acc_vrho = tvrho0;
            let t99 = f64x8::splat(1.0) / t32;
            let t104 = f64x8::splat(4.0) / f64x8::splat(81.0) * t21 * t25 + t64 * t99 * t34 * t68 / f64x8::splat(27.0);
            let t110 = t17 / v_rho;
            let t111 = t110 * t41;
            let t112 = t82 * t99;
            let t116 = t88 * t99 * t68;
            let t120 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t18 * t19 * t104 * t47 + t111 * t112 / f64x8::splat(8.0) + t111 * t116 / f64x8::splat(24.0)));
            let tvsigma0 = f64x8::splat(2.0) * v_rho * t120;
            acc_vsigma = tvsigma0;
            let t124 = f64x8::splat(1.0) / t23 / v_rho;
            let t134 = t17 / t57;
            let t135 = t134 * t41;
            let t140 = t22 * t22;
            let t142 = f64x8::splat(1.0) / t23 / t140;
            let t143 = v_sigma * t142;
            let t147 = f64x8::splat(1.0) / t19 / t57;
            let t153 = t20 * t29 * t3;
            let t154 = t38 * t38;
            let t155 = f64x8::splat(1.0) / t154;
            let t159 = f64x8::splat(352.0) / f64x8::splat(729.0) * t21 * t143 + f64x8::splat(56.0) / f64x8::splat(243.0) * t64 * t32 * t147 * t68 - f64x8::splat(64.0) / f64x8::splat(2187.0) * t153 * t143 * t155;
            let t164 = t78 * t72;
            let t170 = f64x8::splat(1.0) / t19 / t140;
            let t171 = t17 * t170;
            let t173 = f64x8::splat(1.0) / t80 / t44;
            let t174 = t41 * t173;
            let t175 = t171 * t174;
            let t177 = t46 * v_sigma * t31;
            let t180 = t41 * t81;
            let t181 = t171 * t180;
            let t182 = t87 * v_sigma;
            let t183 = t31 * t68;
            let t184 = t182 * t183;
            let t187 = t41 * t45;
            let t188 = t171 * t187;
            let t190 = f64x8::splat(1.0) / t86 / t39;
            let t193 = t155 * t30 * t4;
            let t194 = t190 * v_sigma * t193;
            let t197 = t182 * t193;
            let t200 = t18 * t124 * t41 * t47 / f64x8::splat(12.0) - t18 * t52 * t72 * t47 / f64x8::splat(4.0) + f64x8::splat(5.0) / f64x8::splat(9.0) * t135 * t83 + f64x8::splat(5.0) / f64x8::splat(27.0) * t135 * t90 - f64x8::splat(3.0) / f64x8::splat(8.0) * t18 * t19 * t159 * t47 - f64x8::splat(2.0) / f64x8::splat(3.0) * t164 * t83 - f64x8::splat(2.0) / f64x8::splat(9.0) * t164 * t90 - f64x8::splat(16.0) / f64x8::splat(81.0) * t175 * t177 - f64x8::splat(16.0) / f64x8::splat(243.0) * t181 * t184 - f64x8::splat(16.0) / f64x8::splat(729.0) * t188 * t194 - f64x8::splat(8.0) / f64x8::splat(729.0) * t188 * t197;
            let t201 = ((t2).select(f64x8::splat(0.0), t200));
            let tv2rho20 = f64x8::splat(2.0) * v_rho * t201 + f64x8::splat(4.0) * t94;
            acc_v2rho2 = tv2rho20;
            let t217 = -f64x8::splat(32.0) / f64x8::splat(243.0) * t21 * t59 - f64x8::splat(4.0) / f64x8::splat(81.0) * t64 * t99 * t66 * t68 + f64x8::splat(8.0) / f64x8::splat(729.0) * t153 * t59 * t155;
            let t222 = t78 * t104;
            let t229 = t110 * t72;
            let t232 = t17 * t147;
            let t235 = t173 * t46 * t31;
            let t238 = t232 * t180;
            let t241 = t87 * t30 * t4 * t68;
            let t248 = t232 * t187;
            let t250 = t190 * t155 * t31;
            let t254 = t87 * t155 * t31;
            let t257 = -t18 * t52 * t104 * t47 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t18 * t19 * t217 * t47 - t222 * t83 / f64x8::splat(3.0) - t222 * t90 / f64x8::splat(9.0) - t79 * t112 / f64x8::splat(8.0) + t229 * t112 / f64x8::splat(8.0) + f64x8::splat(2.0) / f64x8::splat(27.0) * t232 * t41 * t235 + f64x8::splat(2.0) / f64x8::splat(81.0) * t238 * t241 - t79 * t116 / f64x8::splat(24.0) + t229 * t116 / f64x8::splat(24.0) + f64x8::splat(2.0) / f64x8::splat(243.0) * t248 * t250 + t248 * t254 / f64x8::splat(243.0);
            let t258 = ((t2).select(f64x8::splat(0.0), t257));
            let tv2rhosigma0 = f64x8::splat(2.0) * v_rho * t258 + f64x8::splat(2.0) * t120;
            acc_v2rhosigma = tv2rhosigma0;
            let t261 = t32 * v_sigma;
            let t262 = f64x8::splat(1.0) / t261;
            let t267 = f64x8::splat(1.0) / v_sigma;
            let t272 = -t64 * t262 * t34 * t68 / f64x8::splat(54.0) - t153 * t267 * t25 * t155 / f64x8::splat(243.0);
            let t277 = t110 * t104;
            let t282 = t17 * t66;
            let t283 = t282 * t174;
            let t285 = t46 * t267 * t31;
            let t288 = t282 * t180;
            let t289 = t87 * t267;
            let t290 = t289 * t183;
            let t293 = t82 * t262;
            let t296 = t282 * t187;
            let t297 = t190 * t267;
            let t298 = t297 * t193;
            let t302 = t88 * t262 * t68;
            let t305 = t289 * t193;
            let t309 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t18 * t19 * t272 * t47 + t277 * t112 / f64x8::splat(4.0) + t277 * t116 / f64x8::splat(12.0) - t283 * t285 / f64x8::splat(36.0) - t288 * t290 / f64x8::splat(108.0) - t111 * t293 / f64x8::splat(16.0) - t296 * t298 / f64x8::splat(324.0) - t111 * t302 / f64x8::splat(48.0) - t296 * t305 / f64x8::splat(648.0)));
            let tv2sigma20 = f64x8::splat(2.0) * v_rho * t309;
            acc_v2sigma2 = tv2sigma20;
        }
        store_add(zk, ip, m, acc_zk);
        store_add(vrho, ip, m, acc_vrho);
        store_add(vsigma, ip, m, acc_vsigma);
        store_add(v2rho2, ip, m, acc_v2rho2);
        store_add(v2rhosigma, ip, m, acc_v2rhosigma);
        store_add(v2sigma2, ip, m, acc_v2sigma2);
        ip += 8;
    }
}
