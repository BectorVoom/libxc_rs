//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 629/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk629<F: Float>(t2259: F, t70: F, t607: F, t627: F, t362: F, t41: F, t47: F, sigma0: F) -> (F, F, F, F, F) {
    let t2260 = t2259 * t70;
    let t2263 = t607 * t627;
    let t2269 = F::new(1.0) / t41 / t362;
    let t2270 = sigma0 * t2269;
    let t2275 = F::new(1.0) / t47;
    (t2260, t2263, t2269, t2270, t2275)
}
