//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 539/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk539<F: Float>(t1882: F, t5970: F, t5862: F, t5871: F, t5937: F, t1384: F, t358: F, t1359: F, t604: F) -> (F, F, F, F, F, F) {
    let t23484 = t1882 * t5970;
    let t23532 = t1882 * t5862;
    let t23534 = t1882 * t5871;
    let t23546 = t1882 * t5937;
    let t23548 = t1384 * t358;
    let t23571 = t604 * t1359;
    (t23484, t23532, t23534, t23546, t23548, t23571)
}
