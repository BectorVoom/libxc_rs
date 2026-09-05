//! LDA_X_ERF kxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_x_erf.c`
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
pub fn lda_x_erf_kxc_unpol(
    rho: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    v2rho2: &mut [f64],
    v3rho3: &mut [f64],
    param_hyb_omega_0: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_hyb_omega_0 = f64x8::splat(param_hyb_omega_0);
    let dens_threshold = f64x8::splat(dens_threshold);
    let zeta_threshold = f64x8::splat(zeta_threshold);
    let mut ip = 0usize;
    while ip < np {
        let m = (np - ip).min(8);
        let v_rho = load(rho, ip, np);
        let mut acc_zk = V_ZERO;
        let mut acc_vrho = V_ZERO;
        let mut acc_v2rho2 = V_ZERO;
        let mut acc_v3rho3 = V_ZERO;
        {
            let t1 = f64x8::splat(M_CBRT3);
            let t3 = (simd::cbrt(f64x8::splat(1.0) / f64x8::splat(M_PI)));
            let t5 = f64x8::splat(M_CBRT4);
            let t6 = t5 * t5;
            let t7 = t1 * t3 * t6;
            let t8 = f64x8::splat(M_CBRT2);
            let t9 = t8 * t8;
            let t10 = (f64x8::splat(1.0)).simd_le(zeta_threshold);
            let t11 = (simd::cbrt(zeta_threshold));
            let t13 = ((t10).select(t11 * zeta_threshold, f64x8::splat(1.0)));
            let t14 = t9 * t13;
            let t15 = (simd::cbrt(v_rho));
            let t16 = (simd::cbrt(f64x8::splat(9.0)));
            let t17 = t16 * t16;
            let t18 = t3 * t3;
            let t20 = t17 * t18 * param_hyb_omega_0;
            let t23 = ((t10).select(t11, f64x8::splat(1.0)));
            let t24 = f64x8::splat(1.0) / t23;
            let t27 = t20 * t1 / t15 * t24 / f64x8::splat(18.0);
            let t28 = (f64x8::splat(1.35)).simd_le(t27);
            let t29 = (f64x8::splat(1.35)).simd_lt(t27);
            let t30 = ((t29).select(t27, f64x8::splat(1.35)));
            let t31 = t30 * t30;
            let t34 = t31 * t31;
            let t35 = f64x8::splat(1.0) / t34;
            let t37 = t34 * t31;
            let t38 = f64x8::splat(1.0) / t37;
            let t40 = t34 * t34;
            let t41 = f64x8::splat(1.0) / t40;
            let t44 = f64x8::splat(1.0) / t40 / t31;
            let t47 = f64x8::splat(1.0) / t40 / t34;
            let t50 = f64x8::splat(1.0) / t40 / t37;
            let t52 = t40 * t40;
            let t53 = f64x8::splat(1.0) / t52;
            let t56 = ((t29).select(f64x8::splat(1.35), t27));
            let t57 = ((f64x8::splat(M_PI)).sqrt());
            let t58 = f64x8::splat(1.0) / t56;
            let t60 = (simd::erf(t58 / f64x8::splat(2.0)));
            let t62 = t56 * t56;
            let t63 = f64x8::splat(1.0) / t62;
            let t65 = (simd::exp(-t63 / f64x8::splat(4.0)));
            let t66 = t65 - f64x8::splat(1.0);
            let t69 = t65 - f64x8::splat(3.0) / f64x8::splat(2.0) - f64x8::splat(2.0) * t62 * t66;
            let t72 = f64x8::splat(2.0) * t56 * t69 + t57 * t60;
            let t76 = ((t28).select(f64x8::splat(1.0) / t31 / f64x8::splat(36.0) - t35 / f64x8::splat(960.0) + t38 / f64x8::splat(26880.0) - t41 / f64x8::splat(829440.0) + t44 / f64x8::splat(28385280.0) - t47 / f64x8::splat(1073479680.0) + t50 / f64x8::splat(44590694400.0) - t53 / f64x8::splat(2021444812800.0), f64x8::splat(1.0) - f64x8::splat(8.0) / f64x8::splat(3.0) * t56 * t72));
            let t79 = t7 * t14 * t15 * t76;
            let tzk0 = -f64x8::splat(3.0) / f64x8::splat(16.0) * t79;
            acc_zk = tzk0;
            let t82 = t15 * v_rho;
            let t84 = t82 * t1 * t3;
            let t85 = t6 * t9;
            let t86 = t31 * t30;
            let t87 = f64x8::splat(1.0) / t86;
            let t92 = t20 * t1 / t82 * t24 / f64x8::splat(54.0);
            let t93 = ((t29).select(-t92, f64x8::splat(0.0)));
            let t96 = t34 * t30;
            let t97 = f64x8::splat(1.0) / t96;
            let t100 = t34 * t86;
            let t101 = f64x8::splat(1.0) / t100;
            let t105 = f64x8::splat(1.0) / t40 / t30;
            let t109 = f64x8::splat(1.0) / t40 / t86;
            let t113 = f64x8::splat(1.0) / t40 / t96;
            let t117 = f64x8::splat(1.0) / t40 / t100;
            let t121 = f64x8::splat(1.0) / t52 / t30;
            let t125 = ((t29).select(f64x8::splat(0.0), -t92));
            let t127 = t65 * t63;
            let t131 = t62 * t56;
            let t132 = f64x8::splat(1.0) / t131;
            let t136 = t56 * t66;
            let t141 = t132 * t125 * t65 / f64x8::splat(2.0) - f64x8::splat(4.0) * t136 * t125 - t58 * t125 * t65;
            let t144 = -t127 * t125 + f64x8::splat(2.0) * t125 * t69 + f64x8::splat(2.0) * t56 * t141;
            let t148 = ((t28).select(-t87 * t93 / f64x8::splat(18.0) + t97 * t93 / f64x8::splat(240.0) - t101 * t93 / f64x8::splat(4480.0) + t105 * t93 / f64x8::splat(103680.0) - t109 * t93 / f64x8::splat(2838528.0) + t113 * t93 / f64x8::splat(89456640.0) - t117 * t93 / f64x8::splat(3185049600.0) + t121 * t93 / f64x8::splat(126340300800.0), -f64x8::splat(8.0) / f64x8::splat(3.0) * t125 * t72 - f64x8::splat(8.0) / f64x8::splat(3.0) * t56 * t144));
            let tvrho0 = -t79 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(16.0) * t84 * t85 * t13 * t148;
            acc_vrho = tvrho0;
            let t153 = t15 * t15;
            let t154 = f64x8::splat(1.0) / t153;
            let t163 = t93 * t93;
            let t166 = v_rho * v_rho;
            let t172 = f64x8::splat(2.0) / f64x8::splat(81.0) * t20 * t1 / t15 / t166 * t24;
            let t173 = ((t29).select(t172, f64x8::splat(0.0)));
            let t201 = f64x8::splat(1.0) / t52 / t31;
            let t206 = t35 * t163 / f64x8::splat(6.0) - t87 * t173 / f64x8::splat(18.0) - t38 * t163 / f64x8::splat(48.0) + t97 * t173 / f64x8::splat(240.0) + t41 * t163 / f64x8::splat(640.0) - t101 * t173 / f64x8::splat(4480.0) - t44 * t163 / f64x8::splat(11520.0) + t105 * t173 / f64x8::splat(103680.0) + t47 * t163 / f64x8::splat(258048.0) - t109 * t173 / f64x8::splat(2838528.0) - t50 * t163 / f64x8::splat(6881280.0) + t113 * t173 / f64x8::splat(89456640.0) + t53 * t163 / f64x8::splat(212336640.0) - t117 * t173 / f64x8::splat(3185049600.0) - t201 * t163 / f64x8::splat(7431782400.0) + t121 * t173 / f64x8::splat(126340300800.0);
            let t207 = ((t29).select(f64x8::splat(0.0), t172));
            let t212 = t62 * t62;
            let t214 = f64x8::splat(1.0) / t212 / t56;
            let t215 = t125 * t125;
            let t216 = t214 * t215;
            let t219 = t65 * t132;
            let t227 = f64x8::splat(1.0) / t212;
            let t235 = f64x8::splat(1.0) / t212 / t62;
            let t236 = t235 * t215;
            let t247 = -f64x8::splat(2.0) * t227 * t215 * t65 + t132 * t207 * t65 / f64x8::splat(2.0) + t236 * t65 / f64x8::splat(4.0) - f64x8::splat(4.0) * t215 * t66 - t63 * t215 * t65 - f64x8::splat(4.0) * t136 * t207 - t58 * t207 * t65;
            let t250 = -t216 * t65 / f64x8::splat(2.0) + f64x8::splat(2.0) * t219 * t215 - t127 * t207 + f64x8::splat(2.0) * t207 * t69 + f64x8::splat(4.0) * t125 * t141 + f64x8::splat(2.0) * t56 * t247;
            let t254 = ((t28).select(t206, -f64x8::splat(8.0) / f64x8::splat(3.0) * t207 * t72 - f64x8::splat(16.0) / f64x8::splat(3.0) * t125 * t144 - f64x8::splat(8.0) / f64x8::splat(3.0) * t56 * t250));
            let tv2rho20 = -t7 * t14 * t154 * t76 / f64x8::splat(12.0) - t7 * t14 * t15 * t148 / f64x8::splat(2.0) - f64x8::splat(3.0) / f64x8::splat(16.0) * t84 * t85 * t13 * t254;
            acc_v2rho2 = tv2rho20;
            let t260 = f64x8::splat(1.0) / t153 / v_rho;
            let t273 = t163 * t93;
            let t276 = t35 * t93;
            let t281 = t38 * t93;
            let t286 = t41 * t93;
            let t291 = t44 * t93;
            let t296 = t47 * t93;
            let t301 = t50 * t93;
            let t304 = -f64x8::splat(2.0) / f64x8::splat(3.0) * t97 * t273 + t276 * t173 / f64x8::splat(2.0) + t101 * t273 / f64x8::splat(8.0) - t281 * t173 / f64x8::splat(16.0) - t105 * t273 / f64x8::splat(80.0) + f64x8::splat(3.0) / f64x8::splat(640.0) * t286 * t173 + t109 * t273 / f64x8::splat(1152.0) - t291 * t173 / f64x8::splat(3840.0) - t113 * t273 / f64x8::splat(21504.0) + t296 * t173 / f64x8::splat(86016.0) + t117 * t273 / f64x8::splat(491520.0) - t301 * t173 / f64x8::splat(2293760.0);
            let t307 = t53 * t93;
            let t311 = f64x8::splat(1.0) / t52 / t86;
            let t314 = t201 * t93;
            let t323 = f64x8::splat(14.0) / f64x8::splat(243.0) * t20 * t1 / t15 / t166 / v_rho * t24;
            let t324 = ((t29).select(-t323, f64x8::splat(0.0)));
            let t341 = -t121 * t273 / f64x8::splat(13271040.0) + t307 * t173 / f64x8::splat(70778880.0) + t311 * t273 / f64x8::splat(412876800.0) - t314 * t173 / f64x8::splat(2477260800.0) - t87 * t324 / f64x8::splat(18.0) + t97 * t324 / f64x8::splat(240.0) - t101 * t324 / f64x8::splat(4480.0) + t105 * t324 / f64x8::splat(103680.0) - t109 * t324 / f64x8::splat(2838528.0) + t113 * t324 / f64x8::splat(89456640.0) - t117 * t324 / f64x8::splat(3185049600.0) + t121 * t324 / f64x8::splat(126340300800.0);
            let t343 = ((t29).select(f64x8::splat(0.0), -t323));
            let t350 = t215 * t125;
            let t354 = t214 * t125;
            let t355 = t65 * t207;
            let t358 = t212 * t212;
            let t359 = f64x8::splat(1.0) / t358;
            let t363 = t65 * t227;
            let t379 = t227 * t125;
            let t383 = f64x8::splat(1.0) / t212 / t131;
            let t391 = t125 * t65;
            let t395 = f64x8::splat(1.0) / t358 / t56;
            let t399 = t125 * t66;
            let t402 = t63 * t125;
            let t409 = f64x8::splat(15.0) / f64x8::splat(2.0) * t214 * t350 * t65 - f64x8::splat(6.0) * t379 * t355 - f64x8::splat(5.0) / f64x8::splat(2.0) * t383 * t350 * t65 + t132 * t343 * t65 / f64x8::splat(2.0) + f64x8::splat(3.0) / f64x8::splat(4.0) * t235 * t207 * t391 + t395 * t350 * t65 / f64x8::splat(8.0) - f64x8::splat(12.0) * t399 * t207 - f64x8::splat(3.0) * t402 * t355 - f64x8::splat(4.0) * t136 * t343 - t58 * t343 * t65;
            let t412 = f64x8::splat(7.0) / f64x8::splat(2.0) * t235 * t350 * t65 - f64x8::splat(3.0) / f64x8::splat(2.0) * t354 * t355 - t359 * t350 * t65 / f64x8::splat(4.0) - f64x8::splat(6.0) * t363 * t350 + f64x8::splat(6.0) * t219 * t125 * t207 - t127 * t343 + f64x8::splat(2.0) * t343 * t69 + f64x8::splat(6.0) * t207 * t141 + f64x8::splat(6.0) * t125 * t247 + f64x8::splat(2.0) * t56 * t409;
            let t416 = ((t28).select(t304 + t341, -f64x8::splat(8.0) / f64x8::splat(3.0) * t343 * t72 - f64x8::splat(8.0) * t207 * t144 - f64x8::splat(8.0) * t125 * t250 - f64x8::splat(8.0) / f64x8::splat(3.0) * t56 * t412));
            let tv3rho30 = t7 * t14 * t260 * t76 / f64x8::splat(18.0) - t7 * t14 * t154 * t148 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(4.0) * t7 * t14 * t15 * t254 - f64x8::splat(3.0) / f64x8::splat(16.0) * t84 * t85 * t13 * t416;
            acc_v3rho3 = tv3rho30;
        }
        store_add(zk, ip, m, acc_zk);
        store_add(vrho, ip, m, acc_vrho);
        store_add(v2rho2, ip, m, acc_v2rho2);
        store_add(v3rho3, ip, m, acc_v3rho3);
        ip += 8;
    }
}
