//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 971/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk971<F: Float>(t21645: F, t3690: F, t446: F, t9744: F, t2354: F, t88068: F, t18370: F, t5120: F, t91: F, t52212: F, t52916: F, t66902: F, t66905: F, t66934: F, t66945: F, t67420: F, t80685: F, t80696: F, t80759: F, t80770: F, t80772: F) -> (F, F, F, F, F) {
    let t88196 = t3690 * t21645;
    let t88198 = t446 * t9744 * t88196;
    let t88201 = t446 * t2354 * t88068;
    let t88213 = t91 * t18370 * t5120;
    let t88215 = 8.0 * t80685 - 8.0 / 3.0 * t66902 + 16.0 / 3.0 * t66905 + 8.0 / 3.0 * t88198 - 8.0 * t88201 + 8.0 / 3.0 * t80696 + 16.0 / 9.0 * t66934 - 8.0 / 9.0 * t66945 + 112.0 / 81.0 * t52212 + 112.0 / 27.0 * t52916 - 16.0 / 9.0 * t80759 - 16.0 / 27.0 * t67420 + 8.0 / 9.0 * t80770 - 8.0 / 9.0 * t80772 + 9.0 / 4.0 * t88213;
    (t88196, t88198, t88201, t88213, t88215)
}
