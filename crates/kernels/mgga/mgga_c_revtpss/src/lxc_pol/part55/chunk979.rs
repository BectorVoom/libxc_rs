//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 979/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk979<F: Float>(t28902: F, t689: F, t225: F, t28888: F, t27899: F, t7515: F, t2097: F, t3999: F) -> (F, F, F, F) {
    let t28903 = t689 * t28902;
    let t28905 = t28888 * t225;
    let t28909 = t27899 * t7515;
    let t28911 = t3999 * t2097;
    (t28903, t28905, t28909, t28911)
}
