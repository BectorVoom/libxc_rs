//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 777/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk777(t1321: f64, t5151: f64, t3974: f64, t549: f64, t743: f64) -> (f64, f64, f64) {
    let t5152 = t5151 * t1321;
    let t5154 = 16.0_f64 / 45.0_f64 * t3974 * t5152;
    let t5155 = t743 * t549;
    (t5152, t5154, t5155)
}
