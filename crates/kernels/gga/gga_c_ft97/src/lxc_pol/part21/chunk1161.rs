//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1161/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1161<F: Float>(t29674: F, t375: F, t89: F, t29669: F, t379: F, t93378: F, t93379: F, t100356: F, t101611: F, t15959: F, t15951: F, t100360: F, t15955: F, t22952: F, t22953: F, t4431: F, t473: F, t5691: F) -> (F, F, F, F, F, F, F) {
    let t116508 = t89 * t375 * t29674;
    let t116509 = t116508 / 3.0;
    let t116512 = t93378 * t93379 * t29669 * t379;
    let t116515 = t100356 * t101611 * t15959;
    let t116518 = t100356 * t93379 * t15951;
    let t116521 = t100356 * t100360 * t15955;
    let t116526 = t22952 * t22953 * t5691 * t4431 * t473;
    (t116508, t116509, t116512, t116515, t116518, t116521, t116526)
}
