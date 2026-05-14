//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 534/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk534<F: Float>(t5513: F, t6427: F, t5522: F, t938: F, t5540: F, t5546: F, t1701: F, t5571: F, t930: F) -> (F, F, F, F, F, F) {
    let t6428 = t5513 * t6427;
    let t6431 = t5522 * t938;
    let t6434 = t5540 * t6427;
    let t6437 = t5546 * t938;
    let t6438 = t1701 * t6437;
    let t6441 = t5571 * t930;
    (t6428, t6431, t6434, t6437, t6438, t6441)
}
