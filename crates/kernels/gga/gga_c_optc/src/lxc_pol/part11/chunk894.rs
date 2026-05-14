//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 894/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk894<F: Float>(t11677: F, t14881: F, t14883: F, t14895: F, t17338: F, t17342: F, t17346: F, t17350: F, t17354: F, t17358: F, t17412: F, t17597: F, t11700: F, t1200: F, t1565: F, t16135: F, t17574: F, t17582: F, t17585: F, t2886: F, t4249: F, t485: F, t5458: F, t5469: F, t9304: F) -> (F, F) {
    let t17609 = -0.80768518518518518518e3 * t17338 - 0.72691666666666666667e3 * t17358 + 0.43614999999999999999e4 * t17354 + 0.29076666666666666666e4 * t17342 - 0.14538333333333333333e4 * t17346 - 0.43614999999999999999e4 * t17350 - 0.34962962962962962963e2 * t17412 - 0.26222222222222222223e3 * t11677 + 0.52444444444444444444e2 * t14895 - 0.31466666666666666667e3 * t14881 + 0.15733333333333333334e3 * t14883;
    let t17610 = t17597 + t17609;
    let t17612 = 6.0 * t11700 * t5458 - t1200 * t17610 - 3.0 * t16135 * t1565 + t17574 * t485 - 6.0 * t9304 * t17582 + 6.0 * t2886 * t17585 - 3.0 * t4249 * t5469;
    (t17610, t17612)
}
