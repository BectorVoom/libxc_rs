//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1115/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1115<F: Float>(t25304: F, t27883: F, t25946: F, t25898: F, t97699: F, t2453: F, t3908: F, t7911: F, t136: F, t2457: F, t7920: F, t94589: F, t2435: F, t27965: F, t14090: F, t26054: F) -> (F, F, F, F, F, F, F) {
    let t97799 = t25304 * t27883;
    let t97800 = t97799 * t25946;
    let t97802 = t97699 * t25898;
    let t97810 = t2453 * t7911 * t3908;
    let t97814 = t7920 * t136 * t2457;
    let t97815 = t94589 * t97814;
    let t97823 = t2435 * t27965;
    let t97825 = t26054 * t14090;
    (t97800, t97802, t97810, t97814, t97815, t97823, t97825)
}
