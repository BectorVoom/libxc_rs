//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 638/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk638<F: Float>(t11470: F, t568: F, t11219: F, t531: F, t11218: F, t189: F, t188: F, t3565: F, t524: F, t1628: F, t3595: F, t3591: F) -> (F, F, F, F, F, F, F) {
    let t11471 = t568 * t11470;
    let t11476 = t531 * t11219;
    let t11481 = t189 * t11218;
    let t11482 = t188 * t11481;
    let t11485 = t524 * t3565;
    let t11490 = t1628 * t3595;
    let t11493 = t1628 * t3591;
    (t11471, t11476, t11481, t11482, t11485, t11490, t11493)
}
