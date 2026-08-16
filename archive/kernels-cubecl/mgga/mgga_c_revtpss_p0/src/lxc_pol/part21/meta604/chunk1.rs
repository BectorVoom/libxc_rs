//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2335/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2335<F: Float>(t22: F, t251: F, t39698: F, t837: F, t2722: F, t860: F, t231: F, t2782: F, t2783: F, t10665: F, t2723: F, t4503: F) -> (F, F, F, F, F) {
    let t39701 = t39698 * t251 * t22 * t837;
    let t39704 = t860 * t2722;
    let t39707 = t2782 * t2783 * t39704 * t231;
    let t39709 = t251 * t10665;
    let t39712 = t2782 * t4503 * t39709 * t2723;
    (t39701, t39704, t39707, t39709, t39712)
}
