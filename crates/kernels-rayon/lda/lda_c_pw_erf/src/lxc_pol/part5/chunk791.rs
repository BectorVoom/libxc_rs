//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 791/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk791(t4215: f64, t4217: f64, t4220: f64, t4227: f64, t4235: f64, t5236: f64, t5240: f64, t5284: f64, t5304: f64, t6862: f64, t6864: f64, t6869: f64, t6871: f64, t6873: f64, t6877: f64, t6879: f64) -> f64 {
    let t7275 = t6862 + t6864 + t6869 - t6871 - t6873 - t6877 + t6879 + t4215 + t4217 + 8.0_f64 / 3.0_f64 * t4220 + 4.0_f64 / 3.0_f64 * t4227 + t4235 - t5236 + t5240 + t5284 - t5304;
    t7275
}
