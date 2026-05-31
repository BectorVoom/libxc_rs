//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 858/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk858<F: Float>(t101: F, t7976: F, t125: F, t1808: F, t2208: F, t2211: F, t296: F, t456: F, t5682: F, t5783: F, t6089: F, t6098: F, t6136: F, t6140: F, t7400: F, t777: F, t7862: F, t7878: F, t7881: F, t7887: F, t7889: F) -> (F, F) {
    let t7977 = t101 * t7976;
    let t7984 = (t7400 + t7862) * t125 + t7878 * t296 - F::cast_from(9.0_f64) * t5783 * t7881 - F::cast_from(0.16213771438917426_f64) * t6136 - F::cast_from(0.0008717022455366076_f64) * t6140 + t777 * t7887 + F::cast_from(18.0_f64) * t1808 * t7889 + t7977 * t456 - F::cast_from(0.03592270203076383_f64) * t5682 + F::cast_from(9.0_f64) * t6089 * t2208 + F::cast_from(9.0_f64) * t2211 * t6098;
    (t7977, t7984)
}
