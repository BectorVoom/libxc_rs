//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 218/1196 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk218<F: Float>(t234: F, t243: F, t808: F, t807: F, t236: F, t786: F, t240: F, t27: F) -> (F, F, F, F) {
    let t810 = t234 * t808 * t243;
    let t812 = 0.71456696863449561619e-5 * t807 * t810;
    let t813 = t786 * t236;
    let t814 = t27 * t240;
    (t810, t812, t813, t814)
}
