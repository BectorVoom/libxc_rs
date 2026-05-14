//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 725/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk725<F: Float>(t1419: F, t458: F, t1416: F, t425: F, t1415: F, t405: F) -> (F, F, F, F, F) {
    let t4881 = t1419 * t458;
    let t4882 = 36.0 * t4881;
    let t4883 = t1416 * t425;
    let t4884 = 60.0 * t4883;
    let t4885 = t405 * t1415;
    (t4881, t4882, t4883, t4884, t4885)
}
