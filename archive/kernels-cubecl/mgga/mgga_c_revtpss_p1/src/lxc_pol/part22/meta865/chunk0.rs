//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3019/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3019<F: Float>(t14860: F, t2661: F, t2662: F, t837: F, t2646: F, t4352: F, t14652: F, t4416: F, t14663: F, t221: F, t2484: F, t2485: F) -> (F, F, F, F, F) {
    let t50732 = t2661 * t2662 * t14860 * t837;
    let t50736 = t2661 * t2662 * t4352 * t2646;
    let t50740 = t2661 * t2662 * t14652 * t837;
    let t50744 = t2661 * t2662 * t4416 * t2646;
    let t50748 = t2484 * t2485 * t221 * t14663;
    (t50732, t50736, t50740, t50744, t50748)
}
