//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 997/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk997<F: Float>(t139361: F, t148435: F, t148439: F, t148443: F, t148446: F, t148449: F, t148454: F, t148457: F, t148460: F, t148464: F, t148467: F, t148470: F, t148473: F, t148477: F, t148481: F, t148486: F) -> (F,) {
    let t148488 = -t148435 / 3.0 - 2.0 * t148439 + t148443 - 2.0 / 3.0 * t148446 - 2.0 / 9.0 * t148449 - 2.0 / 3.0 * t148454 - 8.0 / 9.0 * t148457 + t148460 / 18.0 - 8.0 / 9.0 * t139361 - 8.0 / 9.0 * t148464 + 2.0 / 3.0 * t148467 - 4.0 / 9.0 * t148470 - 2.0 / 9.0 * t148473 + 2.0 / 27.0 * t148477 + 2.0 / 3.0 * t148481 + t148486 / 12.0;
    (t148488,)
}
