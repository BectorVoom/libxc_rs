//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 721/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk721<F: Float>(t1302: F, t14148: F, t14149: F, t20: F, t7351: F, t3140: F, t7690: F, t131: F, t1310: F, t25987: F, t12200: F, t2044: F, t321: F, t7554: F) -> (F, F, F, F) {
    let t70271 = t14148 * t7351 * t14149 * t1302 * t20;
    let t70279 = t7690 * t3140;
    let t70316 = t14148 * t7351 * t131 * t1310 * t25987;
    let t70320 = t12200 * t2044 * t7554 * t321;
    (t70271, t70279, t70316, t70320)
}
