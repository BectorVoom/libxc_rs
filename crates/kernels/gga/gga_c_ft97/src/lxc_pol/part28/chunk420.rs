//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 420/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk420<F: Float>(t574: F, t605: F, t6639: F, t167: F, t6615: F, t1391: F, t569: F, t925: F, t1017: F, t1039: F, t1359: F, t586: F, t28: F, t5890: F, t1969: F, t5900: F) -> (F, F, F, F, F, F, F, F) {
    let t6641 = t574 * t605 * t6639;
    let t6645 = t574 * t167 * t6615;
    let t6649 = t569 * t1391 * t925;
    let t6653 = t574 * t1391 * t1017;
    let t6656 = t1359 * t1039;
    let t6657 = t586 * t6656;
    let t6659 = t5890 * t28 * t6657;
    let t6662 = t1969 * t5900 * t925;
    (t6641, t6645, t6649, t6653, t6656, t6657, t6659, t6662)
}
