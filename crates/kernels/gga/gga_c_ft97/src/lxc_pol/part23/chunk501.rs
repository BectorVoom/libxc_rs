//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 501/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk501<F: Float>(t1476: F, t856: F, t852: F, t193: F, t6308: F, t1486: F, t1487: F, t681: F, t1485: F, t92: F) -> (F, F, F, F, F) {
    let t6309 = t1476 * t856;
    let t6310 = t852 * t6309;
    let t6312 = t6308 * t193 * t6310;
    let t6315 = t1486 * t681 * t1487;
    let t6316 = t6315 / 18.0;
    let t6317 = t1485 * t92;
    (t6310, t6312, t6315, t6316, t6317)
}
