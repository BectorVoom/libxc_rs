//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 686/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk686(t1877: f64, t6488: f64, t1672: f64, t1873: f64, t431: f64, t4993: f64, t68: f64, t434: f64) -> (f64, f64, f64) {
    let t6490 = 12.992782516386768_f64 * t1877 * t6488;
    let t6493 = t1873 * t1672;
    let t6500 = t4993 * t431 * t68;
    let t6501 = t6500 * t434;
    (t6490, t6493, t6501)
}
