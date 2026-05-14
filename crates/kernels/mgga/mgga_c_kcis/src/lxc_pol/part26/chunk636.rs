//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 636/1243 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk636<F: Float>(t1592: F, t2110: F, t4399: F, t5764: F, t5766: F, t6193: F, t626: F, t7102: F, t7106: F, t7109: F, t7196: F, t7199: F, t7205: F, t7208: F, t7260: F, t7264: F, t7490: F, t7510: F) -> (F,) {
    let t7532 = 0.66725e-1 * t1592 * t7510 + 0.15476481481481481481e-2 * t5764 - 0.23214722222222222222e-2 * t5766 - t4399 - 0.23214722222222222222e-2 * t7102 + 0.17024129629629629629e-1 * t7106 - 0.92858888888888888886e-2 * t7109 + 0.17411041666666666666e-2 * t7196 + 0.23214722222222222222e-2 * t7199 + 0.11607361111111111111e-2 * t7205 - 0.34822083333333333332e-2 * t7208 - 0.17411041666666666666e-2 * t7260 - 0.13345e0 * t6193 * t2110 + t7490 * t626 - 0.61905925925925925925e-2 * t7264;
    (t7532,)
}
