//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 403/869 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk403<F: Float>(t684: F, t713: F, t2354: F, t446: F, t113: F, t667: F) -> (F, F, F, F) {
    let t2355 = t684 * t713;
    let t2356 = t2354 * t2355;
    let t2357 = t446 * t2356;
    let t2359 = t667 * t113;
    let t2360 = 1.0 / t2359;
    (t2355, t2356, t2357, t2360)
}
