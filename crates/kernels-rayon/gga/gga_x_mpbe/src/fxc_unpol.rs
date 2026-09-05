//! GGA_X_MPBE fxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_mpbe.c`
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
pub fn gga_x_mpbe_fxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    v2rho2: &mut [f64],
    v2rhosigma: &mut [f64],
    v2sigma2: &mut [f64],
    param_c1: f64,
    param_a: f64,
    param_c2: f64,
    param_c3: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_c1 = f64x8::splat(param_c1);
    let param_a = f64x8::splat(param_a);
    let param_c2 = f64x8::splat(param_c2);
    let param_c3 = f64x8::splat(param_c3);
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
            let t22 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t23 = (simd::cbrt(t22));
            let t24 = t23 * t23;
            let t25 = f64x8::splat(1.0) / t24;
            let t26 = param_c1 * t20 * t25;
            let t27 = f64x8::splat(M_CBRT2);
            let t28 = t27 * t27;
            let t29 = v_sigma * t28;
            let t30 = v_rho * v_rho;
            let t31 = t18 * t18;
            let t33 = f64x8::splat(1.0) / t31 / t30;
            let t34 = param_a * t20;
            let t39 = f64x8::splat(1.0) + t34 * t25 * t29 * t33 / f64x8::splat(24.0);
            let t40 = f64x8::splat(1.0) / t39;
            let t45 = t20 * t20;
            let t48 = f64x8::splat(1.0) / t23 / t22;
            let t49 = param_c2 * t45 * t48;
            let t50 = v_sigma * v_sigma;
            let t51 = t50 * t27;
            let t52 = t30 * t30;
            let t53 = t52 * v_rho;
            let t55 = f64x8::splat(1.0) / t18 / t53;
            let t56 = t39 * t39;
            let t57 = f64x8::splat(1.0) / t56;
            let t58 = t55 * t57;
            let t62 = t22 * t22;
            let t63 = f64x8::splat(1.0) / t62;
            let t64 = param_c3 * t63;
            let t65 = t50 * v_sigma;
            let t66 = t52 * t52;
            let t67 = f64x8::splat(1.0) / t66;
            let t69 = t56 * t39;
            let t70 = f64x8::splat(1.0) / t69;
            let t74 = f64x8::splat(1.0) + t26 * t29 * t33 * t40 / f64x8::splat(24.0) + t49 * t51 * t58 / f64x8::splat(288.0) + t64 * t65 * t67 * t70 / f64x8::splat(576.0);
            let t78 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t74));
            let tzk0 = f64x8::splat(2.0) * t78;
            acc_zk = tzk0;
            let t80 = t17 / t31;
            let t84 = t30 * v_rho;
            let t86 = f64x8::splat(1.0) / t31 / t84;
            let t91 = param_c1 * t45;
            let t93 = t91 * t48 * t50;
            let t94 = t52 * t30;
            let t96 = f64x8::splat(1.0) / t18 / t94;
            let t97 = t27 * t96;
            let t98 = t57 * param_a;
            let t99 = t97 * t98;
            let t102 = t96 * t57;
            let t106 = param_c2 * t63;
            let t107 = t106 * t65;
            let t108 = t66 * v_rho;
            let t109 = f64x8::splat(1.0) / t108;
            let t110 = t109 * t70;
            let t111 = t110 * param_a;
            let t118 = t50 * t50;
            let t119 = t66 * t84;
            let t121 = f64x8::splat(1.0) / t31 / t119;
            let t124 = t56 * t56;
            let t125 = f64x8::splat(1.0) / t124;
            let t128 = t20 * t25 * t28;
            let t129 = t125 * param_a * t128;
            let t132 = -t26 * t29 * t86 * t40 / f64x8::splat(9.0) + t93 * t99 / f64x8::splat(108.0) - t49 * t51 * t102 / f64x8::splat(54.0) + t107 * t111 / f64x8::splat(108.0) - t64 * t65 * t109 * t70 / f64x8::splat(72.0) + t64 * t118 * t121 * t129 / f64x8::splat(1728.0);
            let t137 = ((t2).select(f64x8::splat(0.0), -t6 * t80 * t74 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t132));
            let tvrho0 = f64x8::splat(2.0) * v_rho * t137 + f64x8::splat(2.0) * t78;
            acc_vrho = tvrho0;
            let t146 = t27 * t55;
            let t147 = t146 * t98;
            let t150 = v_sigma * t27;
            let t154 = t106 * t50;
            let t155 = t67 * t70;
            let t156 = t155 * param_a;
            let t163 = t66 * t30;
            let t165 = f64x8::splat(1.0) / t31 / t163;
            let t170 = t26 * t28 * t33 * t40 / f64x8::splat(24.0) - t91 * t48 * v_sigma * t147 / f64x8::splat(288.0) + t49 * t150 * t58 / f64x8::splat(144.0) - t154 * t156 / f64x8::splat(288.0) + t64 * t50 * t67 * t70 / f64x8::splat(192.0) - t64 * t65 * t165 * t129 / f64x8::splat(4608.0);
            let t174 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t170));
            let tvsigma0 = f64x8::splat(2.0) * v_rho * t174;
            acc_vsigma = tvsigma0;
            let t179 = t17 / t31 / v_rho;
            let t187 = f64x8::splat(1.0) / t31 / t52;
            let t192 = t52 * t84;
            let t194 = f64x8::splat(1.0) / t18 / t192;
            let t195 = t27 * t194;
            let t196 = t195 * t98;
            let t199 = param_c1 * t63;
            let t200 = t199 * t65;
            let t201 = f64x8::splat(1.0) / t163;
            let t202 = t201 * t70;
            let t203 = param_a * param_a;
            let t207 = t194 * t57;
            let t211 = t202 * param_a;
            let t214 = t66 * t52;
            let t216 = f64x8::splat(1.0) / t31 / t214;
            let t217 = t118 * t216;
            let t220 = t125 * t203 * t128;
            let t230 = t118 * v_sigma;
            let t233 = f64x8::splat(1.0) / t18 / t66 / t192;
            let t237 = f64x8::splat(1.0) / t124 / t39;
            let t240 = t45 * t48 * t27;
            let t241 = t237 * t203 * t240;
            let t244 = f64x8::splat(11.0) / f64x8::splat(27.0) * t26 * t29 * t187 * t40 - t93 * t196 / f64x8::splat(12.0) + f64x8::splat(2.0) / f64x8::splat(81.0) * t200 * t202 * t203 + f64x8::splat(19.0) / f64x8::splat(162.0) * t49 * t51 * t207 - f64x8::splat(43.0) / f64x8::splat(324.0) * t107 * t211 + t106 * t217 * t220 / f64x8::splat(324.0) + t64 * t65 * t201 * t70 / f64x8::splat(8.0) - f64x8::splat(59.0) / f64x8::splat(5184.0) * t64 * t217 * t129 + t64 * t230 * t233 * t241 / f64x8::splat(1944.0);
            let t249 = ((t2).select(f64x8::splat(0.0), t6 * t179 * t74 / f64x8::splat(12.0) - t6 * t80 * t132 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t244));
            let tv2rho20 = f64x8::splat(2.0) * v_rho * t249 + f64x8::splat(4.0) * t137;
            acc_v2rho2 = tv2rho20;
            let t260 = t91 * t48 * t27;
            let t261 = param_a * v_sigma;
            let t266 = t110 * t203;
            let t274 = t65 * t121;
            let t285 = t66 * t94;
            let t287 = f64x8::splat(1.0) / t18 / t285;
            let t292 = -t26 * t28 * t86 * t40 / f64x8::splat(9.0) + t260 * t102 * t261 / f64x8::splat(36.0) - t199 * t50 * t266 / f64x8::splat(108.0) - t49 * t150 * t102 / f64x8::splat(27.0) + f64x8::splat(5.0) / f64x8::splat(108.0) * t154 * t111 - t106 * t274 * t220 / f64x8::splat(864.0) - t64 * t50 * t109 * t70 / f64x8::splat(24.0) + f64x8::splat(7.0) / f64x8::splat(1728.0) * t64 * t274 * t129 - t64 * t118 * t287 * t241 / f64x8::splat(5184.0);
            let t297 = ((t2).select(f64x8::splat(0.0), -t6 * t80 * t170 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t292));
            let tv2rhosigma0 = f64x8::splat(2.0) * v_rho * t297 + f64x8::splat(2.0) * t174;
            acc_v2rhosigma = tv2rhosigma0;
            let t300 = t91 * t48;
            let t304 = t155 * t203;
            let t313 = t50 * t165;
            let t324 = t66 * t53;
            let t326 = f64x8::splat(1.0) / t18 / t324;
            let t331 = -t300 * t147 / f64x8::splat(144.0) + t199 * v_sigma * t304 / f64x8::splat(288.0) + t49 * t146 * t57 / f64x8::splat(144.0) - t106 * v_sigma * t156 / f64x8::splat(72.0) + t106 * t313 * t220 / f64x8::splat(2304.0) + t64 * v_sigma * t67 * t70 / f64x8::splat(96.0) - t64 * t313 * t129 / f64x8::splat(768.0) + t64 * t65 * t326 * t241 / f64x8::splat(13824.0);
            let t335 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t331));
            let tv2sigma20 = f64x8::splat(2.0) * v_rho * t335;
            acc_v2sigma2 = tv2sigma20;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_vrho.into(); vrho[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_vsigma.into(); vsigma[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v2rho2.into(); v2rho2[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v2rhosigma.into(); v2rhosigma[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v2sigma2.into(); v2sigma2[ip..ip + m].copy_from_slice(&a[..m]); }
        ip += 8;
    }
}
