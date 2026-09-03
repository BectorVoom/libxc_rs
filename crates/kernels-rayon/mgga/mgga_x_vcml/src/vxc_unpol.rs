//! MGGA_X_VCML vxc unpol kernel — explicit SIMD (bit-exact).
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

#[allow(unused_variables, non_snake_case)]
pub fn mgga_x_vcml_vxc_unpol(
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
        let v_rho = load(rho, ip, np);
        let v_sigma = load(sigma, ip, np);
        let v_lapl = load(lapl, ip, np);
        let v_tau = load(tau, ip, np);
        let mut acc_zk = V_ZERO;
        let mut acc_vrho = V_ZERO;
        let mut acc_vsigma = V_ZERO;
        let mut acc_vlapl = V_ZERO;
        let mut acc_vtau = V_ZERO;
        {
            let t3 = (v_rho / f64x8::splat(2.0)).simd_le(dens_threshold);
            let t4 = f64x8::splat(M_CBRT3);
            let t5 = f64x8::splat(M_CBRTPI);
            let t7 = t4 / t5;
            let t8 = (f64x8::splat(1.0)).simd_le(zeta_threshold);
            let t9 = zeta_threshold - f64x8::splat(1.0);
            let t11 = ((t8).select(t9, (t8).select(-t9, f64x8::splat(0.0))));
            let t12 = t11 + f64x8::splat(1.0);
            let t14 = (simd::cbrt(zeta_threshold));
            let t16 = (simd::cbrt(t12));
            let t18 = (((t12).simd_le(zeta_threshold)).select(t14 * zeta_threshold, t16 * t12));
            let t19 = (simd::cbrt(v_rho));
            let t20 = t18 * t19;
            let t21 = f64x8::splat(M_CBRT6);
            let t22 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t23 = (simd::cbrt(t22));
            let t24 = t23 * t23;
            let t25 = f64x8::splat(1.0) / t24;
            let t26 = t21 * t25;
            let t27 = t26 * v_sigma;
            let t28 = f64x8::splat(M_CBRT2);
            let t29 = t28 * t28;
            let t30 = v_rho * v_rho;
            let t31 = t19 * t19;
            let t33 = f64x8::splat(1.0) / t31 / t30;
            let t34 = t29 * t33;
            let t35 = v_sigma * t29;
            let t36 = t35 * t33;
            let t39 = f64x8::splat(6.5124) + t26 * t36 / f64x8::splat(24.0);
            let t40 = f64x8::splat(1.0) / t39;
            let t41 = t34 * t40;
            let t42 = t27 * t41;
            let t44 = t42 / f64x8::splat(12.0) - f64x8::splat(1.0);
            let t45 = t44 * t44;
            let t46 = t45 * t44;
            let t47 = t45 * t45;
            let t48 = t47 * t46;
            let t50 = t47 * t44;
            let t54 = f64x8::splat(429.0) / f64x8::splat(16.0) * t48 - f64x8::splat(693.0) / f64x8::splat(16.0) * t50 + f64x8::splat(315.0) / f64x8::splat(16.0) * t46 - f64x8::splat(35.0) / f64x8::splat(192.0) * t42 + f64x8::splat(35.0) / f64x8::splat(16.0);
            let t55 = v_tau * t29;
            let t57 = f64x8::splat(1.0) / t31 / v_rho;
            let t63 = f64x8::splat(5.0) / f64x8::splat(9.0) * (t55 * t57 - t36 / f64x8::splat(8.0)) * t21 * t25;
            let t64 = (f64x8::splat(10000.0)).simd_le(t63);
            let t65 = (f64x8::splat(10000.0)).simd_lt(t63);
            let t66 = ((t65).select(t63, f64x8::splat(10000.0)));
            let t67 = t66 * t66;
            let t70 = t67 * t66;
            let t71 = f64x8::splat(1.0) / t70;
            let t73 = t67 * t67;
            let t74 = f64x8::splat(1.0) / t73;
            let t77 = ((t65).select(f64x8::splat(10000.0), t63));
            let t78 = t77 * t77;
            let t79 = f64x8::splat(1.0) - t78;
            let t80 = t79 * t79;
            let t81 = t80 * t79;
            let t82 = t78 * t77;
            let t84 = f64x8::splat(1.0) + f64x8::splat(4.0) * t82;
            let t86 = t82 * t84 + f64x8::splat(1.0);
            let t87 = f64x8::splat(1.0) / t86;
            let t89 = ((t64).select(f64x8::splat(3.0) / f64x8::splat(4.0) / t67 + t71 / f64x8::splat(16.0) - f64x8::splat(3.0) / f64x8::splat(4.0) * t74 - f64x8::splat(1.0) / f64x8::splat(4.0), t81 * t87));
            let t90 = t89 * t89;
            let t91 = t90 * t89;
            let t92 = t90 * t90;
            let t93 = t92 * t91;
            let t95 = t92 * t89;
            let t99 = f64x8::splat(429.0) / f64x8::splat(16.0) * t93 - f64x8::splat(693.0) / f64x8::splat(16.0) * t95 + f64x8::splat(315.0) / f64x8::splat(16.0) * t91 - f64x8::splat(35.0) / f64x8::splat(16.0) * t89;
            let t102 = t92 * t90;
            let t106 = -f64x8::splat(5.0) / f64x8::splat(16.0) + f64x8::splat(231.0) / f64x8::splat(16.0) * t102 - f64x8::splat(315.0) / f64x8::splat(16.0) * t92 + f64x8::splat(105.0) / f64x8::splat(16.0) * t90;
            let t112 = f64x8::splat(63.0) / f64x8::splat(8.0) * t95 - f64x8::splat(35.0) / f64x8::splat(4.0) * t91 + f64x8::splat(15.0) / f64x8::splat(8.0) * t89;
            let t117 = f64x8::splat(3.0) / f64x8::splat(8.0) + f64x8::splat(35.0) / f64x8::splat(8.0) * t92 - f64x8::splat(15.0) / f64x8::splat(4.0) * t90;
            let t122 = f64x8::splat(5.0) / f64x8::splat(2.0) * t91 - f64x8::splat(3.0) / f64x8::splat(2.0) * t89;
            let t126 = -f64x8::splat(1.0) / f64x8::splat(2.0) + f64x8::splat(3.0) / f64x8::splat(2.0) * t90;
            let t129 = t54 * t89;
            let t131 = t47 * t45;
            let t135 = -f64x8::splat(5.0) / f64x8::splat(16.0) + f64x8::splat(231.0) / f64x8::splat(16.0) * t131 - f64x8::splat(315.0) / f64x8::splat(16.0) * t47 + f64x8::splat(105.0) / f64x8::splat(16.0) * t45;
            let t148 = t135 * t89;
            let t153 = f64x8::splat(63.0) / f64x8::splat(8.0) * t50 - f64x8::splat(35.0) / f64x8::splat(4.0) * t46 + f64x8::splat(5.0) / f64x8::splat(32.0) * t42 - f64x8::splat(15.0) / f64x8::splat(8.0);
            let t158 = -f64x8::splat(0.00029476504977320184) * t54 * t99 - f64x8::splat(0.00019095139973664826) * t54 * t106 + f64x8::splat(0.0038758929812102785) * t54 * t112 - f64x8::splat(0.00031389079758955066) * t54 * t117 + f64x8::splat(0.010726279571787276) * t54 * t122 - f64x8::splat(0.01006770315965861) * t54 * t126 + f64x8::splat(0.00017309630990864668) * t129 - f64x8::splat(0.00018156466410673526) * t135 * t99 + f64x8::splat(0.001864317026752979) * t135 * t106 - f64x8::splat(0.0031296536914037784) * t135 * t112 + f64x8::splat(0.0008367073496483024) * t135 * t117 - f64x8::splat(0.009195715678311926) * t135 * t122 - f64x8::splat(0.007631605623646023) * t135 * t126 + f64x8::splat(0.0028206838819829017) * t148 - f64x8::splat(0.0005194058669188706) * t153 * t99 - f64x8::splat(0.007555456486598222) * t153 * t106;
            let t167 = t153 * t89;
            let t171 = f64x8::splat(3.0) / f64x8::splat(8.0) + f64x8::splat(35.0) / f64x8::splat(8.0) * t47 - f64x8::splat(15.0) / f64x8::splat(4.0) * t45;
            let t184 = t171 * t89;
            let t188 = f64x8::splat(5.0) / f64x8::splat(2.0) * t46 - t42 / f64x8::splat(8.0) + f64x8::splat(3.0) / f64x8::splat(2.0);
            let t197 = -f64x8::splat(0.0038541498256550073) * t153 * t112 - f64x8::splat(0.0010249162124576494) * t153 * t117 - f64x8::splat(3.656012084198544e-05) * t153 * t122 + f64x8::splat(0.005061925051098745) * t153 * t126 - f64x8::splat(0.0016609256494831233) * t167 - f64x8::splat(1.792697304428732e-05) * t171 * t99 + f64x8::splat(0.0001331797359718674) * t171 * t106 - f64x8::splat(7.261106354828029e-05) * t171 * t112 + f64x8::splat(0.0009891355730978566) * t171 * t117 - f64x8::splat(0.0002571281595426713) * t171 * t122 - f64x8::splat(0.0014878680171769923) * t171 * t126 - f64x8::splat(0.0021100890252897446) * t184 + f64x8::splat(0.0004308565933608885) * t188 * t99 - f64x8::splat(0.000689695394243961) * t188 * t106 - f64x8::splat(0.00019375881298946268) * t188 * t112 - f64x8::splat(0.004704436332280876) * t188 * t117;
            let t203 = t188 * t89;
            let t206 = -f64x8::splat(1.0) / f64x8::splat(2.0) + f64x8::splat(3.0) / f64x8::splat(2.0) * t45;
            let t219 = t206 * t89;
            let t233 = f64x8::splat(0.0027822064319562786) * t188 * t122 - f64x8::splat(7.823588139015819e-05) * t188 * t126 - f64x8::splat(0.016823429546012295) * t203 + f64x8::splat(0.00018939021743243079) * t206 * t99 - f64x8::splat(0.0009048853909642742) * t206 * t106 + f64x8::splat(8.482767148525194e-05) * t206 * t112 + f64x8::splat(0.0003180493235941731) * t206 * t117 - f64x8::splat(0.0008670535705479461) * t206 * t122 - f64x8::splat(0.000835331263170036) * t206 * t126 - f64x8::splat(0.013135604251829597) * t219 + f64x8::splat(0.0023160016166370034) * t44 * t99 + f64x8::splat(0.0005970286163074767) * t44 * t106 + f64x8::splat(0.0016437722411542371) * t44 * t112 + f64x8::splat(0.0050995906979556666) * t44 * t117 + f64x8::splat(0.0024977311122498513) * t44 * t122 + f64x8::splat(0.0012341314639045392) * t44 * t126;
            let t234 = t44 * t89;
            let t250 = f64x8::splat(1.3669196781387443) + f64x8::splat(0.12131628073942294) * t234 + f64x8::splat(0.050197247070683314) * t50 - f64x8::splat(0.011145877912279912) * t42 - f64x8::splat(0.00804750729891458) * t46 + f64x8::splat(0.07300061073803556) * t131 - f64x8::splat(0.05430381430310407) * t93 - f64x8::splat(0.04020419785403348) * t48 + f64x8::splat(0.004414255398135769) * t102 - f64x8::splat(0.01228729376505733) * t92 + f64x8::splat(0.0063559222793315405) * t90 - f64x8::splat(0.38230940935406266) * t45 - f64x8::splat(0.0570844762417126) * t47 - f64x8::splat(0.005923137049970073) * t91 + f64x8::splat(0.19451907596748125) * t89 + f64x8::splat(0.05227978382970764) * t95;
            let t252 = t158 + t197 + t233 + t250;
            let t256 = ((t3).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t7 * t20 * t252));
            let tzk0 = f64x8::splat(2.0) * t256;
            acc_zk = tzk0;
            let t258 = t18 / t31;
            let t262 = t30 * v_rho;
            let t264 = f64x8::splat(1.0) / t31 / t262;
            let t265 = t29 * t264;
            let t266 = t265 * t40;
            let t267 = t27 * t266;
            let t269 = t21 * t21;
            let t271 = f64x8::splat(1.0) / t23 / t22;
            let t272 = t269 * t271;
            let t273 = v_sigma * v_sigma;
            let t274 = t272 * t273;
            let t275 = t30 * t30;
            let t276 = t275 * t30;
            let t278 = f64x8::splat(1.0) / t19 / t276;
            let t280 = t39 * t39;
            let t281 = f64x8::splat(1.0) / t280;
            let t282 = t28 * t278 * t281;
            let t283 = t274 * t282;
            let t285 = -f64x8::splat(2.0) / f64x8::splat(9.0) * t267 + t283 / f64x8::splat(54.0);
            let t286 = t285 * t89;
            let t295 = f64x8::splat(5.0) / f64x8::splat(9.0) * (-f64x8::splat(5.0) / f64x8::splat(3.0) * t55 * t33 + t35 * t264 / f64x8::splat(3.0)) * t21 * t25;
            let t296 = ((t65).select(t295, f64x8::splat(0.0)));
            let t299 = t74 * t296;
            let t302 = f64x8::splat(1.0) / t73 / t66;
            let t303 = t302 * t296;
            let t306 = t80 * t87;
            let t307 = ((t65).select(f64x8::splat(0.0), t295));
            let t308 = t77 * t307;
            let t311 = t86 * t86;
            let t312 = f64x8::splat(1.0) / t311;
            let t313 = t81 * t312;
            let t314 = t78 * t84;
            let t317 = t78 * t78;
            let t318 = t317 * t77;
            let t321 = f64x8::splat(3.0) * t314 * t307 + f64x8::splat(12.0) * t318 * t307;
            let t324 = ((t64).select(-f64x8::splat(3.0) / f64x8::splat(2.0) * t71 * t296 - f64x8::splat(3.0) / f64x8::splat(16.0) * t299 + f64x8::splat(3.0) * t303, -f64x8::splat(6.0) * t306 * t308 - t313 * t321));
            let t325 = t44 * t324;
            let t327 = t285 * t126;
            let t329 = t285 * t122;
            let t331 = t90 * t324;
            let t335 = f64x8::splat(15.0) / f64x8::splat(2.0) * t331 - f64x8::splat(3.0) / f64x8::splat(2.0) * t324;
            let t338 = t285 * t112;
            let t340 = t92 * t324;
            let t345 = f64x8::splat(315.0) / f64x8::splat(8.0) * t340 - f64x8::splat(105.0) / f64x8::splat(4.0) * t331 + f64x8::splat(15.0) / f64x8::splat(8.0) * t324;
            let t348 = t285 * t117;
            let t350 = t91 * t324;
            let t352 = t89 * t324;
            let t356 = f64x8::splat(35.0) / f64x8::splat(2.0) * t350 - f64x8::splat(15.0) / f64x8::splat(2.0) * t352;
            let t359 = t285 * t106;
            let t361 = f64x8::splat(0.12131628073942294) * t286 + f64x8::splat(0.12131628073942294) * t325 + f64x8::splat(0.0012341314639045392) * t327 + f64x8::splat(0.0024977311122498513) * t329 - f64x8::splat(0.017769411149910222) * t331 + f64x8::splat(0.0024977311122498513) * t44 * t335 + f64x8::splat(0.0016437722411542371) * t338 + f64x8::splat(0.2613989191485382) * t340 + f64x8::splat(0.0016437722411542371) * t44 * t345 + f64x8::splat(0.0050995906979556666) * t348 - f64x8::splat(0.04914917506022932) * t350 + f64x8::splat(0.012711844558663081) * t352 + f64x8::splat(0.0050995906979556666) * t44 * t356 + f64x8::splat(0.0005970286163074767) * t359;
            let t362 = t95 * t324;
            let t367 = f64x8::splat(693.0) / f64x8::splat(8.0) * t362 - f64x8::splat(315.0) / f64x8::splat(4.0) * t350 + f64x8::splat(105.0) / f64x8::splat(8.0) * t352;
            let t370 = t206 * t324;
            let t372 = t285 * t99;
            let t374 = t102 * t324;
            let t380 = f64x8::splat(3003.0) / f64x8::splat(16.0) * t374 - f64x8::splat(3465.0) / f64x8::splat(16.0) * t340 + f64x8::splat(945.0) / f64x8::splat(16.0) * t331 - f64x8::splat(35.0) / f64x8::splat(16.0) * t324;
            let t391 = t45 * t285;
            let t396 = f64x8::splat(15.0) / f64x8::splat(2.0) * t391 + t267 / f64x8::splat(3.0) - t283 / f64x8::splat(36.0);
            let t399 = t396 * t89;
            let t402 = f64x8::splat(0.026485532388814615) * t362 + f64x8::splat(0.0005970286163074767) * t44 * t367 - f64x8::splat(0.013135604251829597) * t370 + f64x8::splat(0.0023160016166370034) * t372 - f64x8::splat(0.3801267001217285) * t374 + f64x8::splat(0.0023160016166370034) * t44 * t380 + f64x8::splat(0.0003180493235941731) * t206 * t356 - f64x8::splat(0.0008670535705479461) * t206 * t335 + f64x8::splat(8.482767148525194e-05) * t206 * t345 - f64x8::splat(0.0009048853909642742) * t206 * t367 - f64x8::splat(0.02414252189674374) * t391 - f64x8::splat(7.823588139015819e-05) * t396 * t126 - f64x8::splat(0.016823429546012295) * t399 + f64x8::splat(0.19451907596748125) * t324;
            let t408 = t44 * t285;
            let t431 = f64x8::splat(0.029722341099413095) * t267 - f64x8::splat(0.0024768617582844247) * t283 + f64x8::splat(0.0037023943917136176) * t234 * t324 + f64x8::splat(0.00025448301445575583) * t408 * t112 + f64x8::splat(0.0009541479707825193) * t408 * t117 - f64x8::splat(0.0026011607116438384) * t408 * t122 - f64x8::splat(0.002505993789510108) * t408 * t126 - f64x8::splat(0.002505993789510108) * t219 * t324 - f64x8::splat(0.03940681275548879) * t408 * t89 - f64x8::splat(0.00023470764417047457) * t203 * t324 + f64x8::splat(0.0005681706522972924) * t408 * t99 - f64x8::splat(0.0027146561728928226) * t408 * t106 - f64x8::splat(0.004463604051530977) * t184 * t324 + f64x8::splat(0.015185775153296235) * t167 * t324;
            let t436 = t188 * t324;
            let t460 = -f64x8::splat(0.02289481687093807) * t148 * t324 - f64x8::splat(0.03020310947897583) * t129 * t324 - f64x8::splat(0.016823429546012295) * t436 + f64x8::splat(0.00018939021743243079) * t206 * t380 - f64x8::splat(0.004704436332280876) * t396 * t117 - f64x8::splat(0.004704436332280876) * t188 * t356 + f64x8::splat(0.0027822064319562786) * t396 * t122 + f64x8::splat(0.0027822064319562786) * t188 * t335 - f64x8::splat(0.000689695394243961) * t396 * t106 - f64x8::splat(0.000689695394243961) * t188 * t367 - f64x8::splat(0.00019375881298946268) * t396 * t112 - f64x8::splat(0.00019375881298946268) * t188 * t345 + f64x8::splat(0.0004308565933608885) * t396 * t99 + f64x8::splat(0.0004308565933608885) * t188 * t380;
            let t465 = t46 * t285;
            let t470 = f64x8::splat(35.0) / f64x8::splat(2.0) * t465 - f64x8::splat(15.0) / f64x8::splat(2.0) * t408;
            let t473 = t470 * t89;
            let t475 = t171 * t324;
            let t493 = -f64x8::splat(0.0002571281595426713) * t171 * t335 - f64x8::splat(0.2283379049668504) * t465 - f64x8::splat(0.7646188187081253) * t408 - f64x8::splat(0.0014878680171769923) * t470 * t126 - f64x8::splat(0.0021100890252897446) * t473 - f64x8::splat(0.0021100890252897446) * t475 + f64x8::splat(0.0009891355730978566) * t470 * t117 + f64x8::splat(0.0009891355730978566) * t171 * t356 - f64x8::splat(0.0002571281595426713) * t470 * t122 + f64x8::splat(0.0001331797359718674) * t470 * t106 + f64x8::splat(0.0001331797359718674) * t171 * t367 - f64x8::splat(7.261106354828029e-05) * t470 * t112 - f64x8::splat(7.261106354828029e-05) * t171 * t345 - f64x8::splat(1.792697304428732e-05) * t470 * t99;
            let t498 = t47 * t285;
            let t504 = f64x8::splat(315.0) / f64x8::splat(8.0) * t498 - f64x8::splat(105.0) / f64x8::splat(4.0) * t391 - f64x8::splat(5.0) / f64x8::splat(12.0) * t267 + f64x8::splat(5.0) / f64x8::splat(144.0) * t283;
            let t507 = t504 * t89;
            let t509 = t153 * t324;
            let t527 = -f64x8::splat(1.792697304428732e-05) * t171 * t380 - f64x8::splat(3.656012084198544e-05) * t153 * t335 + f64x8::splat(0.2509862353534166) * t498 + f64x8::splat(0.005061925051098745) * t504 * t126 - f64x8::splat(0.0016609256494831233) * t507 - f64x8::splat(0.0016609256494831233) * t509 - f64x8::splat(0.0010249162124576494) * t504 * t117 - f64x8::splat(0.0010249162124576494) * t153 * t356 - f64x8::splat(3.656012084198544e-05) * t504 * t122 - f64x8::splat(0.007555456486598222) * t504 * t106 - f64x8::splat(0.007555456486598222) * t153 * t367 - f64x8::splat(0.0038541498256550073) * t504 * t112 - f64x8::splat(0.0038541498256550073) * t153 * t345 - f64x8::splat(0.0005194058669188706) * t504 * t99;
            let t533 = t50 * t285;
            let t538 = f64x8::splat(693.0) / f64x8::splat(8.0) * t533 - f64x8::splat(315.0) / f64x8::splat(4.0) * t465 + f64x8::splat(105.0) / f64x8::splat(8.0) * t408;
            let t541 = t538 * t89;
            let t543 = t135 * t324;
            let t561 = -f64x8::splat(0.0005194058669188706) * t153 * t380 - f64x8::splat(0.009195715678311926) * t135 * t335 + f64x8::splat(0.4380036644282133) * t533 - f64x8::splat(0.007631605623646023) * t538 * t126 + f64x8::splat(0.0028206838819829017) * t541 + f64x8::splat(0.0028206838819829017) * t543 + f64x8::splat(0.0008367073496483024) * t538 * t117 + f64x8::splat(0.0008367073496483024) * t135 * t356 - f64x8::splat(0.009195715678311926) * t538 * t122 - f64x8::splat(0.0031296536914037784) * t538 * t112 - f64x8::splat(0.0031296536914037784) * t135 * t345 - f64x8::splat(0.00018156466410673526) * t135 * t380 + f64x8::splat(0.001864317026752979) * t538 * t106 + f64x8::splat(0.001864317026752979) * t135 * t367;
            let t562 = t131 * t285;
            let t569 = f64x8::splat(3003.0) / f64x8::splat(16.0) * t562 - f64x8::splat(3465.0) / f64x8::splat(16.0) * t498 + f64x8::splat(945.0) / f64x8::splat(16.0) * t391 + f64x8::splat(35.0) / f64x8::splat(72.0) * t267 - f64x8::splat(35.0) / f64x8::splat(864.0) * t283;
            let t572 = t569 * t89;
            let t574 = t54 * t324;
            let t598 = -f64x8::splat(0.2814293849782344) * t562 - f64x8::splat(0.01006770315965861) * t569 * t126 + f64x8::splat(0.00017309630990864668) * t572 + f64x8::splat(0.00017309630990864668) * t574 - f64x8::splat(0.00018156466410673526) * t538 * t99 - f64x8::splat(0.00031389079758955066) * t569 * t117 - f64x8::splat(0.00031389079758955066) * t54 * t356 + f64x8::splat(0.010726279571787276) * t569 * t122 + f64x8::splat(0.010726279571787276) * t54 * t335 + f64x8::splat(0.0038758929812102785) * t569 * t112 + f64x8::splat(0.0038758929812102785) * t54 * t345 - f64x8::splat(0.00019095139973664826) * t569 * t106 - f64x8::splat(0.00019095139973664826) * t54 * t367 - f64x8::splat(0.00029476504977320184) * t569 * t99 - f64x8::splat(0.00029476504977320184) * t54 * t380;
            let t601 = t361 + t402 + t431 + t460 + t493 + t527 + t561 + t598;
            let t606 = ((t3).select(f64x8::splat(0.0), -t7 * t258 * t252 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t7 * t20 * t601));
            let tvrho0 = f64x8::splat(2.0) * v_rho * t606 + f64x8::splat(2.0) * t256;
            acc_vrho = tvrho0;
            let t609 = t26 * t34;
            let t610 = f64x8::splat(5.0) / f64x8::splat(72.0) * t609;
            let t611 = ((t65).select(-t610, f64x8::splat(0.0)));
            let t614 = t74 * t611;
            let t616 = t302 * t611;
            let t619 = ((t65).select(f64x8::splat(0.0), -t610));
            let t620 = t77 * t619;
            let t627 = f64x8::splat(3.0) * t314 * t619 + f64x8::splat(12.0) * t318 * t619;
            let t630 = ((t64).select(-f64x8::splat(3.0) / f64x8::splat(2.0) * t71 * t611 - f64x8::splat(3.0) / f64x8::splat(16.0) * t614 + f64x8::splat(3.0) * t616, -f64x8::splat(6.0) * t306 * t620 - t313 * t627));
            let t633 = t275 * v_rho;
            let t637 = t28 / t19 / t633 * t281;
            let t638 = t272 * v_sigma * t637;
            let t640 = t26 * t41;
            let t642 = t102 * t630;
            let t644 = t92 * t630;
            let t646 = t90 * t630;
            let t652 = f64x8::splat(3003.0) / f64x8::splat(16.0) * t642 - f64x8::splat(3465.0) / f64x8::splat(16.0) * t644 + f64x8::splat(945.0) / f64x8::splat(16.0) * t646 - f64x8::splat(35.0) / f64x8::splat(16.0) * t630;
            let t657 = t640 / f64x8::splat(12.0) - t638 / f64x8::splat(144.0);
            let t658 = t47 * t657;
            let t660 = t45 * t657;
            let t666 = f64x8::splat(315.0) / f64x8::splat(8.0) * t658 - f64x8::splat(105.0) / f64x8::splat(4.0) * t660 + f64x8::splat(5.0) / f64x8::splat(32.0) * t640 - f64x8::splat(5.0) / f64x8::splat(384.0) * t638;
            let t669 = t95 * t630;
            let t671 = t91 * t630;
            let t673 = t89 * t630;
            let t678 = f64x8::splat(693.0) / f64x8::splat(8.0) * t669 - f64x8::splat(315.0) / f64x8::splat(4.0) * t671 + f64x8::splat(105.0) / f64x8::splat(8.0) * t673;
            let t681 = f64x8::splat(0.19451907596748125) * t630 + f64x8::splat(0.0009288231593566592) * t638 - f64x8::splat(0.011145877912279912) * t640 - f64x8::splat(0.3801267001217285) * t642 + f64x8::splat(0.2613989191485382) * t644 - f64x8::splat(0.017769411149910222) * t646 - f64x8::splat(0.0005194058669188706) * t153 * t652 + f64x8::splat(0.2509862353534166) * t658 - f64x8::splat(0.02414252189674374) * t660 - f64x8::splat(0.007555456486598222) * t666 * t106 + f64x8::splat(0.026485532388814615) * t669 - f64x8::splat(0.04914917506022932) * t671 + f64x8::splat(0.012711844558663081) * t673 - f64x8::splat(0.007555456486598222) * t153 * t678;
            let t682 = t50 * t657;
            let t684 = t46 * t657;
            let t686 = t44 * t657;
            let t691 = f64x8::splat(693.0) / f64x8::splat(8.0) * t682 - f64x8::splat(315.0) / f64x8::splat(4.0) * t684 + f64x8::splat(105.0) / f64x8::splat(8.0) * t686;
            let t694 = t691 * t89;
            let t696 = t135 * t630;
            let t704 = f64x8::splat(35.0) / f64x8::splat(2.0) * t671 - f64x8::splat(15.0) / f64x8::splat(2.0) * t673;
            let t711 = f64x8::splat(15.0) / f64x8::splat(2.0) * t646 - f64x8::splat(3.0) / f64x8::splat(2.0) * t630;
            let t719 = f64x8::splat(315.0) / f64x8::splat(8.0) * t644 - f64x8::splat(105.0) / f64x8::splat(4.0) * t646 + f64x8::splat(15.0) / f64x8::splat(8.0) * t630;
            let t724 = f64x8::splat(0.4380036644282133) * t682 - f64x8::splat(0.2283379049668504) * t684 - f64x8::splat(0.7646188187081253) * t686 - f64x8::splat(0.007631605623646023) * t691 * t126 + f64x8::splat(0.0028206838819829017) * t694 + f64x8::splat(0.0028206838819829017) * t696 - f64x8::splat(0.0005194058669188706) * t666 * t99 + f64x8::splat(0.0008367073496483024) * t691 * t117 + f64x8::splat(0.0008367073496483024) * t135 * t704 - f64x8::splat(0.009195715678311926) * t691 * t122 - f64x8::splat(0.009195715678311926) * t135 * t711 - f64x8::splat(0.0031296536914037784) * t691 * t112 - f64x8::splat(0.0031296536914037784) * t135 * t719 + f64x8::splat(0.001864317026752979) * t691 * t106;
            let t728 = t131 * t657;
            let t735 = f64x8::splat(3003.0) / f64x8::splat(16.0) * t728 - f64x8::splat(3465.0) / f64x8::splat(16.0) * t658 + f64x8::splat(945.0) / f64x8::splat(16.0) * t660 - f64x8::splat(35.0) / f64x8::splat(192.0) * t640 + f64x8::splat(35.0) / f64x8::splat(2304.0) * t638;
            let t738 = t735 * t89;
            let t740 = t54 * t630;
            let t760 = f64x8::splat(0.001864317026752979) * t135 * t678 - f64x8::splat(0.2814293849782344) * t728 - f64x8::splat(0.01006770315965861) * t735 * t126 + f64x8::splat(0.00017309630990864668) * t738 + f64x8::splat(0.00017309630990864668) * t740 - f64x8::splat(0.00018156466410673526) * t691 * t99 - f64x8::splat(0.00018156466410673526) * t135 * t652 - f64x8::splat(0.00031389079758955066) * t54 * t704 + f64x8::splat(0.010726279571787276) * t735 * t122 + f64x8::splat(0.010726279571787276) * t54 * t711 + f64x8::splat(0.0038758929812102785) * t735 * t112 + f64x8::splat(0.0038758929812102785) * t54 * t719 - f64x8::splat(0.00031389079758955066) * t735 * t117 - f64x8::splat(0.00019095139973664826) * t735 * t106;
            let t781 = t666 * t89;
            let t783 = t153 * t630;
            let t787 = f64x8::splat(35.0) / f64x8::splat(2.0) * t684 - f64x8::splat(15.0) / f64x8::splat(2.0) * t686;
            let t792 = -f64x8::splat(0.00019095139973664826) * t54 * t678 - f64x8::splat(0.00029476504977320184) * t735 * t99 - f64x8::splat(0.00029476504977320184) * t54 * t652 - f64x8::splat(0.0038541498256550073) * t666 * t112 - f64x8::splat(0.0038541498256550073) * t153 * t719 - f64x8::splat(0.0010249162124576494) * t666 * t117 - f64x8::splat(0.0010249162124576494) * t153 * t704 - f64x8::splat(3.656012084198544e-05) * t666 * t122 - f64x8::splat(3.656012084198544e-05) * t153 * t711 + f64x8::splat(0.005061925051098745) * t666 * t126 - f64x8::splat(0.0016609256494831233) * t781 - f64x8::splat(0.0016609256494831233) * t783 - f64x8::splat(1.792697304428732e-05) * t787 * t99 - f64x8::splat(1.792697304428732e-05) * t171 * t652;
            let t813 = t787 * t89;
            let t815 = t171 * t630;
            let t820 = f64x8::splat(15.0) / f64x8::splat(2.0) * t660 - t640 / f64x8::splat(8.0) + t638 / f64x8::splat(96.0);
            let t827 = f64x8::splat(0.0001331797359718674) * t787 * t106 + f64x8::splat(0.0001331797359718674) * t171 * t678 - f64x8::splat(7.261106354828029e-05) * t787 * t112 - f64x8::splat(7.261106354828029e-05) * t171 * t719 + f64x8::splat(0.0009891355730978566) * t787 * t117 + f64x8::splat(0.0009891355730978566) * t171 * t704 - f64x8::splat(0.0002571281595426713) * t787 * t122 - f64x8::splat(0.0002571281595426713) * t171 * t711 - f64x8::splat(0.0014878680171769923) * t787 * t126 - f64x8::splat(0.0021100890252897446) * t813 - f64x8::splat(0.0021100890252897446) * t815 + f64x8::splat(0.0004308565933608885) * t820 * t99 - f64x8::splat(0.03020310947897583) * t129 * t630 - f64x8::splat(0.02289481687093807) * t148 * t630;
            let t856 = f64x8::splat(0.015185775153296235) * t167 * t630 - f64x8::splat(0.004463604051530977) * t184 * t630 - f64x8::splat(0.00023470764417047457) * t203 * t630 + f64x8::splat(0.0005681706522972924) * t686 * t99 - f64x8::splat(0.0027146561728928226) * t686 * t106 + f64x8::splat(0.00025448301445575583) * t686 * t112 + f64x8::splat(0.0009541479707825193) * t686 * t117 - f64x8::splat(0.0026011607116438384) * t686 * t122 - f64x8::splat(0.002505993789510108) * t686 * t126 - f64x8::splat(0.002505993789510108) * t219 * t630 - f64x8::splat(0.03940681275548879) * t686 * t89 + f64x8::splat(0.0037023943917136176) * t234 * t630 + f64x8::splat(0.0004308565933608885) * t188 * t652 - f64x8::splat(0.000689695394243961) * t820 * t106;
            let t874 = t820 * t89;
            let t876 = t188 * t630;
            let t886 = -f64x8::splat(0.000689695394243961) * t188 * t678 - f64x8::splat(0.00019375881298946268) * t820 * t112 - f64x8::splat(0.00019375881298946268) * t188 * t719 - f64x8::splat(0.004704436332280876) * t820 * t117 - f64x8::splat(0.004704436332280876) * t188 * t704 + f64x8::splat(0.0027822064319562786) * t820 * t122 + f64x8::splat(0.0027822064319562786) * t188 * t711 - f64x8::splat(7.823588139015819e-05) * t820 * t126 - f64x8::splat(0.016823429546012295) * t874 - f64x8::splat(0.016823429546012295) * t876 + f64x8::splat(0.00018939021743243079) * t206 * t652 - f64x8::splat(0.0009048853909642742) * t206 * t678 + f64x8::splat(8.482767148525194e-05) * t206 * t719 + f64x8::splat(0.0003180493235941731) * t206 * t704;
            let t889 = t206 * t630;
            let t891 = t657 * t99;
            let t895 = t657 * t106;
            let t899 = t657 * t112;
            let t903 = t657 * t117;
            let t907 = t657 * t122;
            let t911 = t657 * t126;
            let t913 = t657 * t89;
            let t915 = t44 * t630;
            let t917 = -f64x8::splat(0.0008670535705479461) * t206 * t711 - f64x8::splat(0.013135604251829597) * t889 + f64x8::splat(0.0023160016166370034) * t891 + f64x8::splat(0.0023160016166370034) * t44 * t652 + f64x8::splat(0.0005970286163074767) * t895 + f64x8::splat(0.0005970286163074767) * t44 * t678 + f64x8::splat(0.0016437722411542371) * t899 + f64x8::splat(0.0016437722411542371) * t44 * t719 + f64x8::splat(0.0050995906979556666) * t903 + f64x8::splat(0.0050995906979556666) * t44 * t704 + f64x8::splat(0.0024977311122498513) * t907 + f64x8::splat(0.0024977311122498513) * t44 * t711 + f64x8::splat(0.0012341314639045392) * t911 + f64x8::splat(0.12131628073942294) * t913 + f64x8::splat(0.12131628073942294) * t915;
            let t920 = t681 + t724 + t760 + t792 + t827 + t856 + t886 + t917;
            let t924 = ((t3).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t7 * t20 * t920));
            let tvsigma0 = f64x8::splat(2.0) * v_rho * t924;
            acc_vsigma = tvsigma0;
            let tvlapl0 = f64x8::splat(0.0);
            acc_vlapl = tvlapl0;
            let t928 = f64x8::splat(5.0) / f64x8::splat(9.0) * t29 * t57 * t26;
            let t929 = ((t65).select(t928, f64x8::splat(0.0)));
            let t932 = t74 * t929;
            let t934 = t302 * t929;
            let t937 = ((t65).select(f64x8::splat(0.0), t928));
            let t938 = t77 * t937;
            let t945 = f64x8::splat(3.0) * t314 * t937 + f64x8::splat(12.0) * t318 * t937;
            let t948 = ((t64).select(-f64x8::splat(3.0) / f64x8::splat(2.0) * t71 * t929 - f64x8::splat(3.0) / f64x8::splat(16.0) * t932 + f64x8::splat(3.0) * t934, -f64x8::splat(6.0) * t306 * t938 - t313 * t945));
            let t964 = t44 * t948;
            let t966 = t91 * t948;
            let t968 = t89 * t948;
            let t972 = f64x8::splat(35.0) / f64x8::splat(2.0) * t966 - f64x8::splat(15.0) / f64x8::splat(2.0) * t968;
            let t975 = t90 * t948;
            let t979 = f64x8::splat(15.0) / f64x8::splat(2.0) * t975 - f64x8::splat(3.0) / f64x8::splat(2.0) * t948;
            let t982 = f64x8::splat(0.19451907596748125) * t948 - f64x8::splat(0.03020310947897583) * t129 * t948 + f64x8::splat(0.0037023943917136176) * t234 * t948 - f64x8::splat(0.002505993789510108) * t219 * t948 - f64x8::splat(0.00023470764417047457) * t203 * t948 + f64x8::splat(0.015185775153296235) * t167 * t948 - f64x8::splat(0.02289481687093807) * t148 * t948 - f64x8::splat(0.004463604051530977) * t184 * t948 + f64x8::splat(0.12131628073942294) * t964 - f64x8::splat(0.04914917506022932) * t966 + f64x8::splat(0.012711844558663081) * t968 + f64x8::splat(0.0050995906979556666) * t44 * t972 - f64x8::splat(0.017769411149910222) * t975 + f64x8::splat(0.0024977311122498513) * t44 * t979;
            let t983 = t95 * t948;
            let t988 = f64x8::splat(693.0) / f64x8::splat(8.0) * t983 - f64x8::splat(315.0) / f64x8::splat(4.0) * t966 + f64x8::splat(105.0) / f64x8::splat(8.0) * t968;
            let t991 = t92 * t948;
            let t996 = f64x8::splat(315.0) / f64x8::splat(8.0) * t991 - f64x8::splat(105.0) / f64x8::splat(4.0) * t975 + f64x8::splat(15.0) / f64x8::splat(8.0) * t948;
            let t999 = t206 * t948;
            let t1001 = t102 * t948;
            let t1007 = f64x8::splat(3003.0) / f64x8::splat(16.0) * t1001 - f64x8::splat(3465.0) / f64x8::splat(16.0) * t991 + f64x8::splat(945.0) / f64x8::splat(16.0) * t975 - f64x8::splat(35.0) / f64x8::splat(16.0) * t948;
            let t1018 = t188 * t948;
            let t1024 = f64x8::splat(0.026485532388814615) * t983 + f64x8::splat(0.0005970286163074767) * t44 * t988 + f64x8::splat(0.2613989191485382) * t991 + f64x8::splat(0.0016437722411542371) * t44 * t996 - f64x8::splat(0.013135604251829597) * t999 - f64x8::splat(0.3801267001217285) * t1001 + f64x8::splat(0.0023160016166370034) * t44 * t1007 - f64x8::splat(0.0008670535705479461) * t206 * t979 + f64x8::splat(0.0003180493235941731) * t206 * t972 - f64x8::splat(0.0009048853909642742) * t206 * t988 + f64x8::splat(8.482767148525194e-05) * t206 * t996 - f64x8::splat(0.016823429546012295) * t1018 + f64x8::splat(0.00018939021743243079) * t206 * t1007 - f64x8::splat(0.004704436332280876) * t188 * t972;
            let t1036 = t171 * t948;
            let t1044 = t153 * t948;
            let t1054 = f64x8::splat(0.0027822064319562786) * t188 * t979 - f64x8::splat(0.00019375881298946268) * t188 * t996 + f64x8::splat(0.0004308565933608885) * t188 * t1007 - f64x8::splat(0.000689695394243961) * t188 * t988 - f64x8::splat(0.0002571281595426713) * t171 * t979 - f64x8::splat(0.0021100890252897446) * t1036 + f64x8::splat(0.0009891355730978566) * t171 * t972 + f64x8::splat(0.0001331797359718674) * t171 * t988 - f64x8::splat(7.261106354828029e-05) * t171 * t996 - f64x8::splat(0.0016609256494831233) * t1044 - f64x8::splat(1.792697304428732e-05) * t171 * t1007 - f64x8::splat(3.656012084198544e-05) * t153 * t979 - f64x8::splat(0.0010249162124576494) * t153 * t972 - f64x8::splat(0.007555456486598222) * t153 * t988;
            let t1061 = t135 * t948;
            let t1069 = t54 * t948;
            let t1083 = -f64x8::splat(0.0038541498256550073) * t153 * t996 - f64x8::splat(0.0005194058669188706) * t153 * t1007 - f64x8::splat(0.009195715678311926) * t135 * t979 + f64x8::splat(0.0028206838819829017) * t1061 + f64x8::splat(0.0008367073496483024) * t135 * t972 - f64x8::splat(0.0031296536914037784) * t135 * t996 + f64x8::splat(0.001864317026752979) * t135 * t988 + f64x8::splat(0.00017309630990864668) * t1069 - f64x8::splat(0.00018156466410673526) * t135 * t1007 - f64x8::splat(0.00031389079758955066) * t54 * t972 + f64x8::splat(0.010726279571787276) * t54 * t979 + f64x8::splat(0.0038758929812102785) * t54 * t996 - f64x8::splat(0.00019095139973664826) * t54 * t988 - f64x8::splat(0.00029476504977320184) * t54 * t1007;
            let t1085 = t982 + t1024 + t1054 + t1083;
            let t1089 = ((t3).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t7 * t20 * t1085));
            let tvtau0 = f64x8::splat(2.0) * v_rho * t1089;
            acc_vtau = tvtau0;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_vrho.into(); vrho[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_vsigma.into(); vsigma[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_vlapl.into(); vlapl[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_vtau.into(); vtau[ip..ip + m].copy_from_slice(&a[..m]); }
        ip += 8;
    }
}
