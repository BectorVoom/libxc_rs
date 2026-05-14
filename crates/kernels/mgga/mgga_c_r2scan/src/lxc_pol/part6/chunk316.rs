//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 316/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk316<F: Float>(t363: F, t364: F, t358: F, t255: F, t513: F, t550: F, t862: F, t864: F) -> (F, F, F) {
    let t868 = 1.0 / t364 / t363;
    let t869 = t358 * t868;
    let t870 = t869 * t255;
    let t874 = -1.0 * t862 * t864 - 0.14225094736250905555e-1 * t870 * t550 * t513;
    (t868, t870, t874)
}
