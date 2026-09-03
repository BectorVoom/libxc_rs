//! MGGA_X_MCML exc pol kernel — explicit SIMD (bit-exact).
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
pub fn mgga_x_mcml_exc_pol(
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
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        ip += 8;
    }
}
