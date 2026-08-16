//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1281/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1281(t22922: f64, t3974: f64, t4522: f64, t593: f64, t14089: f64, t12765: f64, t1325: f64, t2471: f64, t542: f64, t784: f64, t2098: f64, t5289: f64, t6431: f64) -> (f64, f64, f64, f64) {
    let t22944 = 8.0_f64 / 9.0_f64 * t3974 * t4522 * t22922 * t593;
    let t22945 = 16.0_f64 / 135.0_f64 * t14089;
    let t22950 = 24.0_f64 / 5.0_f64 * t1325 * t12765 * t2471 * t784 * t542;
    let t22954 = 16.0_f64 / 5.0_f64 * t1325 * t5289 * t6431 * t2098;
    (t22944, t22945, t22950, t22954)
}
