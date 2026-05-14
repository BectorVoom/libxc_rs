//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 714/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk714<F: Float>(t7585: F, t8665: F, t1562: F, t7561: F, t1466: F, t7822: F, t1470: F, t2274: F, t7315: F, t2016: F, t2278: F, t500: F, t7329: F, t1462: F, t2001: F, t1089: F, t2080: F, t535: F) -> (F, F, F, F, F, F, F, F, F) {
    let t8666 = t7585 * t8665;
    let t8668 = t7561 * t1562;
    let t8670 = t7822 * t1466;
    let t8672 = t7822 * t1470;
    let t8680 = t7315 * t2274;
    let t8682 = t2016 * t2278;
    let t8684 = t7329 * t500;
    let t8686 = t2001 * t1462;
    let t8689 = t1089 * t535 * t2080;
    (t8666, t8668, t8670, t8672, t8680, t8682, t8684, t8686, t8689)
}
