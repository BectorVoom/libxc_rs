//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 775/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk775<F: Float>(t3627: F, t41: F, t70: F, t11013: F, t3613: F, t2266: F, t2294: F, t925: F, t3052: F, t643: F, t10998: F, t3621: F) -> (F, F, F, F, F) {
    let t12143 = t41 * t3627 * t70;
    let t12144 = t3613 * t11013;
    let t12148 = t2266 * t925 * t2294;
    let t12152 = t2266 * t3052 * t643;
    let t12155 = t3621 * t10998;
    (t12143, t12144, t12148, t12152, t12155)
}
