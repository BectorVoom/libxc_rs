//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 721/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk721<F: Float>(t3140: F, t7690: F, t131: F, t1310: F, t14148: F, t25987: F, t7351: F, t12200: F, t2044: F, t321: F, t7554: F, t212: F, t28: F, t3144: F, t4071: F, t672: F) -> (F, F, F, F) {
    let t70279 = t7690 * t3140;
    let t70316 = t14148 * t7351 * t131 * t1310 * t25987;
    let t70320 = t12200 * t2044 * t7554 * t321;
    let t70328 = t672 * t212 * t4071 * t28 * t3144;
    (t70279, t70316, t70320, t70328)
}
