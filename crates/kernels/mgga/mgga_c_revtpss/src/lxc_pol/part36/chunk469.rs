//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 469/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk469<F: Float>(t3057: F, t378: F, t2846: F, t221: F, t346: F, t696: F, t345: F, t360: F, t365: F, t1038: F, t72: F) -> (F, F, F, F, F) {
    let t3058 = t3057 * t378;
    let t3070 = 0.19755555555555555556e-1 * t2846;
    let t3080 = t221 * t696 * t346;
    let t3082 = t345 * t3080 / 432.0;
    let t3088 = t360 * t365;
    let t3089 = t1038 * t72;
    (t3058, t3070, t3082, t3088, t3089)
}
