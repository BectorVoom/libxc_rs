//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1223/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1223<F: Float>(t101811: F, t101823: F, t101814: F, t101817: F, t101820: F, t101827: F, t101831: F, t101835: F, t101840: F, t101844: F, t101848: F, t101852: F, t101876: F, t101879: F, t101882: F, t101898: F) -> (F, F, F, F, F) {
    let t102239 = 4.0 * t101811;
    let t102243 = 2.0 / 3.0 * t101823;
    let t102251 = t102239 - 4.0 / 3.0 * t101814 + 4.0 / 9.0 * t101817 - 4.0 / 3.0 * t101820 - t102243 - 12.0 * t101827 + 2.0 * t101831 + 4.0 * t101835 - 6.0 * t101840 - t101844 / 12.0 - t101848 / 18.0 - t101852 / 6.0;
    let t102256 = 8.0 / 9.0 * t101876;
    let t102257 = 4.0 / 9.0 * t101879;
    let t102258 = 2.0 / 3.0 * t101882;
    let t102261 = t101898 / 9.0;
    (t102251, t102256, t102257, t102258, t102261)
}
