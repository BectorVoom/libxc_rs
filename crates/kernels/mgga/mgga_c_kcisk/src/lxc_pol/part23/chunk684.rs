//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 684/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk684<F: Float>(t4011: F, t4049: F, t4061: F, t4063: F, t6020: F, t6023: F, t6026: F, t6030: F, t6044: F, t6052: F, t6060: F, t6062: F, t6066: F, t6069: F, t6072: F, t6076: F) -> (F,) {
    let t6078 = -0.9494625e0 * t6044 + 0.1898925e1 * t6052 + t4049 + 0.99655555555555555557e-1 * t4011 + 0.99655555555555555557e-1 * t6020 - 0.19931111111111111111e0 * t6023 + 0.59793333333333333334e0 * t6026 - 0.59793333333333333334e0 * t6030 + 0.15358125e0 * t6060 + 0.3071625e0 * t6062 + t4061 + 0.10954222222222222222e0 * t4063 + 0.10954222222222222222e0 * t6066 - 0.5477111111111111111e-1 * t6069 + 0.32862666666666666666e0 * t6072 - 0.32862666666666666666e0 * t6076;
    (t6078,)
}
