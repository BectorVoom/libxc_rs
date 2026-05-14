//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 961/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk961<F: Float>(t786: F, t9679: F, t1444: F, t2434: F, t123: F, t3915: F, t1359: F, t9292: F, t1363: F, t9288: F, t1362: F, t3911: F, t3920: F, t2237: F, t240: F, t550: F, t816: F) -> (F, F, F, F, F, F, F) {
    let t9680 = t786 * t9679;
    let t9685 = t2434 * t1444;
    let t9686 = t123 * t9685;
    let t9687 = t3915 * t9686;
    let t9691 = 0.17073386770573548589e-1 * t9292 * t1359;
    let t9692 = t1363 * t9288;
    let t9694 = 0.30356481678079769392e-1 * t1362 * t9692;
    let t9695 = t3911 * t3920;
    let t9707 = t2237 * t240;
    let t9709 = t9707 * t550 * t816;
    (t9680, t9687, t9691, t9694, t9695, t9707, t9709)
}
