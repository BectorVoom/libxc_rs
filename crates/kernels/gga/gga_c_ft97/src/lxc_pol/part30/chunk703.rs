//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 703/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk703<F: Float>(t10703: F, t29215: F, t15299: F, t28516: F, t4260: F, t6334: F, t15229: F, t28520: F, t15290: F, t28524: F, t1882: F, t7042: F) -> (F, F, F, F, F, F, F) {
    let t29216 = t10703 * t29215;
    let t29219 = t15299 * t28516;
    let t29222 = t6334 * t4260;
    let t29223 = t10703 * t29222;
    let t29226 = t15229 * t28520;
    let t29229 = t15290 * t28524;
    let t29232 = t1882 * t7042;
    (t29216, t29219, t29222, t29223, t29226, t29229, t29232)
}
