//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 221/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk221<F: Float>(t605: F, t609: F, t144: F, t28: F, t446: F, t568: F, t571: F, t576: F, t599: F, t89: F, t160: F, t597: F) -> (F, F, F, F) {
    let t610 = t605 * t609;
    let t611 = t144 * t610;
    let t614 = -t568 - t446 * t571 / F::cast_from(9.0_f64) - t446 * t576 / F::cast_from(3.0_f64) + t89 * t28 * t599 / F::cast_from(3.0_f64) - t446 * t611 / F::cast_from(3.0_f64);
    let t616 = t597 * t160;
    (t610, t611, t614, t616)
}
