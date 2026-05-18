//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 886/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk886<F: Float>(t236: F, t3351: F, t618: F, t7248: F, t833: F, t1614: F, t1971: F, t495: F, t511: F, t7230: F, t2333: F, t34957: F) -> (F, F, F) {
    let t39355 = t3351 * t7248 * t236 * t618 * t833;
    let t39360 = t7230 * t1971 * t511 * t1614 * t495;
    let t39362 = t34957 * t2333;
    (t39355, t39360, t39362)
}
