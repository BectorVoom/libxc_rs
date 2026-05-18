//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 188/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk188<F: Float>(t605: F, t609: F, t144: F, t28: F, t446: F, t568: F, t571: F, t576: F, t599: F, t89: F, t160: F, t597: F) -> (F, F, F, F) {
    let t610 = t605 * t609;
    let t611 = t144 * t610;
    let t614 = -t568 - t446 * t571 / F::new(9.0) - t446 * t576 / F::new(3.0) + t89 * t28 * t599 / F::new(3.0) - t446 * t611 / F::new(3.0);
    let t616 = t597 * t160;
    (t610, t611, t614, t616)
}
