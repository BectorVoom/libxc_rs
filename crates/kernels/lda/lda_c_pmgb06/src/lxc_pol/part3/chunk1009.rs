//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1009/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1009<F: Float>(t10343: F, t10346: F, t10348: F, t10350: F, t10353: F, t10356: F, t10358: F, t10362: F, t13822: F, t13823: F, t13824: F, t13829: F, t10403: F, t10416: F, t1447: F, t5451: F) -> (F, F, F, F) {
    let t13830 = t13822 - t13823 + t13824 + t10343 + 0.36466666666666664 * t10346 - 2.0 / 9.0 * t10348 - 2.0 / 3.0 * t10350 - 0.040518518518518516 * t10353 - t10356 - t10358 + t10362 + t13829;
    let t13834 = 4.0 / 45.0 * t10403;
    let t13835 = 4.0 / 45.0 * t10416;
    let t13836 = t1447 * t5451;
    (t13830, t13834, t13835, t13836)
}
