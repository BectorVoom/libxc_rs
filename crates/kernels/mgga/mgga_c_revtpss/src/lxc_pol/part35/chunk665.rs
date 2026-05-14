//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 665/1093 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk665<F: Float>(t225: F, t8085: F, t1903: F, t2097: F, t7296: F, t1882: F, t543: F) -> (F, F, F, F) {
    let t8086 = t8085 * t225;
    let t8094 = t2097 * t1903;
    let t8095 = t7296 * t8094;
    let t8099 = t2097 * t1882 * t543;
    (t8086, t8094, t8095, t8099)
}
