//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 698/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk698<F: Float>(t11019: F, t11036: F, t11930: F, t11931: F, t11932: F, t7775: F, t8192: F, t8444: F, t8446: F, t8449: F, t8452: F, t11043: F, t11069: F, t11041: F, t11048: F, t11052: F, t11056: F, t11061: F, t11066: F, t11073: F, t11774: F, t8454: F) -> (F, F) {
    let t11936 = t11019 / 3.0 - t11930 - t11931 + t11932 - 8.0 / 27.0 * t7775 + t8444 + t8446 - t8449 - 8.0 / 9.0 * t8192 + t8452 - 2.0 / 9.0 * t11036;
    let t11939 = 4.0 / 27.0 * t11043;
    let t11946 = 2.0 / 3.0 * t11069;
    let t11948 = -6.0 * t11041 - t11939 - 2.0 / 3.0 * t11048 - 2.0 * t11052 - 2.0 / 3.0 * t11056 + 4.0 / 3.0 * t11061 + t11774 / 2.0 - t8454 - 4.0 / 3.0 * t11066 + t11946 - 2.0 / 3.0 * t11073;
    (t11936, t11948)
}
