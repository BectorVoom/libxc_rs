//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1141/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1141<F: Float>(t29599: F, t38659: F, t4551: F, t5743: F, t8418: F, t1852: F, t29789: F, t492: F, t16480: F, t5710: F, t102033: F, t102076: F, t102079: F, t102082: F, t25525: F, t25530: F, t25535: F, t25558: F, t25570: F, t26125: F, t26130: F, t6414: F, t6418: F) -> (F, F, F, F, F) {
    let t116124 = t38659 * t29599;
    let t116127 = t8418 * t5743 * t4551;
    let t116130 = t1852 * t29789 * t492;
    let t116136 = t5710 * t16480;
    let t116138 = 4.0 / 27.0 * t102076 - t102079 - t102033 * t6418 / 9.0 - t25558 * t25570 / 9.0 - 2.0 / 3.0 * t6414 * t25535 - 2.0 / 3.0 * t6414 * t26125 - 2.0 / 3.0 * t6414 * t25530 - 12.0 * t116124 - 12.0 * t116127 + 4.0 * t116130 - 2.0 / 3.0 * t6414 * t26130 - 2.0 / 3.0 * t6414 * t25525 - 2.0 * t116136 - t102082;
    (t116124, t116127, t116130, t116136, t116138)
}
