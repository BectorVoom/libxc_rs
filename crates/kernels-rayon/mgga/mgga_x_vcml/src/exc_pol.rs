//! MGGA_X_VCML exc pol kernel — explicit SIMD (bit-exact).
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
pub fn mgga_x_vcml_exc_pol(
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
        }
        store_add(zk, ip, m, acc_zk);
        ip += 8;
    }
}
