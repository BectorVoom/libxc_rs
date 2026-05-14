//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1051/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1051<F: Float>(t2014: F, t49575: F, t8599: F, t47672: F, t8598: F, t28196: F, t28198: F, t28056: F, t8634: F, t32129: F, t7898: F, t2007: F, t28042: F, t651: F, t13426: F, t8461: F) -> (F, F, F, F, F, F) {
    let t127349 = 2.0 * t2014 * t8599 * t49575;
    let t127354 = t8598 * t47672;
    let t127357 = 6.0 * t28196 * t127354 * t28198;
    let t127359 = 4.0 * t8634 * t28056;
    let t127361 = 2.0 * t7898 * t32129;
    let t127363 = t651 * t2007 * t28042;
    let t127365 = t13426 * t8461;
    (t127349, t127357, t127359, t127361, t127363, t127365)
}
