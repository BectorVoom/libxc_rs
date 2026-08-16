//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1206/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1206(t1313: f64, t1995: f64, t2961: f64, t519: f64, t1446: f64, t5222: f64, t1245: f64, t2098: f64, t3402: f64, t940: f64, t14188: f64, t14191: f64, t14194: f64, t14197: f64, t14199: f64, t14203: f64, t14208: f64, t14210: f64, t14212: f64, t14216: f64) -> (f64, f64, f64, f64) {
    let t14220 = 4.0_f64 / 45.0_f64 * t519 * t1313 * t1995 * t2961;
    let t14222 = 4.0_f64 / 9.0_f64 * t1446 * t5222;
    let t14227 = 4.0_f64 / 9.0_f64 * t519 * t3402 * t2098 * t1245 * t940;
    let t14228 = -t14188 - t14191 - t14194 + t14197 + t14199 + t14203 + t14208 + t14210 - t14212 - t14216 - t14220 - t14222 - t14227;
    (t14220, t14222, t14227, t14228)
}
