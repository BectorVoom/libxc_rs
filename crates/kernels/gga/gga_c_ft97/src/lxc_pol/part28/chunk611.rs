//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 611/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk611<F: Float>(t26545: F, t28: F, t1058: F, t558: F, t5778: F, t614: F, t6616: F, t376: F, t6621: F, t165: F, t6615: F, t379: F, t1969: F, t6617: F, t23997: F, t3483: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t26546 = t28 * t26545;
    let t26551 = t1058 * t558;
    let t26552 = t5778 * t26551;
    let t26553 = t28 * t26552;
    let t26560 = t6616 * t614;
    let t26561 = t28 * t26560;
    let t26564 = t376 * t6621;
    let t26567 = t6615 * t165;
    let t26568 = t26567 * t379;
    let t26569 = t1969 * t26568;
    let t26574 = t376 * t6617;
    let t26577 = t23997 * t3483;
    (t26546, t26551, t26553, t26561, t26564, t26567, t26568, t26569, t26574, t26577)
}
