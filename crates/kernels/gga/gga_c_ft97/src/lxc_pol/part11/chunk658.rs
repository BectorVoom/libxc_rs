//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 658/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk658<F: Float>(t683: F, t9596: F, t92: F, t9557: F, t9558: F, t9560: F, t9562: F, t9564: F, t9574: F, t9580: F, t9585: F, t9589: F, t9594: F) -> (F, F, F) {
    let t9597 = t683 * t9596;
    let t9598 = t92 * t9597;
    let t9600 = -t9557 - 4.0 / 9.0 * t9558 + 2.0 / 9.0 * t9560 - 2.0 / 3.0 * t9562 + t9564 / 3.0 - 10.0 / 27.0 * t9574 + 4.0 / 3.0 * t9580 - 2.0 / 3.0 * t9585 - 2.0 * t9589 + 2.0 * t9594 - t9598 / 3.0;
    (t9597, t9598, t9600)
}
