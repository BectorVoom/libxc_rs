//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 752/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk752(t1954: f64, t593: f64, t4841: f64, t571: f64, t1472: f64, t2014: f64, t2176: f64, t529: f64, t1976: f64, t542: f64, t519: f64, t4801: f64, t4803: f64, t4806: f64, t4809: f64, t4812: f64, t4815: f64, t4817: f64, t4822: f64, t4824: f64, t4828: f64, t4833: f64, t4836: f64, t4840: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t4842 = t1954 * t593;
    let t4843 = t4841 * t4842;
    let t4845 = 16.0_f64 / 45.0_f64 * t571 * t4843;
    let t4847 = 16.0_f64 / 45.0_f64 * t1472 * t2014;
    let t4848 = t2176 * t529;
    let t4849 = t1976 * t542;
    let t4850 = t4848 * t4849;
    let t4852 = 16.0_f64 / 45.0_f64 * t519 * t4850;
    let t4853 = -t4801 - t4803 + t4806 - t4809 - t4812 + t4815 + t4817 - t4822 + t4824 + t4828 + t4833 - t4836 - t4840 + t4845 - t4847 - t4852;
    (t4842, t4843, t4845, t4847, t4848, t4849, t4850, t4852, t4853)
}
