//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 715/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk715<F: Float>(t21450: F, t446: F, t1131: F, t4973: F, t2354: F, t1091: F, t5053: F, t13722: F, t13739: F, t17720: F, t21433: F, t21437: F, t21440: F, t21444: F, t21448: F, t9699: F) -> (F, F, F, F, F, F, F, F) {
    let t21451 = t446 * t21450;
    let t21453 = t4973 * t1131;
    let t21454 = t2354 * t21453;
    let t21455 = t446 * t21454;
    let t21457 = t1091 * t5053;
    let t21458 = t2354 * t21457;
    let t21459 = t446 * t21458;
    let t21462 = -2.0 / 27.0 * t13722 - t9699 - t17720 / 9.0 - 5.0 / 81.0 * t21433 - t21437 / 3.0 + t21440 / 3.0 + t21444 / 9.0 + 2.0 / 9.0 * t21448 - t21451 / 9.0 + t21455 / 6.0 + t21459 / 6.0 - 2.0 / 9.0 * t13739;
    (t21451, t21453, t21454, t21455, t21457, t21458, t21459, t21462)
}
