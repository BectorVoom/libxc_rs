//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 813/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk813<F: Float>(t12692: F, t12813: F, t1390: F, t313: F, t1336: F, t140: F, t3531: F, t441: F) -> (F, F, F, F) {
    let t12814 = t12692 + t12813;
    let t12825 = 1.0 / t313 / t1390;
    let t12827 = t140 * t1336 * t12825;
    let t12829 = 1.0 / t3531 / t441;
    (t12814, t12825, t12827, t12829)
}
