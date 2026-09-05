//! MGGA_X_MCML vxc unpol kernel — explicit SIMD (bit-exact).
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
pub fn mgga_x_mcml_vxc_unpol(
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
            let t46 = t45 * t45;
            let t47 = t46 * t45;
            let t50 = t45 * t44;
            let t52 = v_tau * t29;
            let t54 = f64x8::splat(1.0) / t31 / v_rho;
            let t60 = f64x8::splat(5.0) / f64x8::splat(9.0) * (t52 * t54 - t36 / f64x8::splat(8.0)) * t21 * t25;
            let t61 = (f64x8::splat(10000.0)).simd_le(t60);
            let t62 = (f64x8::splat(10000.0)).simd_lt(t60);
            let t63 = ((t62).select(t60, f64x8::splat(10000.0)));
            let t64 = t63 * t63;
            let t67 = t64 * t63;
            let t68 = f64x8::splat(1.0) / t67;
            let t70 = t64 * t64;
            let t71 = f64x8::splat(1.0) / t70;
            let t74 = ((t62).select(f64x8::splat(10000.0), t60));
            let t75 = t74 * t74;
            let t76 = f64x8::splat(1.0) - t75;
            let t77 = t76 * t76;
            let t78 = t77 * t76;
            let t79 = t75 * t74;
            let t81 = f64x8::splat(1.0) + f64x8::splat(4.0) * t79;
            let t83 = t79 * t81 + f64x8::splat(1.0);
            let t84 = f64x8::splat(1.0) / t83;
            let t86 = ((t61).select(f64x8::splat(3.0) / f64x8::splat(4.0) / t64 + t68 / f64x8::splat(16.0) - f64x8::splat(3.0) / f64x8::splat(4.0) * t71 - f64x8::splat(1.0) / f64x8::splat(4.0), t78 * t84));
            let t88 = t86 * t86;
            let t89 = t88 * t88;
            let t90 = t89 * t86;
            let t92 = t46 * t50;
            let t94 = t46 * t44;
            let t96 = t88 * t86;
            let t98 = t89 * t88;
            let t103 = t89 * t96;
            let t109 = f64x8::splat(429.0) / f64x8::splat(16.0) * t92 - f64x8::splat(693.0) / f64x8::splat(16.0) * t94 + f64x8::splat(315.0) / f64x8::splat(16.0) * t50 - f64x8::splat(35.0) / f64x8::splat(192.0) * t42 + f64x8::splat(35.0) / f64x8::splat(16.0);
            let t112 = f64x8::splat(3.0) / f64x8::splat(8.0) + f64x8::splat(35.0) / f64x8::splat(8.0) * t89 - f64x8::splat(15.0) / f64x8::splat(4.0) * t88;
            let t117 = f64x8::splat(5.0) / f64x8::splat(2.0) * t96 - f64x8::splat(3.0) / f64x8::splat(2.0) * t86;
            let t121 = -f64x8::splat(1.0) / f64x8::splat(2.0) + f64x8::splat(3.0) / f64x8::splat(2.0) * t88;
            let t124 = f64x8::splat(0.022419222998949863) * t47 - f64x8::splat(0.0010470532939127494) * t46 + f64x8::splat(0.007416880187036192) * t50 + f64x8::splat(0.2074861966146727) * t86 + f64x8::splat(0.08753451580964014) * t90 + f64x8::splat(0.015682422300093094) * t92 - f64x8::splat(0.015887583418757175) * t94 - f64x8::splat(0.03212149513526167) * t96 - f64x8::splat(0.028551704175417886) * t98 + f64x8::splat(0.029439726278665656) * t89 - f64x8::splat(0.005882884490994137) * t88 - f64x8::splat(0.37102687351218927) * t45 - f64x8::splat(0.06746454865517729) * t103 + f64x8::splat(0.00245752591853626) * t109 * t112 + f64x8::splat(0.01243327883803539) * t109 * t117 + f64x8::splat(0.001421391023843761) * t109 * t121;
            let t125 = t109 * t86;
            let t130 = -f64x8::splat(5.0) / f64x8::splat(16.0) + f64x8::splat(231.0) / f64x8::splat(16.0) * t47 - f64x8::splat(315.0) / f64x8::splat(16.0) * t46 + f64x8::splat(105.0) / f64x8::splat(16.0) * t45;
            let t135 = f64x8::splat(429.0) / f64x8::splat(16.0) * t103 - f64x8::splat(693.0) / f64x8::splat(16.0) * t90 + f64x8::splat(315.0) / f64x8::splat(16.0) * t96 - f64x8::splat(35.0) / f64x8::splat(16.0) * t86;
            let t141 = -f64x8::splat(5.0) / f64x8::splat(16.0) + f64x8::splat(231.0) / f64x8::splat(16.0) * t98 - f64x8::splat(315.0) / f64x8::splat(16.0) * t89 + f64x8::splat(105.0) / f64x8::splat(16.0) * t88;
            let t147 = f64x8::splat(63.0) / f64x8::splat(8.0) * t90 - f64x8::splat(35.0) / f64x8::splat(4.0) * t96 + f64x8::splat(15.0) / f64x8::splat(8.0) * t86;
            let t154 = f64x8::splat(5.0) / f64x8::splat(2.0) * t50 - t42 / f64x8::splat(8.0) + f64x8::splat(3.0) / f64x8::splat(2.0);
            let t163 = t154 * t86;
            let t166 = -f64x8::splat(1.0) / f64x8::splat(2.0) + f64x8::splat(3.0) / f64x8::splat(2.0) * t45;
            let t179 = f64x8::splat(0.0003837976998664341) * t125 + f64x8::splat(0.0003807158595350892) * t130 * t135 + f64x8::splat(0.0004260858412001439) * t130 * t141 + f64x8::splat(0.001136485825094485) * t130 * t147 + f64x8::splat(0.0004230264400260503) * t130 * t112 + f64x8::splat(0.0001672905908063297) * t154 * t147 - f64x8::splat(0.002494950550547465) * t154 * t112 + f64x8::splat(0.003712786171321043) * t154 * t117 - f64x8::splat(0.0007090296813211244) * t154 * t121 - f64x8::splat(0.01030571429426108) * t163 - f64x8::splat(0.001175614476758423) * t166 * t135 - f64x8::splat(0.001288306127279617) * t166 * t141 - f64x8::splat(0.001189668304951413) * t166 * t147 - f64x8::splat(0.001863882881010248) * t166 * t112 - f64x8::splat(0.0009641371299507833) * t166 * t117 - f64x8::splat(0.001153807045825489) * t166 * t121;
            let t181 = t166 * t86;
            let t195 = t44 * t86;
            let t207 = f64x8::splat(63.0) / f64x8::splat(8.0) * t94 - f64x8::splat(35.0) / f64x8::splat(4.0) * t50 + f64x8::splat(5.0) / f64x8::splat(32.0) * t42 - f64x8::splat(15.0) / f64x8::splat(8.0);
            let t210 = t207 * t86;
            let t214 = f64x8::splat(3.0) / f64x8::splat(8.0) + f64x8::splat(35.0) / f64x8::splat(8.0) * t46 - f64x8::splat(15.0) / f64x8::splat(4.0) * t45;
            let t219 = -f64x8::splat(0.01437960658302686) * t181 + f64x8::splat(0.001940164714223896) * t44 * t135 + f64x8::splat(0.001491587478361034) * t44 * t141 + f64x8::splat(0.002007295399058147) * t44 * t147 + f64x8::splat(0.002915285520983635) * t44 * t112 + f64x8::splat(0.002125332357775206) * t44 * t117 + f64x8::splat(0.00179463855686441) * t44 * t121 + f64x8::splat(0.1179363564823021) * t195 - f64x8::splat(0.0003695503801501715) * t109 * t135 - f64x8::splat(0.0003682519432462936) * t109 * t141 + f64x8::splat(0.001522474179598972) * t109 * t147 - f64x8::splat(0.01346592172626102) * t42 + f64x8::splat(0.006670848599065867) * t207 * t121 - f64x8::splat(0.000257733338272708) * t210 + f64x8::splat(3.212943141118693e-06) * t214 * t135 + f64x8::splat(0.0002776060240069905) * t214 * t141;
            let t228 = t214 * t86;
            let t238 = t130 * t86;
            let t250 = f64x8::splat(1.3502664484515603) - f64x8::splat(0.0002721968500889238) * t214 * t147 + f64x8::splat(0.0004187827907710905) * t214 * t112 + f64x8::splat(0.001282471852770764) * t214 * t117 + f64x8::splat(0.000137028863545747) * t214 * t121 + f64x8::splat(0.01683215086686233) * t228 + f64x8::splat(0.0004312411759243052) * t154 * t135 - f64x8::splat(0.0006058496834176058) * t154 * t141 - f64x8::splat(0.006510071882485726) * t130 * t117 - f64x8::splat(0.005498112922165805) * t130 * t121 + f64x8::splat(0.002334616776649133) * t238 - f64x8::splat(0.0002202759704065197) * t207 * t135 - f64x8::splat(0.001622621390953226) * t207 * t141 - f64x8::splat(0.0005869916483960576) * t207 * t147 - f64x8::splat(0.001009981263546227) * t207 * t112 + f64x8::splat(0.0002262886186270548) * t207 * t117;
            let t252 = t124 + t179 + t219 + t250;
            let t256 = ((t3).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t7 * t20 * t252));
            let tzk0 = f64x8::splat(2.0) * t256;
            acc_zk = tzk0;
            let t258 = t18 / t31;
            let t264 = t30 * v_rho;
            let t266 = f64x8::splat(1.0) / t31 / t264;
            let t272 = f64x8::splat(5.0) / f64x8::splat(9.0) * (-f64x8::splat(5.0) / f64x8::splat(3.0) * t52 * t33 + t35 * t266 / f64x8::splat(3.0)) * t21 * t25;
            let t273 = ((t62).select(t272, f64x8::splat(0.0)));
            let t276 = t71 * t273;
            let t279 = f64x8::splat(1.0) / t70 / t63;
            let t280 = t279 * t273;
            let t283 = t77 * t84;
            let t284 = ((t62).select(f64x8::splat(0.0), t272));
            let t285 = t74 * t284;
            let t288 = t83 * t83;
            let t289 = f64x8::splat(1.0) / t288;
            let t290 = t78 * t289;
            let t291 = t75 * t81;
            let t294 = t75 * t75;
            let t295 = t294 * t74;
            let t298 = f64x8::splat(3.0) * t291 * t284 + f64x8::splat(12.0) * t295 * t284;
            let t301 = ((t61).select(-f64x8::splat(3.0) / f64x8::splat(2.0) * t68 * t273 - f64x8::splat(3.0) / f64x8::splat(16.0) * t276 + f64x8::splat(3.0) * t280, -f64x8::splat(6.0) * t283 * t285 - t290 * t298));
            let t304 = t29 * t266;
            let t305 = t304 * t40;
            let t306 = t27 * t305;
            let t308 = t21 * t21;
            let t310 = f64x8::splat(1.0) / t23 / t22;
            let t311 = t308 * t310;
            let t312 = v_sigma * v_sigma;
            let t313 = t311 * t312;
            let t314 = t30 * t30;
            let t315 = t314 * t30;
            let t317 = f64x8::splat(1.0) / t19 / t315;
            let t319 = t39 * t39;
            let t320 = f64x8::splat(1.0) / t319;
            let t321 = t28 * t317 * t320;
            let t322 = t313 * t321;
            let t324 = -f64x8::splat(2.0) / f64x8::splat(9.0) * t306 + t322 / f64x8::splat(54.0);
            let t325 = t44 * t324;
            let t352 = f64x8::splat(0.00538391567059323) * t195 * t301 - f64x8::splat(0.003864918381838851) * t325 * t141 - f64x8::splat(0.003569004914854239) * t325 * t147 - f64x8::splat(0.005591648643030744) * t325 * t112 - f64x8::splat(0.0028924113898523497) * t325 * t117 - f64x8::splat(0.003461421137476467) * t325 * t121 - f64x8::splat(0.003461421137476467) * t181 * t301 - f64x8::splat(0.04313881974908058) * t325 * t86 - f64x8::splat(0.0021270890439633733) * t163 * t301 - f64x8::splat(0.003526843430275269) * t325 * t135 + f64x8::splat(0.000411086590637241) * t228 * t301 + f64x8::splat(0.020012545797197603) * t210 * t301 - f64x8::splat(0.016494338766497415) * t238 * t301 + f64x8::splat(0.004264173071531283) * t125 * t301;
            let t356 = t154 * t301;
            let t358 = t45 * t324;
            let t362 = f64x8::splat(15.0) / f64x8::splat(2.0) * t358 + t306 / f64x8::splat(3.0) - t322 / f64x8::splat(36.0);
            let t365 = t96 * t301;
            let t367 = t86 * t301;
            let t369 = f64x8::splat(35.0) / f64x8::splat(2.0) * t365 - f64x8::splat(15.0) / f64x8::splat(2.0) * t367;
            let t374 = t88 * t301;
            let t377 = f64x8::splat(15.0) / f64x8::splat(2.0) * t374 - f64x8::splat(3.0) / f64x8::splat(2.0) * t301;
            let t382 = t90 * t301;
            let t386 = f64x8::splat(693.0) / f64x8::splat(8.0) * t382 - f64x8::splat(315.0) / f64x8::splat(4.0) * t365 + f64x8::splat(105.0) / f64x8::splat(8.0) * t367;
            let t391 = t89 * t301;
            let t395 = f64x8::splat(315.0) / f64x8::splat(8.0) * t391 - f64x8::splat(105.0) / f64x8::splat(4.0) * t374 + f64x8::splat(15.0) / f64x8::splat(8.0) * t301;
            let t398 = t214 * t301;
            let t402 = f64x8::splat(0.2074861966146727) * t301 + f64x8::splat(0.03590912460336272) * t306 - f64x8::splat(0.002992427050280227) * t322 - f64x8::splat(0.01030571429426108) * t356 - f64x8::splat(0.002494950550547465) * t362 * t112 - f64x8::splat(0.002494950550547465) * t154 * t369 + f64x8::splat(0.003712786171321043) * t362 * t117 + f64x8::splat(0.003712786171321043) * t154 * t377 - f64x8::splat(0.0006058496834176058) * t362 * t141 - f64x8::splat(0.0006058496834176058) * t154 * t386 + f64x8::splat(0.0001672905908063297) * t362 * t147 + f64x8::splat(0.0001672905908063297) * t154 * t395 + f64x8::splat(0.01683215086686233) * t398 + f64x8::splat(0.0004312411759243052) * t362 * t135;
            let t406 = t94 * t324;
            let t408 = t50 * t324;
            let t411 = f64x8::splat(693.0) / f64x8::splat(8.0) * t406 - f64x8::splat(315.0) / f64x8::splat(4.0) * t408 + f64x8::splat(105.0) / f64x8::splat(8.0) * t325;
            let t414 = t411 * t86;
            let t422 = t47 * t324;
            let t424 = t46 * t324;
            let t429 = f64x8::splat(3003.0) / f64x8::splat(16.0) * t422 - f64x8::splat(3465.0) / f64x8::splat(16.0) * t424 + f64x8::splat(945.0) / f64x8::splat(16.0) * t358 + f64x8::splat(35.0) / f64x8::splat(72.0) * t306 - f64x8::splat(35.0) / f64x8::splat(864.0) * t322;
            let t432 = t429 * t86;
            let t434 = t109 * t301;
            let t442 = t324 * t141;
            let t445 = -f64x8::splat(0.006510071882485726) * t130 * t377 - f64x8::splat(0.005498112922165805) * t411 * t121 + f64x8::splat(0.002334616776649133) * t414 + f64x8::splat(0.0004230264400260503) * t411 * t112 + f64x8::splat(0.0004230264400260503) * t130 * t369 + f64x8::splat(0.0004260858412001439) * t130 * t386 + f64x8::splat(0.001421391023843761) * t429 * t121 + f64x8::splat(0.0003837976998664341) * t432 + f64x8::splat(0.0003837976998664341) * t434 + f64x8::splat(0.0003807158595350892) * t411 * t135 + f64x8::splat(0.00245752591853626) * t429 * t112 + f64x8::splat(0.002007295399058147) * t44 * t395 + f64x8::splat(0.001491587478361034) * t442 - f64x8::splat(0.17131022505250731) * t382;
            let t448 = t166 * t301;
            let t450 = t324 * t135;
            let t452 = t98 * t301;
            let t458 = f64x8::splat(3003.0) / f64x8::splat(16.0) * t452 - f64x8::splat(3465.0) / f64x8::splat(16.0) * t391 + f64x8::splat(945.0) / f64x8::splat(16.0) * t374 - f64x8::splat(35.0) / f64x8::splat(16.0) * t301;
            let t474 = t362 * t86;
            let t476 = t324 * t117;
            let t478 = f64x8::splat(0.001491587478361034) * t44 * t386 - f64x8::splat(0.01437960658302686) * t448 + f64x8::splat(0.001940164714223896) * t450 - f64x8::splat(0.47225184058624103) * t452 + f64x8::splat(0.001940164714223896) * t44 * t458 - f64x8::splat(0.001863882881010248) * t166 * t369 - f64x8::splat(0.0009641371299507833) * t166 * t377 - f64x8::splat(0.001189668304951413) * t166 * t395 - f64x8::splat(0.001175614476758423) * t166 * t458 - f64x8::splat(0.001288306127279617) * t166 * t386 + f64x8::splat(0.022250640561108575) * t358 - f64x8::splat(0.0007090296813211244) * t362 * t121 - f64x8::splat(0.01030571429426108) * t474 + f64x8::splat(0.002125332357775206) * t476;
            let t484 = t324 * t121;
            let t486 = t324 * t86;
            let t488 = t44 * t301;
            let t490 = t324 * t112;
            let t496 = t324 * t147;
            let t501 = f64x8::splat(35.0) / f64x8::splat(2.0) * t408 - f64x8::splat(15.0) / f64x8::splat(2.0) * t325;
            let t508 = -f64x8::splat(0.09636448540578502) * t374 + f64x8::splat(0.002125332357775206) * t44 * t377 + f64x8::splat(0.00179463855686441) * t484 + f64x8::splat(0.1179363564823021) * t486 + f64x8::splat(0.1179363564823021) * t488 + f64x8::splat(0.002915285520983635) * t490 + f64x8::splat(0.11775890511466262) * t365 - f64x8::splat(0.011765768981988275) * t367 + f64x8::splat(0.002915285520983635) * t44 * t369 + f64x8::splat(0.002007295399058147) * t496 + f64x8::splat(0.43767257904820067) * t391 + f64x8::splat(0.0002776060240069905) * t501 * t141 + f64x8::splat(0.0002776060240069905) * t214 * t386 - f64x8::splat(0.0002721968500889238) * t501 * t147;
            let t511 = t207 * t301;
            let t522 = f64x8::splat(315.0) / f64x8::splat(8.0) * t424 - f64x8::splat(105.0) / f64x8::splat(4.0) * t358 - f64x8::splat(5.0) / f64x8::splat(12.0) * t306 + f64x8::splat(5.0) / f64x8::splat(144.0) * t322;
            let t529 = t522 * t86;
            let t541 = -f64x8::splat(0.0002721968500889238) * t214 * t395 - f64x8::splat(0.000257733338272708) * t511 + f64x8::splat(3.212943141118693e-06) * t501 * t135 + f64x8::splat(3.212943141118693e-06) * t214 * t458 - f64x8::splat(0.07943791709378588) * t424 + f64x8::splat(0.0002262886186270548) * t522 * t117 + f64x8::splat(0.0002262886186270548) * t207 * t377 + f64x8::splat(0.006670848599065867) * t522 * t121 - f64x8::splat(0.000257733338272708) * t529 - f64x8::splat(0.001009981263546227) * t522 * t112 - f64x8::splat(0.001009981263546227) * t207 * t369 - f64x8::splat(0.001622621390953226) * t522 * t141 - f64x8::splat(0.001622621390953226) * t207 * t386 - f64x8::splat(0.0005869916483960576) * t522 * t147;
            let t545 = t130 * t301;
            let t570 = -f64x8::splat(0.0005869916483960576) * t207 * t395 + f64x8::splat(0.002334616776649133) * t545 - f64x8::splat(0.0002202759704065197) * t522 * t135 - f64x8::splat(0.0002202759704065197) * t207 * t458 + f64x8::splat(0.1345153379936992) * t406 - f64x8::splat(0.006510071882485726) * t411 * t117 + f64x8::splat(0.00245752591853626) * t109 * t369 + f64x8::splat(0.01243327883803539) * t429 * t117 + f64x8::splat(0.01243327883803539) * t109 * t377 + f64x8::splat(0.001522474179598972) * t429 * t147 + f64x8::splat(0.001522474179598972) * t109 * t395 - f64x8::splat(0.0003682519432462936) * t429 * t141 - f64x8::splat(0.0003682519432462936) * t109 * t386 - f64x8::splat(0.0003695503801501715) * t429 * t135;
            let t583 = t501 * t86;
            let t598 = -f64x8::splat(0.0003695503801501715) * t109 * t458 + f64x8::splat(0.0004312411759243052) * t154 * t458 - f64x8::splat(0.004188213175650998) * t408 - f64x8::splat(0.7420537470243785) * t325 + f64x8::splat(0.001282471852770764) * t501 * t117 + f64x8::splat(0.001282471852770764) * t214 * t377 + f64x8::splat(0.000137028863545747) * t501 * t121 + f64x8::splat(0.01683215086686233) * t583 + f64x8::splat(0.0004187827907710905) * t501 * t112 + f64x8::splat(0.0004187827907710905) * t214 * t369 + f64x8::splat(0.001136485825094485) * t411 * t147 + f64x8::splat(0.001136485825094485) * t130 * t395 + f64x8::splat(0.0003807158595350892) * t130 * t458 + f64x8::splat(0.0004260858412001439) * t411 * t141 + f64x8::splat(0.10977695610065165) * t422;
            let t601 = t352 + t402 + t445 + t478 + t508 + t541 + t570 + t598;
            let t606 = ((t3).select(f64x8::splat(0.0), -t7 * t258 * t252 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t7 * t20 * t601));
            let tvrho0 = f64x8::splat(2.0) * v_rho * t606 + f64x8::splat(2.0) * t256;
            acc_vrho = tvrho0;
            let t610 = t314 * v_rho;
            let t614 = t28 / t19 / t610 * t320;
            let t615 = t311 * v_sigma * t614;
            let t617 = t26 * t34;
            let t618 = f64x8::splat(5.0) / f64x8::splat(72.0) * t617;
            let t619 = ((t62).select(-t618, f64x8::splat(0.0)));
            let t622 = t71 * t619;
            let t624 = t279 * t619;
            let t627 = ((t62).select(f64x8::splat(0.0), -t618));
            let t628 = t74 * t627;
            let t635 = f64x8::splat(3.0) * t291 * t627 + f64x8::splat(12.0) * t295 * t627;
            let t638 = ((t61).select(-f64x8::splat(3.0) / f64x8::splat(2.0) * t68 * t619 - f64x8::splat(3.0) / f64x8::splat(16.0) * t622 + f64x8::splat(3.0) * t624, -f64x8::splat(6.0) * t283 * t628 - t290 * t635));
            let t641 = t26 * t41;
            let t644 = t641 / f64x8::splat(12.0) - t615 / f64x8::splat(144.0);
            let t645 = t44 * t644;
            let t665 = t98 * t638;
            let t667 = t89 * t638;
            let t669 = f64x8::splat(0.001122160143855085) * t615 - f64x8::splat(0.0021270890439633733) * t163 * t638 - f64x8::splat(0.003526843430275269) * t645 * t135 - f64x8::splat(0.003864918381838851) * t645 * t141 - f64x8::splat(0.003569004914854239) * t645 * t147 - f64x8::splat(0.005591648643030744) * t645 * t112 - f64x8::splat(0.0028924113898523497) * t645 * t117 - f64x8::splat(0.003461421137476467) * t645 * t121 - f64x8::splat(0.003461421137476467) * t181 * t638 - f64x8::splat(0.04313881974908058) * t645 * t86 + f64x8::splat(0.00538391567059323) * t195 * t638 + f64x8::splat(0.2074861966146727) * t638 - f64x8::splat(0.47225184058624103) * t665 + f64x8::splat(0.43767257904820067) * t667;
            let t670 = t88 * t638;
            let t676 = f64x8::splat(3003.0) / f64x8::splat(16.0) * t665 - f64x8::splat(3465.0) / f64x8::splat(16.0) * t667 + f64x8::splat(945.0) / f64x8::splat(16.0) * t670 - f64x8::splat(35.0) / f64x8::splat(16.0) * t638;
            let t679 = t50 * t644;
            let t684 = f64x8::splat(35.0) / f64x8::splat(2.0) * t679 - f64x8::splat(15.0) / f64x8::splat(2.0) * t645;
            let t687 = t90 * t638;
            let t689 = t96 * t638;
            let t691 = t86 * t638;
            let t696 = f64x8::splat(693.0) / f64x8::splat(8.0) * t687 - f64x8::splat(315.0) / f64x8::splat(4.0) * t689 + f64x8::splat(105.0) / f64x8::splat(8.0) * t691;
            let t704 = f64x8::splat(315.0) / f64x8::splat(8.0) * t667 - f64x8::splat(105.0) / f64x8::splat(4.0) * t670 + f64x8::splat(15.0) / f64x8::splat(8.0) * t638;
            let t711 = f64x8::splat(35.0) / f64x8::splat(2.0) * t689 - f64x8::splat(15.0) / f64x8::splat(2.0) * t691;
            let t716 = -f64x8::splat(0.09636448540578502) * t670 + f64x8::splat(3.212943141118693e-06) * t214 * t676 - f64x8::splat(0.004188213175650998) * t679 - f64x8::splat(0.7420537470243785) * t645 + f64x8::splat(0.0002776060240069905) * t684 * t141 - f64x8::splat(0.17131022505250731) * t687 + f64x8::splat(0.11775890511466262) * t689 - f64x8::splat(0.011765768981988275) * t691 + f64x8::splat(0.0002776060240069905) * t214 * t696 - f64x8::splat(0.0002721968500889238) * t684 * t147 - f64x8::splat(0.0002721968500889238) * t214 * t704 + f64x8::splat(0.0004187827907710905) * t684 * t112 + f64x8::splat(0.0004187827907710905) * t214 * t711 + f64x8::splat(0.001282471852770764) * t684 * t117;
            let t720 = f64x8::splat(15.0) / f64x8::splat(2.0) * t670 - f64x8::splat(3.0) / f64x8::splat(2.0) * t638;
            let t725 = t684 * t86;
            let t727 = t214 * t638;
            let t729 = t45 * t644;
            let t734 = f64x8::splat(15.0) / f64x8::splat(2.0) * t729 - t641 / f64x8::splat(8.0) + t615 / f64x8::splat(96.0);
            let t753 = f64x8::splat(0.001282471852770764) * t214 * t720 + f64x8::splat(0.000137028863545747) * t684 * t121 + f64x8::splat(0.01683215086686233) * t725 + f64x8::splat(0.01683215086686233) * t727 + f64x8::splat(0.022250640561108575) * t729 + f64x8::splat(0.0004312411759243052) * t734 * t135 + f64x8::splat(0.0004312411759243052) * t154 * t676 - f64x8::splat(0.0006058496834176058) * t734 * t141 - f64x8::splat(0.0006058496834176058) * t154 * t696 + f64x8::splat(0.0001672905908063297) * t734 * t147 + f64x8::splat(0.0001672905908063297) * t154 * t704 - f64x8::splat(0.002494950550547465) * t734 * t112 - f64x8::splat(0.002494950550547465) * t154 * t711 + f64x8::splat(0.003712786171321043) * t734 * t117;
            let t758 = t734 * t86;
            let t760 = t94 * t644;
            let t764 = f64x8::splat(693.0) / f64x8::splat(8.0) * t760 - f64x8::splat(315.0) / f64x8::splat(4.0) * t679 + f64x8::splat(105.0) / f64x8::splat(8.0) * t645;
            let t775 = t764 * t86;
            let t779 = t47 * t644;
            let t781 = t46 * t644;
            let t786 = f64x8::splat(3003.0) / f64x8::splat(16.0) * t779 - f64x8::splat(3465.0) / f64x8::splat(16.0) * t781 + f64x8::splat(945.0) / f64x8::splat(16.0) * t729 - f64x8::splat(35.0) / f64x8::splat(192.0) * t641 + f64x8::splat(35.0) / f64x8::splat(2304.0) * t615;
            let t789 = t786 * t86;
            let t791 = t109 * t638;
            let t794 = f64x8::splat(0.003712786171321043) * t154 * t720 - f64x8::splat(0.0007090296813211244) * t734 * t121 - f64x8::splat(0.01030571429426108) * t758 + f64x8::splat(0.0004230264400260503) * t764 * t112 + f64x8::splat(0.0004230264400260503) * t130 * t711 - f64x8::splat(0.006510071882485726) * t764 * t117 - f64x8::splat(0.006510071882485726) * t130 * t720 - f64x8::splat(0.005498112922165805) * t764 * t121 + f64x8::splat(0.002334616776649133) * t775 + f64x8::splat(0.01243327883803539) * t109 * t720 + f64x8::splat(0.001421391023843761) * t786 * t121 + f64x8::splat(0.0003837976998664341) * t789 + f64x8::splat(0.0003837976998664341) * t791 + f64x8::splat(0.1345153379936992) * t760;
            let t813 = f64x8::splat(315.0) / f64x8::splat(8.0) * t781 - f64x8::splat(105.0) / f64x8::splat(4.0) * t729 + f64x8::splat(5.0) / f64x8::splat(32.0) * t641 - f64x8::splat(5.0) / f64x8::splat(384.0) * t615;
            let t824 = t813 * t86;
            let t826 = t207 * t638;
            let t829 = f64x8::splat(0.0003807158595350892) * t764 * t135 + f64x8::splat(0.001522474179598972) * t786 * t147 + f64x8::splat(0.001522474179598972) * t109 * t704 + f64x8::splat(0.00245752591853626) * t786 * t112 + f64x8::splat(0.00245752591853626) * t109 * t711 + f64x8::splat(0.01243327883803539) * t786 * t117 - f64x8::splat(0.001009981263546227) * t813 * t112 - f64x8::splat(0.001009981263546227) * t207 * t711 + f64x8::splat(0.0002262886186270548) * t813 * t117 + f64x8::splat(0.0002262886186270548) * t207 * t720 + f64x8::splat(0.006670848599065867) * t813 * t121 - f64x8::splat(0.000257733338272708) * t824 - f64x8::splat(0.000257733338272708) * t826 - f64x8::splat(0.07943791709378588) * t781;
            let t840 = t154 * t638;
            let t852 = t166 * t638;
            let t854 = t644 * t135;
            let t858 = -f64x8::splat(0.0003695503801501715) * t786 * t135 - f64x8::splat(0.0003695503801501715) * t109 * t676 - f64x8::splat(0.0003682519432462936) * t786 * t141 - f64x8::splat(0.0003682519432462936) * t109 * t696 + f64x8::splat(3.212943141118693e-06) * t684 * t135 - f64x8::splat(0.01030571429426108) * t840 - f64x8::splat(0.001175614476758423) * t166 * t676 - f64x8::splat(0.001288306127279617) * t166 * t696 - f64x8::splat(0.001189668304951413) * t166 * t704 - f64x8::splat(0.001863882881010248) * t166 * t711 - f64x8::splat(0.0009641371299507833) * t166 * t720 - f64x8::splat(0.01437960658302686) * t852 + f64x8::splat(0.001940164714223896) * t854 + f64x8::splat(0.001940164714223896) * t44 * t676;
            let t860 = t644 * t141;
            let t864 = t644 * t147;
            let t868 = t644 * t112;
            let t882 = t644 * t117;
            let t886 = t644 * t121;
            let t888 = f64x8::splat(0.001491587478361034) * t860 + f64x8::splat(0.001491587478361034) * t44 * t696 + f64x8::splat(0.002007295399058147) * t864 + f64x8::splat(0.002007295399058147) * t44 * t704 + f64x8::splat(0.002915285520983635) * t868 + f64x8::splat(0.0003807158595350892) * t130 * t676 + f64x8::splat(0.0004260858412001439) * t764 * t141 + f64x8::splat(0.0004260858412001439) * t130 * t696 + f64x8::splat(0.001136485825094485) * t764 * t147 + f64x8::splat(0.001136485825094485) * t130 * t704 + f64x8::splat(0.002915285520983635) * t44 * t711 + f64x8::splat(0.002125332357775206) * t882 + f64x8::splat(0.002125332357775206) * t44 * t720 + f64x8::splat(0.00179463855686441) * t886;
            let t889 = t644 * t86;
            let t891 = t44 * t638;
            let t895 = t130 * t638;
            let t917 = f64x8::splat(0.1179363564823021) * t889 + f64x8::splat(0.1179363564823021) * t891 + f64x8::splat(0.10977695610065165) * t779 - f64x8::splat(0.01346592172626102) * t641 + f64x8::splat(0.002334616776649133) * t895 - f64x8::splat(0.0002202759704065197) * t813 * t135 - f64x8::splat(0.0002202759704065197) * t207 * t676 - f64x8::splat(0.001622621390953226) * t813 * t141 - f64x8::splat(0.001622621390953226) * t207 * t696 - f64x8::splat(0.0005869916483960576) * t813 * t147 - f64x8::splat(0.0005869916483960576) * t207 * t704 + f64x8::splat(0.004264173071531283) * t125 * t638 - f64x8::splat(0.016494338766497415) * t238 * t638 + f64x8::splat(0.020012545797197603) * t210 * t638 + f64x8::splat(0.000411086590637241) * t228 * t638;
            let t920 = t669 + t716 + t753 + t794 + t829 + t858 + t888 + t917;
            let t924 = ((t3).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t7 * t20 * t920));
            let tvsigma0 = f64x8::splat(2.0) * v_rho * t924;
            acc_vsigma = tvsigma0;
            let tvlapl0 = f64x8::splat(0.0);
            acc_vlapl = tvlapl0;
            let t928 = f64x8::splat(5.0) / f64x8::splat(9.0) * t29 * t54 * t26;
            let t929 = ((t62).select(t928, f64x8::splat(0.0)));
            let t932 = t71 * t929;
            let t934 = t279 * t929;
            let t937 = ((t62).select(f64x8::splat(0.0), t928));
            let t938 = t74 * t937;
            let t945 = f64x8::splat(3.0) * t291 * t937 + f64x8::splat(12.0) * t295 * t937;
            let t948 = ((t61).select(-f64x8::splat(3.0) / f64x8::splat(2.0) * t68 * t929 - f64x8::splat(3.0) / f64x8::splat(16.0) * t932 + f64x8::splat(3.0) * t934, -f64x8::splat(6.0) * t283 * t938 - t290 * t945));
            let t950 = t96 * t948;
            let t952 = t86 * t948;
            let t954 = f64x8::splat(35.0) / f64x8::splat(2.0) * t950 - f64x8::splat(15.0) / f64x8::splat(2.0) * t952;
            let t957 = t90 * t948;
            let t961 = f64x8::splat(693.0) / f64x8::splat(8.0) * t957 - f64x8::splat(315.0) / f64x8::splat(4.0) * t950 + f64x8::splat(105.0) / f64x8::splat(8.0) * t952;
            let t964 = t214 * t948;
            let t966 = t98 * t948;
            let t968 = t89 * t948;
            let t970 = t88 * t948;
            let t973 = f64x8::splat(3003.0) / f64x8::splat(16.0) * t966 - f64x8::splat(3465.0) / f64x8::splat(16.0) * t968 + f64x8::splat(945.0) / f64x8::splat(16.0) * t970 - f64x8::splat(35.0) / f64x8::splat(16.0) * t948;
            let t978 = f64x8::splat(15.0) / f64x8::splat(2.0) * t970 - f64x8::splat(3.0) / f64x8::splat(2.0) * t948;
            let t984 = f64x8::splat(315.0) / f64x8::splat(8.0) * t968 - f64x8::splat(105.0) / f64x8::splat(4.0) * t970 + f64x8::splat(15.0) / f64x8::splat(8.0) * t948;
            let t991 = t207 * t948;
            let t1001 = f64x8::splat(0.2074861966146727) * t948 - f64x8::splat(0.002494950550547465) * t154 * t954 - f64x8::splat(0.0006058496834176058) * t154 * t961 + f64x8::splat(0.01683215086686233) * t964 + f64x8::splat(0.0004312411759243052) * t154 * t973 + f64x8::splat(0.001282471852770764) * t214 * t978 - f64x8::splat(0.0002721968500889238) * t214 * t984 + f64x8::splat(0.0004187827907710905) * t214 * t954 + f64x8::splat(0.0002776060240069905) * t214 * t961 - f64x8::splat(0.000257733338272708) * t991 + f64x8::splat(3.212943141118693e-06) * t214 * t973 - f64x8::splat(0.0003695503801501715) * t109 * t973 + f64x8::splat(0.0002262886186270548) * t207 * t978 - f64x8::splat(0.0005869916483960576) * t207 * t984;
            let t1010 = t130 * t948;
            let t1018 = t109 * t948;
            let t1030 = -f64x8::splat(0.001009981263546227) * t207 * t954 - f64x8::splat(0.001622621390953226) * t207 * t961 - f64x8::splat(0.0002202759704065197) * t207 * t973 - f64x8::splat(0.006510071882485726) * t130 * t978 + f64x8::splat(0.002334616776649133) * t1010 + f64x8::splat(0.0004230264400260503) * t130 * t954 + f64x8::splat(0.0004260858412001439) * t130 * t961 + f64x8::splat(0.001136485825094485) * t130 * t984 + f64x8::splat(0.0003837976998664341) * t1018 + f64x8::splat(0.0003807158595350892) * t130 * t973 + f64x8::splat(0.00245752591853626) * t109 * t954 + f64x8::splat(0.01243327883803539) * t109 * t978 - f64x8::splat(0.0003682519432462936) * t109 * t961 + f64x8::splat(0.001522474179598972) * t109 * t984;
            let t1049 = t44 * t948;
            let t1056 = f64x8::splat(0.020012545797197603) * t210 * t948 + f64x8::splat(0.000411086590637241) * t228 * t948 + f64x8::splat(0.00538391567059323) * t195 * t948 - f64x8::splat(0.003461421137476467) * t181 * t948 - f64x8::splat(0.0021270890439633733) * t163 * t948 - f64x8::splat(0.016494338766497415) * t238 * t948 + f64x8::splat(0.004264173071531283) * t125 * t948 - f64x8::splat(0.09636448540578502) * t970 + f64x8::splat(0.002125332357775206) * t44 * t978 + f64x8::splat(0.1179363564823021) * t1049 + f64x8::splat(0.11775890511466262) * t950 - f64x8::splat(0.011765768981988275) * t952 + f64x8::splat(0.002915285520983635) * t44 * t954 + f64x8::splat(0.43767257904820067) * t968;
            let t1062 = t166 * t948;
            let t1079 = t154 * t948;
            let t1083 = f64x8::splat(0.002007295399058147) * t44 * t984 - f64x8::splat(0.17131022505250731) * t957 + f64x8::splat(0.001491587478361034) * t44 * t961 - f64x8::splat(0.01437960658302686) * t1062 - f64x8::splat(0.47225184058624103) * t966 + f64x8::splat(0.001940164714223896) * t44 * t973 - f64x8::splat(0.0009641371299507833) * t166 * t978 - f64x8::splat(0.001863882881010248) * t166 * t954 - f64x8::splat(0.001288306127279617) * t166 * t961 - f64x8::splat(0.001189668304951413) * t166 * t984 - f64x8::splat(0.001175614476758423) * t166 * t973 + f64x8::splat(0.003712786171321043) * t154 * t978 - f64x8::splat(0.01030571429426108) * t1079 + f64x8::splat(0.0001672905908063297) * t154 * t984;
            let t1085 = t1001 + t1030 + t1056 + t1083;
            let t1089 = ((t3).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t7 * t20 * t1085));
            let tvtau0 = f64x8::splat(2.0) * v_rho * t1089;
            acc_vtau = tvtau0;
        }
        store_add(zk, ip, m, acc_zk);
        store_add(vrho, ip, m, acc_vrho);
        store_add(vsigma, ip, m, acc_vsigma);
        store_add(vlapl, ip, m, acc_vlapl);
        store_add(vtau, ip, m, acc_vtau);
        ip += 8;
    }
}
