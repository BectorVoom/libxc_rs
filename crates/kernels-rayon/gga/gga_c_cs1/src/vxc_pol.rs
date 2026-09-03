//! GGA_C_CS1 vxc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_cs1.c`
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
pub fn gga_c_cs1_vxc_pol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
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
        let mut acc_zk = V_ZERO;
        let mut acc_vrho_0 = V_ZERO;
        let mut acc_vrho_1 = V_ZERO;
        let mut acc_vsigma_0 = V_ZERO;
        let mut acc_vsigma_1 = V_ZERO;
        let mut acc_vsigma_2 = V_ZERO;
        {
            let t1 = v_rho0 - v_rho1;
            let t2 = t1 * t1;
            let t3 = v_rho0 + v_rho1;
            let t4 = t3 * t3;
            let t5 = f64x8::splat(1.0) / t4;
            let t7 = -t2 * t5 + f64x8::splat(1.0);
            let t8 = (simd::cbrt(t3));
            let t9 = f64x8::splat(1.0) / t8;
            let t11 = f64x8::splat(1.0) + f64x8::splat(0.349) * t9;
            let t12 = f64x8::splat(1.0) / t11;
            let t13 = t7 * t12;
            let t15 = v_sigma0 + f64x8::splat(2.0) * v_sigma1 + v_sigma2;
            let t16 = t15 * t15;
            let t17 = t4 * t4;
            let t18 = t17 * t3;
            let t20 = f64x8::splat(1.0) / t8 / t18;
            let t22 = t8 * t8;
            let t24 = f64x8::splat(1.0) / t22 / t4;
            let t27 = f64x8::splat(1.0) + f64x8::splat(0.006) * t15 * t24;
            let t28 = t27 * t27;
            let t29 = f64x8::splat(1.0) / t28;
            let t32 = -f64x8::splat(0.159068) + f64x8::splat(2.86308e-07) * t16 * t20 * t29;
            let t34 = t13 * t32 / f64x8::splat(4.0);
            let t35 = f64x8::splat(1.0) / t3;
            let t36 = t1 * t35;
            let t37 = f64x8::splat(1.0) + t36;
            let t38 = (t37).simd_le(zeta_threshold);
            let t39 = ((t38).select(zeta_threshold, t37));
            let t40 = (simd::cbrt(v_rho0));
            let t41 = t39 * t40;
            let t42 = t40 + f64x8::splat(0.349);
            let t43 = f64x8::splat(1.0) / t42;
            let t44 = v_sigma0 * v_sigma0;
            let t45 = v_rho0 * v_rho0;
            let t46 = t45 * t45;
            let t47 = t46 * v_rho0;
            let t49 = f64x8::splat(1.0) / t40 / t47;
            let t51 = t40 * t40;
            let t53 = f64x8::splat(1.0) / t51 / t45;
            let t56 = f64x8::splat(1.0) + f64x8::splat(0.006) * v_sigma0 * t53;
            let t57 = t56 * t56;
            let t58 = f64x8::splat(1.0) / t57;
            let t61 = -f64x8::splat(0.018897) + f64x8::splat(5.58864e-06) * t44 * t49 * t58;
            let t62 = t43 * t61;
            let t64 = t41 * t62 / f64x8::splat(2.0);
            let t65 = f64x8::splat(1.0) - t36;
            let t66 = (t65).simd_le(zeta_threshold);
            let t67 = ((t66).select(zeta_threshold, t65));
            let t68 = (simd::cbrt(v_rho1));
            let t69 = t67 * t68;
            let t70 = t68 + f64x8::splat(0.349);
            let t71 = f64x8::splat(1.0) / t70;
            let t72 = v_sigma2 * v_sigma2;
            let t73 = v_rho1 * v_rho1;
            let t74 = t73 * t73;
            let t75 = t74 * v_rho1;
            let t77 = f64x8::splat(1.0) / t68 / t75;
            let t79 = t68 * t68;
            let t81 = f64x8::splat(1.0) / t79 / t73;
            let t84 = f64x8::splat(1.0) + f64x8::splat(0.006) * v_sigma2 * t81;
            let t85 = t84 * t84;
            let t86 = f64x8::splat(1.0) / t85;
            let t89 = -f64x8::splat(0.018897) + f64x8::splat(5.58864e-06) * t72 * t77 * t86;
            let t90 = t71 * t89;
            let t92 = t69 * t90 / f64x8::splat(2.0);
            let tzk0 = t34 + t64 + t92;
            acc_zk = tzk0;
            let t93 = t1 * t5;
            let t94 = t4 * t3;
            let t95 = f64x8::splat(1.0) / t94;
            let t96 = t2 * t95;
            let t98 = -f64x8::splat(2.0) * t93 + f64x8::splat(2.0) * t96;
            let t99 = t98 * t12;
            let t100 = t99 * t32;
            let t101 = t100 / f64x8::splat(4.0);
            let t102 = t11 * t11;
            let t103 = f64x8::splat(1.0) / t102;
            let t104 = t7 * t103;
            let t106 = f64x8::splat(1.0) / t8 / t3;
            let t107 = t32 * t106;
            let t108 = t104 * t107;
            let t109 = f64x8::splat(0.029083333333333333) * t108;
            let t110 = t17 * t4;
            let t112 = f64x8::splat(1.0) / t8 / t110;
            let t116 = t16 * t15;
            let t117 = t17 * t17;
            let t118 = t117 * t3;
            let t119 = f64x8::splat(1.0) / t118;
            let t122 = f64x8::splat(1.0) / t28 / t27;
            let t125 = -f64x8::splat(1.526976e-06) * t16 * t112 * t29 + f64x8::splat(9.161856e-09) * t116 * t119 * t122;
            let t126 = t13 * t125;
            let t127 = t126 / f64x8::splat(4.0);
            let t128 = t35 - t93;
            let t129 = ((t38).select(f64x8::splat(0.0), t128));
            let t130 = t129 * t40;
            let t131 = t130 * t62;
            let t132 = t131 / f64x8::splat(2.0);
            let t133 = f64x8::splat(1.0) / t51;
            let t134 = t39 * t133;
            let t135 = t134 * t62;
            let t136 = t135 / f64x8::splat(6.0);
            let t137 = f64x8::splat(1.0) / t40;
            let t138 = t39 * t137;
            let t139 = t42 * t42;
            let t140 = f64x8::splat(1.0) / t139;
            let t141 = t140 * t61;
            let t142 = t138 * t141;
            let t143 = t142 / f64x8::splat(6.0);
            let t144 = t46 * t45;
            let t146 = f64x8::splat(1.0) / t40 / t144;
            let t150 = t44 * v_sigma0;
            let t151 = t46 * t46;
            let t152 = t151 * v_rho0;
            let t153 = f64x8::splat(1.0) / t152;
            let t156 = f64x8::splat(1.0) / t57 / t56;
            let t159 = -f64x8::splat(2.980608e-05) * t44 * t146 * t58 + f64x8::splat(1.7883648e-07) * t150 * t153 * t156;
            let t160 = t43 * t159;
            let t161 = t41 * t160;
            let t162 = t161 / f64x8::splat(2.0);
            let t164 = ((t66).select(f64x8::splat(0.0), -t128));
            let t165 = t164 * t68;
            let t166 = t165 * t90;
            let t167 = t166 / f64x8::splat(2.0);
            let tvrho0 = t34 + t64 + t92 + t3 * (t101 + t109 + t127 + t132 + t136 - t143 + t162 + t167);
            acc_vrho_0 = tvrho0;
            let t171 = f64x8::splat(2.0) * t93 + f64x8::splat(2.0) * t96;
            let t172 = t171 * t12;
            let t173 = t172 * t32;
            let t174 = t173 / f64x8::splat(4.0);
            let t175 = -t35 - t93;
            let t176 = ((t38).select(f64x8::splat(0.0), t175));
            let t177 = t176 * t40;
            let t178 = t177 * t62;
            let t179 = t178 / f64x8::splat(2.0);
            let t181 = ((t66).select(f64x8::splat(0.0), -t175));
            let t182 = t181 * t68;
            let t183 = t182 * t90;
            let t184 = t183 / f64x8::splat(2.0);
            let t185 = f64x8::splat(1.0) / t79;
            let t186 = t67 * t185;
            let t187 = t186 * t90;
            let t188 = t187 / f64x8::splat(6.0);
            let t189 = f64x8::splat(1.0) / t68;
            let t190 = t67 * t189;
            let t191 = t70 * t70;
            let t192 = f64x8::splat(1.0) / t191;
            let t193 = t192 * t89;
            let t194 = t190 * t193;
            let t195 = t194 / f64x8::splat(6.0);
            let t196 = t74 * t73;
            let t198 = f64x8::splat(1.0) / t68 / t196;
            let t202 = t72 * v_sigma2;
            let t203 = t74 * t74;
            let t204 = t203 * v_rho1;
            let t205 = f64x8::splat(1.0) / t204;
            let t208 = f64x8::splat(1.0) / t85 / t84;
            let t211 = -f64x8::splat(2.980608e-05) * t72 * t198 * t86 + f64x8::splat(1.7883648e-07) * t202 * t205 * t208;
            let t212 = t71 * t211;
            let t213 = t69 * t212;
            let t214 = t213 / f64x8::splat(2.0);
            let tvrho1 = t34 + t64 + t92 + t3 * (t174 + t109 + t127 + t179 + t184 + t188 - t195 + t214);
            acc_vrho_1 = tvrho1;
            let t218 = t15 * t20 * t29;
            let t220 = f64x8::splat(1.0) / t117;
            let t222 = t16 * t220 * t122;
            let t224 = f64x8::splat(5.72616e-07) * t218 - f64x8::splat(3.435696e-09) * t222;
            let t226 = t13 * t224 / f64x8::splat(4.0);
            let t230 = f64x8::splat(1.0) / t151;
            let t234 = f64x8::splat(1.117728e-05) * v_sigma0 * t49 * t58 - f64x8::splat(6.706368e-08) * t44 * t230 * t156;
            let t235 = t43 * t234;
            let t237 = t41 * t235 / f64x8::splat(2.0);
            let tvsigma0 = t3 * (t226 + t237);
            acc_vsigma_0 = tvsigma0;
            let t239 = t3 * t7;
            let t242 = f64x8::splat(1.145232e-06) * t218 - f64x8::splat(6.871392e-09) * t222;
            let t243 = t12 * t242;
            let tvsigma1 = t239 * t243 / f64x8::splat(4.0);
            acc_vsigma_1 = tvsigma1;
            let t248 = f64x8::splat(1.0) / t203;
            let t252 = f64x8::splat(1.117728e-05) * v_sigma2 * t77 * t86 - f64x8::splat(6.706368e-08) * t72 * t248 * t208;
            let t253 = t71 * t252;
            let t255 = t69 * t253 / f64x8::splat(2.0);
            let tvsigma2 = t3 * (t226 + t255);
            acc_vsigma_2 = tvsigma2;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        store_strided(vrho, ip, m, 2, 0, acc_vrho_0);
        store_strided(vrho, ip, m, 2, 1, acc_vrho_1);
        store_strided(vsigma, ip, m, 3, 0, acc_vsigma_0);
        store_strided(vsigma, ip, m, 3, 1, acc_vsigma_1);
        store_strided(vsigma, ip, m, 3, 2, acc_vsigma_2);
        ip += 8;
    }
}
