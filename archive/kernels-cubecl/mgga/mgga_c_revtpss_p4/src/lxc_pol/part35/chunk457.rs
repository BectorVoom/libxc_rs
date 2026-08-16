//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 457/1234 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk457<F: Float>(t3153: F, t3154: F, t1036: F, t3148: F, t3141: F, t357: F, t127: F, t246: F) -> (F, F, F, F, F) {
    let t3155 = t3153 * t3154;
    let t3160 = t1036 * t3148;
    let t3161 = t3141 * t3160;
    let t3162 = t3153 * t357;
    let t3172 = t246 * t127;
    (t3155, t3160, t3161, t3162, t3172)
}
