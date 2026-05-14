//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 746/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk746<F: Float>(t35343: F, t743: F, t193: F, t6109: F, t33508: F, t33513: F, t35312: F, t35316: F, t35321: F, t35326: F, t35330: F, t35334: F, t35338: F, t35341: F, t1091: F, t2354: F, t33341: F) -> (F, F, F, F) {
    let t35344 = t743 * t35343;
    let t35346 = t6109 * t193 * t35344;
    let t35348 = t35312 / 2.0 + t33508 + 2.0 / 9.0 * t35316 + 4.0 / 3.0 * t35321 - 2.0 / 3.0 * t35326 - t35330 / 6.0 - t33513 - t35334 / 9.0 - t35338 + 2.0 / 3.0 * t35341 + t35346 / 12.0;
    let t35350 = t2354 * t33341 * t1091;
    (t35344, t35346, t35348, t35350)
}
