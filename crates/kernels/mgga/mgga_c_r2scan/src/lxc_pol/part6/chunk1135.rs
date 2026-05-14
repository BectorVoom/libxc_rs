//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1135/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1135<F: Float>(t6493: F, t6500: F, t2169: F, t6504: F, t2183: F, t6474: F, t2185: F, t6212: F, t6211: F, t13866: F, t1591: F, t2202: F, t6064: F, t1592: F, t1593: F, t551: F, t6343: F) -> (F, F, F, F, F, F, F) {
    let t20585 = t6493 * t6500;
    let t20587 = t2169 * t6504;
    let t20589 = t2183 * t6474;
    let t20590 = t6212 * t2185;
    let t20592 = t20589 * t6211 * t20590;
    let t20594 = t1591 * t13866;
    let t20596 = t20594 * t2202 * t6064;
    let t20607 = t1592 * t551 * t6343 * t1593;
    (t20585, t20587, t20589, t20592, t20594, t20596, t20607)
}
