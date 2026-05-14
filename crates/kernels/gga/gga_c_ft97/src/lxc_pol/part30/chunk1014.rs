//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1014/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1014<F: Float>(t143101: F, t143120: F, t143123: F, t152715: F, t152719: F, t152724: F, t152727: F, t152730: F, t152734: F, t152738: F, t152742: F, t152746: F, t152750: F, t152754: F, t152758: F, t152760: F) -> (F,) {
    let t154125 = -t152715 + t152719 - 2.0 / 3.0 * t152724 + 2.0 / 3.0 * t152727 - 2.0 / 9.0 * t152730 + 12.0 * t152734 + 2.0 / 3.0 * t152738 + 3.0 / 2.0 * t152742 + 3.0 / 2.0 * t152746 - t152750 + 3.0 / 4.0 * t152754 - 3.0 * t152758 + 4.0 / 3.0 * t152760 - 4.0 / 9.0 * t143101 - 8.0 / 3.0 * t143120 + 4.0 / 3.0 * t143123;
    (t154125,)
}
