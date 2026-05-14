//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 788/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk788<F: Float>(t319: F, t334: F, t4606: F, t5021: F, t8141: F, t8143: F, t8146: F, t8149: F, t8155: F, t8157: F, t8159: F, t8161: F, t13: F, t3130: F, t8185: F, t902: F, t911: F) -> (F, F) {
    let t8238 = 1.0 * t319 * (-2.109916666666667 * t8141 + 20.2552 * t8143 - 7.501925925925926 * t8146 + 6.564185185185186 * t8149 + 3.100395061728395 * t4606 + 0.06825833333333334 * t8155 - 1.0921333333333334 * t8157 + 1.2134814814814814 * t8159 + 1.0617962962962963 * t8161 + 1.3388493827160495 * t5021) * t334;
    let t8244 = 6207.00176468474 * t13 / t902 / t911 * t8185 * t3130;
    (t8238, t8244)
}
