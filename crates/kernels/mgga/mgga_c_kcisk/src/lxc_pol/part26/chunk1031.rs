//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1031/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1031<F: Float>(t27115: F, t27168: F, t27218: F, t27429: F, t1459: F, t25294: F, t25299: F, t25304: F, t25306: F, t25310: F, t25316: F, t25319: F, t25322: F, t25325: F, t25327: F, t25331: F, t25335: F, t25340: F, t25345: F, t25348: F, t25353: F, t25356: F, t25359: F) -> (F, F, F) {
    let t27431 = t27115 + t27168 + t27218 + t27429;
    let t27432 = t1459 * t27431;
    let t27451 = -0.23214722222222222222e-2 * t25294 + 0.23214722222222222221e-2 * t25299 + 0.69644166666666666664e-2 * t25304 + 0.77382407407407407407e-3 * t25306 + 0.11607361111111111111e-2 * t25310 + 0.11607361111111111111e-2 * t25316 + 0.19345601851851851852e-2 * t25319 + 0.46429444444444444444e-2 * t25322 + 0.11607361111111111111e-2 * t25325 - 0.15476481481481481481e-2 * t25327 + 0.46429444444444444443e-2 * t25331 + 0.61905925925925925925e-2 * t25335 - 0.15476481481481481481e-2 * t25340 + 0.69644166666666666666e-2 * t25345 + 0.11607361111111111111e-2 * t25348 - 0.17411041666666666666e-2 * t25353 - 0.38691203703703703703e-3 * t25356 + 0.23214722222222222222e-2 * t25359;
    (t27431, t27432, t27451)
}
