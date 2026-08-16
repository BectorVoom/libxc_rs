//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1236/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1236(t13115: f64, t6446: f64, t6748: f64, t13035: f64, t7749: f64, t20711: f64, t593: f64, t13122: f64, t4506: f64, t13966: f64, t20712: f64, t13812: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t22237 = 64.0_f64 / 15.0_f64 * t13115 * t6748 * t6446;
    let t22239 = 16.0_f64 / 15.0_f64 * t13035 * t7749;
    let t22240 = t20711 * t593;
    let t22243 = 16.0_f64 / 15.0_f64 * t4506 * t13122 * t22240;
    let t22246 = 8.0_f64 / 5.0_f64 * t4506 * t13966 * t20712;
    let t22249 = 8.0_f64 / 3.0_f64 * t4506 * t13812 * t20712;
    (t22237, t22239, t22240, t22243, t22246, t22249)
}
