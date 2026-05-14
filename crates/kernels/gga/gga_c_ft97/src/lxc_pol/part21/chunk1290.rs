//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1290/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1290<F: Float>(t105827: F, t120010: F, t120013: F, t120017: F, t120021: F, t120025: F, t120029: F, t120033: F, t120037: F, t120041: F, t120044: F, t120048: F, t23657: F, t4668: F, t590: F, t5900: F, t9432: F) -> (F, F) {
    let t120050 = -t105827 - t120010 - t120013 + t120017 / 6.0 - 6.0 * t120021 + 2.0 * t120025 + 2.0 * t120029 + 4.0 * t120033 + 4.0 * t120037 - t120041 - 6.0 * t120044 + t120048 / 3.0;
    let t120055 = t23657 * t9432 * t5900 * t4668 * t590;
    (t120050, t120055)
}
