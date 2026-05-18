//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 1164/1203 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk1164<F: Float>(t129470: F, t1937: F, t34446: F, t6993: F, t129467: F, t1936: F, t7002: F, t27060: F, t7741: F, t29432: F, t28042: F, t7586: F) -> (F, F, F, F, F, F, F, F) {
    let t129471 = t129470 * t1937;
    let t129473 = t34446 * t6993;
    let t129478 = t129467 * t1936;
    let t129479 = t129470 * t1936;
    let t129480 = t34446 * t7002;
    let t129481 = t27060 * t7741;
    let t129482 = t29432 * t7741;
    let t129483 = t7586 * t28042;
    (t129471, t129473, t129478, t129479, t129480, t129481, t129482, t129483)
}
