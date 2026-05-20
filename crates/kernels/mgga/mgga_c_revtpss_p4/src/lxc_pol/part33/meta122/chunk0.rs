//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 693/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk693<F: Float>(t2969: F, t2846: F, t960: F, t964: F, t320: F, t963: F) -> (F, F, F, F) {
    let t2970 = F::new(1.0) / t2969;
    let t2974 = F::cast_from(0.12361111111111111111e-1_f64) * t2846;
    let t2982 = t960 * t964;
    let t2985 = t963 * t320;
    let t2986 = F::new(1.0) / t2985;
    (t2970, t2974, t2982, t2986)
}
