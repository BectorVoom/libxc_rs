//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1126/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1126<F: Float>(t1470: F, t644: F, t6972: F, t8442: F, t640: F, t36: F, t606: F, t7714: F, t1493: F, t33612: F, t8621: F, t37: F) -> (F, F, F, F, F, F, F) {
    let t125260 = t1470 * t644;
    let t125265 = t8442 * t1470 * t6972;
    let t125268 = t1470 * t640;
    let t125274 = t8442 * t7714 * t36 * t606;
    let t125279 = t1493 * t36 * t606;
    let t125294 = t8621 * t33612 * t6972;
    let t125312 = t37 * t606;
    (t125260, t125265, t125268, t125274, t125279, t125294, t125312)
}
