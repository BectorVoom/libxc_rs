//! MGGA_X_MCML exc unpol kernel — explicit SIMD (bit-exact).
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
pub fn mgga_x_mcml_exc_unpol(
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
        let v_rho = load(rho, ip, np);
        let v_sigma = load(sigma, ip, np);
        let v_lapl = load(lapl, ip, np);
        let v_tau = load(tau, ip, np);
        let mut acc_zk = V_ZERO;
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
        }
        store_add(zk, ip, m, acc_zk);
        ip += 8;
    }
}
