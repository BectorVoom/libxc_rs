//! MGGA_X_MBEEFVDW exc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_mbeefvdw.c`
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

/// Load 8 elements with a given stride and offset.
#[inline(always)]
fn load_strided(s: &[f64], ip: usize, np: usize, stride: usize, offset: usize) -> f64x8 {
    let mut b = [0.0f64; 8];
    if ip + 8 <= np {
        let base = ip * stride + offset;
        b[0] = s[base];
        b[1] = s[base + stride];
        b[2] = s[base + 2 * stride];
        b[3] = s[base + 3 * stride];
        b[4] = s[base + 4 * stride];
        b[5] = s[base + 5 * stride];
        b[6] = s[base + 6 * stride];
        b[7] = s[base + 7 * stride];
    } else {
        for k in 0..8 {
            let p = (ip + k).min(np - 1);
            b[k] = s[p * stride + offset];
        }
    }
    f64x8::new(b)
}

/// Store 8 elements with a given stride and offset.
#[inline(always)]
fn store_strided(s: &mut [f64], ip: usize, m: usize, stride: usize, offset: usize, acc: f64x8) {
    let a: [f64; 8] = acc.into();
    if m == 8 {
        let base = ip * stride + offset;
        s[base] = a[0];
        s[base + stride] = a[1];
        s[base + 2 * stride] = a[2];
        s[base + 3 * stride] = a[3];
        s[base + 4 * stride] = a[4];
        s[base + 5 * stride] = a[5];
        s[base + 6 * stride] = a[6];
        s[base + 7 * stride] = a[7];
    } else {
        for k in 0..m {
            s[(ip + k) * stride + offset] = a[k];
        }
    }
}

#[allow(unused_variables, non_snake_case)]
pub fn mgga_x_mbeefvdw_exc_pol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let dens_threshold = f64x8::splat(dens_threshold);
    let zeta_threshold = f64x8::splat(zeta_threshold);
    let mut ip = 0usize;
    while ip < np {
        let m = (np - ip).min(8);
        let v_rho0 = load_strided(rho, ip, np, 2, 0);
        let v_rho1 = load_strided(rho, ip, np, 2, 1);
        let v_sigma0 = load_strided(sigma, ip, np, 3, 0);
        let v_sigma1 = load_strided(sigma, ip, np, 3, 1);
        let v_sigma2 = load_strided(sigma, ip, np, 3, 2);
        let v_lapl0 = load_strided(lapl, ip, np, 2, 0);
        let v_lapl1 = load_strided(lapl, ip, np, 2, 1);
        let v_tau0 = load_strided(tau, ip, np, 2, 0);
        let v_tau1 = load_strided(tau, ip, np, 2, 1);
        let mut acc_zk = V_ZERO;
        {
            let t2 = (v_rho0).simd_le(dens_threshold);
            let t3 = f64x8::splat(M_CBRT3);
            let t4 = f64x8::splat(M_CBRTPI);
            let t6 = t3 / t4;
            let t7 = v_rho0 + v_rho1;
            let t8 = f64x8::splat(1.0) / t7;
            let t11 = (f64x8::splat(2.0) * v_rho0 * t8).simd_le(zeta_threshold);
            let t12 = zeta_threshold - f64x8::splat(1.0);
            let t15 = (f64x8::splat(2.0) * v_rho1 * t8).simd_le(zeta_threshold);
            let t16 = -t12;
            let t17 = v_rho0 - v_rho1;
            let t19 = ((t11).select(t12, (t15).select(t16, t17 * t8)));
            let t20 = t19 + f64x8::splat(1.0);
            let t21 = (t20).simd_le(zeta_threshold);
            let t22 = (simd::cbrt(zeta_threshold));
            let t23 = t22 * zeta_threshold;
            let t24 = (simd::cbrt(t20));
            let t26 = ((t21).select(t23, t24 * t20));
            let t27 = (simd::cbrt(t7));
            let t28 = t26 * t27;
            let t29 = f64x8::splat(M_CBRT6);
            let t30 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t31 = (simd::cbrt(t30));
            let t32 = t31 * t31;
            let t33 = f64x8::splat(1.0) / t32;
            let t34 = t29 * t33;
            let t35 = v_rho0 * v_rho0;
            let t36 = (simd::cbrt(v_rho0));
            let t37 = t36 * t36;
            let t39 = f64x8::splat(1.0) / t37 / t35;
            let t40 = v_sigma0 * t39;
            let t43 = f64x8::splat(6.5124) + t34 * t40 / f64x8::splat(24.0);
            let t44 = f64x8::splat(1.0) / t43;
            let t46 = t34 * t40 * t44;
            let t48 = t46 / f64x8::splat(12.0) - f64x8::splat(1.0);
            let t49 = t48 * t48;
            let t51 = t49 * t48;
            let t54 = t49 * t49;
            let t57 = f64x8::splat(1.0) / t37 / v_rho0;
            let t63 = f64x8::splat(5.0) / f64x8::splat(9.0) * (v_tau0 * t57 - t40 / f64x8::splat(8.0)) * t29 * t33;
            let t64 = (f64x8::splat(10000.0)).simd_le(t63);
            let t65 = (f64x8::splat(10000.0)).simd_lt(t63);
            let t66 = ((t65).select(t63, f64x8::splat(10000.0)));
            let t67 = t66 * t66;
            let t70 = t67 * t66;
            let t71 = f64x8::splat(1.0) / t70;
            let t72 = t67 * t67;
            let t73 = f64x8::splat(1.0) / t72;
            let t76 = ((t65).select(f64x8::splat(10000.0), t63));
            let t77 = t76 * t76;
            let t78 = f64x8::splat(1.0) - t77;
            let t79 = t78 * t78;
            let t80 = t79 * t78;
            let t81 = t77 * t76;
            let t82 = f64x8::splat(1.0) + t81;
            let t84 = t81 * t82 + f64x8::splat(1.0);
            let t85 = f64x8::splat(1.0) / t84;
            let t87 = ((t64).select(f64x8::splat(1.0) - f64x8::splat(3.0) / t67 - t71 + f64x8::splat(3.0) * t73, -t80 * t85));
            let t89 = t87 * t87;
            let t91 = t89 * t87;
            let t93 = t89 * t89;
            let t97 = f64x8::splat(3.0) / f64x8::splat(8.0) + f64x8::splat(35.0) / f64x8::splat(8.0) * t54 - f64x8::splat(15.0) / f64x8::splat(4.0) * t49;
            let t100 = f64x8::splat(3.0) / f64x8::splat(8.0) + f64x8::splat(35.0) / f64x8::splat(8.0) * t93 - f64x8::splat(15.0) / f64x8::splat(4.0) * t89;
            let t105 = f64x8::splat(5.0) / f64x8::splat(2.0) * t91 - f64x8::splat(3.0) / f64x8::splat(2.0) * t87;
            let t109 = -f64x8::splat(1.0) / f64x8::splat(2.0) + f64x8::splat(3.0) / f64x8::splat(2.0) * t89;
            let t112 = t97 * t87;
            let t114 = -f64x8::splat(0.0851282539125) * t49 - f64x8::splat(0.050282912) * t51 + f64x8::splat(0.01214700985) * t46 + f64x8::splat(0.00618699843125) * t54 - f64x8::splat(0.06972770593) * t87 + f64x8::splat(0.0217681859775) * t89 + f64x8::splat(0.00351985355) * t91 + f64x8::splat(0.00061919587625) * t93 - f64x8::splat(3.40722258e-09) * t97 * t100 + f64x8::splat(5.74317889e-08) * t97 * t105 - f64x8::splat(5.00749348e-07) * t97 * t109 + f64x8::splat(9.19317034e-07) * t112;
            let t117 = f64x8::splat(5.0) / f64x8::splat(2.0) * t51 - t46 / f64x8::splat(8.0) + f64x8::splat(3.0) / f64x8::splat(2.0);
            let t124 = t117 * t87;
            let t127 = -f64x8::splat(1.0) / f64x8::splat(2.0) + f64x8::splat(3.0) / f64x8::splat(2.0) * t49;
            let t134 = t127 * t87;
            let t142 = t48 * t87;
            let t144 = f64x8::splat(1.0451438955835) + f64x8::splat(3.97324768e-09) * t117 * t100 - f64x8::splat(5.49909413e-08) * t117 * t105 + f64x8::splat(1.33707403e-07) * t117 * t109 + f64x8::splat(0.0192374554) * t124 + f64x8::splat(2.01895739e-07) * t127 * t100 - f64x8::splat(6.57949254e-07) * t127 * t105 - f64x8::splat(0.00521818079) * t127 * t109 - f64x8::splat(0.0222650139) * t134 - f64x8::splat(1.00478906e-07) * t48 * t100 - f64x8::splat(0.00608338264) * t48 * t105 + f64x8::splat(0.0318024096) * t48 * t109 + f64x8::splat(0.0453837246) * t142;
            let t145 = t114 + t144;
            let t149 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t28 * t145));
            let t150 = (v_rho1).simd_le(dens_threshold);
            let t151 = -t17;
            let t153 = ((t15).select(t12, (t11).select(t16, t151 * t8)));
            let t154 = t153 + f64x8::splat(1.0);
            let t155 = (t154).simd_le(zeta_threshold);
            let t156 = (simd::cbrt(t154));
            let t158 = ((t155).select(t23, t156 * t154));
            let t159 = t158 * t27;
            let t160 = v_rho1 * v_rho1;
            let t161 = (simd::cbrt(v_rho1));
            let t162 = t161 * t161;
            let t164 = f64x8::splat(1.0) / t162 / t160;
            let t165 = v_sigma2 * t164;
            let t168 = f64x8::splat(6.5124) + t34 * t165 / f64x8::splat(24.0);
            let t169 = f64x8::splat(1.0) / t168;
            let t171 = t34 * t165 * t169;
            let t174 = t171 / f64x8::splat(12.0) - f64x8::splat(1.0);
            let t175 = t174 * t174;
            let t176 = t175 * t175;
            let t179 = f64x8::splat(1.0) / t162 / v_rho1;
            let t185 = f64x8::splat(5.0) / f64x8::splat(9.0) * (v_tau1 * t179 - t165 / f64x8::splat(8.0)) * t29 * t33;
            let t186 = (f64x8::splat(10000.0)).simd_le(t185);
            let t187 = (f64x8::splat(10000.0)).simd_lt(t185);
            let t188 = ((t187).select(t185, f64x8::splat(10000.0)));
            let t189 = t188 * t188;
            let t192 = t189 * t188;
            let t193 = f64x8::splat(1.0) / t192;
            let t194 = t189 * t189;
            let t195 = f64x8::splat(1.0) / t194;
            let t198 = ((t187).select(f64x8::splat(10000.0), t185));
            let t199 = t198 * t198;
            let t200 = f64x8::splat(1.0) - t199;
            let t201 = t200 * t200;
            let t202 = t201 * t200;
            let t203 = t199 * t198;
            let t204 = f64x8::splat(1.0) + t203;
            let t206 = t203 * t204 + f64x8::splat(1.0);
            let t207 = f64x8::splat(1.0) / t206;
            let t209 = ((t186).select(f64x8::splat(1.0) - f64x8::splat(3.0) / t189 - t193 + f64x8::splat(3.0) * t195, -t202 * t207));
            let t212 = t209 * t209;
            let t213 = t212 * t212;
            let t215 = t175 * t174;
            let t218 = t212 * t209;
            let t222 = f64x8::splat(3.0) / f64x8::splat(8.0) + f64x8::splat(35.0) / f64x8::splat(8.0) * t176 - f64x8::splat(15.0) / f64x8::splat(4.0) * t175;
            let t225 = f64x8::splat(3.0) / f64x8::splat(8.0) + f64x8::splat(35.0) / f64x8::splat(8.0) * t213 - f64x8::splat(15.0) / f64x8::splat(4.0) * t212;
            let t230 = f64x8::splat(5.0) / f64x8::splat(2.0) * t218 - f64x8::splat(3.0) / f64x8::splat(2.0) * t209;
            let t234 = -f64x8::splat(1.0) / f64x8::splat(2.0) + f64x8::splat(3.0) / f64x8::splat(2.0) * t212;
            let t237 = f64x8::splat(1.0451438955835) + f64x8::splat(0.01214700985) * t171 + f64x8::splat(0.00618699843125) * t176 - f64x8::splat(0.06972770593) * t209 - f64x8::splat(0.0851282539125) * t175 + f64x8::splat(0.00061919587625) * t213 - f64x8::splat(0.050282912) * t215 + f64x8::splat(0.0217681859775) * t212 + f64x8::splat(0.00351985355) * t218 - f64x8::splat(3.40722258e-09) * t222 * t225 + f64x8::splat(5.74317889e-08) * t222 * t230 - f64x8::splat(5.00749348e-07) * t222 * t234;
            let t238 = t222 * t209;
            let t240 = t174 * t209;
            let t249 = -f64x8::splat(1.0) / f64x8::splat(2.0) + f64x8::splat(3.0) / f64x8::splat(2.0) * t175;
            let t250 = t249 * t209;
            let t260 = f64x8::splat(5.0) / f64x8::splat(2.0) * t215 - t171 / f64x8::splat(8.0) + f64x8::splat(3.0) / f64x8::splat(2.0);
            let t261 = t260 * t209;
            let t269 = f64x8::splat(9.19317034e-07) * t238 + f64x8::splat(0.0453837246) * t240 + f64x8::splat(0.0318024096) * t174 * t234 - f64x8::splat(0.00608338264) * t174 * t230 - f64x8::splat(1.00478906e-07) * t174 * t225 - f64x8::splat(0.0222650139) * t250 - f64x8::splat(0.00521818079) * t249 * t234 - f64x8::splat(6.57949254e-07) * t249 * t230 + f64x8::splat(2.01895739e-07) * t249 * t225 + f64x8::splat(0.0192374554) * t261 + f64x8::splat(1.33707403e-07) * t260 * t234 - f64x8::splat(5.49909413e-08) * t260 * t230 + f64x8::splat(3.97324768e-09) * t260 * t225;
            let t270 = t237 + t269;
            let t274 = ((t150).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t159 * t270));
            let tzk0 = t149 + t274;
            acc_zk = tzk0;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        ip += 8;
    }
}
