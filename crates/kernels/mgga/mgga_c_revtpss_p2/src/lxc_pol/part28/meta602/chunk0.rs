//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2079/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2079<F: Float>(t26093: F, t575: F, t116: F, t25832: F, t26133: F, t571: F, t2327: F, t7724: F, t27833: F, t7316: F, t13426: F, t7003: F) -> (F, F, F, F, F, F) {
    let t95127 = t26093 * t575;
    let t95137 = t116 * t25832;
    let t95180 = t571 * t26133;
    let t97593 = t7724 * t2327;
    let t97604 = F::new(2.0) * t27833 * t7316;
    let t97606 = F::new(4.0) * t13426 * t7003;
    (t95127, t95137, t95180, t97593, t97604, t97606)
}
