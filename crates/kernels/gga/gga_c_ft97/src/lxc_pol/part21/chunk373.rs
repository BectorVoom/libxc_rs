//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 373/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk373<F: Float>(t1068: F, t2253: F, t179: F, t422: F, t2984: F, t2266: F, t643: F, t925: F, t71: F, t2993: F, t1576: F, t171: F, t11: F, t41: F, t18: F, t632: F, t72: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t3611 = t2253 * t1068;
    let t3613 = t422 * t179;
    let t3614 = t3613 * t2984;
    let t3618 = t2266 * t925 * t643;
    let t3621 = t71 * t179;
    let t3622 = t3621 * t2993;
    let t3626 = 1.0 / t171 / t1576;
    let t3627 = t11 * t3626;
    let t3628 = t41 * t3627;
    let t3630 = t72 * t632 * t18;
    (t3611, t3613, t3614, t3618, t3621, t3622, t3626, t3627, t3628, t3630)
}
