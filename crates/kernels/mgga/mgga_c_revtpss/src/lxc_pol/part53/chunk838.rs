//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 838/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk838<F: Float>(t27879: F, t27907: F, t27984: F, t28017: F, t532: F, t1450: F, t2014: F, t1931: F, t670: F) -> (F, F, F, F) {
    let t28019 = t27879 + t27907 + t27984 + t28017;
    let t28020 = t532 * t28019;
    let t28021 = t28020 * t1450;
    let t28022 = t2014 * t28021;
    let t28025 = t1931 * t670;
    (t28019, t28021, t28022, t28025)
}
