//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 875/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk875(t1037: f64, t1067: f64, t325: f64, t333: f64, t903: f64, t907: f64, t935: f64, t912: f64, t936: f64, t1039: f64, t1070: f64, t38: f64) -> (f64, f64, f64, f64, f64) {
    let t8499 = t1067 * t1037;
    let t8505 = 3.436685857643691_f64 * t325 * t903 * t935 * t907 * t333;
    let t8509 = 0.4274_f64 * t325 * t912 * t333 * t936;
    let t8510 = t1070 * t1039;
    let t8512 = t38 * t38;
    (t8499, t8505, t8509, t8510, t8512)
}
