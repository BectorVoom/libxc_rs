//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 962/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk962(t4489: f64, t784: f64, t34: f64, t3966: f64, t4507: f64, t811: f64, t2104: f64, t4571: f64, t10557: f64, t197: f64, t2070: f64, t493: f64, t785: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t12956 = t4489 * t784;
    let t12963 = t3966 * t34;
    let t12968 = t4507 * t811;
    let t12974 = t2104 * t4571;
    let t12975 = 8.0_f64 / 45.0_f64 * t12974;
    let t12976 = t10557 * t197;
    let t12984 = t493 * t2070 * t785;
    (t12956, t12963, t12968, t12975, t12976, t12984)
}
