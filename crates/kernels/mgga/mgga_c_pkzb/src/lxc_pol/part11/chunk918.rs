//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 918/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk918<F: Float>(t1096: F, t3577: F, t1095: F, t9422: F, t1073: F, t3528: F, t5511: F, t2754: F, t3532: F, t10769: F, t5520: F, t7357: F, t9148: F, t665: F, t5547: F, t2765: F) -> (F, F, F, F, F, F, F, F, F) {
    let t10792 = t1096 * t3577;
    let t10795 = t9422 * t1095;
    let t10800 = t3528 * t1073;
    let t10801 = t5511 * t10800;
    let t10803 = t2754 * t3532;
    let t10806 = -t5520 + 4.0 / 3.0 * t7357 - t9148 + t10769;
    let t10807 = t665 * t10806;
    let t10812 = t5547 * t10800;
    let t10814 = t2765 * t3532;
    (t10792, t10795, t10800, t10801, t10803, t10806, t10807, t10812, t10814)
}
