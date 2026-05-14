//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 581/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk581<F: Float>(t1908: F, t5339: F, t4808: F, t1990: F, t1993: F, t2030: F, t4583: F, t4800: F, t4806: F, t4812: F, t4814: F, t4819: F, t5050: F, t5057: F, t5066: F, t5071: F) -> (F, F, F) {
    let t5340 = t1908 * t5339;
    let t5344 = 0.38691203703703703703e-3 * t4808;
    let t5348 = t1990 * t1993;
    let t5355 = 0.23214722222222222222e-2 * t4583 - 0.17411041666666666666e-2 * t4800 + 0.11607361111111111111e-2 * t4806 - t5344 - 0.23214722222222222222e-2 * t4812 + 0.15476481481481481481e-2 * t4814 - 0.34822083333333333332e-2 * t4819 - 0.386e0 * t5348 * t2030 + 0.11607361111111111111e-2 * t5050 + 0.19345601851851851852e-2 * t5057 + 0.34822083333333333332e-2 * t5066 - 0.23214722222222222222e-2 * t5071;
    (t5340, t5348, t5355)
}
