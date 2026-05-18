//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 707/1111 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk707<F: Float>(t10082: F, t236: F, t7248: F, t3351: F, t558: F, t618: F) -> (F, F, F) {
    let t10083 = t236 * t10082;
    let t10084 = t7248 * t10083;
    let t10085 = t3351 * t10084;
    let t10086 = F::new(0.25538759935978703638e-4) * t10085;
    let t10088 = t618 * t558;
    (t10084, t10086, t10088)
}
