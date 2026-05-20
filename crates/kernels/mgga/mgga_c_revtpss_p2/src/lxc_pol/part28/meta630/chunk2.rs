//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2274/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2274<F: Float>(t28283: F, t571: F, t28234: F, t575: F, t101558: F, t101563: F, t101609: F, t101651: F, t1456: F, t1458: F, t1914: F, t1921: F, t26094: F, t26133: F, t3: F, t4168: F, t5808: F, t7319: F, t7940: F, t92559: F, t92563: F, t95127: F) -> F {
    let t101656 = F::new(2.0) * t571 * t28283;
    let t101658 = F::new(2.0) * t28234 * t575;
    let t101659 = t26094 * t1921 + F::new(2.0) * t1456 * t28283 + t3 * t101558 * t575 + t7940 * t4168 + t95127 + t101563 + F::new(2.0) * t7319 * t5808 + t1914 * t26133 + t92563 + t1458 * (t101609 + t101651) + F::new(2.0) * t92559 + t101656 + t101658;
    t101659
}
