//! GGA_X_G96 kxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_g96.c`
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
pub fn gga_x_g96_kxc_unpol(
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
            let t2 = (v_rho / f64x8::splat(2.0)).simd_le(dens_threshold);
            let t3 = f64x8::splat(M_CBRT3);
            let t4 = f64x8::splat(M_CBRTPI);
            let t5 = f64x8::splat(1.0) / t4;
            let t6 = t3 * t5;
            let t7 = (f64x8::splat(1.0)).simd_le(zeta_threshold);
            let t8 = zeta_threshold - f64x8::splat(1.0);
            let t10 = ((t7).select(t8, (t7).select(-t8, f64x8::splat(0.0))));
            let t11 = f64x8::splat(1.0) + t10;
            let t13 = (simd::cbrt(zeta_threshold));
            let t15 = (simd::cbrt(t11));
            let t17 = (((t11).simd_le(zeta_threshold)).select(t13 * zeta_threshold, t15 * t11));
            let t18 = (simd::cbrt(v_rho));
            let t20 = t3 * t3;
            let t22 = (simd::cbrt(f64x8::splat(1.0) / f64x8::splat(M_PI)));
            let t23 = f64x8::splat(1.0) / t22;
            let t25 = f64x8::splat(M_CBRT4);
            let t26 = ((v_sigma).sqrt());
            let t27 = f64x8::splat(M_CBRT2);
            let t28 = t26 * t27;
            let t31 = t28 / t18 / v_rho;
            let t32 = ((t31).sqrt());
            let t33 = t32 * t31;
            let t37 = f64x8::splat(1.0) + f64x8::splat(2.0) / f64x8::splat(1233.0) * t20 * t23 * t25 * t33;
            let t41 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t17 * t18 * t37));
            let tzk0 = f64x8::splat(2.0) * t41;
            acc_zk = tzk0;
            let t42 = t18 * t18;
            let t48 = t5 * t17;
            let t49 = v_rho * v_rho;
            let t52 = t48 / t49 * t23;
            let t53 = t25 * t32;
            let t54 = t53 * t28;
            let t58 = ((t2).select(f64x8::splat(0.0), -t6 * t17 / t42 * t37 / f64x8::splat(8.0) + t52 * t54 / f64x8::splat(274.0)));
            let tvrho0 = f64x8::splat(2.0) * v_rho * t58 + f64x8::splat(2.0) * t41;
            acc_vrho = tvrho0;
            let t63 = t48 / v_rho * t23;
            let t64 = f64x8::splat(1.0) / t26;
            let t66 = t53 * t64 * t27;
            let t69 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(2192.0) * t63 * t66));
            let tvsigma0 = f64x8::splat(2.0) * v_rho * t69;
            acc_vsigma = tvsigma0;
            let t78 = t49 * v_rho;
            let t81 = t48 / t78 * t23;
            let t84 = t49 * t49;
            let t86 = f64x8::splat(1.0) / t18 / t84;
            let t88 = t48 * t86 * t23;
            let t89 = f64x8::splat(1.0) / t32;
            let t90 = t25 * t89;
            let t91 = t27 * t27;
            let t92 = v_sigma * t91;
            let t93 = t90 * t92;
            let t97 = ((t2).select(f64x8::splat(0.0), t6 * t17 / t42 / v_rho * t37 / f64x8::splat(12.0) - f64x8::splat(5.0) / f64x8::splat(822.0) * t81 * t54 - t88 * t93 / f64x8::splat(411.0)));
            let tv2rho20 = f64x8::splat(2.0) * v_rho * t97 + f64x8::splat(4.0) * t58;
            acc_v2rho2 = tv2rho20;
            let t103 = f64x8::splat(1.0) / t18 / t78;
            let t105 = t23 * t25;
            let t107 = t105 * t89 * t91;
            let t111 = ((t2).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(2192.0) * t52 * t66 + t48 * t103 * t107 / f64x8::splat(1096.0)));
            let tv2rhosigma0 = f64x8::splat(2.0) * v_rho * t111 + f64x8::splat(2.0) * t69;
            acc_v2rhosigma = tv2rhosigma0;
            let t117 = t48 / t18 / t49 * t23;
            let t118 = f64x8::splat(1.0) / v_sigma;
            let t120 = t90 * t118 * t91;
            let t123 = t26 * v_sigma;
            let t124 = f64x8::splat(1.0) / t123;
            let t126 = t53 * t124 * t27;
            let t130 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8768.0) * t117 * t120 + f64x8::splat(3.0) / f64x8::splat(4384.0) * t63 * t126));
            let tv2sigma20 = f64x8::splat(2.0) * v_rho * t130;
            acc_v2sigma2 = tv2sigma20;
            let t134 = f64x8::splat(1.0) / t42 / t49;
            let t141 = t48 / t84 * t23;
            let t144 = t84 * v_rho;
            let t146 = f64x8::splat(1.0) / t18 / t144;
            let t151 = t84 * t49;
            let t154 = t48 / t42 / t151;
            let t155 = f64x8::splat(1.0) / t33;
            let t157 = t105 * t155 * t123;
            let t161 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(5.0) / f64x8::splat(36.0) * t6 * t17 * t134 * t37 + f64x8::splat(43.0) / f64x8::splat(2466.0) * t141 * t54 + f64x8::splat(2.0) / f64x8::splat(137.0) * t48 * t146 * t23 * t93 - f64x8::splat(4.0) / f64x8::splat(1233.0) * t154 * t157));
            let tv3rho30 = f64x8::splat(2.0) * v_rho * t161 + f64x8::splat(6.0) * t97;
            acc_v3rho3 = tv3rho30;
            let t172 = t48 / t42 / t144;
            let t174 = t105 * t155 * t26;
            let t178 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(1096.0) * t81 * t66 - f64x8::splat(13.0) / f64x8::splat(3288.0) * t48 * t86 * t107 + t172 * t174 / f64x8::splat(822.0)));
            let tv3rho2sigma0 = f64x8::splat(2.0) * v_rho * t178 + f64x8::splat(4.0) * t111;
            acc_v3rho2sigma = tv3rho2sigma0;
            let t182 = t48 * t103 * t23;
            let t187 = t48 / t42 / t84;
            let t189 = t105 * t155 * t64;
            let t195 = ((t2).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(8768.0) * t182 * t120 - t187 * t189 / f64x8::splat(2192.0) - f64x8::splat(3.0) / f64x8::splat(4384.0) * t52 * t126));
            let tv3rhosigma20 = f64x8::splat(2.0) * v_rho * t195 + f64x8::splat(2.0) * t130;
            acc_v3rhosigma2 = tv3rhosigma20;
            let t199 = f64x8::splat(1.0) / t42 / t78;
            let t200 = t48 * t199;
            let t202 = t105 * t155 * t124;
            let t205 = v_sigma * v_sigma;
            let t206 = f64x8::splat(1.0) / t205;
            let t208 = t90 * t206 * t91;
            let t212 = f64x8::splat(1.0) / t26 / t205;
            let t214 = t53 * t212 * t27;
            let t218 = ((t2).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(17536.0) * t200 * t202 + f64x8::splat(9.0) / f64x8::splat(17536.0) * t117 * t208 - f64x8::splat(9.0) / f64x8::splat(8768.0) * t63 * t214));
            let tv3sigma30 = f64x8::splat(2.0) * v_rho * t218;
            acc_v3sigma3 = tv3sigma30;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_vrho.into(); vrho[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_vsigma.into(); vsigma[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v2rho2.into(); v2rho2[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v2rhosigma.into(); v2rhosigma[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v2sigma2.into(); v2sigma2[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v3rho3.into(); v3rho3[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v3rho2sigma.into(); v3rho2sigma[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v3rhosigma2.into(); v3rhosigma2[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v3sigma3.into(); v3sigma3[ip..ip + m].copy_from_slice(&a[..m]); }
        ip += 8;
    }
}
