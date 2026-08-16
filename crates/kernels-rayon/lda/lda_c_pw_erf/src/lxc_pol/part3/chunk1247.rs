//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1247/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1247(t156: f64, t426: f64, t5610: f64, t14650: f64, t5592: f64, t1840: f64, t474: f64, t5599: f64, t5603: f64, t431: f64, t5578: f64, t5594: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t14807 = t426 * t156 * t5610;
    let t14813 = t5592 * t14650;
    let t14814 = 11.75232_f64 * t14813;
    let t14816 = t426 * t474 * t1840;
    let t14817 = 2.0_f64 * t14816;
    let t14819 = t426 * t156 * t5599;
    let t14822 = t426 * t156 * t5603;
    let t14837 = t431 * t5578 * t5594;
    (t14807, t14814, t14817, t14819, t14822, t14837)
}
