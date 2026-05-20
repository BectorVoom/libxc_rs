//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3236/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3236<F: Float>(t1486: F, t1494: F, t19680: F, t21695: F, t21698: F, t21699: F, t21702: F, t22673: F, t22676: F, t4181: F, t4187: F, t4217: F, t4238: F, t5826: F, t5827: F, t5854: F, t641: F, t85: F) -> F {
    let t85206 = -t22673 * t641 / F::new(12.0) - t19680 * t1486 * t85 / F::new(4.0) - t21698 * t1486 * t85 / F::new(4.0) - t5826 * t4217 * t85 / F::new(4.0) - t22676 * t641 / F::new(4.0) - t21695 * t1494 / F::new(4.0) - t21699 * t1494 / F::new(4.0) - t21702 * t1494 / F::new(4.0) - t5827 * t4238 / F::new(4.0) - t4181 * t5854 * t85 / F::new(4.0) - t4187 * t5854 * t85 / F::new(4.0);
    t85206
}
