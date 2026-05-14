//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 700/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk700<F: Float>(t5080: F, t5344: F, t6664: F, t6670: F, t6678: F, t6682: F, t6687: F, t6692: F, t6695: F, t6700: F, t6705: F, t6710: F, t6717: F, t6721: F, t6725: F, t6947: F, t6949: F, t6951: F, t7528: F, t795: F) -> (F,) {
    let t7534 = 0.77382407407407407407e-3 * t6664 - 0.23214722222222222222e-2 * t6670 + 0.19345601851851851852e-2 * t6678 - 0.11607361111111111111e-2 * t6682 + 0.34822083333333333332e-2 * t6687 - 0.11607361111111111111e-2 * t6692 + 0.11607361111111111111e-2 * t6695 - 0.30952962962962962962e-2 * t6700 - 0.11607361111111111111e-2 * t6705 - 0.38691203703703703703e-3 * t6710 - 0.23214722222222222222e-2 * t6717 + 0.11607361111111111111e-2 * t6721 - t5344 + 0.11607361111111111111e-2 * t5080 + t7528 * t795 - 0.46429444444444444443e-2 * t6725 + 0.17411041666666666666e-2 * t6947 + 0.77382407407407407407e-3 * t6949 - 0.11607361111111111111e-2 * t6951;
    (t7534,)
}
