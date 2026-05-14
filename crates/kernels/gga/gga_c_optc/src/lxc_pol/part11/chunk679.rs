//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 679/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk679<F: Float>(t330: F, t7448: F, t896: F, t9: F, t22: F, t2595: F, t2263: F, t2662: F) -> (F, F, F, F, F) {
    let t7449 = t330 * t7448;
    let t7467 = t9 * t896;
    let t7481 = t22 * t2595;
    let t7482 = t7481 * t2263;
    let t7491 = t2662 * t7448;
    (t7449, t7467, t7481, t7482, t7491)
}
