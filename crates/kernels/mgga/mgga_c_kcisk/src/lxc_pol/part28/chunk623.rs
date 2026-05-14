//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 623/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk623<F: Float>(t1248: F, t1720: F, t6764: F, t3117: F, t657: F, t6714: F, t4838: F, t4876: F, t4888: F, t4891: F, t7076: F, t7079: F, t7082: F, t7086: F, t7100: F, t7108: F, t7116: F, t7118: F, t7122: F, t7125: F) -> (F, F, F, F) {
    let t7128 = t1248 * t1720 * t6764;
    let t7130 = t3117 * t657;
    let t7132 = t1248 * t7130 * t6714;
    let t7134 = -0.9494625e0 * t7100 + 0.1898925e1 * t7108 + t4876 + 0.99655555555555555557e-1 * t4838 + 0.99655555555555555557e-1 * t7076 - 0.19931111111111111111e0 * t7079 + 0.59793333333333333334e0 * t7082 + 0.59793333333333333334e0 * t7086 + 0.15358125e0 * t7116 + 0.3071625e0 * t7118 + t4888 + 0.10954222222222222222e0 * t4891 + 0.10954222222222222222e0 * t7122 - 0.5477111111111111111e-1 * t7125 + 0.32862666666666666666e0 * t7128 + 0.32862666666666666666e0 * t7132;
    (t7128, t7130, t7132, t7134)
}
