//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 1005/1056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk1005<F: Float>(t31608: F, t6883: F, t1377: F, t7213: F, t1307: F, t22633: F, t22635: F, t1992: F, t31558: F, t3911: F, t22716: F, t8622: F) -> (F, F, F, F) {
    let t115294 = t6883 * t31608;
    let t115296 = t1377 * t7213;
    let t115299 = t22633 * t22635 * t115296 * t1307;
    let t115303 = t1992 * t22635 * t31558 * t3911;
    let t115305 = t22716 * t8622;
    (t115294, t115299, t115303, t115305)
}
