//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2096/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2096<F: Float>(t28283: F, t571: F, t28234: F, t575: F, t1455: F, t7956: F, t1464: F, t7939: F, t2037: F, t5808: F, t1921: F, t7318: F) -> (F, F, F, F, F, F) {
    let t101656 = F::cast_from(2.0_f64) * t571 * t28283;
    let t101658 = F::cast_from(2.0_f64) * t28234 * t575;
    let t101661 = F::cast_from(2.0_f64) * t1455 * t7956;
    let t101668 = F::cast_from(2.0_f64) * t7939 * t1464;
    let t101670 = F::cast_from(2.0_f64) * t2037 * t5808;
    let t101672 = F::cast_from(2.0_f64) * t7318 * t1921;
    (t101656, t101658, t101661, t101668, t101670, t101672)
}
