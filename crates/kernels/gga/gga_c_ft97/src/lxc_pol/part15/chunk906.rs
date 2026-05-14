//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 906/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk906<F: Float>(t1564: F, t446: F, t86075: F, t15601: F, t28: F, t4495: F, t89: F, t1555: F, t7764: F, t85469: F, t1558: F, t85451: F, t57491: F, t57527: F, t86199: F, t86202: F, t86205: F, t86208: F, t86211: F, t86214: F, t86217: F, t86220: F, t86223: F) -> (F, F, F, F, F) {
    let t86226 = t446 * t1564 * t86075;
    let t86232 = t89 * t28 * t15601 * t4495;
    let t86236 = t89 * t1555 * t7764 * t85469;
    let t86240 = t89 * t1555 * t1558 * t85451;
    let t86242 = -8.0 * t86199 + 4.0 / 3.0 * t86202 + 2.0 * t86205 + 8.0 * t86208 - 8.0 / 9.0 * t86211 + 4.0 / 3.0 * t86214 + 40.0 / 27.0 * t86217 - 20.0 / 9.0 * t86220 + 8.0 * t86223 + 8.0 * t86226 + 16.0 / 9.0 * t57491 - 16.0 / 27.0 * t57527 - 36.0 * t86232 - 8.0 * t86236 - 2.0 / 3.0 * t86240;
    (t86226, t86232, t86236, t86240, t86242)
}
