//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1258/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1258<F: Float>(t1384: F, t17355: F, t2179: F, t30133: F, t9276: F, t30357: F, t609: F, t17086: F, t95403: F, t17181: F, t23997: F, t27191: F, t3578: F, t30280: F, t604: F, t1359: F, t4837: F) -> (F, F, F, F, F, F, F, F) {
    let t119477 = t2179 * t1384 * t17355;
    let t119479 = t9276 * t30133;
    let t119482 = t2179 * t30357 * t609;
    let t119484 = t95403 * t17086;
    let t119486 = t23997 * t17181;
    let t119488 = t3578 * t27191;
    let t119491 = t30280 * t604;
    let t119492 = t119491 * t609;
    let t119496 = t1359 * t4837;
    (t119477, t119479, t119482, t119484, t119486, t119488, t119492, t119496)
}
