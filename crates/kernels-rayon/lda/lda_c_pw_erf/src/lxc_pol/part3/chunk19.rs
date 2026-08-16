//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 19/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk19(t11: f64, t14: f64, t17: f64, t25: f64) -> (f64, f64, f64) {
    let t27 = 3.79785_f64 * t14 + 0.8969_f64 * t11 + 0.204775_f64 * t17 + 0.123235_f64 * t25;
    let t30 = 1.0_f64 + 16.081824322151103_f64 / t27;
    let t31 = f64::ln(t30);
    (t27, t30, t31)
}
