//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1065/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1065<F: Float>(t144826: F, t144888: F, t144943: F, t144981: F, t145034: F, t145603: F, t145665: F, t145701: F, t488: F, t22943: F, t25598: F, t31995: F, t6414: F) -> (F, F, F) {
    let t145705 = t488 * (t144826 + t144888 + t144943 + t144981 + t145034 + t145603 + t145665 + t145701);
    let t145719 = t22943 * t25598;
    let t145731 = t6414 * t31995;
    (t145705, t145719, t145731)
}
