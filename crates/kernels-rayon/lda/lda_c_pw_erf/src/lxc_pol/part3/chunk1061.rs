//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1061/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1061(t3828: f64, t4506: f64, t4508: f64, t1401: f64, t1484: f64, t3833: f64, t833: f64, t3837: f64, t3974: f64, t5151: f64, t12373: f64, t4488: f64, t4494: f64) -> (f64, f64, f64, f64, f64) {
    let t12427 = 8.0_f64 / 15.0_f64 * t4506 * t4508 * t3828;
    let t12428 = t1484 * t1401;
    let t12432 = 8.0_f64 / 9.0_f64 * t4506 * t12428 * t833 * t3833;
    let t12435 = 16.0_f64 / 15.0_f64 * t3974 * t5151 * t3837;
    let t12438 = 8.0_f64 / 15.0_f64 * t4488 * t4494 * t12373;
    (t12427, t12428, t12432, t12435, t12438)
}
