//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1152/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1152(t3402: f64, t519: f64, t542: f64, t7639: f64, t1446: f64, t7692: f64, t1313: f64, t6557: f64, t806: f64, t2098: f64, t2437: f64, t7695: f64) -> (f64, f64, f64, f64, f64) {
    let t21173 = 8.0_f64 / 9.0_f64 * t519 * t3402 * t7639 * t542;
    let t21175 = 4.0_f64 / 15.0_f64 * t1446 * t7692;
    let t21179 = 4.0_f64 / 15.0_f64 * t519 * t1313 * t6557 * t806;
    let t21183 = 4.0_f64 / 15.0_f64 * t519 * t1313 * t2437 * t2098;
    let t21185 = 8.0_f64 / 15.0_f64 * t1446 * t7695;
    (t21173, t21175, t21179, t21183, t21185)
}
