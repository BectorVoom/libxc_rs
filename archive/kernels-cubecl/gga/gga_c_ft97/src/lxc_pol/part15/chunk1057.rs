//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1057/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1057<F: Float>(t4698: F, t40033: F, t58708: F, t58719: F, t74068: F, t74126: F, t74143: F, t74148: F, t74153: F, t74162: F, t85454: F, t85458: F) -> (F, F) {
    let t86829 = t4698 * t4698;
    let t86850 = F::cast_from(0.22226000364197530865e-1_f64) * t74162 - t40033 - F::cast_from(0.22226000364197530866e-1_f64) * t58708 - F::cast_from(0.1333560021851851852e0_f64) * t74126 + F::cast_from(0.88904001456790123462e-1_f64) * t74068 + F::cast_from(0.1333560021851851852e0_f64) * t74143 + F::cast_from(0.69147556688614540471e-1_f64) * t74148 - F::cast_from(0.17780800291358024693e0_f64) * t74153 - F::cast_from(0.29634667152263374488e-1_f64) * t58719 - F::cast_from(0.10001700163888888889e0_f64) * t85454 - F::cast_from(0.13335600218518518519e0_f64) * t85458;
    (t86829, t86850)
}
