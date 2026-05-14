//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1210/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1210<F: Float>(t1154: F, t27819: F, t3938: F, t6119: F, t729: F, t24543: F, t30959: F, t18206: F, t42500: F, t6118: F, t10157: F, t18622: F, t31014: F, t446: F, t713: F, t13521: F, t30651: F) -> (F, F, F, F, F, F) {
    let t122706 = t27819 * t729 * t6119 * t1154 * t3938;
    let t122708 = t24543 * t30959;
    let t122712 = t6118 * t42500 * t6119 * t18206;
    let t122716 = t6118 * t10157 * t6119 * t18622;
    let t122720 = t446 * t10157 * t31014 * t713;
    let t122729 = t30651 * t13521;
    (t122706, t122708, t122712, t122716, t122720, t122729)
}
