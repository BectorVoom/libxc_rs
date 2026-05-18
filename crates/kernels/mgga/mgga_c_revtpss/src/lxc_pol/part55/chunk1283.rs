//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1283/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1283<F: Float>(t116: F, t34775: F, t670: F, t8885: F, t125384: F, t125386: F, t125388: F, t125390: F, t129467: F, t129470: F, t2055: F, t27060: F, t28683: F, t29427: F, t29432: F, t34446: F, t7373: F, t7586: F, t7983: F) -> (F, F, F) {
    let t130929 = t34775 * t116;
    let t130932 = t8885 * t670;
    let t130946 = t129467 * t2055 + t129470 * t2055 + t27060 * t7983 + t28683 * t7586 + t29427 * t7373 + t29432 * t7983 + t34446 * t7373 + t125384 + t125386 + t125388 + t125390;
    (t130929, t130932, t130946)
}
