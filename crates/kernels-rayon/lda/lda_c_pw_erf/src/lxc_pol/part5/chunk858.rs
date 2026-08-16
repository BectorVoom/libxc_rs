//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 858/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk858(t101: f64, t7976: f64, t125: f64, t1808: f64, t2208: f64, t2211: f64, t296: f64, t456: f64, t5682: f64, t5783: f64, t6089: f64, t6098: f64, t6136: f64, t6140: f64, t7400: f64, t777: f64, t7862: f64, t7878: f64, t7881: f64, t7887: f64, t7889: f64) -> (f64, f64) {
    let t7977 = t101 * t7976;
    let t7984 = (t7400 + t7862) * t125 + t7878 * t296 - 9.0_f64 * t5783 * t7881 - 0.16213771438917426_f64 * t6136 - 0.0008717022455366076_f64 * t6140 + t777 * t7887 + 18.0_f64 * t1808 * t7889 + t7977 * t456 - 0.03592270203076383_f64 * t5682 + 9.0_f64 * t6089 * t2208 + 9.0_f64 * t2211 * t6098;
    (t7977, t7984)
}
