//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 1004/1050 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk1004<F: Float>(t34258: F, t7003: F, t2014: F, t49575: F, t8599: F, t47672: F, t8598: F, t28196: F, t28198: F, t28056: F, t8634: F, t32129: F, t7898: F, t13426: F, t8461: F, t18227: F) -> (F, F, F, F, F, F, F) {
    let t127346 = 4.0 * t34258 * t7003;
    let t127349 = 2.0 * t2014 * t8599 * t49575;
    let t127354 = t8598 * t47672;
    let t127357 = 6.0 * t28196 * t127354 * t28198;
    let t127359 = 4.0 * t8634 * t28056;
    let t127361 = 2.0 * t7898 * t32129;
    let t127365 = t13426 * t8461;
    let t127366 = 2.0 * t127365;
    let t127368 = t18227 * t8461;
    (t127346, t127349, t127357, t127359, t127361, t127366, t127368)
}
