//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1057/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1057(t4698: f64, t40033: f64, t58708: f64, t58719: f64, t74068: f64, t74126: f64, t74143: f64, t74148: f64, t74153: f64, t74162: f64, t85454: f64, t85458: f64) -> (f64, f64) {
    let t86829 = t4698 * t4698;
    let t86850 = 0.22226000364197530865e-1_f64 * t74162 - t40033 - 0.22226000364197530866e-1_f64 * t58708 - 0.1333560021851851852e0_f64 * t74126 + 0.88904001456790123462e-1_f64 * t74068 + 0.1333560021851851852e0_f64 * t74143 + 0.69147556688614540471e-1_f64 * t74148 - 0.17780800291358024693e0_f64 * t74153 - 0.29634667152263374488e-1_f64 * t58719 - 0.10001700163888888889e0_f64 * t85454 - 0.13335600218518518519e0_f64 * t85458;
    (t86829, t86850)
}
