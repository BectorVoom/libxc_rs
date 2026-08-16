//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1358/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1358(t13948: f64, t13950: f64, t13971: f64, t17842: f64, t17843: f64, t17844: f64, t17845: f64, t17846: f64, t17847: f64, t17848: f64, t17849: f64, t17850: f64, t17851: f64, t17852: f64, t17853: f64) -> (f64, f64, f64, f64) {
    let t17854 = 8.0_f64 / 405.0_f64 * t13948;
    let t17855 = 8.0_f64 / 135.0_f64 * t13950;
    let t17856 = 4.0_f64 / 45.0_f64 * t13971;
    let t17857 = t17842 - t17843 - t17844 - t17845 - t17846 + t17847 - t17848 - t17849 + t17850 - t17851 - t17852 + t17853 + t17854 + t17855 - t17856;
    (t17854, t17855, t17856, t17857)
}
