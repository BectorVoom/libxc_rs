//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 677/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk677<F: Float>(t20809: F, t20836: F, t579: F, t91: F, t17351: F, t17352: F, t20543: F, t20547: F, t20554: F, t20558: F, t20562: F, t20566: F, t20570: F, t20658: F, t20663: F, t20784: F) -> (F, F, F) {
    let t20837 = t20809 + t20836;
    let t20839 = t91 * t579 * t20837;
    let t20850 = t20784 / 8.0 + t20839 / 6.0 + t17351 - t17352 - t20658 / 3.0 - 2.0 * t20663 - 2.0 / 9.0 * t20554 + t20558 / 3.0 + t20562 / 3.0 - 2.0 / 3.0 * t20566 - 2.0 / 3.0 * t20570 + 2.0 / 3.0 * t20543 + 2.0 / 9.0 * t20547;
    (t20837, t20839, t20850)
}
