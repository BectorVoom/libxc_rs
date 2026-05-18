//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 701/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk701<F: Float>(t11050: F, t447: F, t446: F, t1588: F, t925: F, t7824: F, t7800: F, t920: F, t1559: F) -> (F, F, F, F) {
    let t11051 = t447 * t11050;
    let t11052 = t446 * t11051;
    let t11054 = t925 * t1588;
    let t11055 = t7824 * t11054;
    let t11056 = t446 * t11055;
    let t11058 = t7800 * t920;
    let t11059 = t11058 * t1559;
    (t11052, t11054, t11056, t11059)
}
