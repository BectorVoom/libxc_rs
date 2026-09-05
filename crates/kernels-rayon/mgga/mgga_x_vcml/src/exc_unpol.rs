//! MGGA_X_VCML exc unpol kernel — explicit SIMD (bit-exact).
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

#[allow(unused_variables, non_snake_case)]
pub fn mgga_x_vcml_exc_unpol(
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
        }
        store_add(zk, ip, m, acc_zk);
        ip += 8;
    }
}
