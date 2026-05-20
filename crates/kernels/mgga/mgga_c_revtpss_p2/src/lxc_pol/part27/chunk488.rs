//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 488/1333 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk488<F: Float>(t2703: F, t802: F, t124: F, t2430: F, t800: F, t234: F, t2453: F) -> (F, F, F, F) {
    let t2704 = t2703 * t802;
    let t2706 = t124 * t2430;
    let t2707 = t800 * t2706;
    let t2710 = t2453 * t234;
    (t2704, t2706, t2707, t2710)
}
