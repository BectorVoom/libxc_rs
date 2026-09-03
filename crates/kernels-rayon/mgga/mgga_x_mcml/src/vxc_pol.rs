//! MGGA_X_MCML vxc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_mcml.c`
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
pub fn mgga_x_mcml_vxc_pol(
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
            let t76 = t75 * t74;
            let t80 = t40 * t45;
            let t83 = f64x8::splat(6.5124) + t80 * t37 / f64x8::splat(24.0);
            let t84 = f64x8::splat(1.0) / t83;
            let t86 = t80 * t37 * t84;
            let t88 = t86 / f64x8::splat(12.0) - f64x8::splat(1.0);
            let t89 = t88 * t88;
            let t90 = t89 * t89;
            let t91 = t90 * t89;
            let t93 = t89 * t88;
            let t94 = t90 * t93;
            let t96 = t90 * t88;
            let t103 = t75 * t73;
            let t105 = t74 * t73;
            let t107 = t75 * t105;
            let t113 = f64x8::splat(429.0) / f64x8::splat(16.0) * t94 - f64x8::splat(693.0) / f64x8::splat(16.0) * t96 + f64x8::splat(315.0) / f64x8::splat(16.0) * t93 - f64x8::splat(35.0) / f64x8::splat(192.0) * t86 + f64x8::splat(35.0) / f64x8::splat(16.0);
            let t118 = f64x8::splat(429.0) / f64x8::splat(16.0) * t107 - f64x8::splat(693.0) / f64x8::splat(16.0) * t103 + f64x8::splat(315.0) / f64x8::splat(16.0) * t105 - f64x8::splat(35.0) / f64x8::splat(16.0) * t73;
            let t121 = f64x8::splat(1.3502664484515603) - f64x8::splat(0.028551704175417886) * t76 + f64x8::splat(0.029439726278665656) * t75 - f64x8::splat(0.005882884490994137) * t74 + f64x8::splat(0.022419222998949863) * t91 + f64x8::splat(0.015682422300093094) * t94 - f64x8::splat(0.015887583418757175) * t96 - f64x8::splat(0.01346592172626102) * t86 - f64x8::splat(0.37102687351218927) * t89 + f64x8::splat(0.007416880187036192) * t93 - f64x8::splat(0.0010470532939127494) * t90 + f64x8::splat(0.2074861966146727) * t73 + f64x8::splat(0.08753451580964014) * t103 - f64x8::splat(0.03212149513526167) * t105 - f64x8::splat(0.06746454865517729) * t107 - f64x8::splat(0.0003695503801501715) * t113 * t118;
            let t125 = -f64x8::splat(5.0) / f64x8::splat(16.0) + f64x8::splat(231.0) / f64x8::splat(16.0) * t76 - f64x8::splat(315.0) / f64x8::splat(16.0) * t75 + f64x8::splat(105.0) / f64x8::splat(16.0) * t74;
            let t131 = f64x8::splat(63.0) / f64x8::splat(8.0) * t103 - f64x8::splat(35.0) / f64x8::splat(4.0) * t105 + f64x8::splat(15.0) / f64x8::splat(8.0) * t73;
            let t136 = f64x8::splat(3.0) / f64x8::splat(8.0) + f64x8::splat(35.0) / f64x8::splat(8.0) * t75 - f64x8::splat(15.0) / f64x8::splat(4.0) * t74;
            let t141 = f64x8::splat(5.0) / f64x8::splat(2.0) * t105 - f64x8::splat(3.0) / f64x8::splat(2.0) * t73;
            let t145 = -f64x8::splat(1.0) / f64x8::splat(2.0) + f64x8::splat(3.0) / f64x8::splat(2.0) * t74;
            let t148 = t113 * t73;
            let t153 = -f64x8::splat(5.0) / f64x8::splat(16.0) + f64x8::splat(231.0) / f64x8::splat(16.0) * t91 - f64x8::splat(315.0) / f64x8::splat(16.0) * t90 + f64x8::splat(105.0) / f64x8::splat(16.0) * t89;
            let t166 = t153 * t73;
            let t171 = f64x8::splat(63.0) / f64x8::splat(8.0) * t96 - f64x8::splat(35.0) / f64x8::splat(4.0) * t93 + f64x8::splat(5.0) / f64x8::splat(32.0) * t86 - f64x8::splat(15.0) / f64x8::splat(8.0);
            let t178 = -f64x8::splat(0.0003682519432462936) * t113 * t125 + f64x8::splat(0.001522474179598972) * t113 * t131 + f64x8::splat(0.00245752591853626) * t113 * t136 + f64x8::splat(0.01243327883803539) * t113 * t141 + f64x8::splat(0.001421391023843761) * t113 * t145 + f64x8::splat(0.0003837976998664341) * t148 + f64x8::splat(0.0003807158595350892) * t153 * t118 + f64x8::splat(0.0004260858412001439) * t153 * t125 + f64x8::splat(0.001136485825094485) * t153 * t131 + f64x8::splat(0.0004230264400260503) * t153 * t136 - f64x8::splat(0.006510071882485726) * t153 * t141 - f64x8::splat(0.005498112922165805) * t153 * t145 + f64x8::splat(0.002334616776649133) * t166 - f64x8::splat(0.0002202759704065197) * t171 * t118 - f64x8::splat(0.001622621390953226) * t171 * t125 - f64x8::splat(0.0005869916483960576) * t171 * t131;
            let t186 = t171 * t73;
            let t190 = f64x8::splat(3.0) / f64x8::splat(8.0) + f64x8::splat(35.0) / f64x8::splat(8.0) * t90 - f64x8::splat(15.0) / f64x8::splat(4.0) * t89;
            let t203 = t190 * t73;
            let t207 = f64x8::splat(5.0) / f64x8::splat(2.0) * t93 - t86 / f64x8::splat(8.0) + f64x8::splat(3.0) / f64x8::splat(2.0);
            let t218 = -f64x8::splat(0.001009981263546227) * t171 * t136 + f64x8::splat(0.0002262886186270548) * t171 * t141 + f64x8::splat(0.006670848599065867) * t171 * t145 - f64x8::splat(0.000257733338272708) * t186 + f64x8::splat(3.212943141118693e-06) * t190 * t118 + f64x8::splat(0.0002776060240069905) * t190 * t125 - f64x8::splat(0.0002721968500889238) * t190 * t131 + f64x8::splat(0.0004187827907710905) * t190 * t136 + f64x8::splat(0.001282471852770764) * t190 * t141 + f64x8::splat(0.000137028863545747) * t190 * t145 + f64x8::splat(0.01683215086686233) * t203 + f64x8::splat(0.0004312411759243052) * t207 * t118 - f64x8::splat(0.0006058496834176058) * t207 * t125 + f64x8::splat(0.0001672905908063297) * t207 * t131 - f64x8::splat(0.002494950550547465) * t207 * t136 + f64x8::splat(0.003712786171321043) * t207 * t141;
            let t221 = t207 * t73;
            let t224 = -f64x8::splat(1.0) / f64x8::splat(2.0) + f64x8::splat(3.0) / f64x8::splat(2.0) * t89;
            let t237 = t224 * t73;
            let t251 = t88 * t73;
            let t253 = -f64x8::splat(0.0007090296813211244) * t207 * t145 - f64x8::splat(0.01030571429426108) * t221 - f64x8::splat(0.001175614476758423) * t224 * t118 - f64x8::splat(0.001288306127279617) * t224 * t125 - f64x8::splat(0.001189668304951413) * t224 * t131 - f64x8::splat(0.001863882881010248) * t224 * t136 - f64x8::splat(0.0009641371299507833) * t224 * t141 - f64x8::splat(0.001153807045825489) * t224 * t145 - f64x8::splat(0.01437960658302686) * t237 + f64x8::splat(0.001940164714223896) * t88 * t118 + f64x8::splat(0.001491587478361034) * t88 * t125 + f64x8::splat(0.002007295399058147) * t88 * t131 + f64x8::splat(0.002915285520983635) * t88 * t136 + f64x8::splat(0.002125332357775206) * t88 * t141 + f64x8::splat(0.00179463855686441) * t88 * t145 + f64x8::splat(0.1179363564823021) * t251;
            let t255 = t121 + t178 + t218 + t253;
            let t259 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t28 * t255));
            let t260 = (v_rho1).simd_le(dens_threshold);
            let t261 = -t17;
            let t263 = ((t15).select(t12, (t11).select(t16, t261 * t8)));
            let t264 = t263 + f64x8::splat(1.0);
            let t265 = (t264).simd_le(zeta_threshold);
            let t266 = (simd::cbrt(t264));
            let t268 = ((t265).select(t23, t266 * t264));
            let t269 = t268 * t27;
            let t270 = (simd::cbrt(v_rho1));
            let t271 = t270 * t270;
            let t273 = f64x8::splat(1.0) / t271 / v_rho1;
            let t275 = v_rho1 * v_rho1;
            let t277 = f64x8::splat(1.0) / t271 / t275;
            let t278 = v_sigma2 * t277;
            let t283 = f64x8::splat(5.0) / f64x8::splat(9.0) * (v_tau1 * t273 - t278 / f64x8::splat(8.0)) * t40 * t45;
            let t284 = (f64x8::splat(10000.0)).simd_le(t283);
            let t285 = (f64x8::splat(10000.0)).simd_lt(t283);
            let t286 = ((t285).select(t283, f64x8::splat(10000.0)));
            let t287 = t286 * t286;
            let t290 = t287 * t286;
            let t291 = f64x8::splat(1.0) / t290;
            let t293 = t287 * t287;
            let t294 = f64x8::splat(1.0) / t293;
            let t297 = ((t285).select(f64x8::splat(10000.0), t283));
            let t298 = t297 * t297;
            let t299 = f64x8::splat(1.0) - t298;
            let t300 = t299 * t299;
            let t301 = t300 * t299;
            let t302 = t298 * t297;
            let t304 = f64x8::splat(1.0) + f64x8::splat(4.0) * t302;
            let t306 = t302 * t304 + f64x8::splat(1.0);
            let t307 = f64x8::splat(1.0) / t306;
            let t309 = ((t284).select(-f64x8::splat(1.0) / f64x8::splat(4.0) + f64x8::splat(3.0) / f64x8::splat(4.0) / t287 + t291 / f64x8::splat(16.0) - f64x8::splat(3.0) / f64x8::splat(4.0) * t294, t301 * t307));
            let t310 = t309 * t309;
            let t311 = t310 * t310;
            let t312 = t311 * t310;
            let t318 = f64x8::splat(6.5124) + t80 * t278 / f64x8::splat(24.0);
            let t319 = f64x8::splat(1.0) / t318;
            let t321 = t80 * t278 * t319;
            let t323 = t321 / f64x8::splat(12.0) - f64x8::splat(1.0);
            let t324 = t323 * t323;
            let t325 = t324 * t324;
            let t326 = t325 * t324;
            let t330 = t325 * t323;
            let t333 = t324 * t323;
            let t334 = t325 * t333;
            let t336 = t311 * t309;
            let t338 = t310 * t309;
            let t340 = t311 * t338;
            let t348 = f64x8::splat(429.0) / f64x8::splat(16.0) * t334 - f64x8::splat(693.0) / f64x8::splat(16.0) * t330 + f64x8::splat(315.0) / f64x8::splat(16.0) * t333 - f64x8::splat(35.0) / f64x8::splat(192.0) * t321 + f64x8::splat(35.0) / f64x8::splat(16.0);
            let t353 = f64x8::splat(429.0) / f64x8::splat(16.0) * t340 - f64x8::splat(693.0) / f64x8::splat(16.0) * t336 + f64x8::splat(315.0) / f64x8::splat(16.0) * t338 - f64x8::splat(35.0) / f64x8::splat(16.0) * t309;
            let t356 = f64x8::splat(1.3502664484515603) - f64x8::splat(0.028551704175417886) * t312 + f64x8::splat(0.029439726278665656) * t311 - f64x8::splat(0.005882884490994137) * t310 + f64x8::splat(0.022419222998949863) * t326 - f64x8::splat(0.0010470532939127494) * t325 + f64x8::splat(0.2074861966146727) * t309 - f64x8::splat(0.015887583418757175) * t330 - f64x8::splat(0.01346592172626102) * t321 + f64x8::splat(0.015682422300093094) * t334 + f64x8::splat(0.08753451580964014) * t336 - f64x8::splat(0.03212149513526167) * t338 - f64x8::splat(0.06746454865517729) * t340 + f64x8::splat(0.007416880187036192) * t333 - f64x8::splat(0.37102687351218927) * t324 - f64x8::splat(0.0003695503801501715) * t348 * t353;
            let t360 = -f64x8::splat(5.0) / f64x8::splat(16.0) + f64x8::splat(231.0) / f64x8::splat(16.0) * t312 - f64x8::splat(315.0) / f64x8::splat(16.0) * t311 + f64x8::splat(105.0) / f64x8::splat(16.0) * t310;
            let t366 = f64x8::splat(63.0) / f64x8::splat(8.0) * t336 - f64x8::splat(35.0) / f64x8::splat(4.0) * t338 + f64x8::splat(15.0) / f64x8::splat(8.0) * t309;
            let t371 = f64x8::splat(3.0) / f64x8::splat(8.0) + f64x8::splat(35.0) / f64x8::splat(8.0) * t311 - f64x8::splat(15.0) / f64x8::splat(4.0) * t310;
            let t376 = f64x8::splat(5.0) / f64x8::splat(2.0) * t338 - f64x8::splat(3.0) / f64x8::splat(2.0) * t309;
            let t380 = -f64x8::splat(1.0) / f64x8::splat(2.0) + f64x8::splat(3.0) / f64x8::splat(2.0) * t310;
            let t383 = t348 * t309;
            let t388 = -f64x8::splat(5.0) / f64x8::splat(16.0) + f64x8::splat(231.0) / f64x8::splat(16.0) * t326 - f64x8::splat(315.0) / f64x8::splat(16.0) * t325 + f64x8::splat(105.0) / f64x8::splat(16.0) * t324;
            let t401 = t388 * t309;
            let t406 = f64x8::splat(63.0) / f64x8::splat(8.0) * t330 - f64x8::splat(35.0) / f64x8::splat(4.0) * t333 + f64x8::splat(5.0) / f64x8::splat(32.0) * t321 - f64x8::splat(15.0) / f64x8::splat(8.0);
            let t413 = -f64x8::splat(0.0003682519432462936) * t348 * t360 + f64x8::splat(0.001522474179598972) * t348 * t366 + f64x8::splat(0.00245752591853626) * t348 * t371 + f64x8::splat(0.01243327883803539) * t348 * t376 + f64x8::splat(0.001421391023843761) * t348 * t380 + f64x8::splat(0.0003837976998664341) * t383 + f64x8::splat(0.0003807158595350892) * t388 * t353 + f64x8::splat(0.0004260858412001439) * t388 * t360 + f64x8::splat(0.001136485825094485) * t388 * t366 + f64x8::splat(0.0004230264400260503) * t388 * t371 - f64x8::splat(0.006510071882485726) * t388 * t376 - f64x8::splat(0.005498112922165805) * t388 * t380 + f64x8::splat(0.002334616776649133) * t401 - f64x8::splat(0.0002202759704065197) * t406 * t353 - f64x8::splat(0.001622621390953226) * t406 * t360 - f64x8::splat(0.0005869916483960576) * t406 * t366;
            let t421 = t406 * t309;
            let t425 = f64x8::splat(3.0) / f64x8::splat(8.0) + f64x8::splat(35.0) / f64x8::splat(8.0) * t325 - f64x8::splat(15.0) / f64x8::splat(4.0) * t324;
            let t438 = t425 * t309;
            let t442 = f64x8::splat(5.0) / f64x8::splat(2.0) * t333 - t321 / f64x8::splat(8.0) + f64x8::splat(3.0) / f64x8::splat(2.0);
            let t453 = -f64x8::splat(0.001009981263546227) * t406 * t371 + f64x8::splat(0.0002262886186270548) * t406 * t376 + f64x8::splat(0.006670848599065867) * t406 * t380 - f64x8::splat(0.000257733338272708) * t421 + f64x8::splat(3.212943141118693e-06) * t425 * t353 + f64x8::splat(0.0002776060240069905) * t425 * t360 - f64x8::splat(0.0002721968500889238) * t425 * t366 + f64x8::splat(0.0004187827907710905) * t425 * t371 + f64x8::splat(0.001282471852770764) * t425 * t376 + f64x8::splat(0.000137028863545747) * t425 * t380 + f64x8::splat(0.01683215086686233) * t438 + f64x8::splat(0.0004312411759243052) * t442 * t353 - f64x8::splat(0.0006058496834176058) * t442 * t360 + f64x8::splat(0.0001672905908063297) * t442 * t366 - f64x8::splat(0.002494950550547465) * t442 * t371 + f64x8::splat(0.003712786171321043) * t442 * t376;
            let t456 = t442 * t309;
            let t459 = -f64x8::splat(1.0) / f64x8::splat(2.0) + f64x8::splat(3.0) / f64x8::splat(2.0) * t324;
            let t470 = t323 * t309;
            let t484 = t459 * t309;
            let t488 = -f64x8::splat(0.0007090296813211244) * t442 * t380 - f64x8::splat(0.01030571429426108) * t456 - f64x8::splat(0.001175614476758423) * t459 * t353 - f64x8::splat(0.001288306127279617) * t459 * t360 - f64x8::splat(0.001189668304951413) * t459 * t366 - f64x8::splat(0.001863882881010248) * t459 * t371 - f64x8::splat(0.0009641371299507833) * t459 * t376 + f64x8::splat(0.1179363564823021) * t470 + f64x8::splat(0.00179463855686441) * t323 * t380 + f64x8::splat(0.002125332357775206) * t323 * t376 + f64x8::splat(0.002915285520983635) * t323 * t371 + f64x8::splat(0.002007295399058147) * t323 * t366 + f64x8::splat(0.001491587478361034) * t323 * t360 + f64x8::splat(0.001940164714223896) * t323 * t353 - f64x8::splat(0.01437960658302686) * t484 - f64x8::splat(0.001153807045825489) * t459 * t380;
            let t490 = t356 + t413 + t453 + t488;
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
            let t555 = t80 * t518 * t84;
            let t557 = t40 * t40;
            let t559 = f64x8::splat(1.0) / t43 / t42;
            let t560 = t557 * t559;
            let t561 = v_sigma0 * v_sigma0;
            let t562 = t34 * t34;
            let t563 = t562 * t34;
            let t565 = f64x8::splat(1.0) / t29 / t563;
            let t567 = t83 * t83;
            let t568 = f64x8::splat(1.0) / t567;
            let t570 = t560 * t561 * t565 * t568;
            let t576 = -f64x8::splat(2.0) / f64x8::splat(9.0) * t555 + t570 / f64x8::splat(108.0);
            let t577 = t88 * t576;
            let t598 = f64x8::splat(0.2074861966146727) * t552 + f64x8::splat(0.03590912460336272) * t555 - f64x8::splat(0.0014962135251401134) * t570 + f64x8::splat(0.00538391567059323) * t251 * t552 - f64x8::splat(0.003864918381838851) * t577 * t125 - f64x8::splat(0.003569004914854239) * t577 * t131 - f64x8::splat(0.005591648643030744) * t577 * t136 - f64x8::splat(0.0028924113898523497) * t577 * t141 - f64x8::splat(0.003461421137476467) * t577 * t145 - f64x8::splat(0.003461421137476467) * t237 * t552 - f64x8::splat(0.04313881974908058) * t577 * t73 - f64x8::splat(0.0021270890439633733) * t221 * t552 - f64x8::splat(0.003526843430275269) * t577 * t118 + f64x8::splat(0.000411086590637241) * t203 * t552;
            let t605 = t576 * t145;
            let t607 = t576 * t73;
            let t609 = t88 * t552;
            let t611 = t576 * t136;
            let t613 = t105 * t552;
            let t615 = t73 * t552;
            let t619 = f64x8::splat(35.0) / f64x8::splat(2.0) * t613 - f64x8::splat(15.0) / f64x8::splat(2.0) * t615;
            let t624 = t74 * t552;
            let t628 = f64x8::splat(15.0) / f64x8::splat(2.0) * t624 - f64x8::splat(3.0) / f64x8::splat(2.0) * t552;
            let t631 = t576 * t125;
            let t633 = f64x8::splat(0.020012545797197603) * t186 * t552 - f64x8::splat(0.016494338766497415) * t166 * t552 + f64x8::splat(0.004264173071531283) * t148 * t552 + f64x8::splat(0.00179463855686441) * t605 + f64x8::splat(0.1179363564823021) * t607 + f64x8::splat(0.1179363564823021) * t609 + f64x8::splat(0.002915285520983635) * t611 + f64x8::splat(0.11775890511466262) * t613 - f64x8::splat(0.011765768981988275) * t615 + f64x8::splat(0.002915285520983635) * t88 * t619 + f64x8::splat(0.002125332357775206) * t576 * t141 - f64x8::splat(0.09636448540578502) * t624 + f64x8::splat(0.002125332357775206) * t88 * t628 + f64x8::splat(0.001491587478361034) * t631;
            let t635 = t103 * t552;
            let t640 = f64x8::splat(693.0) / f64x8::splat(8.0) * t635 - f64x8::splat(315.0) / f64x8::splat(4.0) * t613 + f64x8::splat(105.0) / f64x8::splat(8.0) * t615;
            let t643 = t576 * t131;
            let t645 = t75 * t552;
            let t650 = f64x8::splat(315.0) / f64x8::splat(8.0) * t645 - f64x8::splat(105.0) / f64x8::splat(4.0) * t624 + f64x8::splat(15.0) / f64x8::splat(8.0) * t552;
            let t653 = t224 * t552;
            let t655 = t576 * t118;
            let t657 = t76 * t552;
            let t663 = f64x8::splat(3003.0) / f64x8::splat(16.0) * t657 - f64x8::splat(3465.0) / f64x8::splat(16.0) * t645 + f64x8::splat(945.0) / f64x8::splat(16.0) * t624 - f64x8::splat(35.0) / f64x8::splat(16.0) * t552;
            let t674 = t89 * t576;
            let t676 = -f64x8::splat(0.17131022505250731) * t635 + f64x8::splat(0.001491587478361034) * t88 * t640 + f64x8::splat(0.002007295399058147) * t643 + f64x8::splat(0.43767257904820067) * t645 + f64x8::splat(0.002007295399058147) * t88 * t650 - f64x8::splat(0.01437960658302686) * t653 + f64x8::splat(0.001940164714223896) * t655 - f64x8::splat(0.47225184058624103) * t657 + f64x8::splat(0.001940164714223896) * t88 * t663 - f64x8::splat(0.0009641371299507833) * t224 * t628 - f64x8::splat(0.001189668304951413) * t224 * t650 - f64x8::splat(0.001863882881010248) * t224 * t619 - f64x8::splat(0.001288306127279617) * t224 * t640 + f64x8::splat(0.022250640561108575) * t674;
            let t680 = f64x8::splat(15.0) / f64x8::splat(2.0) * t674 + t555 / f64x8::splat(3.0) - t570 / f64x8::splat(72.0);
            let t683 = t680 * t73;
            let t685 = t207 * t552;
            let t705 = t93 * t576;
            let t708 = -f64x8::splat(0.0007090296813211244) * t680 * t145 - f64x8::splat(0.01030571429426108) * t683 - f64x8::splat(0.01030571429426108) * t685 - f64x8::splat(0.001175614476758423) * t224 * t663 - f64x8::splat(0.002494950550547465) * t680 * t136 - f64x8::splat(0.002494950550547465) * t207 * t619 + f64x8::splat(0.003712786171321043) * t680 * t141 + f64x8::splat(0.003712786171321043) * t207 * t628 + f64x8::splat(0.0001672905908063297) * t680 * t131 + f64x8::splat(0.0001672905908063297) * t207 * t650 - f64x8::splat(0.0006058496834176058) * t680 * t125 - f64x8::splat(0.0006058496834176058) * t207 * t640 - f64x8::splat(0.004188213175650998) * t705 - f64x8::splat(0.7420537470243785) * t577;
            let t713 = f64x8::splat(35.0) / f64x8::splat(2.0) * t705 - f64x8::splat(15.0) / f64x8::splat(2.0) * t577;
            let t716 = t713 * t73;
            let t718 = t190 * t552;
            let t742 = f64x8::splat(0.000137028863545747) * t713 * t145 + f64x8::splat(0.01683215086686233) * t716 + f64x8::splat(0.01683215086686233) * t718 + f64x8::splat(0.0004312411759243052) * t680 * t118 + f64x8::splat(0.0004312411759243052) * t207 * t663 + f64x8::splat(0.0004187827907710905) * t713 * t136 + f64x8::splat(0.0004187827907710905) * t190 * t619 + f64x8::splat(0.001282471852770764) * t713 * t141 + f64x8::splat(0.001282471852770764) * t190 * t628 - f64x8::splat(0.0002721968500889238) * t713 * t131 - f64x8::splat(0.0002721968500889238) * t190 * t650 + f64x8::splat(3.212943141118693e-06) * t713 * t118 + f64x8::splat(3.212943141118693e-06) * t190 * t663 + f64x8::splat(0.0002776060240069905) * t713 * t125;
            let t745 = t90 * t576;
            let t751 = f64x8::splat(315.0) / f64x8::splat(8.0) * t745 - f64x8::splat(105.0) / f64x8::splat(4.0) * t674 - f64x8::splat(5.0) / f64x8::splat(12.0) * t555 + f64x8::splat(5.0) / f64x8::splat(288.0) * t570;
            let t754 = t751 * t73;
            let t756 = t171 * t552;
            let t776 = f64x8::splat(0.0002776060240069905) * t190 * t640 - f64x8::splat(0.07943791709378588) * t745 + f64x8::splat(0.006670848599065867) * t751 * t145 - f64x8::splat(0.000257733338272708) * t754 - f64x8::splat(0.000257733338272708) * t756 - f64x8::splat(0.001009981263546227) * t751 * t136 - f64x8::splat(0.001009981263546227) * t171 * t619 + f64x8::splat(0.0002262886186270548) * t751 * t141 + f64x8::splat(0.0002262886186270548) * t171 * t628 - f64x8::splat(0.001622621390953226) * t171 * t640 - f64x8::splat(0.0005869916483960576) * t751 * t131 - f64x8::splat(0.0005869916483960576) * t171 * t650 - f64x8::splat(0.0002202759704065197) * t751 * t118 - f64x8::splat(0.0002202759704065197) * t171 * t663;
            let t780 = t96 * t576;
            let t785 = f64x8::splat(693.0) / f64x8::splat(8.0) * t780 - f64x8::splat(315.0) / f64x8::splat(4.0) * t705 + f64x8::splat(105.0) / f64x8::splat(8.0) * t577;
            let t788 = t785 * t73;
            let t790 = t153 * t552;
            let t808 = t91 * t576;
            let t810 = -f64x8::splat(0.001622621390953226) * t751 * t125 + f64x8::splat(0.1345153379936992) * t780 - f64x8::splat(0.005498112922165805) * t785 * t145 + f64x8::splat(0.002334616776649133) * t788 + f64x8::splat(0.002334616776649133) * t790 + f64x8::splat(0.0004230264400260503) * t785 * t136 + f64x8::splat(0.0004230264400260503) * t153 * t619 - f64x8::splat(0.006510071882485726) * t785 * t141 - f64x8::splat(0.006510071882485726) * t153 * t628 + f64x8::splat(0.001136485825094485) * t785 * t131 + f64x8::splat(0.001136485825094485) * t153 * t650 + f64x8::splat(0.0004260858412001439) * t785 * t125 + f64x8::splat(0.0004260858412001439) * t153 * t640 + f64x8::splat(0.10977695610065165) * t808;
            let t816 = f64x8::splat(3003.0) / f64x8::splat(16.0) * t808 - f64x8::splat(3465.0) / f64x8::splat(16.0) * t745 + f64x8::splat(945.0) / f64x8::splat(16.0) * t674 + f64x8::splat(35.0) / f64x8::splat(72.0) * t555 - f64x8::splat(35.0) / f64x8::splat(1728.0) * t570;
            let t819 = t816 * t73;
            let t821 = t113 * t552;
            let t847 = f64x8::splat(0.001421391023843761) * t816 * t145 + f64x8::splat(0.0003837976998664341) * t819 + f64x8::splat(0.0003837976998664341) * t821 + f64x8::splat(0.0003807158595350892) * t785 * t118 + f64x8::splat(0.0003807158595350892) * t153 * t663 + f64x8::splat(0.00245752591853626) * t113 * t619 + f64x8::splat(0.01243327883803539) * t816 * t141 + f64x8::splat(0.01243327883803539) * t113 * t628 + f64x8::splat(0.001522474179598972) * t816 * t131 + f64x8::splat(0.001522474179598972) * t113 * t650 + f64x8::splat(0.00245752591853626) * t816 * t136 - f64x8::splat(0.0003682519432462936) * t816 * t125 - f64x8::splat(0.0003682519432462936) * t113 * t640 - f64x8::splat(0.0003695503801501715) * t816 * t118 - f64x8::splat(0.0003695503801501715) * t113 * t663;
            let t850 = t598 + t633 + t676 + t708 + t742 + t776 + t810 + t847;
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
            let t896 = t275 * v_rho1;
            let t898 = f64x8::splat(1.0) / t271 / t896;
            let t899 = v_sigma2 * t898;
            let t904 = f64x8::splat(5.0) / f64x8::splat(9.0) * (-f64x8::splat(5.0) / f64x8::splat(3.0) * v_tau1 * t277 + t899 / f64x8::splat(3.0)) * t40 * t45;
            let t905 = ((t285).select(t904, f64x8::splat(0.0)));
            let t908 = t294 * t905;
            let t911 = f64x8::splat(1.0) / t293 / t286;
            let t912 = t911 * t905;
            let t915 = t300 * t307;
            let t916 = ((t285).select(f64x8::splat(0.0), t904));
            let t917 = t297 * t916;
            let t920 = t306 * t306;
            let t921 = f64x8::splat(1.0) / t920;
            let t922 = t301 * t921;
            let t923 = t298 * t304;
            let t926 = t298 * t298;
            let t927 = t926 * t297;
            let t930 = f64x8::splat(3.0) * t923 * t916 + f64x8::splat(12.0) * t927 * t916;
            let t933 = ((t284).select(-f64x8::splat(3.0) / f64x8::splat(2.0) * t291 * t905 - f64x8::splat(3.0) / f64x8::splat(16.0) * t908 + f64x8::splat(3.0) * t912, -f64x8::splat(6.0) * t915 * t917 - t922 * t930));
            let t936 = t80 * t899 * t319;
            let t938 = v_sigma2 * v_sigma2;
            let t939 = t275 * t275;
            let t940 = t939 * t275;
            let t942 = f64x8::splat(1.0) / t270 / t940;
            let t944 = t318 * t318;
            let t945 = f64x8::splat(1.0) / t944;
            let t947 = t560 * t938 * t942 * t945;
            let t949 = t312 * t933;
            let t951 = t311 * t933;
            let t953 = t310 * t933;
            let t956 = f64x8::splat(3003.0) / f64x8::splat(16.0) * t949 - f64x8::splat(3465.0) / f64x8::splat(16.0) * t951 + f64x8::splat(945.0) / f64x8::splat(16.0) * t953 - f64x8::splat(35.0) / f64x8::splat(16.0) * t933;
            let t961 = -f64x8::splat(2.0) / f64x8::splat(9.0) * t936 + t947 / f64x8::splat(108.0);
            let t962 = t326 * t961;
            let t964 = t325 * t961;
            let t966 = t324 * t961;
            let t970 = f64x8::splat(3003.0) / f64x8::splat(16.0) * t962 - f64x8::splat(3465.0) / f64x8::splat(16.0) * t964 + f64x8::splat(945.0) / f64x8::splat(16.0) * t966 + f64x8::splat(35.0) / f64x8::splat(72.0) * t936 - f64x8::splat(35.0) / f64x8::splat(1728.0) * t947;
            let t978 = f64x8::splat(315.0) / f64x8::splat(8.0) * t951 - f64x8::splat(105.0) / f64x8::splat(4.0) * t953 + f64x8::splat(15.0) / f64x8::splat(8.0) * t933;
            let t983 = t336 * t933;
            let t985 = t338 * t933;
            let t987 = t309 * t933;
            let t989 = f64x8::splat(693.0) / f64x8::splat(8.0) * t983 - f64x8::splat(315.0) / f64x8::splat(4.0) * t985 + f64x8::splat(105.0) / f64x8::splat(8.0) * t987;
            let t998 = f64x8::splat(35.0) / f64x8::splat(2.0) * t985 - f64x8::splat(15.0) / f64x8::splat(2.0) * t987;
            let t1005 = f64x8::splat(15.0) / f64x8::splat(2.0) * t953 - f64x8::splat(3.0) / f64x8::splat(2.0) * t933;
            let t1008 = f64x8::splat(0.2074861966146727) * t933 + f64x8::splat(0.03590912460336272) * t936 - f64x8::splat(0.0014962135251401134) * t947 - f64x8::splat(0.0003695503801501715) * t348 * t956 - f64x8::splat(0.0003695503801501715) * t970 * t353 + f64x8::splat(0.001522474179598972) * t970 * t366 + f64x8::splat(0.001522474179598972) * t348 * t978 - f64x8::splat(0.0003682519432462936) * t970 * t360 - f64x8::splat(0.0003682519432462936) * t348 * t989 + f64x8::splat(0.001421391023843761) * t970 * t380 + f64x8::splat(0.00245752591853626) * t970 * t371 + f64x8::splat(0.00245752591853626) * t348 * t998 + f64x8::splat(0.01243327883803539) * t970 * t376 + f64x8::splat(0.01243327883803539) * t348 * t1005;
            let t1012 = t970 * t309;
            let t1014 = t348 * t933;
            let t1016 = t330 * t961;
            let t1018 = t333 * t961;
            let t1020 = t323 * t961;
            let t1022 = f64x8::splat(693.0) / f64x8::splat(8.0) * t1016 - f64x8::splat(315.0) / f64x8::splat(4.0) * t1018 + f64x8::splat(105.0) / f64x8::splat(8.0) * t1020;
            let t1040 = t1022 * t309;
            let t1042 = f64x8::splat(0.0004260858412001439) * t388 * t989 + f64x8::splat(0.10977695610065165) * t962 + f64x8::splat(0.0003837976998664341) * t1012 + f64x8::splat(0.0003837976998664341) * t1014 + f64x8::splat(0.0003807158595350892) * t1022 * t353 + f64x8::splat(0.0003807158595350892) * t388 * t956 - f64x8::splat(0.006510071882485726) * t1022 * t376 + f64x8::splat(0.001136485825094485) * t1022 * t366 + f64x8::splat(0.001136485825094485) * t388 * t978 + f64x8::splat(0.0004260858412001439) * t1022 * t360 - f64x8::splat(0.006510071882485726) * t388 * t1005 + f64x8::splat(0.1345153379936992) * t1016 - f64x8::splat(0.005498112922165805) * t1022 * t380 + f64x8::splat(0.002334616776649133) * t1040;
            let t1052 = f64x8::splat(315.0) / f64x8::splat(8.0) * t964 - f64x8::splat(105.0) / f64x8::splat(4.0) * t966 - f64x8::splat(5.0) / f64x8::splat(12.0) * t936 + f64x8::splat(5.0) / f64x8::splat(288.0) * t947;
            let t1059 = t388 * t933;
            let t1077 = f64x8::splat(35.0) / f64x8::splat(2.0) * t1018 - f64x8::splat(15.0) / f64x8::splat(2.0) * t1020;
            let t1080 = f64x8::splat(0.0004230264400260503) * t1022 * t371 + f64x8::splat(0.0004230264400260503) * t388 * t998 - f64x8::splat(0.0002202759704065197) * t1052 * t353 - f64x8::splat(0.0002202759704065197) * t406 * t956 - f64x8::splat(0.0005869916483960576) * t1052 * t366 + f64x8::splat(0.002334616776649133) * t1059 + f64x8::splat(0.0002262886186270548) * t406 * t1005 + f64x8::splat(0.006670848599065867) * t1052 * t380 - f64x8::splat(0.0005869916483960576) * t406 * t978 - f64x8::splat(0.001009981263546227) * t1052 * t371 - f64x8::splat(0.001009981263546227) * t406 * t998 - f64x8::splat(0.001622621390953226) * t1052 * t360 - f64x8::splat(0.001622621390953226) * t406 * t989 + f64x8::splat(0.0002776060240069905) * t1077 * t360;
            let t1084 = t1052 * t309;
            let t1086 = t406 * t933;
            let t1106 = f64x8::splat(0.0002776060240069905) * t425 * t989 - f64x8::splat(0.07943791709378588) * t964 - f64x8::splat(0.000257733338272708) * t1084 - f64x8::splat(0.000257733338272708) * t1086 + f64x8::splat(3.212943141118693e-06) * t1077 * t353 + f64x8::splat(3.212943141118693e-06) * t425 * t956 + f64x8::splat(0.0002262886186270548) * t1052 * t376 + f64x8::splat(0.001282471852770764) * t425 * t1005 - f64x8::splat(0.0002721968500889238) * t1077 * t366 - f64x8::splat(0.0002721968500889238) * t425 * t978 + f64x8::splat(0.0004187827907710905) * t1077 * t371 + f64x8::splat(0.0004187827907710905) * t425 * t998 - f64x8::splat(0.004188213175650998) * t1018 - f64x8::splat(0.7420537470243785) * t1020;
            let t1111 = t1077 * t309;
            let t1113 = t425 * t933;
            let t1118 = f64x8::splat(15.0) / f64x8::splat(2.0) * t966 + t936 / f64x8::splat(3.0) - t947 / f64x8::splat(72.0);
            let t1141 = f64x8::splat(0.000137028863545747) * t1077 * t380 + f64x8::splat(0.01683215086686233) * t1111 + f64x8::splat(0.01683215086686233) * t1113 + f64x8::splat(0.0004312411759243052) * t1118 * t353 + f64x8::splat(0.001282471852770764) * t1077 * t376 - f64x8::splat(0.0007090296813211244) * t1118 * t380 - f64x8::splat(0.002494950550547465) * t1118 * t371 - f64x8::splat(0.002494950550547465) * t442 * t998 + f64x8::splat(0.0001672905908063297) * t1118 * t366 + f64x8::splat(0.0001672905908063297) * t442 * t978 + f64x8::splat(0.0004312411759243052) * t442 * t956 - f64x8::splat(0.0006058496834176058) * t1118 * t360 - f64x8::splat(0.0006058496834176058) * t442 * t989 - f64x8::splat(0.0009641371299507833) * t459 * t1005;
            let t1151 = t1118 * t309;
            let t1153 = t442 * t933;
            let t1159 = t459 * t933;
            let t1166 = -f64x8::splat(0.001189668304951413) * t459 * t978 - f64x8::splat(0.001863882881010248) * t459 * t998 - f64x8::splat(0.001288306127279617) * t459 * t989 - f64x8::splat(0.001175614476758423) * t459 * t956 + f64x8::splat(0.022250640561108575) * t966 - f64x8::splat(0.01030571429426108) * t1151 - f64x8::splat(0.01030571429426108) * t1153 + f64x8::splat(0.003712786171321043) * t1118 * t376 + f64x8::splat(0.003712786171321043) * t442 * t1005 - f64x8::splat(0.01437960658302686) * t1159 - f64x8::splat(0.47225184058624103) * t949 + f64x8::splat(0.43767257904820067) * t951 - f64x8::splat(0.09636448540578502) * t953 + f64x8::splat(0.001940164714223896) * t323 * t956;
            let t1168 = t961 * t360;
            let t1175 = t961 * t353;
            let t1177 = t961 * t366;
            let t1181 = t961 * t380;
            let t1183 = t961 * t376;
            let t1187 = t961 * t371;
            let t1191 = t961 * t309;
            let t1193 = f64x8::splat(0.001491587478361034) * t1168 - f64x8::splat(0.17131022505250731) * t983 + f64x8::splat(0.11775890511466262) * t985 - f64x8::splat(0.011765768981988275) * t987 + f64x8::splat(0.001491587478361034) * t323 * t989 + f64x8::splat(0.001940164714223896) * t1175 + f64x8::splat(0.002007295399058147) * t1177 + f64x8::splat(0.002007295399058147) * t323 * t978 + f64x8::splat(0.00179463855686441) * t1181 + f64x8::splat(0.002125332357775206) * t1183 + f64x8::splat(0.002125332357775206) * t323 * t1005 + f64x8::splat(0.002915285520983635) * t1187 + f64x8::splat(0.002915285520983635) * t323 * t998 + f64x8::splat(0.1179363564823021) * t1191;
            let t1194 = t323 * t933;
            let t1224 = f64x8::splat(0.1179363564823021) * t1194 + f64x8::splat(0.000411086590637241) * t438 * t933 - f64x8::splat(0.016494338766497415) * t401 * t933 + f64x8::splat(0.020012545797197603) * t421 * t933 + f64x8::splat(0.004264173071531283) * t383 * t933 - f64x8::splat(0.04313881974908058) * t1020 * t309 - f64x8::splat(0.003461421137476467) * t1020 * t380 - f64x8::splat(0.003461421137476467) * t484 * t933 - f64x8::splat(0.003569004914854239) * t1020 * t366 - f64x8::splat(0.005591648643030744) * t1020 * t371 - f64x8::splat(0.0028924113898523497) * t1020 * t376 + f64x8::splat(0.00538391567059323) * t470 * t933 - f64x8::splat(0.0021270890439633733) * t456 * t933 - f64x8::splat(0.003526843430275269) * t1020 * t353 - f64x8::splat(0.003864918381838851) * t1020 * t360;
            let t1227 = t1008 + t1042 + t1080 + t1106 + t1141 + t1166 + t1193 + t1224;
            let t1232 = ((t260).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t890 * t490 - t869 - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t269 * t1227));
            let tvrho1 = t259 + t494 + t7 * (t884 + t1232);
            acc_vrho_1 = tvrho1;
            let t1235 = t80 * t36;
            let t1236 = f64x8::splat(5.0) / f64x8::splat(72.0) * t1235;
            let t1237 = ((t49).select(-t1236, f64x8::splat(0.0)));
            let t1240 = t58 * t1237;
            let t1242 = t530 * t1237;
            let t1245 = ((t49).select(f64x8::splat(0.0), -t1236));
            let t1246 = t61 * t1245;
            let t1253 = f64x8::splat(3.0) * t542 * t1245 + f64x8::splat(12.0) * t546 * t1245;
            let t1256 = ((t48).select(-f64x8::splat(3.0) / f64x8::splat(2.0) * t55 * t1237 - f64x8::splat(3.0) / f64x8::splat(16.0) * t1240 + f64x8::splat(3.0) * t1242, -f64x8::splat(6.0) * t534 * t1246 - t541 * t1253));
            let t1259 = t80 * t36 * t84;
            let t1261 = t562 * v_rho0;
            let t1263 = f64x8::splat(1.0) / t29 / t1261;
            let t1266 = t560 * v_sigma0 * t1263 * t568;
            let t1270 = t1259 / f64x8::splat(12.0) - t1266 / f64x8::splat(288.0);
            let t1271 = t1270 * t136;
            let t1273 = t105 * t1256;
            let t1275 = t73 * t1256;
            let t1277 = f64x8::splat(35.0) / f64x8::splat(2.0) * t1273 - f64x8::splat(15.0) / f64x8::splat(2.0) * t1275;
            let t1280 = t1270 * t125;
            let t1282 = t103 * t1256;
            let t1286 = f64x8::splat(693.0) / f64x8::splat(8.0) * t1282 - f64x8::splat(315.0) / f64x8::splat(4.0) * t1273 + f64x8::splat(105.0) / f64x8::splat(8.0) * t1275;
            let t1289 = t224 * t1256;
            let t1291 = t1270 * t118;
            let t1293 = t76 * t1256;
            let t1295 = t75 * t1256;
            let t1297 = t74 * t1256;
            let t1300 = f64x8::splat(3003.0) / f64x8::splat(16.0) * t1293 - f64x8::splat(3465.0) / f64x8::splat(16.0) * t1295 + f64x8::splat(945.0) / f64x8::splat(16.0) * t1297 - f64x8::splat(35.0) / f64x8::splat(16.0) * t1256;
            let t1307 = f64x8::splat(15.0) / f64x8::splat(2.0) * t1297 - f64x8::splat(3.0) / f64x8::splat(2.0) * t1256;
            let t1310 = t89 * t1270;
            let t1314 = f64x8::splat(15.0) / f64x8::splat(2.0) * t1310 - t1259 / f64x8::splat(8.0) + t1266 / f64x8::splat(192.0);
            let t1319 = f64x8::splat(0.2074861966146727) * t1256 - f64x8::splat(0.01346592172626102) * t1259 + f64x8::splat(0.0005610800719275425) * t1266 + f64x8::splat(0.002915285520983635) * t1271 + f64x8::splat(0.002915285520983635) * t88 * t1277 + f64x8::splat(0.001491587478361034) * t1280 + f64x8::splat(0.001491587478361034) * t88 * t1286 - f64x8::splat(0.01437960658302686) * t1289 + f64x8::splat(0.001940164714223896) * t1291 + f64x8::splat(0.001940164714223896) * t88 * t1300 - f64x8::splat(0.001863882881010248) * t224 * t1277 - f64x8::splat(0.0009641371299507833) * t224 * t1307 - f64x8::splat(0.002494950550547465) * t1314 * t136 - f64x8::splat(0.002494950550547465) * t207 * t1277;
            let t1324 = t88 * t1256;
            let t1326 = t1270 * t141;
            let t1330 = t1270 * t145;
            let t1332 = t1270 * t73;
            let t1334 = t1270 * t131;
            let t1339 = f64x8::splat(315.0) / f64x8::splat(8.0) * t1295 - f64x8::splat(105.0) / f64x8::splat(4.0) * t1297 + f64x8::splat(15.0) / f64x8::splat(8.0) * t1256;
            let t1350 = t1314 * t73;
            let t1352 = f64x8::splat(0.003712786171321043) * t1314 * t141 + f64x8::splat(0.003712786171321043) * t207 * t1307 + f64x8::splat(0.1179363564823021) * t1324 + f64x8::splat(0.002125332357775206) * t1326 + f64x8::splat(0.002125332357775206) * t88 * t1307 + f64x8::splat(0.00179463855686441) * t1330 + f64x8::splat(0.1179363564823021) * t1332 + f64x8::splat(0.002007295399058147) * t1334 + f64x8::splat(0.002007295399058147) * t88 * t1339 - f64x8::splat(0.001189668304951413) * t224 * t1339 - f64x8::splat(0.001175614476758423) * t224 * t1300 - f64x8::splat(0.001288306127279617) * t224 * t1286 - f64x8::splat(0.0007090296813211244) * t1314 * t145 - f64x8::splat(0.01030571429426108) * t1350;
            let t1354 = t207 * t1256;
            let t1356 = t96 * t1270;
            let t1358 = t93 * t1270;
            let t1360 = t88 * t1270;
            let t1362 = f64x8::splat(693.0) / f64x8::splat(8.0) * t1356 - f64x8::splat(315.0) / f64x8::splat(4.0) * t1358 + f64x8::splat(105.0) / f64x8::splat(8.0) * t1360;
            let t1367 = t90 * t1270;
            let t1372 = f64x8::splat(315.0) / f64x8::splat(8.0) * t1367 - f64x8::splat(105.0) / f64x8::splat(4.0) * t1310 + f64x8::splat(5.0) / f64x8::splat(32.0) * t1259 - f64x8::splat(5.0) / f64x8::splat(768.0) * t1266;
            let t1391 = t1362 * t73;
            let t1393 = t153 * t1256;
            let t1395 = -f64x8::splat(0.01030571429426108) * t1354 + f64x8::splat(0.0004230264400260503) * t1362 * t136 + f64x8::splat(0.0004230264400260503) * t153 * t1277 - f64x8::splat(0.0002202759704065197) * t1372 * t118 - f64x8::splat(0.0002202759704065197) * t171 * t1300 - f64x8::splat(0.006510071882485726) * t1362 * t141 - f64x8::splat(0.006510071882485726) * t153 * t1307 - f64x8::splat(0.005498112922165805) * t1362 * t145 - f64x8::splat(0.001009981263546227) * t1372 * t136 - f64x8::splat(0.001009981263546227) * t171 * t1277 - f64x8::splat(0.001622621390953226) * t1372 * t125 - f64x8::splat(0.001622621390953226) * t171 * t1286 + f64x8::splat(0.002334616776649133) * t1391 + f64x8::splat(0.002334616776649133) * t1393;
            let t1408 = f64x8::splat(35.0) / f64x8::splat(2.0) * t1358 - f64x8::splat(15.0) / f64x8::splat(2.0) * t1360;
            let t1411 = t1372 * t73;
            let t1413 = t171 * t1256;
            let t1419 = t1408 * t73;
            let t1427 = f64x8::splat(0.0002262886186270548) * t1372 * t141 + f64x8::splat(0.0002262886186270548) * t171 * t1307 + f64x8::splat(0.006670848599065867) * t1372 * t145 - f64x8::splat(0.0005869916483960576) * t1372 * t131 - f64x8::splat(0.0005869916483960576) * t171 * t1339 - f64x8::splat(0.0002721968500889238) * t1408 * t131 - f64x8::splat(0.000257733338272708) * t1411 - f64x8::splat(0.000257733338272708) * t1413 + f64x8::splat(3.212943141118693e-06) * t1408 * t118 + f64x8::splat(3.212943141118693e-06) * t190 * t1300 + f64x8::splat(0.01683215086686233) * t1419 - f64x8::splat(0.0002721968500889238) * t190 * t1339 + f64x8::splat(0.0004187827907710905) * t1408 * t136 + f64x8::splat(0.0004187827907710905) * t190 * t1277;
            let t1436 = t190 * t1256;
            let t1448 = t91 * t1270;
            let t1454 = f64x8::splat(3003.0) / f64x8::splat(16.0) * t1448 - f64x8::splat(3465.0) / f64x8::splat(16.0) * t1367 + f64x8::splat(945.0) / f64x8::splat(16.0) * t1310 - f64x8::splat(35.0) / f64x8::splat(192.0) * t1259 + f64x8::splat(35.0) / f64x8::splat(4608.0) * t1266;
            let t1465 = f64x8::splat(0.0002776060240069905) * t1408 * t125 + f64x8::splat(0.0002776060240069905) * t190 * t1286 + f64x8::splat(0.0001672905908063297) * t207 * t1339 + f64x8::splat(0.01683215086686233) * t1436 + f64x8::splat(0.0004312411759243052) * t1314 * t118 + f64x8::splat(0.0004312411759243052) * t207 * t1300 + f64x8::splat(0.001282471852770764) * t1408 * t141 + f64x8::splat(0.001282471852770764) * t190 * t1307 + f64x8::splat(0.000137028863545747) * t1408 * t145 - f64x8::splat(0.0003695503801501715) * t1454 * t118 - f64x8::splat(0.0006058496834176058) * t1314 * t125 - f64x8::splat(0.0006058496834176058) * t207 * t1286 + f64x8::splat(0.0001672905908063297) * t1314 * t131 - f64x8::splat(0.0003682519432462936) * t1454 * t125;
            let t1472 = t1454 * t73;
            let t1474 = t113 * t1256;
            let t1492 = f64x8::splat(0.001522474179598972) * t1454 * t131 + f64x8::splat(0.001522474179598972) * t113 * t1339 - f64x8::splat(0.0003695503801501715) * t113 * t1300 + f64x8::splat(0.0003837976998664341) * t1472 + f64x8::splat(0.0003837976998664341) * t1474 + f64x8::splat(0.00245752591853626) * t1454 * t136 + f64x8::splat(0.00245752591853626) * t113 * t1277 + f64x8::splat(0.01243327883803539) * t1454 * t141 - f64x8::splat(0.0003682519432462936) * t113 * t1286 + f64x8::splat(0.0003807158595350892) * t1362 * t118 - f64x8::splat(0.47225184058624103) * t1293 + f64x8::splat(0.0003807158595350892) * t153 * t1300 + f64x8::splat(0.01243327883803539) * t113 * t1307 + f64x8::splat(0.10977695610065165) * t1448;
            let t1513 = -f64x8::splat(0.07943791709378588) * t1367 + f64x8::splat(0.022250640561108575) * t1310 + f64x8::splat(0.001421391023843761) * t1454 * t145 + f64x8::splat(0.0004260858412001439) * t1362 * t125 - f64x8::splat(0.17131022505250731) * t1282 + f64x8::splat(0.11775890511466262) * t1273 - f64x8::splat(0.011765768981988275) * t1275 + f64x8::splat(0.0004260858412001439) * t153 * t1286 + f64x8::splat(0.001136485825094485) * t1362 * t131 + f64x8::splat(0.43767257904820067) * t1295 - f64x8::splat(0.09636448540578502) * t1297 + f64x8::splat(0.001136485825094485) * t153 * t1339 + f64x8::splat(0.1345153379936992) * t1356 - f64x8::splat(0.004188213175650998) * t1358;
            let t1543 = -f64x8::splat(0.7420537470243785) * t1360 + f64x8::splat(0.00538391567059323) * t251 * t1256 - f64x8::splat(0.003864918381838851) * t1360 * t125 - f64x8::splat(0.003569004914854239) * t1360 * t131 - f64x8::splat(0.005591648643030744) * t1360 * t136 - f64x8::splat(0.0028924113898523497) * t1360 * t141 - f64x8::splat(0.003461421137476467) * t1360 * t145 - f64x8::splat(0.003461421137476467) * t237 * t1256 - f64x8::splat(0.04313881974908058) * t1360 * t73 - f64x8::splat(0.0021270890439633733) * t221 * t1256 - f64x8::splat(0.003526843430275269) * t1360 * t118 + f64x8::splat(0.000411086590637241) * t203 * t1256 + f64x8::splat(0.020012545797197603) * t186 * t1256 - f64x8::splat(0.016494338766497415) * t166 * t1256 + f64x8::splat(0.004264173071531283) * t148 * t1256;
            let t1546 = t1319 + t1352 + t1395 + t1427 + t1465 + t1492 + t1513 + t1543;
            let t1550 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t28 * t1546));
            let tvsigma0 = t7 * t1550;
            acc_vsigma_0 = tvsigma0;
            let tvsigma1 = f64x8::splat(0.0);
            acc_vsigma_1 = tvsigma1;
            let t1551 = t939 * v_rho1;
            let t1553 = f64x8::splat(1.0) / t270 / t1551;
            let t1556 = t560 * v_sigma2 * t1553 * t945;
            let t1559 = t80 * t277 * t319;
            let t1561 = t80 * t277;
            let t1562 = f64x8::splat(5.0) / f64x8::splat(72.0) * t1561;
            let t1563 = ((t285).select(-t1562, f64x8::splat(0.0)));
            let t1566 = t294 * t1563;
            let t1568 = t911 * t1563;
            let t1571 = ((t285).select(f64x8::splat(0.0), -t1562));
            let t1572 = t297 * t1571;
            let t1579 = f64x8::splat(3.0) * t923 * t1571 + f64x8::splat(12.0) * t927 * t1571;
            let t1582 = ((t284).select(-f64x8::splat(3.0) / f64x8::splat(2.0) * t291 * t1563 - f64x8::splat(3.0) / f64x8::splat(16.0) * t1566 + f64x8::splat(3.0) * t1568, -f64x8::splat(6.0) * t915 * t1572 - t922 * t1579));
            let t1584 = t310 * t1582;
            let t1587 = f64x8::splat(15.0) / f64x8::splat(2.0) * t1584 - f64x8::splat(3.0) / f64x8::splat(2.0) * t1582;
            let t1592 = t1559 / f64x8::splat(12.0) - t1556 / f64x8::splat(288.0);
            let t1593 = t1592 * t371;
            let t1595 = t338 * t1582;
            let t1597 = t309 * t1582;
            let t1599 = f64x8::splat(35.0) / f64x8::splat(2.0) * t1595 - f64x8::splat(15.0) / f64x8::splat(2.0) * t1597;
            let t1602 = t1592 * t309;
            let t1604 = t323 * t1582;
            let t1606 = t1592 * t380;
            let t1612 = t311 * t1582;
            let t1616 = f64x8::splat(315.0) / f64x8::splat(8.0) * t1612 - f64x8::splat(105.0) / f64x8::splat(4.0) * t1584 + f64x8::splat(15.0) / f64x8::splat(8.0) * t1582;
            let t1619 = t312 * t1582;
            let t1624 = f64x8::splat(3003.0) / f64x8::splat(16.0) * t1619 - f64x8::splat(3465.0) / f64x8::splat(16.0) * t1612 + f64x8::splat(945.0) / f64x8::splat(16.0) * t1584 - f64x8::splat(35.0) / f64x8::splat(16.0) * t1582;
            let t1627 = t336 * t1582;
            let t1631 = f64x8::splat(693.0) / f64x8::splat(8.0) * t1627 - f64x8::splat(315.0) / f64x8::splat(4.0) * t1595 + f64x8::splat(105.0) / f64x8::splat(8.0) * t1597;
            let t1634 = f64x8::splat(0.0005610800719275425) * t1556 - f64x8::splat(0.01346592172626102) * t1559 + f64x8::splat(0.2074861966146727) * t1582 + f64x8::splat(0.002125332357775206) * t323 * t1587 + f64x8::splat(0.002915285520983635) * t1593 + f64x8::splat(0.002915285520983635) * t323 * t1599 + f64x8::splat(0.1179363564823021) * t1602 + f64x8::splat(0.1179363564823021) * t1604 + f64x8::splat(0.00179463855686441) * t1606 - f64x8::splat(0.001863882881010248) * t459 * t1599 - f64x8::splat(0.0009641371299507833) * t459 * t1587 - f64x8::splat(0.001189668304951413) * t459 * t1616 - f64x8::splat(0.001175614476758423) * t459 * t1624 - f64x8::splat(0.001288306127279617) * t459 * t1631;
            let t1635 = t333 * t1592;
            let t1637 = t323 * t1592;
            let t1639 = f64x8::splat(35.0) / f64x8::splat(2.0) * t1635 - f64x8::splat(15.0) / f64x8::splat(2.0) * t1637;
            let t1644 = t459 * t1582;
            let t1646 = t1592 * t353;
            let t1650 = t1592 * t366;
            let t1654 = t1592 * t360;
            let t1658 = t1592 * t376;
            let t1660 = t325 * t1592;
            let t1662 = t324 * t1592;
            let t1666 = f64x8::splat(315.0) / f64x8::splat(8.0) * t1660 - f64x8::splat(105.0) / f64x8::splat(4.0) * t1662 + f64x8::splat(5.0) / f64x8::splat(32.0) * t1559 - f64x8::splat(5.0) / f64x8::splat(768.0) * t1556;
            let t1667 = t1666 * t309;
            let t1669 = t406 * t1582;
            let t1675 = -f64x8::splat(0.0002721968500889238) * t1639 * t366 - f64x8::splat(0.0002721968500889238) * t425 * t1616 - f64x8::splat(0.01437960658302686) * t1644 + f64x8::splat(0.001940164714223896) * t1646 + f64x8::splat(0.001940164714223896) * t323 * t1624 + f64x8::splat(0.002007295399058147) * t1650 + f64x8::splat(0.002007295399058147) * t323 * t1616 + f64x8::splat(0.001491587478361034) * t1654 + f64x8::splat(0.001491587478361034) * t323 * t1631 + f64x8::splat(0.002125332357775206) * t1658 - f64x8::splat(0.000257733338272708) * t1667 - f64x8::splat(0.000257733338272708) * t1669 + f64x8::splat(3.212943141118693e-06) * t1639 * t353 + f64x8::splat(3.212943141118693e-06) * t425 * t1624;
            let t1701 = t330 * t1592;
            let t1705 = f64x8::splat(693.0) / f64x8::splat(8.0) * t1701 - f64x8::splat(315.0) / f64x8::splat(4.0) * t1635 + f64x8::splat(105.0) / f64x8::splat(8.0) * t1637;
            let t1706 = t1705 * t309;
            let t1708 = t388 * t1582;
            let t1710 = f64x8::splat(0.0002776060240069905) * t1639 * t360 + f64x8::splat(0.0002776060240069905) * t425 * t1631 - f64x8::splat(0.001009981263546227) * t406 * t1599 + f64x8::splat(0.0002262886186270548) * t1666 * t376 + f64x8::splat(0.0002262886186270548) * t406 * t1587 + f64x8::splat(0.006670848599065867) * t1666 * t380 - f64x8::splat(0.0002202759704065197) * t406 * t1624 - f64x8::splat(0.001622621390953226) * t1666 * t360 - f64x8::splat(0.001622621390953226) * t406 * t1631 - f64x8::splat(0.0005869916483960576) * t1666 * t366 - f64x8::splat(0.0005869916483960576) * t406 * t1616 - f64x8::splat(0.001009981263546227) * t1666 * t371 + f64x8::splat(0.002334616776649133) * t1706 + f64x8::splat(0.002334616776649133) * t1708;
            let t1733 = t326 * t1592;
            let t1739 = f64x8::splat(3003.0) / f64x8::splat(16.0) * t1733 - f64x8::splat(3465.0) / f64x8::splat(16.0) * t1660 + f64x8::splat(945.0) / f64x8::splat(16.0) * t1662 - f64x8::splat(35.0) / f64x8::splat(192.0) * t1559 + f64x8::splat(35.0) / f64x8::splat(4608.0) * t1556;
            let t1746 = -f64x8::splat(0.0002202759704065197) * t1666 * t353 + f64x8::splat(0.0004230264400260503) * t1705 * t371 + f64x8::splat(0.0004230264400260503) * t388 * t1599 - f64x8::splat(0.006510071882485726) * t1705 * t376 - f64x8::splat(0.006510071882485726) * t388 * t1587 - f64x8::splat(0.005498112922165805) * t1705 * t380 + f64x8::splat(0.0003807158595350892) * t388 * t1624 + f64x8::splat(0.0004260858412001439) * t1705 * t360 + f64x8::splat(0.0004260858412001439) * t388 * t1631 + f64x8::splat(0.001136485825094485) * t1705 * t366 + f64x8::splat(0.001136485825094485) * t388 * t1616 + f64x8::splat(0.01243327883803539) * t1739 * t376 + f64x8::splat(0.01243327883803539) * t348 * t1587 + f64x8::splat(0.001421391023843761) * t1739 * t380;
            let t1749 = t1739 * t309;
            let t1751 = t348 * t1582;
            let t1763 = t1639 * t309;
            let t1765 = t425 * t1582;
            let t1770 = f64x8::splat(15.0) / f64x8::splat(2.0) * t1662 - t1559 / f64x8::splat(8.0) + t1556 / f64x8::splat(192.0);
            let t1781 = f64x8::splat(0.0003837976998664341) * t1749 + f64x8::splat(0.0003837976998664341) * t1751 + f64x8::splat(0.0003807158595350892) * t1705 * t353 + f64x8::splat(0.001522474179598972) * t1739 * t366 + f64x8::splat(0.001522474179598972) * t348 * t1616 + f64x8::splat(0.00245752591853626) * t1739 * t371 + f64x8::splat(0.00245752591853626) * t348 * t1599 + f64x8::splat(0.01683215086686233) * t1763 + f64x8::splat(0.01683215086686233) * t1765 + f64x8::splat(0.0004312411759243052) * t1770 * t353 + f64x8::splat(0.0004312411759243052) * t442 * t1624 + f64x8::splat(0.0004187827907710905) * t1639 * t371 + f64x8::splat(0.0004187827907710905) * t425 * t1599 + f64x8::splat(0.001282471852770764) * t1639 * t376;
            let t1806 = f64x8::splat(0.001282471852770764) * t425 * t1587 - f64x8::splat(0.0007090296813211244) * t1770 * t380 + f64x8::splat(0.0001672905908063297) * t1770 * t366 + f64x8::splat(0.0001672905908063297) * t442 * t1616 - f64x8::splat(0.002494950550547465) * t1770 * t371 - f64x8::splat(0.002494950550547465) * t442 * t1599 - f64x8::splat(0.0006058496834176058) * t1770 * t360 - f64x8::splat(0.0006058496834176058) * t442 * t1631 - f64x8::splat(0.7420537470243785) * t1637 + f64x8::splat(0.000137028863545747) * t1639 * t380 - f64x8::splat(0.0003682519432462936) * t1739 * t360 - f64x8::splat(0.17131022505250731) * t1627 + f64x8::splat(0.11775890511466262) * t1595 - f64x8::splat(0.011765768981988275) * t1597;
            let t1810 = t1770 * t309;
            let t1812 = t442 * t1582;
            let t1828 = -f64x8::splat(0.0003682519432462936) * t348 * t1631 - f64x8::splat(0.01030571429426108) * t1810 - f64x8::splat(0.01030571429426108) * t1812 + f64x8::splat(0.003712786171321043) * t1770 * t376 + f64x8::splat(0.003712786171321043) * t442 * t1587 + f64x8::splat(0.10977695610065165) * t1733 + f64x8::splat(0.1345153379936992) * t1701 - f64x8::splat(0.07943791709378588) * t1660 - f64x8::splat(0.004188213175650998) * t1635 + f64x8::splat(0.022250640561108575) * t1662 - f64x8::splat(0.0003695503801501715) * t1739 * t353 - f64x8::splat(0.47225184058624103) * t1619 + f64x8::splat(0.43767257904820067) * t1612 - f64x8::splat(0.09636448540578502) * t1584;
            let t1859 = -f64x8::splat(0.0003695503801501715) * t348 * t1624 - f64x8::splat(0.016494338766497415) * t401 * t1582 + f64x8::splat(0.004264173071531283) * t383 * t1582 + f64x8::splat(0.020012545797197603) * t421 * t1582 + f64x8::splat(0.000411086590637241) * t438 * t1582 - f64x8::splat(0.0021270890439633733) * t456 * t1582 - f64x8::splat(0.003526843430275269) * t1637 * t353 - f64x8::splat(0.003864918381838851) * t1637 * t360 - f64x8::splat(0.003569004914854239) * t1637 * t366 - f64x8::splat(0.005591648643030744) * t1637 * t371 - f64x8::splat(0.0028924113898523497) * t1637 * t376 + f64x8::splat(0.00538391567059323) * t470 * t1582 - f64x8::splat(0.04313881974908058) * t1637 * t309 - f64x8::splat(0.003461421137476467) * t1637 * t380 - f64x8::splat(0.003461421137476467) * t484 * t1582;
            let t1862 = t1634 + t1675 + t1710 + t1746 + t1781 + t1806 + t1828 + t1859;
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
            let t1891 = t76 * t1889;
            let t1893 = t75 * t1889;
            let t1895 = t74 * t1889;
            let t1898 = f64x8::splat(3003.0) / f64x8::splat(16.0) * t1891 - f64x8::splat(3465.0) / f64x8::splat(16.0) * t1893 + f64x8::splat(945.0) / f64x8::splat(16.0) * t1895 - f64x8::splat(35.0) / f64x8::splat(16.0) * t1889;
            let t1901 = t113 * t1889;
            let t1905 = t105 * t1889;
            let t1907 = t73 * t1889;
            let t1909 = f64x8::splat(35.0) / f64x8::splat(2.0) * t1905 - f64x8::splat(15.0) / f64x8::splat(2.0) * t1907;
            let t1914 = f64x8::splat(15.0) / f64x8::splat(2.0) * t1895 - f64x8::splat(3.0) / f64x8::splat(2.0) * t1889;
            let t1917 = t103 * t1889;
            let t1921 = f64x8::splat(693.0) / f64x8::splat(8.0) * t1917 - f64x8::splat(315.0) / f64x8::splat(4.0) * t1905 + f64x8::splat(105.0) / f64x8::splat(8.0) * t1907;
            let t1927 = f64x8::splat(315.0) / f64x8::splat(8.0) * t1893 - f64x8::splat(105.0) / f64x8::splat(4.0) * t1895 + f64x8::splat(15.0) / f64x8::splat(8.0) * t1889;
            let t1938 = t153 * t1889;
            let t1942 = f64x8::splat(0.2074861966146727) * t1889 - f64x8::splat(0.0003695503801501715) * t113 * t1898 + f64x8::splat(0.0003837976998664341) * t1901 + f64x8::splat(0.0003807158595350892) * t153 * t1898 + f64x8::splat(0.00245752591853626) * t113 * t1909 + f64x8::splat(0.01243327883803539) * t113 * t1914 - f64x8::splat(0.0003682519432462936) * t113 * t1921 + f64x8::splat(0.001522474179598972) * t113 * t1927 - f64x8::splat(0.0005869916483960576) * t171 * t1927 - f64x8::splat(0.0002202759704065197) * t171 * t1898 - f64x8::splat(0.001622621390953226) * t171 * t1921 - f64x8::splat(0.006510071882485726) * t153 * t1914 + f64x8::splat(0.002334616776649133) * t1938 + f64x8::splat(0.001136485825094485) * t153 * t1927;
            let t1951 = t171 * t1889;
            let t1963 = t207 * t1889;
            let t1971 = f64x8::splat(0.0004230264400260503) * t153 * t1909 + f64x8::splat(0.0004260858412001439) * t153 * t1921 + f64x8::splat(0.0002776060240069905) * t190 * t1921 - f64x8::splat(0.0002721968500889238) * t190 * t1927 - f64x8::splat(0.000257733338272708) * t1951 + f64x8::splat(3.212943141118693e-06) * t190 * t1898 - f64x8::splat(0.001009981263546227) * t171 * t1909 + f64x8::splat(0.0002262886186270548) * t171 * t1914 - f64x8::splat(0.001288306127279617) * t224 * t1921 + f64x8::splat(0.003712786171321043) * t207 * t1914 - f64x8::splat(0.01030571429426108) * t1963 + f64x8::splat(0.0001672905908063297) * t207 * t1927 - f64x8::splat(0.002494950550547465) * t207 * t1909 - f64x8::splat(0.0006058496834176058) * t207 * t1921;
            let t1973 = t190 * t1889;
            let t1981 = t224 * t1889;
            let t1994 = t88 * t1889;
            let t1998 = f64x8::splat(0.01683215086686233) * t1973 + f64x8::splat(0.0004312411759243052) * t207 * t1898 + f64x8::splat(0.0004187827907710905) * t190 * t1909 + f64x8::splat(0.001282471852770764) * t190 * t1914 - f64x8::splat(0.01437960658302686) * t1981 - f64x8::splat(0.47225184058624103) * t1891 + f64x8::splat(0.001940164714223896) * t88 * t1898 - f64x8::splat(0.001863882881010248) * t224 * t1909 - f64x8::splat(0.0009641371299507833) * t224 * t1914 - f64x8::splat(0.001189668304951413) * t224 * t1927 - f64x8::splat(0.001175614476758423) * t224 * t1898 + f64x8::splat(0.1179363564823021) * t1994 + f64x8::splat(0.11775890511466262) * t1905 - f64x8::splat(0.011765768981988275) * t1907;
            let t2024 = f64x8::splat(0.002915285520983635) * t88 * t1909 - f64x8::splat(0.09636448540578502) * t1895 + f64x8::splat(0.002125332357775206) * t88 * t1914 - f64x8::splat(0.17131022505250731) * t1917 + f64x8::splat(0.001491587478361034) * t88 * t1921 + f64x8::splat(0.43767257904820067) * t1893 + f64x8::splat(0.002007295399058147) * t88 * t1927 + f64x8::splat(0.000411086590637241) * t203 * t1889 - f64x8::splat(0.0021270890439633733) * t221 * t1889 + f64x8::splat(0.020012545797197603) * t186 * t1889 - f64x8::splat(0.016494338766497415) * t166 * t1889 + f64x8::splat(0.004264173071531283) * t148 * t1889 + f64x8::splat(0.00538391567059323) * t251 * t1889 - f64x8::splat(0.003461421137476467) * t237 * t1889;
            let t2026 = t1942 + t1971 + t1998 + t2024;
            let t2030 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t28 * t2026));
            let tvtau0 = t7 * t2030;
            acc_vtau_0 = tvtau0;
            let t2033 = f64x8::splat(5.0) / f64x8::splat(9.0) * t273 * t40 * t45;
            let t2034 = ((t285).select(t2033, f64x8::splat(0.0)));
            let t2037 = t294 * t2034;
            let t2039 = t911 * t2034;
            let t2042 = ((t285).select(f64x8::splat(0.0), t2033));
            let t2043 = t297 * t2042;
            let t2050 = f64x8::splat(3.0) * t923 * t2042 + f64x8::splat(12.0) * t927 * t2042;
            let t2053 = ((t284).select(-f64x8::splat(3.0) / f64x8::splat(2.0) * t291 * t2034 - f64x8::splat(3.0) / f64x8::splat(16.0) * t2037 + f64x8::splat(3.0) * t2039, -f64x8::splat(6.0) * t915 * t2043 - t922 * t2050));
            let t2055 = t338 * t2053;
            let t2057 = t309 * t2053;
            let t2059 = f64x8::splat(35.0) / f64x8::splat(2.0) * t2055 - f64x8::splat(15.0) / f64x8::splat(2.0) * t2057;
            let t2062 = t310 * t2053;
            let t2065 = f64x8::splat(15.0) / f64x8::splat(2.0) * t2062 - f64x8::splat(3.0) / f64x8::splat(2.0) * t2053;
            let t2068 = t311 * t2053;
            let t2072 = f64x8::splat(315.0) / f64x8::splat(8.0) * t2068 - f64x8::splat(105.0) / f64x8::splat(4.0) * t2062 + f64x8::splat(15.0) / f64x8::splat(8.0) * t2053;
            let t2075 = t312 * t2053;
            let t2080 = f64x8::splat(3003.0) / f64x8::splat(16.0) * t2075 - f64x8::splat(3465.0) / f64x8::splat(16.0) * t2068 + f64x8::splat(945.0) / f64x8::splat(16.0) * t2062 - f64x8::splat(35.0) / f64x8::splat(16.0) * t2053;
            let t2083 = t336 * t2053;
            let t2087 = f64x8::splat(693.0) / f64x8::splat(8.0) * t2083 - f64x8::splat(315.0) / f64x8::splat(4.0) * t2055 + f64x8::splat(105.0) / f64x8::splat(8.0) * t2057;
            let t2092 = t425 * t2053;
            let t2100 = t406 * t2053;
            let t2106 = f64x8::splat(0.2074861966146727) * t2053 + f64x8::splat(0.0004230264400260503) * t388 * t2059 - f64x8::splat(0.006510071882485726) * t388 * t2065 + f64x8::splat(0.001136485825094485) * t388 * t2072 + f64x8::splat(0.0003807158595350892) * t388 * t2080 + f64x8::splat(0.0004260858412001439) * t388 * t2087 + f64x8::splat(0.001282471852770764) * t425 * t2065 + f64x8::splat(0.01683215086686233) * t2092 + f64x8::splat(0.0004312411759243052) * t442 * t2080 - f64x8::splat(0.0002721968500889238) * t425 * t2072 + f64x8::splat(0.0004187827907710905) * t425 * t2059 - f64x8::splat(0.000257733338272708) * t2100 + f64x8::splat(3.212943141118693e-06) * t425 * t2080 - f64x8::splat(0.001009981263546227) * t406 * t2059;
            let t2111 = t323 * t2053;
            let t2123 = t442 * t2053;
            let t2135 = f64x8::splat(0.0002262886186270548) * t406 * t2065 - f64x8::splat(0.0002202759704065197) * t406 * t2080 + f64x8::splat(0.1179363564823021) * t2111 + f64x8::splat(0.002125332357775206) * t323 * t2065 - f64x8::splat(0.0009641371299507833) * t459 * t2065 - f64x8::splat(0.001189668304951413) * t459 * t2072 - f64x8::splat(0.001863882881010248) * t459 * t2059 - f64x8::splat(0.001288306127279617) * t459 * t2087 - f64x8::splat(0.01030571429426108) * t2123 - f64x8::splat(0.001175614476758423) * t459 * t2080 - f64x8::splat(0.002494950550547465) * t442 * t2059 + f64x8::splat(0.003712786171321043) * t442 * t2065 + f64x8::splat(0.0001672905908063297) * t442 * t2072 - f64x8::splat(0.0006058496834176058) * t442 * t2087;
            let t2137 = t459 * t2053;
            let t2151 = t388 * t2053;
            let t2155 = t348 * t2053;
            let t2164 = -f64x8::splat(0.01437960658302686) * t2137 + f64x8::splat(0.001491587478361034) * t323 * t2087 + f64x8::splat(0.001940164714223896) * t323 * t2080 + f64x8::splat(0.002915285520983635) * t323 * t2059 + f64x8::splat(0.002007295399058147) * t323 * t2072 - f64x8::splat(0.001622621390953226) * t406 * t2087 - f64x8::splat(0.0005869916483960576) * t406 * t2072 + f64x8::splat(0.002334616776649133) * t2151 + f64x8::splat(0.01243327883803539) * t348 * t2065 + f64x8::splat(0.0003837976998664341) * t2155 + f64x8::splat(0.001522474179598972) * t348 * t2072 + f64x8::splat(0.00245752591853626) * t348 * t2059 - f64x8::splat(0.0003682519432462936) * t348 * t2087 - f64x8::splat(0.47225184058624103) * t2075;
            let t2188 = f64x8::splat(0.43767257904820067) * t2068 - f64x8::splat(0.09636448540578502) * t2062 - f64x8::splat(0.0003695503801501715) * t348 * t2080 - f64x8::splat(0.17131022505250731) * t2083 + f64x8::splat(0.11775890511466262) * t2055 - f64x8::splat(0.011765768981988275) * t2057 + f64x8::splat(0.0002776060240069905) * t425 * t2087 - f64x8::splat(0.016494338766497415) * t401 * t2053 + f64x8::splat(0.004264173071531283) * t383 * t2053 - f64x8::splat(0.003461421137476467) * t484 * t2053 + f64x8::splat(0.00538391567059323) * t470 * t2053 - f64x8::splat(0.0021270890439633733) * t456 * t2053 + f64x8::splat(0.000411086590637241) * t438 * t2053 + f64x8::splat(0.020012545797197603) * t421 * t2053;
            let t2190 = t2106 + t2135 + t2164 + t2188;
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
