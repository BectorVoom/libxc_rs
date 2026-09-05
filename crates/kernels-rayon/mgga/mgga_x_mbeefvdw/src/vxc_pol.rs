//! MGGA_X_MBEEFVDW vxc pol kernel — explicit SIMD (bit-exact).
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

/// Accumulate 8 elements with a given stride and offset.
///
/// `+=`, not `=`: the scalar kernel this was translated from writes
/// `out[ip * stride + offset] += v`, and a plain store is not the same
/// operation. It differs on the sign of zero -- `0.0 + -0.0` is `+0.0`
/// while a store of `-0.0` keeps the sign -- which is a bit difference
/// the fingerprint gate sees, and it would silently drop a caller's
/// existing contribution if one were ever there.
///
/// The read is not free on this path: a polarized `kxc`/`lxc` kernel
/// writes many strided outputs per point, and `lda_c_pw_erf kxc pol`
/// measured 84 -> 114 ns/pt (1.36x). It is charged anyway, because the
/// scalar kernel this is compared against does the same read. Gathering
/// into a vector, adding once and scattering back was tried and is no
/// faster (117 ns/pt), so the cost is the load itself, not scheduling.
#[inline(always)]
fn store_strided(s: &mut [f64], ip: usize, m: usize, stride: usize, offset: usize, acc: f64x8) {
    let a: [f64; 8] = acc.into();
    if m == 8 {
        let base = ip * stride + offset;
        s[base] += a[0];
        s[base + stride] += a[1];
        s[base + 2 * stride] += a[2];
        s[base + 3 * stride] += a[3];
        s[base + 4 * stride] += a[4];
        s[base + 5 * stride] += a[5];
        s[base + 6 * stride] += a[6];
        s[base + 7 * stride] += a[7];
    } else {
        for k in 0..m {
            s[(ip + k) * stride + offset] += a[k];
        }
    }
}

#[allow(unused_variables, non_snake_case)]
pub fn mgga_x_mbeefvdw_vxc_pol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    vlapl: &mut [f64],
    vtau: &mut [f64],
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
        let mut acc_vrho_0 = V_ZERO;
        let mut acc_vrho_1 = V_ZERO;
        let mut acc_vsigma_0 = V_ZERO;
        let mut acc_vsigma_1 = V_ZERO;
        let mut acc_vsigma_2 = V_ZERO;
        let mut acc_vlapl_0 = V_ZERO;
        let mut acc_vlapl_1 = V_ZERO;
        let mut acc_vtau_0 = V_ZERO;
        let mut acc_vtau_1 = V_ZERO;
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
            let t275 = t7 * t7;
            let t276 = f64x8::splat(1.0) / t275;
            let t277 = t17 * t276;
            let t279 = ((t11).select(f64x8::splat(0.0), (t15).select(f64x8::splat(0.0), t8 - t277)));
            let t282 = ((t21).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t24 * t279));
            let t283 = t282 * t27;
            let t287 = t27 * t27;
            let t288 = f64x8::splat(1.0) / t287;
            let t289 = t26 * t288;
            let t292 = t6 * t289 * t145 / f64x8::splat(8.0);
            let t295 = t35 * v_rho0;
            let t297 = f64x8::splat(1.0) / t37 / t295;
            let t298 = v_sigma0 * t297;
            let t303 = f64x8::splat(5.0) / f64x8::splat(9.0) * (-f64x8::splat(5.0) / f64x8::splat(3.0) * v_tau0 * t39 + t298 / f64x8::splat(3.0)) * t29 * t33;
            let t304 = ((t65).select(t303, f64x8::splat(0.0)));
            let t307 = t73 * t304;
            let t310 = f64x8::splat(1.0) / t72 / t66;
            let t311 = t310 * t304;
            let t314 = t79 * t85;
            let t315 = ((t65).select(f64x8::splat(0.0), t303));
            let t316 = t76 * t315;
            let t319 = t84 * t84;
            let t320 = f64x8::splat(1.0) / t319;
            let t321 = t80 * t320;
            let t322 = t77 * t82;
            let t324 = t77 * t77;
            let t325 = t324 * t76;
            let t328 = f64x8::splat(3.0) * t322 * t315 + f64x8::splat(3.0) * t325 * t315;
            let t331 = ((t64).select(f64x8::splat(6.0) * t71 * t304 + f64x8::splat(3.0) * t307 - f64x8::splat(12.0) * t311, f64x8::splat(6.0) * t314 * t316 + t321 * t328));
            let t333 = t29 * t29;
            let t335 = f64x8::splat(1.0) / t31 / t30;
            let t336 = t333 * t335;
            let t337 = v_sigma0 * v_sigma0;
            let t338 = t35 * t35;
            let t339 = t338 * t35;
            let t341 = f64x8::splat(1.0) / t36 / t339;
            let t343 = t43 * t43;
            let t344 = f64x8::splat(1.0) / t343;
            let t346 = t336 * t337 * t341 * t344;
            let t349 = t34 * t298 * t44;
            let t352 = -f64x8::splat(2.0) / f64x8::splat(9.0) * t349 + t346 / f64x8::splat(108.0);
            let t353 = t48 * t352;
            let t370 = t352 * t87;
            let t372 = t48 * t331;
            let t374 = t352 * t105;
            let t376 = t89 * t331;
            let t380 = f64x8::splat(15.0) / f64x8::splat(2.0) * t376 - f64x8::splat(3.0) / f64x8::splat(2.0) * t331;
            let t383 = t352 * t109;
            let t385 = t127 * t331;
            let t387 = t352 * t100;
            let t389 = t91 * t331;
            let t391 = t87 * t331;
            let t393 = -f64x8::splat(0.06972770593) * t331 + f64x8::splat(0.0013496677611111111) * t346 + f64x8::splat(6.05687217e-07) * t353 * t100 - f64x8::splat(1.973847762e-06) * t353 * t105 - f64x8::splat(0.01565454237) * t353 * t109 - f64x8::splat(0.01565454237) * t134 * t331 - f64x8::splat(0.0667950417) * t353 * t87 + f64x8::splat(0.0954072288) * t142 * t331 - f64x8::splat(1.502248044e-06) * t112 * t331 + f64x8::splat(4.01122209e-07) * t124 * t331 + f64x8::splat(0.0453837246) * t370 + f64x8::splat(0.0453837246) * t372 - f64x8::splat(0.00608338264) * t374 + f64x8::splat(0.01055956065) * t376 - f64x8::splat(0.00608338264) * t48 * t380 + f64x8::splat(0.0318024096) * t383 - f64x8::splat(0.0222650139) * t385 - f64x8::splat(1.00478906e-07) * t387 + f64x8::splat(0.002476783505) * t389 + f64x8::splat(0.043536371955) * t391;
            let t396 = f64x8::splat(35.0) / f64x8::splat(2.0) * t389 - f64x8::splat(15.0) / f64x8::splat(2.0) * t391;
            let t399 = t117 * t331;
            let t405 = t49 * t352;
            let t410 = f64x8::splat(15.0) / f64x8::splat(2.0) * t405 + t349 / f64x8::splat(3.0) - t346 / f64x8::splat(72.0);
            let t417 = t410 * t87;
            let t419 = t51 * t352;
            let t425 = f64x8::splat(35.0) / f64x8::splat(2.0) * t419 - f64x8::splat(15.0) / f64x8::splat(2.0) * t353;
            let t426 = t425 * t87;
            let t428 = t97 * t331;
            let t445 = -f64x8::splat(0.170256507825) * t353 + f64x8::splat(9.19317034e-07) * t426 + f64x8::splat(9.19317034e-07) * t428 + f64x8::splat(3.97324768e-09) * t410 * t100 + f64x8::splat(3.97324768e-09) * t117 * t396 - f64x8::splat(3.40722258e-09) * t97 * t396 + f64x8::splat(5.74317889e-08) * t425 * t105 + f64x8::splat(5.74317889e-08) * t97 * t380 - f64x8::splat(5.00749348e-07) * t425 * t109 - f64x8::splat(3.40722258e-09) * t425 * t100 - f64x8::splat(0.032392026266666665) * t349;
            let t447 = t393 - f64x8::splat(1.00478906e-07) * t48 * t396 + f64x8::splat(0.0192374554) * t399 + f64x8::splat(2.01895739e-07) * t127 * t396 - f64x8::splat(6.57949254e-07) * t127 * t380 - f64x8::splat(0.150848736) * t405 - f64x8::splat(5.49909413e-08) * t410 * t105 - f64x8::splat(5.49909413e-08) * t117 * t380 + f64x8::splat(1.33707403e-07) * t410 * t109 + f64x8::splat(0.0192374554) * t417 + f64x8::splat(0.024747993725) * t419 + t445;
            let t452 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t283 * t145 - t292 - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t28 * t447));
            let t453 = t151 * t276;
            let t455 = ((t15).select(f64x8::splat(0.0), (t11).select(f64x8::splat(0.0), -t8 - t453)));
            let t458 = ((t155).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t156 * t455));
            let t459 = t458 * t27;
            let t463 = t158 * t288;
            let t466 = t6 * t463 * t270 / f64x8::splat(8.0);
            let t468 = ((t150).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t459 * t270 - t466));
            let tvrho0 = t149 + t274 + t7 * (t452 + t468);
            acc_vrho_0 = tvrho0;
            let t472 = ((t11).select(f64x8::splat(0.0), (t15).select(f64x8::splat(0.0), -t8 - t277)));
            let t475 = ((t21).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t24 * t472));
            let t476 = t475 * t27;
            let t481 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t476 * t145 - t292));
            let t483 = ((t15).select(f64x8::splat(0.0), (t11).select(f64x8::splat(0.0), t8 - t453)));
            let t486 = ((t155).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t156 * t483));
            let t487 = t486 * t27;
            let t493 = t160 * v_rho1;
            let t495 = f64x8::splat(1.0) / t162 / t493;
            let t496 = v_sigma2 * t495;
            let t501 = f64x8::splat(5.0) / f64x8::splat(9.0) * (-f64x8::splat(5.0) / f64x8::splat(3.0) * v_tau1 * t164 + t496 / f64x8::splat(3.0)) * t29 * t33;
            let t502 = ((t187).select(t501, f64x8::splat(0.0)));
            let t505 = t195 * t502;
            let t508 = f64x8::splat(1.0) / t194 / t188;
            let t509 = t508 * t502;
            let t512 = t201 * t207;
            let t513 = ((t187).select(f64x8::splat(0.0), t501));
            let t514 = t198 * t513;
            let t517 = t206 * t206;
            let t518 = f64x8::splat(1.0) / t517;
            let t519 = t202 * t518;
            let t520 = t199 * t204;
            let t522 = t199 * t199;
            let t523 = t522 * t198;
            let t526 = f64x8::splat(3.0) * t520 * t513 + f64x8::splat(3.0) * t523 * t513;
            let t529 = ((t186).select(f64x8::splat(6.0) * t193 * t502 + f64x8::splat(3.0) * t505 - f64x8::splat(12.0) * t509, f64x8::splat(6.0) * t512 * t514 + t519 * t526));
            let t532 = t34 * t496 * t169;
            let t534 = v_sigma2 * v_sigma2;
            let t535 = t160 * t160;
            let t536 = t535 * t160;
            let t538 = f64x8::splat(1.0) / t161 / t536;
            let t540 = t168 * t168;
            let t541 = f64x8::splat(1.0) / t540;
            let t543 = t336 * t534 * t538 * t541;
            let t547 = -f64x8::splat(2.0) / f64x8::splat(9.0) * t532 + t543 / f64x8::splat(108.0);
            let t548 = t174 * t547;
            let t565 = t212 * t529;
            let t569 = f64x8::splat(15.0) / f64x8::splat(2.0) * t565 - f64x8::splat(3.0) / f64x8::splat(2.0) * t529;
            let t572 = t175 * t547;
            let t577 = f64x8::splat(15.0) / f64x8::splat(2.0) * t572 + t532 / f64x8::splat(3.0) - t543 / f64x8::splat(72.0);
            let t580 = t218 * t529;
            let t582 = t209 * t529;
            let t586 = f64x8::splat(35.0) / f64x8::splat(2.0) * t580 - f64x8::splat(15.0) / f64x8::splat(2.0) * t582;
            let t589 = t577 * t209;
            let t591 = t260 * t529;
            let t593 = -f64x8::splat(0.06972770593) * t529 - f64x8::splat(0.032392026266666665) * t532 + f64x8::splat(0.0013496677611111111) * t543 - f64x8::splat(1.973847762e-06) * t548 * t230 + f64x8::splat(6.05687217e-07) * t548 * t225 + f64x8::splat(4.01122209e-07) * t261 * t529 - f64x8::splat(1.502248044e-06) * t238 * t529 + f64x8::splat(0.0954072288) * t240 * t529 - f64x8::splat(0.0667950417) * t548 * t209 - f64x8::splat(0.01565454237) * t548 * t234 - f64x8::splat(0.01565454237) * t250 * t529 + f64x8::splat(0.01055956065) * t565 - f64x8::splat(5.49909413e-08) * t260 * t569 - f64x8::splat(0.150848736) * t572 + f64x8::splat(3.97324768e-09) * t577 * t225 + f64x8::splat(0.002476783505) * t580 + f64x8::splat(0.043536371955) * t582 + f64x8::splat(3.97324768e-09) * t260 * t586 + f64x8::splat(0.0192374554) * t589 + f64x8::splat(0.0192374554) * t591;
            let t604 = t249 * t529;
            let t606 = t547 * t234;
            let t608 = t547 * t230;
            let t612 = t547 * t225;
            let t615 = t215 * t547;
            let t620 = f64x8::splat(35.0) / f64x8::splat(2.0) * t615 - f64x8::splat(15.0) / f64x8::splat(2.0) * t548;
            let t623 = t620 * t209;
            let t625 = t222 * t529;
            let t627 = t547 * t209;
            let t629 = t174 * t529;
            let t639 = f64x8::splat(0.024747993725) * t615 - f64x8::splat(0.170256507825) * t548 - f64x8::splat(5.00749348e-07) * t620 * t234 + f64x8::splat(9.19317034e-07) * t623 + f64x8::splat(9.19317034e-07) * t625 + f64x8::splat(0.0453837246) * t627 + f64x8::splat(0.0453837246) * t629 - f64x8::splat(3.40722258e-09) * t620 * t225 - f64x8::splat(3.40722258e-09) * t222 * t586 + f64x8::splat(5.74317889e-08) * t620 * t230 + f64x8::splat(5.74317889e-08) * t222 * t569;
            let t641 = t593 + f64x8::splat(1.33707403e-07) * t577 * t234 - f64x8::splat(5.49909413e-08) * t577 * t230 - f64x8::splat(6.57949254e-07) * t249 * t569 + f64x8::splat(2.01895739e-07) * t249 * t586 - f64x8::splat(1.00478906e-07) * t174 * t586 - f64x8::splat(0.0222650139) * t604 + f64x8::splat(0.0318024096) * t606 - f64x8::splat(0.00608338264) * t608 - f64x8::splat(0.00608338264) * t174 * t569 - f64x8::splat(1.00478906e-07) * t612 + t639;
            let t646 = ((t150).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t487 * t270 - t466 - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t159 * t641));
            let tvrho1 = t149 + t274 + t7 * (t481 + t646);
            acc_vrho_1 = tvrho1;
            let t650 = t34 * t39 * t44;
            let t652 = t338 * v_rho0;
            let t654 = f64x8::splat(1.0) / t36 / t652;
            let t657 = t336 * v_sigma0 * t654 * t344;
            let t659 = t34 * t39;
            let t660 = f64x8::splat(5.0) / f64x8::splat(72.0) * t659;
            let t661 = ((t65).select(-t660, f64x8::splat(0.0)));
            let t664 = t73 * t661;
            let t666 = t310 * t661;
            let t669 = ((t65).select(f64x8::splat(0.0), -t660));
            let t670 = t76 * t669;
            let t676 = f64x8::splat(3.0) * t322 * t669 + f64x8::splat(3.0) * t325 * t669;
            let t679 = ((t64).select(f64x8::splat(6.0) * t71 * t661 + f64x8::splat(3.0) * t664 - f64x8::splat(12.0) * t666, f64x8::splat(6.0) * t314 * t670 + t321 * t676));
            let t683 = t650 / f64x8::splat(12.0) - t657 / f64x8::splat(288.0);
            let t684 = t48 * t683;
            let t701 = t683 * t87;
            let t703 = t48 * t679;
            let t705 = t683 * t105;
            let t707 = t89 * t679;
            let t711 = f64x8::splat(15.0) / f64x8::splat(2.0) * t707 - f64x8::splat(3.0) / f64x8::splat(2.0) * t679;
            let t714 = t683 * t109;
            let t716 = t127 * t679;
            let t718 = t683 * t100;
            let t720 = t91 * t679;
            let t722 = f64x8::splat(0.01214700985) * t650 - f64x8::splat(0.0005061254104166666) * t657 - f64x8::splat(0.06972770593) * t679 + f64x8::splat(6.05687217e-07) * t684 * t100 - f64x8::splat(1.973847762e-06) * t684 * t105 - f64x8::splat(0.01565454237) * t684 * t109 - f64x8::splat(0.01565454237) * t134 * t679 - f64x8::splat(0.0667950417) * t684 * t87 + f64x8::splat(0.0954072288) * t142 * t679 - f64x8::splat(1.502248044e-06) * t112 * t679 + f64x8::splat(4.01122209e-07) * t124 * t679 + f64x8::splat(0.0453837246) * t701 + f64x8::splat(0.0453837246) * t703 - f64x8::splat(0.00608338264) * t705 + f64x8::splat(0.01055956065) * t707 - f64x8::splat(0.00608338264) * t48 * t711 + f64x8::splat(0.0318024096) * t714 - f64x8::splat(0.0222650139) * t716 - f64x8::splat(1.00478906e-07) * t718 + f64x8::splat(0.002476783505) * t720;
            let t723 = t87 * t679;
            let t727 = f64x8::splat(35.0) / f64x8::splat(2.0) * t720 - f64x8::splat(15.0) / f64x8::splat(2.0) * t723;
            let t730 = t49 * t683;
            let t735 = f64x8::splat(15.0) / f64x8::splat(2.0) * t730 - t650 / f64x8::splat(8.0) + t657 / f64x8::splat(192.0);
            let t736 = t735 * t87;
            let t738 = t117 * t679;
            let t751 = t51 * t683;
            let t756 = f64x8::splat(35.0) / f64x8::splat(2.0) * t751 - f64x8::splat(15.0) / f64x8::splat(2.0) * t684;
            let t759 = t756 * t87;
            let t761 = t97 * t679;
            let t775 = f64x8::splat(0.024747993725) * t751 - f64x8::splat(0.170256507825) * t684 - f64x8::splat(5.00749348e-07) * t756 * t109 + f64x8::splat(9.19317034e-07) * t759 + f64x8::splat(9.19317034e-07) * t761 + f64x8::splat(3.97324768e-09) * t735 * t100 + f64x8::splat(3.97324768e-09) * t117 * t727 - f64x8::splat(3.40722258e-09) * t756 * t100 - f64x8::splat(3.40722258e-09) * t97 * t727 + f64x8::splat(5.74317889e-08) * t756 * t105 + f64x8::splat(5.74317889e-08) * t97 * t711;
            let t777 = t722 + f64x8::splat(0.043536371955) * t723 - f64x8::splat(1.00478906e-07) * t48 * t727 - f64x8::splat(0.150848736) * t730 + f64x8::splat(0.0192374554) * t736 + f64x8::splat(0.0192374554) * t738 + f64x8::splat(2.01895739e-07) * t127 * t727 - f64x8::splat(6.57949254e-07) * t127 * t711 - f64x8::splat(5.49909413e-08) * t735 * t105 - f64x8::splat(5.49909413e-08) * t117 * t711 + f64x8::splat(1.33707403e-07) * t735 * t109 + t775;
            let t781 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t28 * t777));
            let tvsigma0 = t7 * t781;
            acc_vsigma_0 = tvsigma0;
            let tvsigma1 = f64x8::splat(0.0);
            acc_vsigma_1 = tvsigma1;
            let t783 = t34 * t164 * t169;
            let t785 = t34 * t164;
            let t786 = f64x8::splat(5.0) / f64x8::splat(72.0) * t785;
            let t787 = ((t187).select(-t786, f64x8::splat(0.0)));
            let t790 = t195 * t787;
            let t792 = t508 * t787;
            let t795 = ((t187).select(f64x8::splat(0.0), -t786));
            let t796 = t198 * t795;
            let t802 = f64x8::splat(3.0) * t520 * t795 + f64x8::splat(3.0) * t523 * t795;
            let t805 = ((t186).select(f64x8::splat(6.0) * t193 * t787 + f64x8::splat(3.0) * t790 - f64x8::splat(12.0) * t792, f64x8::splat(6.0) * t512 * t796 + t519 * t802));
            let t807 = t535 * v_rho1;
            let t809 = f64x8::splat(1.0) / t161 / t807;
            let t812 = t336 * v_sigma2 * t809 * t541;
            let t816 = t783 / f64x8::splat(12.0) - t812 / f64x8::splat(288.0);
            let t817 = t174 * t816;
            let t834 = t215 * t816;
            let t836 = t175 * t816;
            let t839 = t218 * t805;
            let t843 = f64x8::splat(35.0) / f64x8::splat(2.0) * t834 - f64x8::splat(15.0) / f64x8::splat(2.0) * t817;
            let t846 = t209 * t805;
            let t850 = f64x8::splat(35.0) / f64x8::splat(2.0) * t839 - f64x8::splat(15.0) / f64x8::splat(2.0) * t846;
            let t855 = t212 * t805;
            let t857 = f64x8::splat(0.01214700985) * t783 - f64x8::splat(0.06972770593) * t805 - f64x8::splat(0.0005061254104166666) * t812 - f64x8::splat(0.0667950417) * t817 * t209 - f64x8::splat(0.01565454237) * t817 * t234 - f64x8::splat(0.01565454237) * t250 * t805 - f64x8::splat(1.973847762e-06) * t817 * t230 + f64x8::splat(6.05687217e-07) * t817 * t225 + f64x8::splat(4.01122209e-07) * t261 * t805 - f64x8::splat(1.502248044e-06) * t238 * t805 + f64x8::splat(0.0954072288) * t240 * t805 + f64x8::splat(0.024747993725) * t834 - f64x8::splat(0.150848736) * t836 - f64x8::splat(0.170256507825) * t817 + f64x8::splat(0.002476783505) * t839 - f64x8::splat(3.40722258e-09) * t843 * t225 + f64x8::splat(0.043536371955) * t846 - f64x8::splat(3.40722258e-09) * t222 * t850 + f64x8::splat(5.74317889e-08) * t843 * t230 + f64x8::splat(0.01055956065) * t855;
            let t860 = f64x8::splat(15.0) / f64x8::splat(2.0) * t855 - f64x8::splat(3.0) / f64x8::splat(2.0) * t805;
            let t865 = t843 * t209;
            let t867 = t222 * t805;
            let t869 = t816 * t209;
            let t871 = t174 * t805;
            let t873 = t816 * t234;
            let t875 = t816 * t230;
            let t879 = t816 * t225;
            let t884 = t249 * t805;
            let t893 = f64x8::splat(15.0) / f64x8::splat(2.0) * t836 - t783 / f64x8::splat(8.0) + t812 / f64x8::splat(192.0);
            let t894 = t893 * t209;
            let t896 = t260 * t805;
            let t908 = -f64x8::splat(1.00478906e-07) * t174 * t850 - f64x8::splat(0.0222650139) * t884 - f64x8::splat(6.57949254e-07) * t249 * t860 + f64x8::splat(2.01895739e-07) * t249 * t850 + f64x8::splat(0.0192374554) * t894 + f64x8::splat(0.0192374554) * t896 + f64x8::splat(1.33707403e-07) * t893 * t234 - f64x8::splat(5.49909413e-08) * t893 * t230 - f64x8::splat(5.49909413e-08) * t260 * t860 + f64x8::splat(3.97324768e-09) * t893 * t225 + f64x8::splat(3.97324768e-09) * t260 * t850;
            let t910 = t857 + f64x8::splat(5.74317889e-08) * t222 * t860 - f64x8::splat(5.00749348e-07) * t843 * t234 + f64x8::splat(9.19317034e-07) * t865 + f64x8::splat(9.19317034e-07) * t867 + f64x8::splat(0.0453837246) * t869 + f64x8::splat(0.0453837246) * t871 + f64x8::splat(0.0318024096) * t873 - f64x8::splat(0.00608338264) * t875 - f64x8::splat(0.00608338264) * t174 * t860 - f64x8::splat(1.00478906e-07) * t879 + t908;
            let t914 = ((t150).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t159 * t910));
            let tvsigma2 = t7 * t914;
            acc_vsigma_2 = tvsigma2;
            let tvlapl0 = f64x8::splat(0.0);
            acc_vlapl_0 = tvlapl0;
            let tvlapl1 = f64x8::splat(0.0);
            acc_vlapl_1 = tvlapl1;
            let t917 = f64x8::splat(5.0) / f64x8::splat(9.0) * t57 * t29 * t33;
            let t918 = ((t65).select(t917, f64x8::splat(0.0)));
            let t921 = t73 * t918;
            let t923 = t310 * t918;
            let t926 = ((t65).select(f64x8::splat(0.0), t917));
            let t927 = t76 * t926;
            let t933 = f64x8::splat(3.0) * t322 * t926 + f64x8::splat(3.0) * t325 * t926;
            let t936 = ((t64).select(f64x8::splat(6.0) * t71 * t918 + f64x8::splat(3.0) * t921 - f64x8::splat(12.0) * t923, f64x8::splat(6.0) * t314 * t927 + t321 * t933));
            let t937 = t91 * t936;
            let t939 = t87 * t936;
            let t941 = f64x8::splat(35.0) / f64x8::splat(2.0) * t937 - f64x8::splat(15.0) / f64x8::splat(2.0) * t939;
            let t944 = t89 * t936;
            let t947 = f64x8::splat(15.0) / f64x8::splat(2.0) * t944 - f64x8::splat(3.0) / f64x8::splat(2.0) * t936;
            let t952 = t97 * t936;
            let t960 = t117 * t936;
            let t968 = t127 * t936;
            let t979 = t48 * t936;
            let t982 = -f64x8::splat(3.40722258e-09) * t97 * t941 + f64x8::splat(5.74317889e-08) * t97 * t947 - f64x8::splat(1.502248044e-06) * t112 * t936 + f64x8::splat(9.19317034e-07) * t952 + f64x8::splat(3.97324768e-09) * t117 * t941 - f64x8::splat(5.49909413e-08) * t117 * t947 + f64x8::splat(4.01122209e-07) * t124 * t936 + f64x8::splat(0.0192374554) * t960 + f64x8::splat(2.01895739e-07) * t127 * t941 - f64x8::splat(6.57949254e-07) * t127 * t947 - f64x8::splat(0.01565454237) * t134 * t936 - f64x8::splat(0.0222650139) * t968 - f64x8::splat(1.00478906e-07) * t48 * t941 - f64x8::splat(0.00608338264) * t48 * t947 + f64x8::splat(0.0954072288) * t142 * t936 + f64x8::splat(0.002476783505) * t937 + f64x8::splat(0.01055956065) * t944 + f64x8::splat(0.043536371955) * t939 + f64x8::splat(0.0453837246) * t979 - f64x8::splat(0.06972770593) * t936;
            let t986 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t28 * t982));
            let tvtau0 = t7 * t986;
            acc_vtau_0 = tvtau0;
            let t989 = f64x8::splat(5.0) / f64x8::splat(9.0) * t179 * t29 * t33;
            let t990 = ((t187).select(t989, f64x8::splat(0.0)));
            let t993 = t195 * t990;
            let t995 = t508 * t990;
            let t998 = ((t187).select(f64x8::splat(0.0), t989));
            let t999 = t198 * t998;
            let t1005 = f64x8::splat(3.0) * t520 * t998 + f64x8::splat(3.0) * t523 * t998;
            let t1008 = ((t186).select(f64x8::splat(6.0) * t193 * t990 + f64x8::splat(3.0) * t993 - f64x8::splat(12.0) * t995, t519 * t1005 + f64x8::splat(6.0) * t512 * t999));
            let t1009 = t218 * t1008;
            let t1012 = t209 * t1008;
            let t1014 = f64x8::splat(35.0) / f64x8::splat(2.0) * t1009 - f64x8::splat(15.0) / f64x8::splat(2.0) * t1012;
            let t1017 = t212 * t1008;
            let t1020 = f64x8::splat(15.0) / f64x8::splat(2.0) * t1017 - f64x8::splat(3.0) / f64x8::splat(2.0) * t1008;
            let t1025 = t222 * t1008;
            let t1027 = t174 * t1008;
            let t1036 = t249 * t1008;
            let t1046 = t260 * t1008;
            let t1054 = f64x8::splat(0.002476783505) * t1009 - f64x8::splat(3.40722258e-09) * t222 * t1014 + f64x8::splat(5.74317889e-08) * t222 * t1020 - f64x8::splat(1.502248044e-06) * t238 * t1008 + f64x8::splat(9.19317034e-07) * t1025 + f64x8::splat(0.0453837246) * t1027 + f64x8::splat(0.0954072288) * t240 * t1008 - f64x8::splat(0.00608338264) * t174 * t1020 - f64x8::splat(0.06972770593) * t1008 - f64x8::splat(1.00478906e-07) * t174 * t1014 - f64x8::splat(0.0222650139) * t1036 - f64x8::splat(0.01565454237) * t250 * t1008 - f64x8::splat(6.57949254e-07) * t249 * t1020 + f64x8::splat(2.01895739e-07) * t249 * t1014 + f64x8::splat(0.043536371955) * t1012 + f64x8::splat(0.01055956065) * t1017 + f64x8::splat(0.0192374554) * t1046 + f64x8::splat(4.01122209e-07) * t261 * t1008 - f64x8::splat(5.49909413e-08) * t260 * t1020 + f64x8::splat(3.97324768e-09) * t260 * t1014;
            let t1058 = ((t150).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t159 * t1054));
            let tvtau1 = t7 * t1058;
            acc_vtau_1 = tvtau1;
        }
        store_add(zk, ip, m, acc_zk);
        store_strided(vrho, ip, m, 2, 0, acc_vrho_0);
        store_strided(vrho, ip, m, 2, 1, acc_vrho_1);
        store_strided(vsigma, ip, m, 3, 0, acc_vsigma_0);
        store_strided(vsigma, ip, m, 3, 1, acc_vsigma_1);
        store_strided(vsigma, ip, m, 3, 2, acc_vsigma_2);
        store_strided(vlapl, ip, m, 2, 0, acc_vlapl_0);
        store_strided(vlapl, ip, m, 2, 1, acc_vlapl_1);
        store_strided(vtau, ip, m, 2, 0, acc_vtau_0);
        store_strided(vtau, ip, m, 2, 1, acc_vtau_1);
        ip += 8;
    }
}
