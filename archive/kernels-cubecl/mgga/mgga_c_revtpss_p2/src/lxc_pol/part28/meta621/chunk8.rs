//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2197/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2197<F: Float>(t892: F, t99536: F, t1940: F, t1963: F, t580: F, t4343: F, t605: F, t27383: F, t63164: F, t2411: F, t27363: F, t25207: F, t61102: F) -> (F, F, F, F, F, F) {
    let t99537 = t99536 * t892;
    let t99542 = t1940 * t1963 * t580;
    let t99543 = t605 * t4343;
    let t99550 = t27383 * t63164;
    let t99555 = t27363 * t2411;
    let t99558 = t25207 * t61102;
    (t99537, t99542, t99543, t99550, t99555, t99558)
}
