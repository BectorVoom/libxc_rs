//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 735/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk735<F: Float>(t1535: F, t9419: F, t6519: F, t9439: F, t9448: F, t10531: F, t1433: F, t1065: F, t883: F, t900: F, t1423: F, t6589: F) -> (F, F, F, F, F, F, F) {
    let t20687 = t1535 * t9419;
    let t20696 = t9439 * t6519;
    let t20700 = t9448 * t6519;
    let t20796 = t1433 * t10531;
    let t20883 = t883 * t1065;
    let t20884 = t900 * t20883;
    let t20967 = t1423 * t6589;
    (t20687, t20696, t20700, t20796, t20883, t20884, t20967)
}
