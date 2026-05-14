//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 956/1050 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk956<F: Float>(t28167: F, t37956: F, t5627: F, t27833: F, t8596: F, t1353: F, t7933: F, t25082: F, t8717: F, t1907: F, t7311: F, t28196: F, t28197: F, t120967: F, t1399: F, t1868: F, t247: F, t561: F) -> (F, F, F, F, F) {
    let t125536 = 6.0 * t28167 * t37956 * t5627;
    let t125558 = t27833 * t8596;
    let t125559 = t7933 * t1353;
    let t125562 = 6.0 * t25082 * t8717 * t125559;
    let t125563 = t1907 * t7311;
    let t125566 = 4.0 * t28196 * t28197 * t125563;
    let t125570 = t120967 * t247 * t561 * t1868 * t1399;
    (t125536, t125558, t125562, t125566, t125570)
}
