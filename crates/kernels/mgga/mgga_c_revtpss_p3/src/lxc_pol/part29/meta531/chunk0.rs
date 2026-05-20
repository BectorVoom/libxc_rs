//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1860/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1860<F: Float>(t2247: F, t2251: F, t68: F, t26205: F, t6963: F, t45972: F, t7342: F, t10309: F, t26178: F, t25159: F, t2047: F, t92569: F) -> (F, F, F, F, F, F) {
    let t95310 = t2247 * t2251 * t68;
    let t95314 = t6963 * t26205;
    let t95316 = t45972 * t7342;
    let t95319 = t10309 * t26178;
    let t95320 = t95319 * t25159;
    let t95340 = t2047 * t92569;
    (t95310, t95314, t95316, t95319, t95320, t95340)
}
