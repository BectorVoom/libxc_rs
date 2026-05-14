//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 896/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk896<F: Float>(t227: F, t3288: F, t967: F, t10447: F, t7409: F, sigma2: F) -> (F, F, F, F) {
    let t15783 = 1.0 / t3288 / t227;
    let t15821 = 2.0 * t967;
    let t15822 = 6.0 * t10447;
    let t15851 = t7409 * sigma2;
    (t15783, t15821, t15822, t15851)
}
