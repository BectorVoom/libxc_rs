//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 713/1112 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk713<F: Float>(t1266: F, t359: F, t259: F, t2298: F, t363: F, t364: F, t358: F) -> (F, F, F) {
    let t6848 = t359 * t1266;
    let t6849 = t259 * t6848;
    let t6852 = t2298 * t363;
    let t6854 = 1.0 / t364 / t6852;
    let t6855 = t358 * t6854;
    (t6849, t6854, t6855)
}
