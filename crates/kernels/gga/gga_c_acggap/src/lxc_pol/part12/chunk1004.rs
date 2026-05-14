//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1004/1092 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1004<F: Float>(t11883: F, t642: F, t10761: F, t1679: F, t560: F, t32262: F, t495: F, t694: F, t9455: F, t9449: F, t96: F, t1674: F, t9108: F, t104: F, t9447: F, t1427: F, t1954: F, t2166: F, t24589: F, t32283: F, t32298: F, t32301: F, t36611: F, t567: F, t7297: F, t8040: F, t8372: F, t9096: F, t9448: F) -> (F,) {
    let t36729 = t642 * t11883;
    let t36744 = 2.0 * t1679 * t10761 * t560;
    let t36747 = 6.0 * t694 * t32262 * t495;
    let t36750 = 6.0 * t694 * t9455;
    let t36753 = 2.0 * t96 * t9449;
    let t36755 = 12.0 * t1674 * t9108;
    let t36756 = t104 * t9447;
    let t36760 = 12.0 * t1427 * t32262 * t8372 + 6.0 * t1954 * t36756 * t567 - 2.0 * t2166 * t567 * t9448 - 6.0 * t24589 * t7297 * t8040 - 6.0 * t36611 * t36729 * t9096 - 2.0 * t32283 + 3.0 * t32298 + 6.0 * t32301 - t36744 + t36747 - t36750 + t36753 + t36755;
    (t36760,)
}
