//! MGGA_C_REVTPSS lxc pol — lxc_pol part 40 (v4rho3tau_3) CSE chunk 1008/1507 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1008<F: Float>(t3911: F, t3920: F, t3957: F, t3961: F, t3829: F, t4011: F, t547: F, t807: F, t2237: F, t240: F, t550: F, t816: F) -> (F, F, F, F, F) {
    let t9695 = t3911 * t3920;
    let t9697 = t3957 * t3961;
    let t9703 = t4011 * t3829;
    let t9704 = t547 * t9703;
    let t9705 = t807 * t9704;
    let t9707 = t2237 * t240;
    let t9709 = t9707 * t550 * t816;
    (t9695, t9697, t9705, t9707, t9709)
}
