//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 709/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk709<F: Float>(t225: F, t7506: F, t2097: F, t213: F, t2102: F, t72: F, t686: F) -> (F, F, F, F) {
    let t7507 = t7506 * t225;
    let t7511 = t213 * t2097;
    let t7514 = t2102 * t72;
    let t7515 = t7514 * t686;
    (t7507, t7511, t7514, t7515)
}
