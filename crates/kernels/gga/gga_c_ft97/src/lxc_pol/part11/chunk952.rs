//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 952/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk952<F: Float>(t1969: F, t39735: F, t446: F, t558: F, t7973: F, t37264: F, t569: F, t2205: F, t37269: F, t378: F, t7368: F, t358: F, t363: F, t9017: F) -> (F, F, F, F, F, F, F) {
    let t39737 = t446 * t1969 * t39735;
    let t39739 = t7973 * t558;
    let t39741 = t446 * t1969 * t39739;
    let t39744 = t446 * t569 * t37264;
    let t39747 = t446 * t2205 * t37269;
    let t39749 = t378 * t7368;
    let t39751 = t9017 * t358 * t363;
    (t39737, t39739, t39741, t39744, t39747, t39749, t39751)
}
