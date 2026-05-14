//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1016/1093 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1016<F: Float>(t1513: F, t94975: F, t7706: F, t95293: F, t60224: F, t7342: F, t13272: F, t26178: F, t26205: F, t7709: F, t7702: F, t1923: F, t26204: F, t7719: F, t28150: F, t7348: F) -> (F, F, F, F, F, F, F, F) {
    let t101451 = t94975 * t1513;
    let t101783 = t95293 * t7706;
    let t101785 = t60224 * t7342;
    let t101788 = t13272 * t26178;
    let t101793 = t7709 * t26205;
    let t101907 = t7702 * t26205;
    let t101929 = t1923 * t26204 * t7719;
    let t101970 = t7348 * t28150;
    (t101451, t101783, t101785, t101788, t101793, t101907, t101929, t101970)
}
