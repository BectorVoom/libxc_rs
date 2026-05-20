//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3882/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3882<F: Float>(t2661: F, t74026: F, t9835: F, t9934: F, t22016: F, t22025: F, t46609: F, t6846: F, t9909: F, t1399: F, t22236: F, t3992: F) -> (F, F, F, F) {
    let t74579 = t2661 * t9934 * t74026 * t9835;
    let t74583 = t2661 * t46609 * t22025 * t22016;
    let t74585 = t9909 * t6846;
    let t74589 = t2661 * t3992 * t22236 * t1399;
    (t74579, t74583, t74585, t74589)
}
