//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1910/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1910<F: Float>(t1873: F, t94519: F, t26004: F, t5690: F, t13951: F, t2018: F, t807: F, t25240: F, t3964: F, t5617: F, t543: F, t97870: F) -> (F, F, F, F, F) {
    let t98260 = t94519 * t1873;
    let t98269 = t26004 * t5690;
    let t98281 = t807 * t2018 * t13951;
    let t98285 = t3964 * t25240 * t5617;
    let t98299 = t97870 * t543;
    (t98260, t98269, t98281, t98285, t98299)
}
