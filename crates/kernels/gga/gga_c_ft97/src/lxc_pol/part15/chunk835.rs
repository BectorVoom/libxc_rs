//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 835/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk835<F: Float>(t2681: F, t4218: F, t5299: F, t21572: F, t848: F, t10570: F, t192: F, t21978: F, t10580: F, t21584: F, t10603: F, t21945: F) -> (F, F, F, F, F) {
    let t22284 = t2681 * t4218 * t5299;
    let t22287 = t848 * t21572;
    let t22291 = t192 * t10570 * t21978;
    let t22294 = t10580 * t21584;
    let t22298 = t10603 * t21945;
    (t22284, t22287, t22291, t22294, t22298)
}
