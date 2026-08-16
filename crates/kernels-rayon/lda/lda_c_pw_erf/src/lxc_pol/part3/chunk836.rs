//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 836/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk836(t4793: f64, t4797: f64, t4801: f64, t4803: f64, t4806: f64, t4809: f64, t4812: f64, t4815: f64, t4817: f64, t4822: f64, t4824: f64, t4828: f64, t4833: f64, t4836: f64, t4840: f64, t4845: f64, t4847: f64) -> f64 {
    let t5851 = -t4793 + t4797 - t4801 - t4803 + t4806 - t4809 - t4812 + t4815 + t4817 - t4822 + t4824 + t4828 + t4833 - t4836 - t4840 + t4845 - t4847;
    t5851
}
