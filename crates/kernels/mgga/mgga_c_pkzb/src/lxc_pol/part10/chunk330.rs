//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 330/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk330<F: Float>(t1073: F, t665: F, t672: F, t1066: F, t208: F, t218: F, t219: F, t1068: F, t670: F, t678: F) -> (F, F, F, F, F) {
    let t1074 = t665 * t1073;
    let t1077 = t672 * t1073;
    let t1079 = t208 * t1066;
    let t1081 = t218 * t219 * t1079;
    let t1083 = 0.1898925e1 * t1074 - t670 + 0.8969e0 * t1068 + 0.3071625e0 * t1077 - t678 + 0.24647e0 * t1081;
    (t1074, t1077, t1079, t1081, t1083)
}
