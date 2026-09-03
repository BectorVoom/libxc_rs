//! LDA_C_ML1 vxc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_c_ml1.c`
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
pub fn lda_c_ml1_vxc_pol(
    rho: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    param_fc: f64,
    param_q: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_fc = f64x8::splat(param_fc);
    let param_q = f64x8::splat(param_q);
    let dens_threshold = f64x8::splat(dens_threshold);
    let zeta_threshold = f64x8::splat(zeta_threshold);
    let mut ip = 0usize;
    while ip < np {
        let m = (np - ip).min(8);
        let v_rho0 = load_strided(rho, ip, np, 2, 0);
        let v_rho1 = load_strided(rho, ip, np, 2, 1);
        let mut acc_zk = V_ZERO;
        let mut acc_vrho_0 = V_ZERO;
        let mut acc_vrho_1 = V_ZERO;
        {
            let t1 = v_rho0 + v_rho1;
            let t2 = v_rho0 - v_rho1;
            let t3 = f64x8::splat(1.0) / t1;
            let t4 = t2 * t3;
            let t5 = ((t4).abs());
            let t7 = (f64x8::splat(1.0) - t5).simd_le(zeta_threshold);
            let t8 = t2 * t2;
            let t9 = t1 * t1;
            let t10 = f64x8::splat(1.0) / t9;
            let t12 = -t8 * t10 + f64x8::splat(1.0);
            let t13 = (simd::cbrt(t1));
            let t14 = t13 * param_fc;
            let t16 = (f64x8::splat(1.0) + t4).simd_le(zeta_threshold);
            let t17 = zeta_threshold - f64x8::splat(1.0);
            let t19 = (f64x8::splat(1.0) - t4).simd_le(zeta_threshold);
            let t21 = ((t16).select(t17, (t19).select(-t17, t4)));
            let t22 = f64x8::splat(1.0) + t21;
            let t23 = (simd::pow(t22, param_q));
            let t24 = f64x8::splat(1.0) - t21;
            let t25 = (simd::pow(t24, param_q));
            let t26 = t23 + t25;
            let t27 = t21 * t21;
            let t28 = f64x8::splat(1.0) - t27;
            let t29 = (simd::cbrt(t28));
            let t30 = t26 * t29;
            let t31 = (simd::cbrt(t22));
            let t32 = (simd::cbrt(t24));
            let t33 = t31 + t32;
            let t34 = f64x8::splat(1.0) / t33;
            let t35 = t30 * t34;
            let t38 = f64x8::splat(1.0) + f64x8::splat(10.874334072525) * t14 * t35;
            let t41 = f64x8::splat(1.0) / t13;
            let t42 = f64x8::splat(1.0) / param_fc;
            let t43 = t41 * t42;
            let t44 = f64x8::splat(1.0) / t26;
            let t45 = f64x8::splat(1.0) / t29;
            let t46 = t44 * t45;
            let t47 = t46 * t33;
            let t48 = t43 * t47;
            let t50 = f64x8::splat(1.0) + f64x8::splat(0.09195962397381102) * t48;
            let t51 = (simd::ln(t50));
            let t52 = t51 * t41;
            let t53 = t52 * t42;
            let t57 = t13 * t13;
            let t58 = f64x8::splat(1.0) / t57;
            let t59 = param_fc * param_fc;
            let t60 = f64x8::splat(1.0) / t59;
            let t61 = t58 * t60;
            let t62 = t26 * t26;
            let t63 = f64x8::splat(1.0) / t62;
            let t64 = t29 * t29;
            let t65 = f64x8::splat(1.0) / t64;
            let t66 = t63 * t65;
            let t67 = t33 * t33;
            let t68 = t66 * t67;
            let t71 = -f64x8::splat(2.763169) / t38 + f64x8::splat(0.28144540420067765) * t53 * t47 + f64x8::splat(0.2541000285260132) * t48 - f64x8::splat(0.049248579417833935) * t61 * t68;
            let t74 = ((t7).select(f64x8::splat(0.0), t12 * t71 / f64x8::splat(4.0)));
            let tzk0 = t1 * t74;
            acc_zk = tzk0;
            let t75 = f64x8::splat(2.0) * tzk0;
            let t76 = t2 * t10;
            let t77 = t9 * t1;
            let t78 = f64x8::splat(1.0) / t77;
            let t79 = t8 * t78;
            let t81 = -f64x8::splat(2.0) * t76 + f64x8::splat(2.0) * t79;
            let t83 = t38 * t38;
            let t84 = f64x8::splat(1.0) / t83;
            let t85 = t58 * param_fc;
            let t87 = f64x8::splat(3.624778024175) * t85 * t35;
            let t88 = t23 * param_q;
            let t90 = ((t16).select(f64x8::splat(0.0), (t19).select(f64x8::splat(0.0), t3 - t76)));
            let t91 = f64x8::splat(1.0) / t22;
            let t94 = t25 * param_q;
            let t95 = f64x8::splat(1.0) / t24;
            let t98 = t88 * t90 * t91 - t94 * t90 * t95;
            let t100 = t98 * t29 * t34;
            let t103 = t14 * t26;
            let t104 = t65 * t34;
            let t105 = t21 * t90;
            let t106 = t104 * t105;
            let t109 = f64x8::splat(1.0) / t67;
            let t110 = t29 * t109;
            let t111 = t31 * t31;
            let t112 = f64x8::splat(1.0) / t111;
            let t114 = t32 * t32;
            let t115 = f64x8::splat(1.0) / t114;
            let t118 = t112 * t90 / f64x8::splat(3.0) - t115 * t90 / f64x8::splat(3.0);
            let t119 = t110 * t118;
            let t122 = t87 + f64x8::splat(10.874334072525) * t14 * t100 - f64x8::splat(7.24955604835) * t103 * t106 - f64x8::splat(10.874334072525) * t103 * t119;
            let t126 = f64x8::splat(1.0) / t13 / t1;
            let t127 = t126 * t42;
            let t128 = t127 * t47;
            let t129 = f64x8::splat(0.03065320799127034) * t128;
            let t130 = t43 * t63;
            let t131 = t45 * t33;
            let t132 = t131 * t98;
            let t133 = t130 * t132;
            let t135 = t43 * t44;
            let t137 = f64x8::splat(1.0) / t29 / t28;
            let t138 = t137 * t33;
            let t139 = t138 * t105;
            let t140 = t135 * t139;
            let t142 = t46 * t118;
            let t143 = t43 * t142;
            let t145 = -t129 - f64x8::splat(0.09195962397381102) * t133 + f64x8::splat(0.06130641598254068) * t140 + f64x8::splat(0.09195962397381102) * t143;
            let t146 = f64x8::splat(1.0) / t50;
            let t147 = t145 * t146;
            let t148 = t147 * t41;
            let t149 = t42 * t44;
            let t150 = t149 * t131;
            let t153 = t51 * t126;
            let t154 = t153 * t42;
            let t156 = f64x8::splat(0.09381513473355922) * t154 * t47;
            let t157 = t63 * t45;
            let t158 = t33 * t98;
            let t159 = t157 * t158;
            let t162 = t52 * t149;
            let t167 = f64x8::splat(0.08470000950867107) * t128;
            let t172 = f64x8::splat(1.0) / t57 / t1;
            let t173 = t172 * t60;
            let t175 = f64x8::splat(0.032832386278555954) * t173 * t68;
            let t177 = f64x8::splat(1.0) / t62 / t26;
            let t178 = t61 * t177;
            let t179 = t65 * t67;
            let t180 = t179 * t98;
            let t183 = t61 * t63;
            let t185 = f64x8::splat(1.0) / t64 / t28;
            let t186 = t185 * t67;
            let t187 = t186 * t105;
            let t190 = t65 * t33;
            let t191 = t190 * t118;
            let t194 = f64x8::splat(2.763169) * t84 * t122 + f64x8::splat(0.28144540420067765) * t148 * t150 - t156 - f64x8::splat(0.28144540420067765) * t53 * t159 + f64x8::splat(0.18763026946711844) * t162 * t139 + f64x8::splat(0.28144540420067765) * t53 * t142 - t167 - f64x8::splat(0.2541000285260132) * t133 + f64x8::splat(0.16940001901734214) * t140 + f64x8::splat(0.2541000285260132) * t143 + t175 + f64x8::splat(0.09849715883566787) * t178 * t180 - f64x8::splat(0.06566477255711191) * t183 * t187 - f64x8::splat(0.09849715883566787) * t183 * t191;
            let t198 = ((t7).select(f64x8::splat(0.0), t12 * t194 / f64x8::splat(4.0) + t81 * t71 / f64x8::splat(4.0)));
            let tvrho0 = t9 * t198 + t75;
            acc_vrho_0 = tvrho0;
            let t201 = f64x8::splat(2.0) * t76 + f64x8::splat(2.0) * t79;
            let t204 = ((t16).select(f64x8::splat(0.0), (t19).select(f64x8::splat(0.0), -t3 - t76)));
            let t209 = t88 * t204 * t91 - t94 * t204 * t95;
            let t211 = t209 * t29 * t34;
            let t214 = t21 * t204;
            let t215 = t104 * t214;
            let t221 = t112 * t204 / f64x8::splat(3.0) - t115 * t204 / f64x8::splat(3.0);
            let t222 = t110 * t221;
            let t225 = t87 + f64x8::splat(10.874334072525) * t14 * t211 - f64x8::splat(7.24955604835) * t103 * t215 - f64x8::splat(10.874334072525) * t103 * t222;
            let t228 = t131 * t209;
            let t229 = t130 * t228;
            let t231 = t138 * t214;
            let t232 = t135 * t231;
            let t234 = t46 * t221;
            let t235 = t43 * t234;
            let t237 = -t129 - f64x8::splat(0.09195962397381102) * t229 + f64x8::splat(0.06130641598254068) * t232 + f64x8::splat(0.09195962397381102) * t235;
            let t238 = t237 * t146;
            let t239 = t238 * t41;
            let t242 = t33 * t209;
            let t243 = t157 * t242;
            let t253 = t179 * t209;
            let t256 = t186 * t214;
            let t259 = t190 * t221;
            let t262 = f64x8::splat(2.763169) * t84 * t225 + f64x8::splat(0.28144540420067765) * t239 * t150 - t156 - f64x8::splat(0.28144540420067765) * t53 * t243 + f64x8::splat(0.18763026946711844) * t162 * t231 + f64x8::splat(0.28144540420067765) * t53 * t234 - t167 - f64x8::splat(0.2541000285260132) * t229 + f64x8::splat(0.16940001901734214) * t232 + f64x8::splat(0.2541000285260132) * t235 + t175 + f64x8::splat(0.09849715883566787) * t178 * t253 - f64x8::splat(0.06566477255711191) * t183 * t256 - f64x8::splat(0.09849715883566787) * t183 * t259;
            let t266 = ((t7).select(f64x8::splat(0.0), t12 * t262 / f64x8::splat(4.0) + t201 * t71 / f64x8::splat(4.0)));
            let tvrho1 = t9 * t266 + t75;
            acc_vrho_1 = tvrho1;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        store_strided(vrho, ip, m, 2, 0, acc_vrho_0);
        store_strided(vrho, ip, m, 2, 1, acc_vrho_1);
        ip += 8;
    }
}
