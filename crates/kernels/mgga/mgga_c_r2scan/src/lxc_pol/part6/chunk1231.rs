//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1231/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1231<F: Float>(t1604: F, t20531: F, t1616: F, t2201: F, t2252: F, t785: F, t1234: F, t2207: F, t110: F, t146: F, t252: F, t6359: F, t20373: F, t6363: F, t2148: F, t20572: F) -> (F, F, F, F, F, F) {
    let t22805 = t1604 * t20531;
    let t22809 = t2201 * t785 * t1616 * t2252;
    let t22813 = t2207 * t785 * t1616 * t1234;
    let t22820 = t146 * t110 * t6359 * t252;
    let t22821 = t20373 * t6363;
    let t22823 = t22820 * t2148 * t22821;
    let t22825 = t1604 * t20572;
    (t22805, t22809, t22813, t22820, t22823, t22825)
}
