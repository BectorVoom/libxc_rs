//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 749/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk749(t2007: f64, t4804: f64, t1319: f64, t4693: f64, t571: f64, t2017: f64, t4671: f64, t4689: f64, t4758: f64, t1472: f64, t2018: f64, t1351: f64, t833: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t4806 = 16.0_f64 / 45.0_f64 * t4804 * t2007;
    let t4807 = t1319 * t4693;
    let t4809 = 8.0_f64 / 45.0_f64 * t571 * t4807;
    let t4810 = t2017 * t4671;
    let t4812 = 8.0_f64 / 9.0_f64 * t571 * t4810;
    let t4813 = t4758 * t4689;
    let t4815 = 32.0_f64 / 45.0_f64 * t571 * t4813;
    let t4817 = 8.0_f64 / 27.0_f64 * t1472 * t2018;
    let t4818 = t833 * t1351;
    (t4806, t4807, t4809, t4810, t4812, t4813, t4815, t4817, t4818)
}
