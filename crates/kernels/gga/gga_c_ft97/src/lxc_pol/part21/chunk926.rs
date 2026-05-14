//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 926/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk926<F: Float>(t11120: F, t29482: F, t22576: F, t4449: F, t4467: F, t5571: F, t22535: F, t25759: F, t925: F, t15805: F, t6426: F) -> (F, F, F, F, F, F) {
    let t29483 = t29482 * t11120;
    let t29486 = t22576 * t4449;
    let t29490 = t5571 * t4467;
    let t29494 = t22535 * t4449;
    let t29498 = t25759 * t925;
    let t29502 = t6426 * t15805;
    (t29483, t29486, t29490, t29494, t29498, t29502)
}
