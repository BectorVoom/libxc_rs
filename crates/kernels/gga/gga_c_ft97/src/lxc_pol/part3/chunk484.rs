//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 484/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk484<F: Float>(t2594: F, t3691: F, t446: F, t1091: F, t713: F, t2354: F, t2360: F, t992: F) -> (F, F, F, F, F, F) {
    let t3692 = t2594 * t3691;
    let t3693 = t446 * t3692;
    let t3695 = t1091 * t713;
    let t3696 = t2354 * t3695;
    let t3697 = t446 * t3696;
    let t3699 = t2360 * t992;
    (t3692, t3693, t3695, t3696, t3697, t3699)
}
