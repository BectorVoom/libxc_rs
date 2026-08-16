//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1356/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1356(t13840: f64, t13842: f64, t13844: f64, t13846: f64, t13848: f64, t13850: f64, t13883: f64, t13885: f64, t13887: f64, t17819: f64, t17822: f64, t17824: f64, t17828: f64, t17829: f64, t17830: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t17831 = 32.0_f64 / 135.0_f64 * t13840;
    let t17832 = 8.0_f64 / 81.0_f64 * t13842;
    let t17833 = 8.0_f64 / 81.0_f64 * t13844;
    let t17834 = 4.0_f64 / 81.0_f64 * t13846;
    let t17835 = 32.0_f64 / 243.0_f64 * t13848;
    let t17836 = 16.0_f64 / 81.0_f64 * t13850;
    let t17837 = 8.0_f64 / 135.0_f64 * t13883;
    let t17838 = 8.0_f64 / 135.0_f64 * t13885;
    let t17839 = 4.0_f64 / 135.0_f64 * t13887;
    let t17840 = -t17819 + t17822 - t17824 - t17828 - t17829 - t17830 + t17831 + t17832 + t17833 + t17834 + t17835 - t17836 - t17837 - t17838 - t17839;
    (t17831, t17832, t17833, t17834, t17835, t17836, t17837, t17838, t17839, t17840)
}
