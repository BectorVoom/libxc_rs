//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 516/869 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk516<F: Float>(t184: F, t3596: F, t1063: F, t5: F, t1068: F, t2253: F, t179: F, t422: F, t2984: F, t2266: F, t643: F, t925: F, t71: F, t2993: F, t1576: F, t171: F) -> (F, F, F, F, F, F, F, F, F) {
    let t3597 = t3596 * t184;
    let t3601 = t5 * t1063;
    let t3611 = t2253 * t1068;
    let t3613 = t422 * t179;
    let t3614 = t3613 * t2984;
    let t3618 = t2266 * t925 * t643;
    let t3621 = t71 * t179;
    let t3622 = t3621 * t2993;
    let t3626 = 1.0 / t171 / t1576;
    (t3597, t3601, t3611, t3613, t3614, t3618, t3621, t3622, t3626)
}
