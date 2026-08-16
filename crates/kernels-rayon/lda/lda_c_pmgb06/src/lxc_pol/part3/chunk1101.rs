//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1101/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1101(t13084: f64, t13085: f64, t13088: f64, t13091: f64, t13093: f64, t13095: f64, t13096: f64, t13097: f64, t13099: f64, t13103: f64, t13106: f64, t13108: f64) -> f64 {
    let t13109 = t13084 - t13085 + t13088 - t13091 - t13093 - t13095 + t13096 - t13097 + t13099 + t13103 + t13106 + t13108;
    t13109
}
