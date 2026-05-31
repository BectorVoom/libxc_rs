//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2257/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2257<F: Float>(t30197: F, t571: F, t2045: F, t6936: F, t1921: F, t7939: F, t2037: F, t6951: F, t101656: F, t101658: F, t101661: F, t101668: F, t101670: F, t101672: F, t101674: F, t1456: F, t6937: F, t7337: F) -> F {
    let t109345 = t571 * t30197;
    let t109348 = t6936 * t2045;
    let t109349 = t7939 * t1921;
    let t109351 = t2037 * t6951;
    let t109352 = t1456 * t30197 + t6937 * t7337 + t101656 + t101658 + t101661 + t101668 + t101670 + t101672 + t101674 + t109345 + t109348 + F::cast_from(2.0_f64) * t109349 + t109351;
    t109352
}
