//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 438/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk438(t2093: f64, t529: f64, t166: f64, t161: f64, t486: f64, t853: f64, t1639: f64, t851: f64, t531: f64, t831: f64, t464: f64, t813: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t2094 = t2093 * t529;
    let t2095 = t166 * t2094;
    let t2097 = t161 * t2095 / 30.0_f64;
    let t2099 = t486 * t853 / 30.0_f64;
    let t2100 = t1639 * t851;
    let t2101 = t166 * t2100;
    let t2103 = t161 * t2101 / 30.0_f64;
    let t2105 = t831 * t531 / 30.0_f64;
    let t2106 = t813 * t464;
    (t2094, t2095, t2097, t2099, t2100, t2101, t2103, t2105, t2106)
}
