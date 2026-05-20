//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1137/1234 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1137<F: Float>(t1549: F, t92968: F, t2689: F, t27239: F, t14760: F, t93015: F, t1955: F, t27198: F, t2769: F, t2453: F, t27212: F, t1568: F, t786: F) -> (F, F, F, F, F, F) {
    let t99050 = t92968 * t1549;
    let t99091 = t2689 * t27239;
    let t99113 = t93015 * t14760;
    let t99191 = t1955 * t27198 * t2769;
    let t99257 = t2453 * t27212;
    let t99403 = t786 * t1568;
    (t99050, t99091, t99113, t99191, t99257, t99403)
}
