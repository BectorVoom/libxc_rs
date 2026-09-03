//! MGGA_X_VCML vxc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_vcml.c`
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
pub fn mgga_x_vcml_vxc_pol(
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
            let t29 = (simd::cbrt(v_rho0));
            let t30 = t29 * t29;
            let t32 = f64x8::splat(1.0) / t30 / v_rho0;
            let t34 = v_rho0 * v_rho0;
            let t36 = f64x8::splat(1.0) / t30 / t34;
            let t37 = v_sigma0 * t36;
            let t40 = f64x8::splat(M_CBRT6);
            let t42 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t43 = (simd::cbrt(t42));
            let t44 = t43 * t43;
            let t45 = f64x8::splat(1.0) / t44;
            let t47 = f64x8::splat(5.0) / f64x8::splat(9.0) * (v_tau0 * t32 - t37 / f64x8::splat(8.0)) * t40 * t45;
            let t48 = (f64x8::splat(10000.0)).simd_le(t47);
            let t49 = (f64x8::splat(10000.0)).simd_lt(t47);
            let t50 = ((t49).select(t47, f64x8::splat(10000.0)));
            let t51 = t50 * t50;
            let t54 = t51 * t50;
            let t55 = f64x8::splat(1.0) / t54;
            let t57 = t51 * t51;
            let t58 = f64x8::splat(1.0) / t57;
            let t61 = ((t49).select(f64x8::splat(10000.0), t47));
            let t62 = t61 * t61;
            let t63 = f64x8::splat(1.0) - t62;
            let t64 = t63 * t63;
            let t65 = t64 * t63;
            let t66 = t62 * t61;
            let t68 = f64x8::splat(1.0) + f64x8::splat(4.0) * t66;
            let t70 = t66 * t68 + f64x8::splat(1.0);
            let t71 = f64x8::splat(1.0) / t70;
            let t73 = ((t48).select(f64x8::splat(3.0) / f64x8::splat(4.0) / t51 + t55 / f64x8::splat(16.0) - f64x8::splat(3.0) / f64x8::splat(4.0) * t58 - f64x8::splat(1.0) / f64x8::splat(4.0), t65 * t71));
            let t74 = t73 * t73;
            let t75 = t74 * t74;
            let t79 = t75 * t73;
            let t81 = t74 * t73;
            let t83 = t75 * t74;
            let t85 = t40 * t45;
            let t88 = f64x8::splat(6.5124) + t85 * t37 / f64x8::splat(24.0);
            let t89 = f64x8::splat(1.0) / t88;
            let t91 = t85 * t37 * t89;
            let t93 = t91 / f64x8::splat(12.0) - f64x8::splat(1.0);
            let t94 = t93 * t93;
            let t95 = t94 * t93;
            let t96 = t94 * t94;
            let t97 = t96 * t95;
            let t100 = t96 * t94;
            let t102 = t96 * t93;
            let t109 = f64x8::splat(63.0) / f64x8::splat(8.0) * t102 - f64x8::splat(35.0) / f64x8::splat(4.0) * t95 + f64x8::splat(5.0) / f64x8::splat(32.0) * t91 - f64x8::splat(15.0) / f64x8::splat(8.0);
            let t110 = t75 * t81;
            let t115 = f64x8::splat(429.0) / f64x8::splat(16.0) * t110 - f64x8::splat(693.0) / f64x8::splat(16.0) * t79 + f64x8::splat(315.0) / f64x8::splat(16.0) * t81 - f64x8::splat(35.0) / f64x8::splat(16.0) * t73;
            let t121 = -f64x8::splat(5.0) / f64x8::splat(16.0) + f64x8::splat(231.0) / f64x8::splat(16.0) * t83 - f64x8::splat(315.0) / f64x8::splat(16.0) * t75 + f64x8::splat(105.0) / f64x8::splat(16.0) * t74;
            let t127 = f64x8::splat(63.0) / f64x8::splat(8.0) * t79 - f64x8::splat(35.0) / f64x8::splat(4.0) * t81 + f64x8::splat(15.0) / f64x8::splat(8.0) * t73;
            let t132 = f64x8::splat(3.0) / f64x8::splat(8.0) + f64x8::splat(35.0) / f64x8::splat(8.0) * t75 - f64x8::splat(15.0) / f64x8::splat(4.0) * t74;
            let t135 = -f64x8::splat(0.01228729376505733) * t75 + f64x8::splat(0.0063559222793315405) * t74 + f64x8::splat(0.19451907596748125) * t73 + f64x8::splat(0.05227978382970764) * t79 - f64x8::splat(0.005923137049970073) * t81 + f64x8::splat(0.004414255398135769) * t83 - f64x8::splat(0.04020419785403348) * t97 - f64x8::splat(0.38230940935406266) * t94 + f64x8::splat(0.07300061073803556) * t100 + f64x8::splat(0.050197247070683314) * t102 - f64x8::splat(0.00804750729891458) * t95 - f64x8::splat(0.011145877912279912) * t91 - f64x8::splat(0.0005194058669188706) * t109 * t115 - f64x8::splat(0.007555456486598222) * t109 * t121 - f64x8::splat(0.0038541498256550073) * t109 * t127 - f64x8::splat(0.0010249162124576494) * t109 * t132;
            let t138 = f64x8::splat(5.0) / f64x8::splat(2.0) * t81 - f64x8::splat(3.0) / f64x8::splat(2.0) * t73;
            let t142 = -f64x8::splat(1.0) / f64x8::splat(2.0) + f64x8::splat(3.0) / f64x8::splat(2.0) * t74;
            let t145 = t109 * t73;
            let t149 = f64x8::splat(3.0) / f64x8::splat(8.0) + f64x8::splat(35.0) / f64x8::splat(8.0) * t96 - f64x8::splat(15.0) / f64x8::splat(4.0) * t94;
            let t158 = f64x8::splat(429.0) / f64x8::splat(16.0) * t97 - f64x8::splat(693.0) / f64x8::splat(16.0) * t102 + f64x8::splat(315.0) / f64x8::splat(16.0) * t95 - f64x8::splat(35.0) / f64x8::splat(192.0) * t91 + f64x8::splat(35.0) / f64x8::splat(16.0);
            let t173 = -f64x8::splat(1.0) / f64x8::splat(2.0) + f64x8::splat(3.0) / f64x8::splat(2.0) * t94;
            let t182 = -f64x8::splat(3.656012084198544e-05) * t109 * t138 + f64x8::splat(0.005061925051098745) * t109 * t142 - f64x8::splat(0.0016609256494831233) * t145 - f64x8::splat(1.792697304428732e-05) * t149 * t115 + f64x8::splat(0.0001331797359718674) * t149 * t121 - f64x8::splat(0.00029476504977320184) * t158 * t115 - f64x8::splat(0.00019095139973664826) * t158 * t121 + f64x8::splat(0.0038758929812102785) * t158 * t127 - f64x8::splat(0.00031389079758955066) * t158 * t132 + f64x8::splat(0.010726279571787276) * t158 * t138 - f64x8::splat(0.01006770315965861) * t158 * t142 - f64x8::splat(0.0570844762417126) * t96 + f64x8::splat(0.00018939021743243079) * t173 * t115 - f64x8::splat(0.0009048853909642742) * t173 * t121 + f64x8::splat(8.482767148525194e-05) * t173 * t127 + f64x8::splat(0.0003180493235941731) * t173 * t132;
            let t188 = t173 * t73;
            let t202 = t93 * t73;
            let t204 = t158 * t73;
            let t209 = -f64x8::splat(5.0) / f64x8::splat(16.0) + f64x8::splat(231.0) / f64x8::splat(16.0) * t100 - f64x8::splat(315.0) / f64x8::splat(16.0) * t96 + f64x8::splat(105.0) / f64x8::splat(16.0) * t94;
            let t220 = -f64x8::splat(0.0008670535705479461) * t173 * t138 - f64x8::splat(0.000835331263170036) * t173 * t142 - f64x8::splat(0.013135604251829597) * t188 + f64x8::splat(0.0023160016166370034) * t93 * t115 + f64x8::splat(0.0005970286163074767) * t93 * t121 + f64x8::splat(0.0016437722411542371) * t93 * t127 + f64x8::splat(0.0050995906979556666) * t93 * t132 + f64x8::splat(0.0024977311122498513) * t93 * t138 + f64x8::splat(0.0012341314639045392) * t93 * t142 + f64x8::splat(0.12131628073942294) * t202 + f64x8::splat(0.00017309630990864668) * t204 - f64x8::splat(0.00018156466410673526) * t209 * t115 + f64x8::splat(0.001864317026752979) * t209 * t121 - f64x8::splat(0.0031296536914037784) * t209 * t127 + f64x8::splat(0.0008367073496483024) * t209 * t132 - f64x8::splat(0.009195715678311926) * t209 * t138;
            let t223 = t209 * t73;
            let t233 = t149 * t73;
            let t237 = f64x8::splat(5.0) / f64x8::splat(2.0) * t95 - t91 / f64x8::splat(8.0) + f64x8::splat(3.0) / f64x8::splat(2.0);
            let t250 = t237 * t73;
            let t253 = f64x8::splat(1.3669196781387443) - f64x8::splat(0.007631605623646023) * t209 * t142 + f64x8::splat(0.0028206838819829017) * t223 - f64x8::splat(7.261106354828029e-05) * t149 * t127 + f64x8::splat(0.0009891355730978566) * t149 * t132 - f64x8::splat(0.0002571281595426713) * t149 * t138 - f64x8::splat(0.0014878680171769923) * t149 * t142 - f64x8::splat(0.0021100890252897446) * t233 + f64x8::splat(0.0004308565933608885) * t237 * t115 - f64x8::splat(0.000689695394243961) * t237 * t121 - f64x8::splat(0.00019375881298946268) * t237 * t127 - f64x8::splat(0.004704436332280876) * t237 * t132 + f64x8::splat(0.0027822064319562786) * t237 * t138 - f64x8::splat(7.823588139015819e-05) * t237 * t142 - f64x8::splat(0.016823429546012295) * t250 - f64x8::splat(0.05430381430310407) * t110;
            let t255 = t135 + t182 + t220 + t253;
            let t259 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t28 * t255));
            let t260 = (v_rho1).simd_le(dens_threshold);
            let t261 = -t17;
            let t263 = ((t15).select(t12, (t11).select(t16, t261 * t8)));
            let t264 = t263 + f64x8::splat(1.0);
            let t265 = (t264).simd_le(zeta_threshold);
            let t266 = (simd::cbrt(t264));
            let t268 = ((t265).select(t23, t266 * t264));
            let t269 = t268 * t27;
            let t270 = v_rho1 * v_rho1;
            let t271 = (simd::cbrt(v_rho1));
            let t272 = t271 * t271;
            let t274 = f64x8::splat(1.0) / t272 / t270;
            let t275 = v_sigma2 * t274;
            let t278 = f64x8::splat(6.5124) + t85 * t275 / f64x8::splat(24.0);
            let t279 = f64x8::splat(1.0) / t278;
            let t281 = t85 * t275 * t279;
            let t283 = t281 / f64x8::splat(12.0) - f64x8::splat(1.0);
            let t284 = t283 * t283;
            let t285 = t284 * t284;
            let t286 = t285 * t284;
            let t288 = t284 * t283;
            let t289 = t285 * t288;
            let t292 = f64x8::splat(1.0) / t272 / v_rho1;
            let t298 = f64x8::splat(5.0) / f64x8::splat(9.0) * (v_tau1 * t292 - t275 / f64x8::splat(8.0)) * t40 * t45;
            let t299 = (f64x8::splat(10000.0)).simd_le(t298);
            let t300 = (f64x8::splat(10000.0)).simd_lt(t298);
            let t301 = ((t300).select(t298, f64x8::splat(10000.0)));
            let t302 = t301 * t301;
            let t305 = t302 * t301;
            let t306 = f64x8::splat(1.0) / t305;
            let t308 = t302 * t302;
            let t309 = f64x8::splat(1.0) / t308;
            let t312 = ((t300).select(f64x8::splat(10000.0), t298));
            let t313 = t312 * t312;
            let t314 = f64x8::splat(1.0) - t313;
            let t315 = t314 * t314;
            let t316 = t315 * t314;
            let t317 = t313 * t312;
            let t319 = f64x8::splat(1.0) + f64x8::splat(4.0) * t317;
            let t321 = t317 * t319 + f64x8::splat(1.0);
            let t322 = f64x8::splat(1.0) / t321;
            let t324 = ((t299).select(-f64x8::splat(1.0) / f64x8::splat(4.0) + f64x8::splat(3.0) / f64x8::splat(4.0) / t302 + t306 / f64x8::splat(16.0) - f64x8::splat(3.0) / f64x8::splat(4.0) * t309, t316 * t322));
            let t326 = t324 * t324;
            let t327 = t326 * t326;
            let t328 = t327 * t324;
            let t330 = t326 * t324;
            let t333 = t327 * t330;
            let t336 = t285 * t283;
            let t342 = f64x8::splat(63.0) / f64x8::splat(8.0) * t336 - f64x8::splat(35.0) / f64x8::splat(4.0) * t288 + f64x8::splat(5.0) / f64x8::splat(32.0) * t281 - f64x8::splat(15.0) / f64x8::splat(8.0);
            let t344 = -f64x8::splat(1.0) / f64x8::splat(2.0) + f64x8::splat(3.0) / f64x8::splat(2.0) * t326;
            let t347 = t342 * t324;
            let t351 = f64x8::splat(3.0) / f64x8::splat(8.0) + f64x8::splat(35.0) / f64x8::splat(8.0) * t285 - f64x8::splat(15.0) / f64x8::splat(4.0) * t284;
            let t356 = f64x8::splat(429.0) / f64x8::splat(16.0) * t333 - f64x8::splat(693.0) / f64x8::splat(16.0) * t328 + f64x8::splat(315.0) / f64x8::splat(16.0) * t330 - f64x8::splat(35.0) / f64x8::splat(16.0) * t324;
            let t359 = t327 * t326;
            let t363 = -f64x8::splat(5.0) / f64x8::splat(16.0) + f64x8::splat(231.0) / f64x8::splat(16.0) * t359 - f64x8::splat(315.0) / f64x8::splat(16.0) * t327 + f64x8::splat(105.0) / f64x8::splat(16.0) * t326;
            let t369 = f64x8::splat(63.0) / f64x8::splat(8.0) * t328 - f64x8::splat(35.0) / f64x8::splat(4.0) * t330 + f64x8::splat(15.0) / f64x8::splat(8.0) * t324;
            let t374 = f64x8::splat(3.0) / f64x8::splat(8.0) + f64x8::splat(35.0) / f64x8::splat(8.0) * t327 - f64x8::splat(15.0) / f64x8::splat(4.0) * t326;
            let t377 = f64x8::splat(0.07300061073803556) * t286 - f64x8::splat(0.04020419785403348) * t289 + f64x8::splat(0.19451907596748125) * t324 + f64x8::splat(0.05227978382970764) * t328 - f64x8::splat(0.005923137049970073) * t330 - f64x8::splat(0.0570844762417126) * t285 - f64x8::splat(0.05430381430310407) * t333 - f64x8::splat(0.011145877912279912) * t281 + f64x8::splat(0.050197247070683314) * t336 - f64x8::splat(0.00804750729891458) * t288 + f64x8::splat(0.005061925051098745) * t342 * t344 - f64x8::splat(0.0016609256494831233) * t347 - f64x8::splat(1.792697304428732e-05) * t351 * t356 + f64x8::splat(0.0001331797359718674) * t351 * t363 - f64x8::splat(7.261106354828029e-05) * t351 * t369 + f64x8::splat(0.0009891355730978566) * t351 * t374;
            let t380 = f64x8::splat(5.0) / f64x8::splat(2.0) * t330 - f64x8::splat(3.0) / f64x8::splat(2.0) * t324;
            let t385 = t351 * t324;
            let t389 = f64x8::splat(5.0) / f64x8::splat(2.0) * t288 - t281 / f64x8::splat(8.0) + f64x8::splat(3.0) / f64x8::splat(2.0);
            let t399 = -f64x8::splat(1.0) / f64x8::splat(2.0) + f64x8::splat(3.0) / f64x8::splat(2.0) * t284;
            let t400 = t399 * t324;
            let t414 = t389 * t324;
            let t418 = -f64x8::splat(0.0002571281595426713) * t351 * t380 - f64x8::splat(0.0014878680171769923) * t351 * t344 - f64x8::splat(0.0021100890252897446) * t385 + f64x8::splat(0.0004308565933608885) * t389 * t356 + f64x8::splat(0.0016437722411542371) * t283 * t369 + f64x8::splat(0.0005970286163074767) * t283 * t363 + f64x8::splat(0.0023160016166370034) * t283 * t356 - f64x8::splat(0.013135604251829597) * t400 - f64x8::splat(0.000835331263170036) * t399 * t344 - f64x8::splat(0.000689695394243961) * t389 * t363 - f64x8::splat(0.00019375881298946268) * t389 * t369 - f64x8::splat(0.004704436332280876) * t389 * t374 + f64x8::splat(0.0027822064319562786) * t389 * t380 - f64x8::splat(7.823588139015819e-05) * t389 * t344 - f64x8::splat(0.016823429546012295) * t414 + f64x8::splat(0.00018939021743243079) * t399 * t356;
            let t428 = t283 * t324;
            let t440 = f64x8::splat(429.0) / f64x8::splat(16.0) * t289 - f64x8::splat(693.0) / f64x8::splat(16.0) * t336 + f64x8::splat(315.0) / f64x8::splat(16.0) * t288 - f64x8::splat(35.0) / f64x8::splat(192.0) * t281 + f64x8::splat(35.0) / f64x8::splat(16.0);
            let t448 = -f64x8::splat(5.0) / f64x8::splat(16.0) + f64x8::splat(231.0) / f64x8::splat(16.0) * t286 - f64x8::splat(315.0) / f64x8::splat(16.0) * t285 + f64x8::splat(105.0) / f64x8::splat(16.0) * t284;
            let t455 = t448 * t324;
            let t461 = -f64x8::splat(0.0009048853909642742) * t399 * t363 + f64x8::splat(8.482767148525194e-05) * t399 * t369 + f64x8::splat(0.0003180493235941731) * t399 * t374 - f64x8::splat(0.0008670535705479461) * t399 * t380 + f64x8::splat(0.12131628073942294) * t428 + f64x8::splat(0.0012341314639045392) * t283 * t344 + f64x8::splat(0.0024977311122498513) * t283 * t380 + f64x8::splat(0.0050995906979556666) * t283 * t374 - f64x8::splat(0.00029476504977320184) * t440 * t356 - f64x8::splat(0.00019095139973664826) * t440 * t363 + f64x8::splat(0.0008367073496483024) * t448 * t374 - f64x8::splat(0.009195715678311926) * t448 * t380 - f64x8::splat(0.007631605623646023) * t448 * t344 + f64x8::splat(0.0028206838819829017) * t455 - f64x8::splat(0.0005194058669188706) * t342 * t356 - f64x8::splat(0.007555456486598222) * t342 * t363;
            let t480 = t440 * t324;
            let t488 = f64x8::splat(1.3669196781387443) - f64x8::splat(0.0038541498256550073) * t342 * t369 - f64x8::splat(0.0010249162124576494) * t342 * t374 - f64x8::splat(3.656012084198544e-05) * t342 * t380 + f64x8::splat(0.004414255398135769) * t359 - f64x8::splat(0.01228729376505733) * t327 + f64x8::splat(0.0063559222793315405) * t326 - f64x8::splat(0.38230940935406266) * t284 + f64x8::splat(0.0038758929812102785) * t440 * t369 - f64x8::splat(0.00031389079758955066) * t440 * t374 + f64x8::splat(0.010726279571787276) * t440 * t380 - f64x8::splat(0.01006770315965861) * t440 * t344 + f64x8::splat(0.00017309630990864668) * t480 - f64x8::splat(0.00018156466410673526) * t448 * t356 + f64x8::splat(0.001864317026752979) * t448 * t363 - f64x8::splat(0.0031296536914037784) * t448 * t369;
            let t490 = t377 + t418 + t461 + t488;
            let t494 = ((t260).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t269 * t490));
            let tzk0 = t259 + t494;
            acc_zk = tzk0;
            let t495 = t7 * t7;
            let t496 = f64x8::splat(1.0) / t495;
            let t497 = t17 * t496;
            let t499 = ((t11).select(f64x8::splat(0.0), (t15).select(f64x8::splat(0.0), t8 - t497)));
            let t502 = ((t21).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t24 * t499));
            let t503 = t502 * t27;
            let t507 = t27 * t27;
            let t508 = f64x8::splat(1.0) / t507;
            let t509 = t26 * t508;
            let t512 = t6 * t509 * t255 / f64x8::splat(8.0);
            let t515 = t34 * v_rho0;
            let t517 = f64x8::splat(1.0) / t30 / t515;
            let t518 = v_sigma0 * t517;
            let t523 = f64x8::splat(5.0) / f64x8::splat(9.0) * (-f64x8::splat(5.0) / f64x8::splat(3.0) * v_tau0 * t36 + t518 / f64x8::splat(3.0)) * t40 * t45;
            let t524 = ((t49).select(t523, f64x8::splat(0.0)));
            let t527 = t58 * t524;
            let t530 = f64x8::splat(1.0) / t57 / t50;
            let t531 = t530 * t524;
            let t534 = t64 * t71;
            let t535 = ((t49).select(f64x8::splat(0.0), t523));
            let t536 = t61 * t535;
            let t539 = t70 * t70;
            let t540 = f64x8::splat(1.0) / t539;
            let t541 = t65 * t540;
            let t542 = t62 * t68;
            let t545 = t62 * t62;
            let t546 = t545 * t61;
            let t549 = f64x8::splat(3.0) * t542 * t535 + f64x8::splat(12.0) * t546 * t535;
            let t552 = ((t48).select(-f64x8::splat(3.0) / f64x8::splat(2.0) * t55 * t524 - f64x8::splat(3.0) / f64x8::splat(16.0) * t527 + f64x8::splat(3.0) * t531, -f64x8::splat(6.0) * t534 * t536 - t541 * t549));
            let t555 = t85 * t518 * t89;
            let t557 = t40 * t40;
            let t559 = f64x8::splat(1.0) / t43 / t42;
            let t560 = t557 * t559;
            let t561 = v_sigma0 * v_sigma0;
            let t562 = t34 * t34;
            let t563 = t562 * t34;
            let t565 = f64x8::splat(1.0) / t29 / t563;
            let t567 = t88 * t88;
            let t568 = f64x8::splat(1.0) / t567;
            let t570 = t560 * t561 * t565 * t568;
            let t574 = -f64x8::splat(2.0) / f64x8::splat(9.0) * t555 + t570 / f64x8::splat(108.0);
            let t575 = t102 * t574;
            let t577 = t95 * t574;
            let t579 = t93 * t574;
            let t581 = f64x8::splat(693.0) / f64x8::splat(8.0) * t575 - f64x8::splat(315.0) / f64x8::splat(4.0) * t577 + f64x8::splat(105.0) / f64x8::splat(8.0) * t579;
            let t582 = t581 * t73;
            let t584 = t209 * t552;
            let t586 = t96 * t574;
            let t588 = t94 * t574;
            let t592 = f64x8::splat(315.0) / f64x8::splat(8.0) * t586 - f64x8::splat(105.0) / f64x8::splat(4.0) * t588 - f64x8::splat(5.0) / f64x8::splat(12.0) * t555 + f64x8::splat(5.0) / f64x8::splat(288.0) * t570;
            let t597 = t75 * t552;
            let t599 = t74 * t552;
            let t602 = f64x8::splat(315.0) / f64x8::splat(8.0) * t597 - f64x8::splat(105.0) / f64x8::splat(4.0) * t599 + f64x8::splat(15.0) / f64x8::splat(8.0) * t552;
            let t607 = t79 * t552;
            let t609 = t81 * t552;
            let t611 = t73 * t552;
            let t613 = f64x8::splat(693.0) / f64x8::splat(8.0) * t607 - f64x8::splat(315.0) / f64x8::splat(4.0) * t609 + f64x8::splat(105.0) / f64x8::splat(8.0) * t611;
            let t616 = t100 * t574;
            let t623 = f64x8::splat(3003.0) / f64x8::splat(16.0) * t616 - f64x8::splat(3465.0) / f64x8::splat(16.0) * t586 + f64x8::splat(945.0) / f64x8::splat(16.0) * t588 + f64x8::splat(35.0) / f64x8::splat(72.0) * t555 - f64x8::splat(35.0) / f64x8::splat(1728.0) * t570;
            let t624 = t623 * t73;
            let t630 = f64x8::splat(0.19451907596748125) * t552 + f64x8::splat(0.029722341099413095) * t555 - f64x8::splat(0.0012384308791422124) * t570 + f64x8::splat(0.0028206838819829017) * t582 + f64x8::splat(0.0028206838819829017) * t584 - f64x8::splat(0.0005194058669188706) * t592 * t115 - f64x8::splat(0.0031296536914037784) * t581 * t127 - f64x8::splat(0.0031296536914037784) * t209 * t602 + f64x8::splat(0.001864317026752979) * t581 * t121 + f64x8::splat(0.001864317026752979) * t209 * t613 - f64x8::splat(0.2814293849782344) * t616 + f64x8::splat(0.00017309630990864668) * t624 - f64x8::splat(0.02289481687093807) * t223 * t552 - f64x8::splat(0.03020310947897583) * t204 * t552;
            let t659 = f64x8::splat(0.0037023943917136176) * t202 * t552 + f64x8::splat(0.00025448301445575583) * t579 * t127 + f64x8::splat(0.0009541479707825193) * t579 * t132 - f64x8::splat(0.0026011607116438384) * t579 * t138 - f64x8::splat(0.002505993789510108) * t579 * t142 - f64x8::splat(0.002505993789510108) * t188 * t552 - f64x8::splat(0.03940681275548879) * t579 * t73 - f64x8::splat(0.00023470764417047457) * t250 * t552 + f64x8::splat(0.0005681706522972924) * t579 * t115 - f64x8::splat(0.0027146561728928226) * t579 * t121 - f64x8::splat(0.004463604051530977) * t233 * t552 + f64x8::splat(0.015185775153296235) * t145 * t552 - f64x8::splat(0.00019095139973664826) * t623 * t121 - f64x8::splat(0.00019095139973664826) * t158 * t613;
            let t663 = t83 * t552;
            let t668 = f64x8::splat(3003.0) / f64x8::splat(16.0) * t663 - f64x8::splat(3465.0) / f64x8::splat(16.0) * t597 + f64x8::splat(945.0) / f64x8::splat(16.0) * t599 - f64x8::splat(35.0) / f64x8::splat(16.0) * t552;
            let t675 = f64x8::splat(35.0) / f64x8::splat(2.0) * t609 - f64x8::splat(15.0) / f64x8::splat(2.0) * t611;
            let t682 = f64x8::splat(15.0) / f64x8::splat(2.0) * t599 - f64x8::splat(3.0) / f64x8::splat(2.0) * t552;
            let t687 = f64x8::splat(35.0) / f64x8::splat(2.0) * t577 - f64x8::splat(15.0) / f64x8::splat(2.0) * t579;
            let t703 = -f64x8::splat(0.00029476504977320184) * t623 * t115 - f64x8::splat(0.00029476504977320184) * t158 * t668 + f64x8::splat(0.0008367073496483024) * t581 * t132 + f64x8::splat(0.0008367073496483024) * t209 * t675 - f64x8::splat(0.009195715678311926) * t581 * t138 - f64x8::splat(0.009195715678311926) * t209 * t682 - f64x8::splat(0.0002571281595426713) * t687 * t138 - f64x8::splat(0.0002571281595426713) * t149 * t682 - f64x8::splat(7.261106354828029e-05) * t687 * t127 - f64x8::splat(7.261106354828029e-05) * t149 * t602 + f64x8::splat(0.0001331797359718674) * t687 * t121 + f64x8::splat(0.0001331797359718674) * t149 * t613 + f64x8::splat(0.2509862353534166) * t586 + f64x8::splat(0.005061925051098745) * t592 * t142;
            let t706 = t574 * t73;
            let t708 = t93 * t552;
            let t710 = t574 * t132;
            let t716 = t574 * t138;
            let t718 = t592 * t73;
            let t720 = t109 * t552;
            let t730 = f64x8::splat(0.0012341314639045392) * t574 * t142 + f64x8::splat(0.12131628073942294) * t706 + f64x8::splat(0.12131628073942294) * t708 + f64x8::splat(0.0050995906979556666) * t710 - f64x8::splat(0.04914917506022932) * t609 + f64x8::splat(0.012711844558663081) * t611 + f64x8::splat(0.0050995906979556666) * t93 * t675 + f64x8::splat(0.0024977311122498513) * t716 - f64x8::splat(0.0016609256494831233) * t718 - f64x8::splat(0.0016609256494831233) * t720 - f64x8::splat(1.792697304428732e-05) * t687 * t115 - f64x8::splat(1.792697304428732e-05) * t149 * t668 - f64x8::splat(0.0010249162124576494) * t592 * t132 - f64x8::splat(0.0010249162124576494) * t109 * t675;
            let t744 = f64x8::splat(15.0) / f64x8::splat(2.0) * t588 + t555 / f64x8::splat(3.0) - t570 / f64x8::splat(72.0);
            let t761 = t158 * t552;
            let t765 = -f64x8::splat(3.656012084198544e-05) * t592 * t138 - f64x8::splat(3.656012084198544e-05) * t109 * t682 + f64x8::splat(0.00018939021743243079) * t173 * t668 - f64x8::splat(0.004704436332280876) * t237 * t675 + f64x8::splat(0.0027822064319562786) * t744 * t138 + f64x8::splat(0.0027822064319562786) * t237 * t682 - f64x8::splat(7.823588139015819e-05) * t744 * t142 - f64x8::splat(0.00019375881298946268) * t744 * t127 - f64x8::splat(0.00019375881298946268) * t237 * t602 - f64x8::splat(0.004704436332280876) * t744 * t132 - f64x8::splat(0.000689695394243961) * t744 * t121 - f64x8::splat(0.000689695394243961) * t237 * t613 + f64x8::splat(0.00017309630990864668) * t761 - f64x8::splat(0.00018156466410673526) * t581 * t115;
            let t785 = t574 * t121;
            let t790 = t574 * t127;
            let t792 = -f64x8::splat(0.00018156466410673526) * t209 * t668 - f64x8::splat(0.00031389079758955066) * t158 * t675 + f64x8::splat(0.010726279571787276) * t623 * t138 + f64x8::splat(0.010726279571787276) * t158 * t682 - f64x8::splat(0.01006770315965861) * t623 * t142 + f64x8::splat(0.0038758929812102785) * t623 * t127 + f64x8::splat(0.0038758929812102785) * t158 * t602 - f64x8::splat(0.00031389079758955066) * t623 * t132 - f64x8::splat(0.017769411149910222) * t599 + f64x8::splat(0.0024977311122498513) * t93 * t682 + f64x8::splat(0.0005970286163074767) * t785 + f64x8::splat(0.026485532388814615) * t607 + f64x8::splat(0.0005970286163074767) * t93 * t613 + f64x8::splat(0.0016437722411542371) * t790;
            let t797 = t574 * t115;
            let t804 = t173 * t552;
            let t813 = t744 * t73;
            let t815 = t237 * t552;
            let t818 = f64x8::splat(0.2613989191485382) * t597 + f64x8::splat(0.0016437722411542371) * t93 * t602 + f64x8::splat(0.0023160016166370034) * t797 - f64x8::splat(0.3801267001217285) * t663 + f64x8::splat(0.0023160016166370034) * t93 * t668 - f64x8::splat(0.0008670535705479461) * t173 * t682 - f64x8::splat(0.013135604251829597) * t804 + f64x8::splat(8.482767148525194e-05) * t173 * t602 + f64x8::splat(0.0003180493235941731) * t173 * t675 - f64x8::splat(0.0009048853909642742) * t173 * t613 - f64x8::splat(0.02414252189674374) * t588 - f64x8::splat(0.016823429546012295) * t813 - f64x8::splat(0.016823429546012295) * t815 - f64x8::splat(0.2283379049668504) * t577;
            let t822 = t687 * t73;
            let t824 = t149 * t552;
            let t847 = -f64x8::splat(0.7646188187081253) * t579 - f64x8::splat(0.0014878680171769923) * t687 * t142 - f64x8::splat(0.0021100890252897446) * t822 - f64x8::splat(0.0021100890252897446) * t824 + f64x8::splat(0.0004308565933608885) * t744 * t115 + f64x8::splat(0.0004308565933608885) * t237 * t668 + f64x8::splat(0.0009891355730978566) * t687 * t132 + f64x8::splat(0.0009891355730978566) * t149 * t675 - f64x8::splat(0.0038541498256550073) * t592 * t127 - f64x8::splat(0.0038541498256550073) * t109 * t602 - f64x8::splat(0.0005194058669188706) * t109 * t668 - f64x8::splat(0.007555456486598222) * t592 * t121 - f64x8::splat(0.007555456486598222) * t109 * t613 + f64x8::splat(0.4380036644282133) * t575 - f64x8::splat(0.007631605623646023) * t581 * t142;
            let t850 = t630 + t659 + t703 + t730 + t765 + t792 + t818 + t847;
            let t855 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t503 * t255 - t512 - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t28 * t850));
            let t856 = t261 * t496;
            let t858 = ((t15).select(f64x8::splat(0.0), (t11).select(f64x8::splat(0.0), -t8 - t856)));
            let t861 = ((t265).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t266 * t858));
            let t862 = t861 * t27;
            let t866 = t268 * t508;
            let t869 = t6 * t866 * t490 / f64x8::splat(8.0);
            let t871 = ((t260).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t862 * t490 - t869));
            let tvrho0 = t259 + t494 + t7 * (t855 + t871);
            acc_vrho_0 = tvrho0;
            let t875 = ((t11).select(f64x8::splat(0.0), (t15).select(f64x8::splat(0.0), -t8 - t497)));
            let t878 = ((t21).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t24 * t875));
            let t879 = t878 * t27;
            let t884 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t879 * t255 - t512));
            let t886 = ((t15).select(f64x8::splat(0.0), (t11).select(f64x8::splat(0.0), t8 - t856)));
            let t889 = ((t265).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t266 * t886));
            let t890 = t889 * t27;
            let t896 = t270 * v_rho1;
            let t898 = f64x8::splat(1.0) / t272 / t896;
            let t899 = v_sigma2 * t898;
            let t904 = f64x8::splat(5.0) / f64x8::splat(9.0) * (-f64x8::splat(5.0) / f64x8::splat(3.0) * v_tau1 * t274 + t899 / f64x8::splat(3.0)) * t40 * t45;
            let t905 = ((t300).select(t904, f64x8::splat(0.0)));
            let t908 = t309 * t905;
            let t911 = f64x8::splat(1.0) / t308 / t301;
            let t912 = t911 * t905;
            let t915 = t315 * t322;
            let t916 = ((t300).select(f64x8::splat(0.0), t904));
            let t917 = t312 * t916;
            let t920 = t321 * t321;
            let t921 = f64x8::splat(1.0) / t920;
            let t922 = t316 * t921;
            let t923 = t313 * t319;
            let t926 = t313 * t313;
            let t927 = t926 * t312;
            let t930 = f64x8::splat(3.0) * t923 * t916 + f64x8::splat(12.0) * t927 * t916;
            let t933 = ((t299).select(-f64x8::splat(3.0) / f64x8::splat(2.0) * t306 * t905 - f64x8::splat(3.0) / f64x8::splat(16.0) * t908 + f64x8::splat(3.0) * t912, -f64x8::splat(6.0) * t915 * t917 - t922 * t930));
            let t936 = t85 * t899 * t279;
            let t938 = v_sigma2 * v_sigma2;
            let t939 = t270 * t270;
            let t940 = t939 * t270;
            let t942 = f64x8::splat(1.0) / t271 / t940;
            let t944 = t278 * t278;
            let t945 = f64x8::splat(1.0) / t944;
            let t947 = t560 * t938 * t942 * t945;
            let t951 = -f64x8::splat(2.0) / f64x8::splat(9.0) * t936 + t947 / f64x8::splat(108.0);
            let t952 = t283 * t951;
            let t975 = f64x8::splat(0.19451907596748125) * t933 + f64x8::splat(0.029722341099413095) * t936 - f64x8::splat(0.0012384308791422124) * t947 - f64x8::splat(0.03940681275548879) * t952 * t324 - f64x8::splat(0.002505993789510108) * t952 * t344 - f64x8::splat(0.002505993789510108) * t400 * t933 - f64x8::splat(0.0027146561728928226) * t952 * t363 + f64x8::splat(0.00025448301445575583) * t952 * t369 + f64x8::splat(0.0009541479707825193) * t952 * t374 - f64x8::splat(0.0026011607116438384) * t952 * t380 + f64x8::splat(0.0037023943917136176) * t428 * t933 - f64x8::splat(0.00023470764417047457) * t414 * t933 + f64x8::splat(0.0005681706522972924) * t952 * t356 - f64x8::splat(0.004463604051530977) * t385 * t933;
            let t982 = t336 * t951;
            let t984 = t288 * t951;
            let t987 = f64x8::splat(693.0) / f64x8::splat(8.0) * t982 - f64x8::splat(315.0) / f64x8::splat(4.0) * t984 + f64x8::splat(105.0) / f64x8::splat(8.0) * t952;
            let t990 = t328 * t933;
            let t992 = t330 * t933;
            let t994 = t324 * t933;
            let t996 = f64x8::splat(693.0) / f64x8::splat(8.0) * t990 - f64x8::splat(315.0) / f64x8::splat(4.0) * t992 + f64x8::splat(105.0) / f64x8::splat(8.0) * t994;
            let t999 = t286 * t951;
            let t1002 = t285 * t951;
            let t1004 = t284 * t951;
            let t1008 = f64x8::splat(3003.0) / f64x8::splat(16.0) * t999 - f64x8::splat(3465.0) / f64x8::splat(16.0) * t1002 + f64x8::splat(945.0) / f64x8::splat(16.0) * t1004 + f64x8::splat(35.0) / f64x8::splat(72.0) * t936 - f64x8::splat(35.0) / f64x8::splat(1728.0) * t947;
            let t1009 = t1008 * t324;
            let t1011 = t440 * t933;
            let t1015 = t359 * t933;
            let t1017 = t327 * t933;
            let t1019 = t326 * t933;
            let t1022 = f64x8::splat(3003.0) / f64x8::splat(16.0) * t1015 - f64x8::splat(3465.0) / f64x8::splat(16.0) * t1017 + f64x8::splat(945.0) / f64x8::splat(16.0) * t1019 - f64x8::splat(35.0) / f64x8::splat(16.0) * t933;
            let t1029 = f64x8::splat(35.0) / f64x8::splat(2.0) * t984 - f64x8::splat(15.0) / f64x8::splat(2.0) * t952;
            let t1034 = f64x8::splat(15.0) / f64x8::splat(2.0) * t1019 - f64x8::splat(3.0) / f64x8::splat(2.0) * t933;
            let t1039 = f64x8::splat(0.015185775153296235) * t347 * t933 - f64x8::splat(0.02289481687093807) * t455 * t933 - f64x8::splat(0.03020310947897583) * t480 * t933 + f64x8::splat(0.001864317026752979) * t987 * t363 + f64x8::splat(0.001864317026752979) * t448 * t996 - f64x8::splat(0.2814293849782344) * t999 + f64x8::splat(0.00017309630990864668) * t1009 + f64x8::splat(0.00017309630990864668) * t1011 - f64x8::splat(0.00018156466410673526) * t987 * t356 - f64x8::splat(0.00018156466410673526) * t448 * t1022 + f64x8::splat(0.0004308565933608885) * t389 * t1022 - f64x8::splat(0.0002571281595426713) * t1029 * t380 - f64x8::splat(0.0002571281595426713) * t351 * t1034 - f64x8::splat(0.0014878680171769923) * t1029 * t344;
            let t1044 = f64x8::splat(315.0) / f64x8::splat(8.0) * t1017 - f64x8::splat(105.0) / f64x8::splat(4.0) * t1019 + f64x8::splat(15.0) / f64x8::splat(8.0) * t933;
            let t1051 = f64x8::splat(35.0) / f64x8::splat(2.0) * t992 - f64x8::splat(15.0) / f64x8::splat(2.0) * t994;
            let t1071 = f64x8::splat(315.0) / f64x8::splat(8.0) * t1002 - f64x8::splat(105.0) / f64x8::splat(4.0) * t1004 - f64x8::splat(5.0) / f64x8::splat(12.0) * t936 + f64x8::splat(5.0) / f64x8::splat(288.0) * t947;
            let t1074 = t1071 * t324;
            let t1081 = f64x8::splat(15.0) / f64x8::splat(2.0) * t1004 + t936 / f64x8::splat(3.0) - t947 / f64x8::splat(72.0);
            let t1084 = -f64x8::splat(7.261106354828029e-05) * t351 * t1044 + f64x8::splat(0.0009891355730978566) * t1029 * t374 + f64x8::splat(0.0009891355730978566) * t351 * t1051 + f64x8::splat(0.0001331797359718674) * t1029 * t363 + f64x8::splat(0.0001331797359718674) * t351 * t996 - f64x8::splat(7.261106354828029e-05) * t1029 * t369 - f64x8::splat(1.792697304428732e-05) * t1029 * t356 - f64x8::splat(1.792697304428732e-05) * t351 * t1022 - f64x8::splat(3.656012084198544e-05) * t342 * t1034 + f64x8::splat(0.2509862353534166) * t1002 + f64x8::splat(0.005061925051098745) * t1071 * t344 - f64x8::splat(0.0016609256494831233) * t1074 - f64x8::splat(0.004704436332280876) * t389 * t1051 + f64x8::splat(0.0027822064319562786) * t1081 * t380;
            let t1097 = t1029 * t324;
            let t1099 = t351 * t933;
            let t1111 = f64x8::splat(0.0027822064319562786) * t389 * t1034 - f64x8::splat(0.00019375881298946268) * t1081 * t369 - f64x8::splat(0.00019375881298946268) * t389 * t1044 - f64x8::splat(0.000689695394243961) * t1081 * t363 - f64x8::splat(0.000689695394243961) * t389 * t996 - f64x8::splat(0.2283379049668504) * t984 - f64x8::splat(0.7646188187081253) * t952 - f64x8::splat(0.0021100890252897446) * t1097 - f64x8::splat(0.0021100890252897446) * t1099 + f64x8::splat(0.0004308565933608885) * t1081 * t356 + f64x8::splat(0.010726279571787276) * t1008 * t380 + f64x8::splat(0.010726279571787276) * t440 * t1034 - f64x8::splat(0.01006770315965861) * t1008 * t344 - f64x8::splat(0.00031389079758955066) * t1008 * t374;
            let t1116 = t342 * t933;
            let t1139 = t987 * t324;
            let t1141 = -f64x8::splat(0.00031389079758955066) * t440 * t1051 - f64x8::splat(0.0016609256494831233) * t1116 - f64x8::splat(0.0010249162124576494) * t1071 * t374 - f64x8::splat(0.0010249162124576494) * t342 * t1051 - f64x8::splat(3.656012084198544e-05) * t1071 * t380 - f64x8::splat(0.007555456486598222) * t342 * t996 - f64x8::splat(0.0038541498256550073) * t1071 * t369 - f64x8::splat(0.0038541498256550073) * t342 * t1044 - f64x8::splat(0.0005194058669188706) * t1071 * t356 - f64x8::splat(0.0005194058669188706) * t342 * t1022 - f64x8::splat(0.007555456486598222) * t1071 * t363 + f64x8::splat(0.4380036644282133) * t982 - f64x8::splat(0.007631605623646023) * t987 * t344 + f64x8::splat(0.0028206838819829017) * t1139;
            let t1142 = t448 * t933;
            let t1170 = f64x8::splat(0.0028206838819829017) * t1142 - f64x8::splat(0.00029476504977320184) * t1008 * t356 - f64x8::splat(0.00029476504977320184) * t440 * t1022 + f64x8::splat(0.0008367073496483024) * t987 * t374 + f64x8::splat(0.0008367073496483024) * t448 * t1051 - f64x8::splat(0.009195715678311926) * t987 * t380 - f64x8::splat(0.009195715678311926) * t448 * t1034 - f64x8::splat(0.0031296536914037784) * t987 * t369 - f64x8::splat(0.0031296536914037784) * t448 * t1044 + f64x8::splat(0.0038758929812102785) * t440 * t1044 - f64x8::splat(0.00019095139973664826) * t1008 * t363 - f64x8::splat(0.00019095139973664826) * t440 * t996 + f64x8::splat(0.0038758929812102785) * t1008 * t369 + f64x8::splat(0.0050995906979556666) * t283 * t1051;
            let t1172 = t283 * t933;
            let t1174 = t951 * t344;
            let t1176 = t951 * t324;
            let t1187 = t1081 * t324;
            let t1189 = t389 * t933;
            let t1197 = t399 * t933;
            let t1199 = f64x8::splat(0.12131628073942294) * t1172 + f64x8::splat(0.0012341314639045392) * t1174 + f64x8::splat(0.12131628073942294) * t1176 - f64x8::splat(0.0008670535705479461) * t399 * t1034 + f64x8::splat(0.0003180493235941731) * t399 * t1051 + f64x8::splat(8.482767148525194e-05) * t399 * t1044 - f64x8::splat(0.0009048853909642742) * t399 * t996 - f64x8::splat(0.02414252189674374) * t1004 - f64x8::splat(0.016823429546012295) * t1187 - f64x8::splat(0.016823429546012295) * t1189 + f64x8::splat(0.00018939021743243079) * t399 * t1022 - f64x8::splat(7.823588139015819e-05) * t1081 * t344 - f64x8::splat(0.004704436332280876) * t1081 * t374 - f64x8::splat(0.013135604251829597) * t1197;
            let t1200 = t951 * t363;
            let t1207 = t951 * t356;
            let t1214 = t951 * t369;
            let t1218 = t951 * t380;
            let t1222 = t951 * t374;
            let t1224 = f64x8::splat(0.0005970286163074767) * t1200 + f64x8::splat(0.026485532388814615) * t990 - f64x8::splat(0.04914917506022932) * t992 + f64x8::splat(0.012711844558663081) * t994 + f64x8::splat(0.0005970286163074767) * t283 * t996 + f64x8::splat(0.0023160016166370034) * t1207 - f64x8::splat(0.3801267001217285) * t1015 + f64x8::splat(0.2613989191485382) * t1017 - f64x8::splat(0.017769411149910222) * t1019 + f64x8::splat(0.0023160016166370034) * t283 * t1022 + f64x8::splat(0.0016437722411542371) * t1214 + f64x8::splat(0.0016437722411542371) * t283 * t1044 + f64x8::splat(0.0024977311122498513) * t1218 + f64x8::splat(0.0024977311122498513) * t283 * t1034 + f64x8::splat(0.0050995906979556666) * t1222;
            let t1227 = t975 + t1039 + t1084 + t1111 + t1141 + t1170 + t1199 + t1224;
            let t1232 = ((t260).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t890 * t490 - t869 - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t269 * t1227));
            let tvrho1 = t259 + t494 + t7 * (t884 + t1232);
            acc_vrho_1 = tvrho1;
            let t1235 = t85 * t36;
            let t1236 = f64x8::splat(5.0) / f64x8::splat(72.0) * t1235;
            let t1237 = ((t49).select(-t1236, f64x8::splat(0.0)));
            let t1240 = t58 * t1237;
            let t1242 = t530 * t1237;
            let t1245 = ((t49).select(f64x8::splat(0.0), -t1236));
            let t1246 = t61 * t1245;
            let t1253 = f64x8::splat(3.0) * t542 * t1245 + f64x8::splat(12.0) * t546 * t1245;
            let t1256 = ((t48).select(-f64x8::splat(3.0) / f64x8::splat(2.0) * t55 * t1237 - f64x8::splat(3.0) / f64x8::splat(16.0) * t1240 + f64x8::splat(3.0) * t1242, -f64x8::splat(6.0) * t534 * t1246 - t541 * t1253));
            let t1265 = t85 * t36 * t89;
            let t1267 = t562 * v_rho0;
            let t1269 = f64x8::splat(1.0) / t29 / t1267;
            let t1272 = t560 * v_sigma0 * t1269 * t568;
            let t1274 = t1265 / f64x8::splat(12.0) - t1272 / f64x8::splat(288.0);
            let t1275 = t1274 * t142;
            let t1277 = t1274 * t73;
            let t1279 = t93 * t1256;
            let t1281 = t1274 * t132;
            let t1283 = t81 * t1256;
            let t1285 = t73 * t1256;
            let t1289 = f64x8::splat(35.0) / f64x8::splat(2.0) * t1283 - f64x8::splat(15.0) / f64x8::splat(2.0) * t1285;
            let t1292 = t1274 * t138;
            let t1294 = t74 * t1256;
            let t1298 = f64x8::splat(15.0) / f64x8::splat(2.0) * t1294 - f64x8::splat(3.0) / f64x8::splat(2.0) * t1256;
            let t1301 = f64x8::splat(0.19451907596748125) * t1256 + f64x8::splat(0.015185775153296235) * t145 * t1256 - f64x8::splat(0.02289481687093807) * t223 * t1256 - f64x8::splat(0.03020310947897583) * t204 * t1256 + f64x8::splat(0.0012341314639045392) * t1275 + f64x8::splat(0.12131628073942294) * t1277 + f64x8::splat(0.12131628073942294) * t1279 + f64x8::splat(0.0050995906979556666) * t1281 - f64x8::splat(0.04914917506022932) * t1283 + f64x8::splat(0.012711844558663081) * t1285 + f64x8::splat(0.0050995906979556666) * t93 * t1289 + f64x8::splat(0.0024977311122498513) * t1292 - f64x8::splat(0.017769411149910222) * t1294 + f64x8::splat(0.0024977311122498513) * t93 * t1298;
            let t1302 = t1274 * t127;
            let t1304 = t75 * t1256;
            let t1309 = f64x8::splat(315.0) / f64x8::splat(8.0) * t1304 - f64x8::splat(105.0) / f64x8::splat(4.0) * t1294 + f64x8::splat(15.0) / f64x8::splat(8.0) * t1256;
            let t1312 = t1274 * t115;
            let t1314 = t83 * t1256;
            let t1320 = f64x8::splat(3003.0) / f64x8::splat(16.0) * t1314 - f64x8::splat(3465.0) / f64x8::splat(16.0) * t1304 + f64x8::splat(945.0) / f64x8::splat(16.0) * t1294 - f64x8::splat(35.0) / f64x8::splat(16.0) * t1256;
            let t1323 = t1274 * t121;
            let t1325 = t79 * t1256;
            let t1330 = f64x8::splat(693.0) / f64x8::splat(8.0) * t1325 - f64x8::splat(315.0) / f64x8::splat(4.0) * t1283 + f64x8::splat(105.0) / f64x8::splat(8.0) * t1285;
            let t1335 = t93 * t1274;
            let t1344 = f64x8::splat(0.0016437722411542371) * t1302 + f64x8::splat(0.2613989191485382) * t1304 + f64x8::splat(0.0016437722411542371) * t93 * t1309 + f64x8::splat(0.0023160016166370034) * t1312 - f64x8::splat(0.3801267001217285) * t1314 + f64x8::splat(0.0023160016166370034) * t93 * t1320 + f64x8::splat(0.0005970286163074767) * t1323 + f64x8::splat(0.026485532388814615) * t1325 + f64x8::splat(0.0005970286163074767) * t93 * t1330 + f64x8::splat(0.0037023943917136176) * t202 * t1256 + f64x8::splat(0.00025448301445575583) * t1335 * t127 + f64x8::splat(0.0009541479707825193) * t1335 * t132 - f64x8::splat(0.0026011607116438384) * t1335 * t138 - f64x8::splat(0.002505993789510108) * t1335 * t142;
            let t1359 = t94 * t1274;
            let t1363 = f64x8::splat(15.0) / f64x8::splat(2.0) * t1359 - t1265 / f64x8::splat(8.0) + t1272 / f64x8::splat(192.0);
            let t1368 = t102 * t1274;
            let t1370 = t95 * t1274;
            let t1373 = f64x8::splat(693.0) / f64x8::splat(8.0) * t1368 - f64x8::splat(315.0) / f64x8::splat(4.0) * t1370 + f64x8::splat(105.0) / f64x8::splat(8.0) * t1335;
            let t1378 = t100 * t1274;
            let t1381 = t96 * t1274;
            let t1386 = f64x8::splat(3003.0) / f64x8::splat(16.0) * t1378 - f64x8::splat(3465.0) / f64x8::splat(16.0) * t1381 + f64x8::splat(945.0) / f64x8::splat(16.0) * t1359 - f64x8::splat(35.0) / f64x8::splat(192.0) * t1265 + f64x8::splat(35.0) / f64x8::splat(4608.0) * t1272;
            let t1387 = t1386 * t73;
            let t1389 = t158 * t1256;
            let t1391 = -f64x8::splat(0.002505993789510108) * t188 * t1256 - f64x8::splat(0.03940681275548879) * t1335 * t73 - f64x8::splat(0.00023470764417047457) * t250 * t1256 + f64x8::splat(0.0005681706522972924) * t1335 * t115 - f64x8::splat(0.0027146561728928226) * t1335 * t121 - f64x8::splat(0.004463604051530977) * t233 * t1256 - f64x8::splat(0.011145877912279912) * t1265 + f64x8::splat(0.0004308565933608885) * t1363 * t115 + f64x8::splat(0.0004308565933608885) * t237 * t1320 + f64x8::splat(0.001864317026752979) * t1373 * t121 + f64x8::splat(0.001864317026752979) * t209 * t1330 - f64x8::splat(0.2814293849782344) * t1378 + f64x8::splat(0.00017309630990864668) * t1387 + f64x8::splat(0.00017309630990864668) * t1389;
            let t1406 = f64x8::splat(315.0) / f64x8::splat(8.0) * t1381 - f64x8::splat(105.0) / f64x8::splat(4.0) * t1359 + f64x8::splat(5.0) / f64x8::splat(32.0) * t1265 - f64x8::splat(5.0) / f64x8::splat(768.0) * t1272;
            let t1407 = t1406 * t73;
            let t1409 = t109 * t1256;
            let t1413 = f64x8::splat(35.0) / f64x8::splat(2.0) * t1370 - f64x8::splat(15.0) / f64x8::splat(2.0) * t1335;
            let t1428 = -f64x8::splat(0.00018156466410673526) * t1373 * t115 - f64x8::splat(0.00018156466410673526) * t209 * t1320 + f64x8::splat(0.010726279571787276) * t1386 * t138 + f64x8::splat(0.010726279571787276) * t158 * t1298 - f64x8::splat(0.01006770315965861) * t1386 * t142 - f64x8::splat(0.0016609256494831233) * t1407 - f64x8::splat(0.0016609256494831233) * t1409 - f64x8::splat(1.792697304428732e-05) * t1413 * t115 - f64x8::splat(1.792697304428732e-05) * t149 * t1320 + f64x8::splat(0.0001331797359718674) * t1413 * t121 + f64x8::splat(0.0001331797359718674) * t149 * t1330 - f64x8::splat(7.261106354828029e-05) * t1413 * t127 + f64x8::splat(8.482767148525194e-05) * t173 * t1309 - f64x8::splat(0.0038541498256550073) * t1406 * t127;
            let t1455 = t1413 * t73;
            let t1457 = t149 * t1256;
            let t1459 = -f64x8::splat(0.0038541498256550073) * t109 * t1309 - f64x8::splat(0.0010249162124576494) * t1406 * t132 - f64x8::splat(0.0010249162124576494) * t109 * t1289 - f64x8::splat(3.656012084198544e-05) * t1406 * t138 - f64x8::splat(3.656012084198544e-05) * t109 * t1298 + f64x8::splat(0.005061925051098745) * t1406 * t142 - f64x8::splat(7.261106354828029e-05) * t149 * t1309 + f64x8::splat(0.0009891355730978566) * t1413 * t132 + f64x8::splat(0.0009891355730978566) * t149 * t1289 - f64x8::splat(0.0002571281595426713) * t1413 * t138 - f64x8::splat(0.0002571281595426713) * t149 * t1298 - f64x8::splat(0.0014878680171769923) * t1413 * t142 - f64x8::splat(0.0021100890252897446) * t1455 - f64x8::splat(0.0021100890252897446) * t1457;
            let t1484 = t1363 * t73;
            let t1486 = t237 * t1256;
            let t1488 = f64x8::splat(0.0038758929812102785) * t158 * t1309 - f64x8::splat(0.00031389079758955066) * t1386 * t132 - f64x8::splat(0.00031389079758955066) * t158 * t1289 - f64x8::splat(0.00019095139973664826) * t1386 * t121 - f64x8::splat(0.00019095139973664826) * t158 * t1330 - f64x8::splat(0.00019375881298946268) * t1363 * t127 - f64x8::splat(0.00019375881298946268) * t237 * t1309 - f64x8::splat(0.004704436332280876) * t1363 * t132 - f64x8::splat(0.004704436332280876) * t237 * t1289 + f64x8::splat(0.0027822064319562786) * t1363 * t138 + f64x8::splat(0.0027822064319562786) * t237 * t1298 - f64x8::splat(7.823588139015819e-05) * t1363 * t142 - f64x8::splat(0.016823429546012295) * t1484 - f64x8::splat(0.016823429546012295) * t1486;
            let t1506 = t173 * t1256;
            let t1515 = f64x8::splat(0.00018939021743243079) * t173 * t1320 - f64x8::splat(0.0009048853909642742) * t173 * t1330 + f64x8::splat(0.0038758929812102785) * t1386 * t127 - f64x8::splat(0.00029476504977320184) * t1386 * t115 - f64x8::splat(0.00029476504977320184) * t158 * t1320 - f64x8::splat(0.000689695394243961) * t1363 * t121 - f64x8::splat(0.000689695394243961) * t237 * t1330 - f64x8::splat(0.0008670535705479461) * t173 * t1298 - f64x8::splat(0.013135604251829597) * t1506 + f64x8::splat(0.0003180493235941731) * t173 * t1289 + f64x8::splat(0.4380036644282133) * t1368 - f64x8::splat(0.2283379049668504) * t1370 - f64x8::splat(0.7646188187081253) * t1335 - f64x8::splat(0.0031296536914037784) * t1373 * t127;
            let t1529 = t1373 * t73;
            let t1531 = t209 * t1256;
            let t1543 = -f64x8::splat(0.0031296536914037784) * t209 * t1309 + f64x8::splat(0.0008367073496483024) * t1373 * t132 + f64x8::splat(0.0008367073496483024) * t209 * t1289 - f64x8::splat(0.009195715678311926) * t1373 * t138 - f64x8::splat(0.009195715678311926) * t209 * t1298 + f64x8::splat(0.0004644115796783296) * t1272 - f64x8::splat(0.007631605623646023) * t1373 * t142 + f64x8::splat(0.0028206838819829017) * t1529 + f64x8::splat(0.0028206838819829017) * t1531 + f64x8::splat(0.2509862353534166) * t1381 - f64x8::splat(0.02414252189674374) * t1359 - f64x8::splat(0.0005194058669188706) * t1406 * t115 - f64x8::splat(0.0005194058669188706) * t109 * t1320 - f64x8::splat(0.007555456486598222) * t1406 * t121 - f64x8::splat(0.007555456486598222) * t109 * t1330;
            let t1546 = t1301 + t1344 + t1391 + t1428 + t1459 + t1488 + t1515 + t1543;
            let t1550 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t28 * t1546));
            let tvsigma0 = t7 * t1550;
            acc_vsigma_0 = tvsigma0;
            let tvsigma1 = f64x8::splat(0.0);
            acc_vsigma_1 = tvsigma1;
            let t1551 = t85 * t274;
            let t1552 = f64x8::splat(5.0) / f64x8::splat(72.0) * t1551;
            let t1553 = ((t300).select(-t1552, f64x8::splat(0.0)));
            let t1556 = t309 * t1553;
            let t1558 = t911 * t1553;
            let t1561 = ((t300).select(f64x8::splat(0.0), -t1552));
            let t1562 = t312 * t1561;
            let t1569 = f64x8::splat(3.0) * t923 * t1561 + f64x8::splat(12.0) * t927 * t1561;
            let t1572 = ((t299).select(-f64x8::splat(3.0) / f64x8::splat(2.0) * t306 * t1553 - f64x8::splat(3.0) / f64x8::splat(16.0) * t1556 + f64x8::splat(3.0) * t1558, -f64x8::splat(6.0) * t915 * t1562 - t922 * t1569));
            let t1575 = t85 * t274 * t279;
            let t1578 = t939 * v_rho1;
            let t1580 = f64x8::splat(1.0) / t271 / t1578;
            let t1583 = t560 * v_sigma2 * t1580 * t945;
            let t1585 = t1575 / f64x8::splat(12.0) - t1583 / f64x8::splat(288.0);
            let t1586 = t286 * t1585;
            let t1588 = t285 * t1585;
            let t1590 = t284 * t1585;
            let t1594 = f64x8::splat(3003.0) / f64x8::splat(16.0) * t1586 - f64x8::splat(3465.0) / f64x8::splat(16.0) * t1588 + f64x8::splat(945.0) / f64x8::splat(16.0) * t1590 - f64x8::splat(35.0) / f64x8::splat(192.0) * t1575 + f64x8::splat(35.0) / f64x8::splat(4608.0) * t1583;
            let t1597 = t359 * t1572;
            let t1599 = t327 * t1572;
            let t1601 = t326 * t1572;
            let t1604 = f64x8::splat(3003.0) / f64x8::splat(16.0) * t1597 - f64x8::splat(3465.0) / f64x8::splat(16.0) * t1599 + f64x8::splat(945.0) / f64x8::splat(16.0) * t1601 - f64x8::splat(35.0) / f64x8::splat(16.0) * t1572;
            let t1609 = t328 * t1572;
            let t1611 = t330 * t1572;
            let t1613 = t324 * t1572;
            let t1615 = f64x8::splat(693.0) / f64x8::splat(8.0) * t1609 - f64x8::splat(315.0) / f64x8::splat(4.0) * t1611 + f64x8::splat(105.0) / f64x8::splat(8.0) * t1613;
            let t1621 = f64x8::splat(15.0) / f64x8::splat(2.0) * t1590 - t1575 / f64x8::splat(8.0) + t1583 / f64x8::splat(192.0);
            let t1627 = f64x8::splat(315.0) / f64x8::splat(8.0) * t1599 - f64x8::splat(105.0) / f64x8::splat(4.0) * t1601 + f64x8::splat(15.0) / f64x8::splat(8.0) * t1572;
            let t1638 = f64x8::splat(315.0) / f64x8::splat(8.0) * t1588 - f64x8::splat(105.0) / f64x8::splat(4.0) * t1590 + f64x8::splat(5.0) / f64x8::splat(32.0) * t1575 - f64x8::splat(5.0) / f64x8::splat(768.0) * t1583;
            let t1643 = f64x8::splat(0.19451907596748125) * t1572 - f64x8::splat(0.011145877912279912) * t1575 - f64x8::splat(0.00029476504977320184) * t1594 * t356 - f64x8::splat(0.00029476504977320184) * t440 * t1604 - f64x8::splat(0.00019095139973664826) * t1594 * t363 - f64x8::splat(0.00019095139973664826) * t440 * t1615 - f64x8::splat(0.00019375881298946268) * t1621 * t369 - f64x8::splat(0.00019375881298946268) * t389 * t1627 + f64x8::splat(0.0004308565933608885) * t1621 * t356 + f64x8::splat(0.2509862353534166) * t1588 - f64x8::splat(0.02414252189674374) * t1590 - f64x8::splat(0.0010249162124576494) * t1638 * t374 - f64x8::splat(0.04914917506022932) * t1611 + f64x8::splat(0.012711844558663081) * t1613;
            let t1646 = f64x8::splat(35.0) / f64x8::splat(2.0) * t1611 - f64x8::splat(15.0) / f64x8::splat(2.0) * t1613;
            let t1654 = f64x8::splat(15.0) / f64x8::splat(2.0) * t1601 - f64x8::splat(3.0) / f64x8::splat(2.0) * t1572;
            let t1657 = t283 * t1585;
            let t1678 = -f64x8::splat(0.0010249162124576494) * t342 * t1646 - f64x8::splat(3.656012084198544e-05) * t1638 * t380 - f64x8::splat(0.017769411149910222) * t1601 - f64x8::splat(3.656012084198544e-05) * t342 * t1654 - f64x8::splat(0.03940681275548879) * t1657 * t324 - f64x8::splat(0.002505993789510108) * t1657 * t344 - f64x8::splat(0.002505993789510108) * t400 * t1572 + f64x8::splat(0.0009541479707825193) * t1657 * t374 - f64x8::splat(0.0026011607116438384) * t1657 * t380 + f64x8::splat(0.0037023943917136176) * t428 * t1572 - f64x8::splat(0.00023470764417047457) * t414 * t1572 + f64x8::splat(0.0005681706522972924) * t1657 * t356 - f64x8::splat(0.0027146561728928226) * t1657 * t363 + f64x8::splat(0.00025448301445575583) * t1657 * t369;
            let t1696 = t1638 * t324;
            let t1698 = t342 * t1572;
            let t1700 = t288 * t1585;
            let t1703 = f64x8::splat(35.0) / f64x8::splat(2.0) * t1700 - f64x8::splat(15.0) / f64x8::splat(2.0) * t1657;
            let t1710 = -f64x8::splat(0.004463604051530977) * t385 * t1572 + f64x8::splat(0.015185775153296235) * t347 * t1572 - f64x8::splat(0.02289481687093807) * t455 * t1572 - f64x8::splat(0.03020310947897583) * t480 * t1572 - f64x8::splat(0.3801267001217285) * t1597 + f64x8::splat(0.2613989191485382) * t1599 - f64x8::splat(0.00031389079758955066) * t1594 * t374 - f64x8::splat(0.00031389079758955066) * t440 * t1646 + f64x8::splat(0.005061925051098745) * t1638 * t344 - f64x8::splat(0.0016609256494831233) * t1696 - f64x8::splat(0.0016609256494831233) * t1698 - f64x8::splat(1.792697304428732e-05) * t1703 * t356 - f64x8::splat(1.792697304428732e-05) * t351 * t1604 + f64x8::splat(0.0001331797359718674) * t1703 * t363;
            let t1717 = t399 * t1572;
            let t1719 = t1585 * t363;
            let t1723 = t1585 * t356;
            let t1729 = t1585 * t369;
            let t1733 = t1585 * t324;
            let t1735 = t283 * t1572;
            let t1737 = t1585 * t344;
            let t1739 = f64x8::splat(0.0001331797359718674) * t351 * t1615 - f64x8::splat(7.261106354828029e-05) * t1703 * t369 - f64x8::splat(7.261106354828029e-05) * t351 * t1627 - f64x8::splat(0.013135604251829597) * t1717 + f64x8::splat(0.0005970286163074767) * t1719 + f64x8::splat(0.0005970286163074767) * t283 * t1615 + f64x8::splat(0.0023160016166370034) * t1723 + f64x8::splat(0.0023160016166370034) * t283 * t1604 + f64x8::splat(0.0050995906979556666) * t283 * t1646 + f64x8::splat(0.0016437722411542371) * t1729 + f64x8::splat(0.0016437722411542371) * t283 * t1627 + f64x8::splat(0.12131628073942294) * t1733 + f64x8::splat(0.12131628073942294) * t1735 + f64x8::splat(0.0012341314639045392) * t1737;
            let t1742 = t1585 * t380;
            let t1746 = t1585 * t374;
            let t1760 = t1621 * t324;
            let t1762 = t389 * t1572;
            let t1770 = f64x8::splat(0.0024977311122498513) * t1742 + f64x8::splat(0.0024977311122498513) * t283 * t1654 + f64x8::splat(0.0050995906979556666) * t1746 + f64x8::splat(0.0003180493235941731) * t399 * t1646 - f64x8::splat(0.0008670535705479461) * t399 * t1654 - f64x8::splat(0.0009048853909642742) * t399 * t1615 + f64x8::splat(8.482767148525194e-05) * t399 * t1627 + f64x8::splat(0.00018939021743243079) * t399 * t1604 - f64x8::splat(7.823588139015819e-05) * t1621 * t344 - f64x8::splat(0.016823429546012295) * t1760 - f64x8::splat(0.016823429546012295) * t1762 - f64x8::splat(0.004704436332280876) * t1621 * t374 - f64x8::splat(0.004704436332280876) * t389 * t1646 + f64x8::splat(0.0027822064319562786) * t1621 * t380;
            let t1793 = t1703 * t324;
            let t1795 = t351 * t1572;
            let t1799 = f64x8::splat(0.0027822064319562786) * t389 * t1654 + f64x8::splat(0.010726279571787276) * t1594 * t380 + f64x8::splat(0.010726279571787276) * t440 * t1654 - f64x8::splat(0.01006770315965861) * t1594 * t344 + f64x8::splat(0.0038758929812102785) * t1594 * t369 + f64x8::splat(0.0038758929812102785) * t440 * t1627 + f64x8::splat(0.0004308565933608885) * t389 * t1604 - f64x8::splat(0.000689695394243961) * t1621 * t363 - f64x8::splat(0.000689695394243961) * t389 * t1615 - f64x8::splat(0.0002571281595426713) * t351 * t1654 - f64x8::splat(0.0014878680171769923) * t1703 * t344 - f64x8::splat(0.0021100890252897446) * t1793 - f64x8::splat(0.0021100890252897446) * t1795 + f64x8::splat(0.0009891355730978566) * t1703 * t374;
            let t1808 = t336 * t1585;
            let t1815 = f64x8::splat(693.0) / f64x8::splat(8.0) * t1808 - f64x8::splat(315.0) / f64x8::splat(4.0) * t1700 + f64x8::splat(105.0) / f64x8::splat(8.0) * t1657;
            let t1818 = t1815 * t324;
            let t1820 = t448 * t1572;
            let t1830 = f64x8::splat(0.0009891355730978566) * t351 * t1646 + f64x8::splat(0.0004644115796783296) * t1583 - f64x8::splat(0.0005194058669188706) * t342 * t1604 - f64x8::splat(0.007555456486598222) * t1638 * t363 + f64x8::splat(0.4380036644282133) * t1808 - f64x8::splat(0.2283379049668504) * t1700 - f64x8::splat(0.7646188187081253) * t1657 - f64x8::splat(0.007631605623646023) * t1815 * t344 + f64x8::splat(0.0028206838819829017) * t1818 + f64x8::splat(0.0028206838819829017) * t1820 - f64x8::splat(0.0005194058669188706) * t1638 * t356 - f64x8::splat(0.0002571281595426713) * t1703 * t380 - f64x8::splat(0.007555456486598222) * t342 * t1615 - f64x8::splat(0.0038541498256550073) * t1638 * t369;
            let t1851 = t1594 * t324;
            let t1853 = t440 * t1572;
            let t1859 = -f64x8::splat(0.0038541498256550073) * t342 * t1627 + f64x8::splat(0.0008367073496483024) * t1815 * t374 + f64x8::splat(0.0008367073496483024) * t448 * t1646 - f64x8::splat(0.009195715678311926) * t1815 * t380 - f64x8::splat(0.009195715678311926) * t448 * t1654 - f64x8::splat(0.0031296536914037784) * t1815 * t369 - f64x8::splat(0.0031296536914037784) * t448 * t1627 + f64x8::splat(0.001864317026752979) * t1815 * t363 + f64x8::splat(0.026485532388814615) * t1609 + f64x8::splat(0.001864317026752979) * t448 * t1615 - f64x8::splat(0.2814293849782344) * t1586 + f64x8::splat(0.00017309630990864668) * t1851 + f64x8::splat(0.00017309630990864668) * t1853 - f64x8::splat(0.00018156466410673526) * t1815 * t356 - f64x8::splat(0.00018156466410673526) * t448 * t1604;
            let t1862 = t1643 + t1678 + t1710 + t1739 + t1770 + t1799 + t1830 + t1859;
            let t1866 = ((t260).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t269 * t1862));
            let tvsigma2 = t7 * t1866;
            acc_vsigma_2 = tvsigma2;
            let tvlapl0 = f64x8::splat(0.0);
            acc_vlapl_0 = tvlapl0;
            let tvlapl1 = f64x8::splat(0.0);
            acc_vlapl_1 = tvlapl1;
            let t1869 = f64x8::splat(5.0) / f64x8::splat(9.0) * t32 * t40 * t45;
            let t1870 = ((t49).select(t1869, f64x8::splat(0.0)));
            let t1873 = t58 * t1870;
            let t1875 = t530 * t1870;
            let t1878 = ((t49).select(f64x8::splat(0.0), t1869));
            let t1879 = t61 * t1878;
            let t1886 = f64x8::splat(3.0) * t542 * t1878 + f64x8::splat(12.0) * t546 * t1878;
            let t1889 = ((t48).select(-f64x8::splat(3.0) / f64x8::splat(2.0) * t55 * t1870 - f64x8::splat(3.0) / f64x8::splat(16.0) * t1873 + f64x8::splat(3.0) * t1875, -f64x8::splat(6.0) * t534 * t1879 - t541 * t1886));
            let t1905 = t79 * t1889;
            let t1907 = t81 * t1889;
            let t1909 = t73 * t1889;
            let t1911 = f64x8::splat(693.0) / f64x8::splat(8.0) * t1905 - f64x8::splat(315.0) / f64x8::splat(4.0) * t1907 + f64x8::splat(105.0) / f64x8::splat(8.0) * t1909;
            let t1914 = t74 * t1889;
            let t1917 = f64x8::splat(15.0) / f64x8::splat(2.0) * t1914 - f64x8::splat(3.0) / f64x8::splat(2.0) * t1889;
            let t1920 = t149 * t1889;
            let t1922 = t83 * t1889;
            let t1924 = t75 * t1889;
            let t1928 = f64x8::splat(3003.0) / f64x8::splat(16.0) * t1922 - f64x8::splat(3465.0) / f64x8::splat(16.0) * t1924 + f64x8::splat(945.0) / f64x8::splat(16.0) * t1914 - f64x8::splat(35.0) / f64x8::splat(16.0) * t1889;
            let t1934 = f64x8::splat(315.0) / f64x8::splat(8.0) * t1924 - f64x8::splat(105.0) / f64x8::splat(4.0) * t1914 + f64x8::splat(15.0) / f64x8::splat(8.0) * t1889;
            let t1939 = f64x8::splat(35.0) / f64x8::splat(2.0) * t1907 - f64x8::splat(15.0) / f64x8::splat(2.0) * t1909;
            let t1942 = f64x8::splat(0.19451907596748125) * t1889 + f64x8::splat(0.0037023943917136176) * t202 * t1889 - f64x8::splat(0.002505993789510108) * t188 * t1889 - f64x8::splat(0.00023470764417047457) * t250 * t1889 + f64x8::splat(0.015185775153296235) * t145 * t1889 - f64x8::splat(0.004463604051530977) * t233 * t1889 - f64x8::splat(0.02289481687093807) * t223 * t1889 - f64x8::splat(0.03020310947897583) * t204 * t1889 - f64x8::splat(0.000689695394243961) * t237 * t1911 - f64x8::splat(0.0002571281595426713) * t149 * t1917 - f64x8::splat(0.0021100890252897446) * t1920 + f64x8::splat(0.0004308565933608885) * t237 * t1928 - f64x8::splat(7.261106354828029e-05) * t149 * t1934 + f64x8::splat(0.0009891355730978566) * t149 * t1939;
            let t1945 = t109 * t1889;
            let t1961 = t173 * t1889;
            let t1966 = f64x8::splat(0.0001331797359718674) * t149 * t1911 - f64x8::splat(0.0016609256494831233) * t1945 - f64x8::splat(1.792697304428732e-05) * t149 * t1928 - f64x8::splat(0.0010249162124576494) * t109 * t1939 + f64x8::splat(0.2613989191485382) * t1924 + f64x8::splat(0.0016437722411542371) * t93 * t1934 - f64x8::splat(0.04914917506022932) * t1907 + f64x8::splat(0.012711844558663081) * t1909 + f64x8::splat(0.0050995906979556666) * t93 * t1939 + f64x8::splat(0.026485532388814615) * t1905 + f64x8::splat(0.0005970286163074767) * t93 * t1911 - f64x8::splat(0.013135604251829597) * t1961 - f64x8::splat(0.3801267001217285) * t1922 + f64x8::splat(0.0023160016166370034) * t93 * t1928;
            let t1976 = t237 * t1889;
            let t1992 = t209 * t1889;
            let t1996 = f64x8::splat(0.0003180493235941731) * t173 * t1939 - f64x8::splat(0.0008670535705479461) * t173 * t1917 - f64x8::splat(0.0009048853909642742) * t173 * t1911 + f64x8::splat(8.482767148525194e-05) * t173 * t1934 - f64x8::splat(0.016823429546012295) * t1976 + f64x8::splat(0.00018939021743243079) * t173 * t1928 - f64x8::splat(0.004704436332280876) * t237 * t1939 + f64x8::splat(0.0027822064319562786) * t237 * t1917 - f64x8::splat(0.00019375881298946268) * t237 * t1934 - f64x8::splat(3.656012084198544e-05) * t109 * t1917 - f64x8::splat(0.007555456486598222) * t109 * t1911 - f64x8::splat(0.0038541498256550073) * t109 * t1934 + f64x8::splat(0.0028206838819829017) * t1992 - f64x8::splat(0.0005194058669188706) * t109 * t1928;
            let t2014 = t93 * t1889;
            let t2020 = t158 * t1889;
            let t2024 = f64x8::splat(0.0008367073496483024) * t209 * t1939 - f64x8::splat(0.009195715678311926) * t209 * t1917 - f64x8::splat(0.0031296536914037784) * t209 * t1934 - f64x8::splat(0.00018156466410673526) * t209 * t1928 - f64x8::splat(0.00031389079758955066) * t158 * t1939 - f64x8::splat(0.00019095139973664826) * t158 * t1911 - f64x8::splat(0.00029476504977320184) * t158 * t1928 - f64x8::splat(0.017769411149910222) * t1914 + f64x8::splat(0.0024977311122498513) * t93 * t1917 + f64x8::splat(0.12131628073942294) * t2014 + f64x8::splat(0.001864317026752979) * t209 * t1911 + f64x8::splat(0.010726279571787276) * t158 * t1917 + f64x8::splat(0.00017309630990864668) * t2020 + f64x8::splat(0.0038758929812102785) * t158 * t1934;
            let t2026 = t1942 + t1966 + t1996 + t2024;
            let t2030 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t28 * t2026));
            let tvtau0 = t7 * t2030;
            acc_vtau_0 = tvtau0;
            let t2033 = f64x8::splat(5.0) / f64x8::splat(9.0) * t292 * t40 * t45;
            let t2034 = ((t300).select(t2033, f64x8::splat(0.0)));
            let t2037 = t309 * t2034;
            let t2039 = t911 * t2034;
            let t2042 = ((t300).select(f64x8::splat(0.0), t2033));
            let t2043 = t312 * t2042;
            let t2050 = f64x8::splat(3.0) * t923 * t2042 + f64x8::splat(12.0) * t927 * t2042;
            let t2053 = ((t299).select(-f64x8::splat(3.0) / f64x8::splat(2.0) * t306 * t2034 - f64x8::splat(3.0) / f64x8::splat(16.0) * t2037 + f64x8::splat(3.0) * t2039, -f64x8::splat(6.0) * t915 * t2043 - t922 * t2050));
            let t2055 = t448 * t2053;
            let t2057 = t359 * t2053;
            let t2059 = t327 * t2053;
            let t2061 = t326 * t2053;
            let t2064 = f64x8::splat(3003.0) / f64x8::splat(16.0) * t2057 - f64x8::splat(3465.0) / f64x8::splat(16.0) * t2059 + f64x8::splat(945.0) / f64x8::splat(16.0) * t2061 - f64x8::splat(35.0) / f64x8::splat(16.0) * t2053;
            let t2067 = t328 * t2053;
            let t2069 = t330 * t2053;
            let t2071 = t324 * t2053;
            let t2073 = f64x8::splat(693.0) / f64x8::splat(8.0) * t2067 - f64x8::splat(315.0) / f64x8::splat(4.0) * t2069 + f64x8::splat(105.0) / f64x8::splat(8.0) * t2071;
            let t2084 = t399 * t2053;
            let t2090 = f64x8::splat(35.0) / f64x8::splat(2.0) * t2069 - f64x8::splat(15.0) / f64x8::splat(2.0) * t2071;
            let t2095 = f64x8::splat(15.0) / f64x8::splat(2.0) * t2061 - f64x8::splat(3.0) / f64x8::splat(2.0) * t2053;
            let t2103 = f64x8::splat(315.0) / f64x8::splat(8.0) * t2059 - f64x8::splat(105.0) / f64x8::splat(4.0) * t2061 + f64x8::splat(15.0) / f64x8::splat(8.0) * t2053;
            let t2106 = f64x8::splat(0.19451907596748125) * t2053 + f64x8::splat(0.0028206838819829017) * t2055 - f64x8::splat(0.0005194058669188706) * t342 * t2064 - f64x8::splat(0.0009048853909642742) * t399 * t2073 - f64x8::splat(0.00019095139973664826) * t440 * t2073 - f64x8::splat(0.00029476504977320184) * t440 * t2064 + f64x8::splat(0.0005970286163074767) * t283 * t2073 + f64x8::splat(0.0023160016166370034) * t283 * t2064 - f64x8::splat(0.013135604251829597) * t2084 + f64x8::splat(0.001864317026752979) * t448 * t2073 - f64x8::splat(0.00031389079758955066) * t440 * t2090 + f64x8::splat(0.0024977311122498513) * t283 * t2095 + f64x8::splat(0.0050995906979556666) * t283 * t2090 + f64x8::splat(0.0016437722411542371) * t283 * t2103;
            let t2113 = t342 * t2053;
            let t2123 = t389 * t2053;
            let t2133 = -f64x8::splat(0.0008670535705479461) * t399 * t2095 + f64x8::splat(0.0001331797359718674) * t351 * t2073 - f64x8::splat(3.656012084198544e-05) * t342 * t2095 - f64x8::splat(0.0016609256494831233) * t2113 - f64x8::splat(1.792697304428732e-05) * t351 * t2064 - f64x8::splat(0.0010249162124576494) * t342 * t2090 - f64x8::splat(0.007555456486598222) * t342 * t2073 - f64x8::splat(0.0038541498256550073) * t342 * t2103 - f64x8::splat(0.016823429546012295) * t2123 - f64x8::splat(0.3801267001217285) * t2057 + f64x8::splat(0.00018939021743243079) * t399 * t2064 - f64x8::splat(0.004704436332280876) * t389 * t2090 + f64x8::splat(0.0027822064319562786) * t389 * t2095 + f64x8::splat(0.026485532388814615) * t2067;
            let t2143 = t351 * t2053;
            let t2163 = -f64x8::splat(0.000689695394243961) * t389 * t2073 - f64x8::splat(0.00019375881298946268) * t389 * t2103 + f64x8::splat(0.0004308565933608885) * t389 * t2064 - f64x8::splat(0.0002571281595426713) * t351 * t2095 - f64x8::splat(0.0021100890252897446) * t2143 - f64x8::splat(7.261106354828029e-05) * t351 * t2103 + f64x8::splat(0.0009891355730978566) * t351 * t2090 - f64x8::splat(0.002505993789510108) * t400 * t2053 + f64x8::splat(0.0037023943917136176) * t428 * t2053 - f64x8::splat(0.00023470764417047457) * t414 * t2053 - f64x8::splat(0.004463604051530977) * t385 * t2053 + f64x8::splat(0.015185775153296235) * t347 * t2053 - f64x8::splat(0.02289481687093807) * t455 * t2053 - f64x8::splat(0.03020310947897583) * t480 * t2053;
            let t2178 = t283 * t2053;
            let t2180 = t440 * t2053;
            let t2188 = f64x8::splat(0.2613989191485382) * t2059 - f64x8::splat(0.017769411149910222) * t2061 + f64x8::splat(8.482767148525194e-05) * t399 * t2103 - f64x8::splat(0.04914917506022932) * t2069 + f64x8::splat(0.012711844558663081) * t2071 + f64x8::splat(0.0003180493235941731) * t399 * t2090 - f64x8::splat(0.0031296536914037784) * t448 * t2103 + f64x8::splat(0.0008367073496483024) * t448 * t2090 - f64x8::splat(0.009195715678311926) * t448 * t2095 + f64x8::splat(0.12131628073942294) * t2178 + f64x8::splat(0.00017309630990864668) * t2180 - f64x8::splat(0.00018156466410673526) * t448 * t2064 + f64x8::splat(0.010726279571787276) * t440 * t2095 + f64x8::splat(0.0038758929812102785) * t440 * t2103;
            let t2190 = t2106 + t2133 + t2163 + t2188;
            let t2194 = ((t260).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t269 * t2190));
            let tvtau1 = t7 * t2194;
            acc_vtau_1 = tvtau1;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
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
