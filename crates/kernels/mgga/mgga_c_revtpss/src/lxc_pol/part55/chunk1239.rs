//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1239/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1239<F: Float>(t26399: F, t7741: F, t28658: F, t28042: F, t7359: F, t1936: F, t4245: F, t2055: F, t1501: F, t7002: F, t34258: F, t7373: F) -> (F, F, F, F, F, F, F, F) {
    let t128339 = t26399 * t7741;
    let t128340 = t28658 * t7741;
    let t128341 = t7359 * t28042;
    let t128353 = t4245 * t1936;
    let t128354 = t128353 * t2055;
    let t128355 = t1501 * t7002;
    let t128356 = t128355 * t2055;
    let t128357 = t34258 * t7373;
    (t128339, t128340, t128341, t128353, t128354, t128355, t128356, t128357)
}
