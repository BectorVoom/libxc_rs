//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1145/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1145<F: Float>(t108187: F, t25878: F, t30081: F, t689: F, t94768: F, t94763: F, t5722: F, t97783: F, t6871: F, t94429: F, t22102: F, t94423: F, t26004: F, t6884: F, t6850: F, t94513: F) -> (F, F, F, F, F, F, F, F) {
    let t108474 = t25878 * t108187;
    let t108493 = t30081 * t689;
    let t108494 = t94768 * t108493;
    let t108496 = t94763 * t108493;
    let t108498 = t97783 * t5722;
    let t108516 = t94429 * t6871;
    let t108524 = t94423 * t22102;
    let t108537 = t26004 * t6884;
    let t108539 = t94513 * t6850;
    (t108474, t108494, t108496, t108498, t108516, t108524, t108537, t108539)
}
