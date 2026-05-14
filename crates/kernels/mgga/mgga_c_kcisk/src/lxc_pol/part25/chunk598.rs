//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 598/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk598<F: Float>(t2030: F, t4583: F, t4800: F, t4806: F, t4812: F, t4814: F, t4819: F, t5050: F, t5057: F, t5066: F, t5071: F, t5344: F, t5348: F, t4636: F, t4638: F, t4642: F, t4646: F, t4650: F) -> (F, F, F) {
    let t5355 = 0.23214722222222222222e-2 * t4583 - 0.17411041666666666666e-2 * t4800 + 0.11607361111111111111e-2 * t4806 - t5344 - 0.23214722222222222222e-2 * t4812 + 0.15476481481481481481e-2 * t4814 - 0.34822083333333333332e-2 * t4819 - 0.386e0 * t5348 * t2030 + 0.11607361111111111111e-2 * t5050 + 0.19345601851851851852e-2 * t5057 + 0.34822083333333333332e-2 * t5066 - 0.23214722222222222222e-2 * t5071;
    let t5360 = 0.22831111111111111111e-1 * t4636;
    let t5365 = t5360 + 0.11415555555555555555e-1 * t4638 - 0.11415555555555555555e-1 * t4642 + 0.34246666666666666666e-1 * t4646 - 0.17123333333333333333e-1 * t4650;
    (t5355, t5360, t5365)
}
