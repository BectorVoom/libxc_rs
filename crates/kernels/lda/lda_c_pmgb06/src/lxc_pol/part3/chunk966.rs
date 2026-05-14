//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 966/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk966<F: Float>(t13100: F, t493: F, t499: F, t224: F, t5431: F, t500: F, t1451: F, t5305: F, t13084: F, t13085: F, t13088: F, t13091: F, t13093: F, t13095: F, t13096: F, t13097: F, t13099: F) -> (F, F, F, F) {
    let t13103 = t493 * t13100 * t499 / 15.0;
    let t13104 = t5431 * t224;
    let t13106 = t13104 * t500 / 15.0;
    let t13108 = 2.0 / 15.0 * t5305 * t1451;
    let t13109 = t13084 - t13085 + t13088 - t13091 - t13093 - t13095 + t13096 - t13097 + t13099 + t13103 + t13106 + t13108;
    (t13103, t13106, t13108, t13109)
}
