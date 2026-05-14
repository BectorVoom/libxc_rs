//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 893/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk893<F: Float>(t17561: F, t17573: F, t1565: F, t5458: F, t5469: F, t11671: F, t14885: F, t14887: F, t14889: F, t17389: F, t17392: F, t17406: F, t17409: F, t17419: F, t9311: F, t9312: F) -> (F, F, F, F) {
    let t17574 = t17561 + t17573;
    let t17582 = t5458 * t1565;
    let t17585 = t1565 * t5469;
    let t17597 = -0.96922222222222222223e3 * t11671 - 0.78666666666666666667e2 * t17419 - t9311 - t9312 - 0.14538333333333333333e4 * t14887 + 0.72691666666666666668e3 * t14889 + 0.48461111111111111112e3 * t14885 + 0.15733333333333333333e3 * t17406 - 0.78666666666666666666e2 * t17389 - 0.47199999999999999999e3 * t17409 + 0.47199999999999999999e3 * t17392;
    (t17574, t17582, t17585, t17597)
}
