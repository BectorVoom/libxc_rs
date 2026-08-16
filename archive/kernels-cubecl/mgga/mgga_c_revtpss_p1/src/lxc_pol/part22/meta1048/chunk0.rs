//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3682/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3682<F: Float>(t68679: F, t68704: F, t68736: F, t68773: F, t68943: F, t69031: F, t69600: F, t69606: F, t5284: F, t487: F, t3565: F, t6563: F) -> (F, F, F, F) {
    let t69609 = t68679 + t68704 + t68736 + t68773 + t68943 + t69031 + t69600 + t69606;
    let t69623 = t5284 * t5284;
    let t69624 = t487 * t69623;
    let t69636 = t6563 * t3565;
    (t69609, t69623, t69624, t69636)
}
