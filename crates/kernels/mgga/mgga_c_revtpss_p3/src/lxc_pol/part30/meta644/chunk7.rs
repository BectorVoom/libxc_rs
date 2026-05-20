//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2268/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2268<F: Float>(t29490: F, t571: F, t105792: F, t105794: F, t105796: F, t105798: F, t105800: F, t105802: F, t18217: F, t2168: F, t96684: F, t96692: F, t96694: F, t97580: F, t97586: F) -> F {
    let t105804 = F::new(2.0) * t571 * t29490;
    let t105806 = t18217 * t2168 + t105792 + t105794 + t105796 + t105798 + t105800 + t105802 + t105804 + F::new(2.0) * t96684 + t96692 + t96694 + t97580 + t97586;
    t105806
}
