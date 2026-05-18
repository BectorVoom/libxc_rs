//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 895/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk895<F: Float>(t226: F, t2383: F, t17817: F, t3725: F, t6: F, t4952: F, t2393: F, t4947: F, t3771: F, t1109: F, t4951: F, t688: F) -> (F, F, F, F) {
    let t17818 = t2383 * t226;
    let t17819 = t17817 * t17818;
    let t17820 = t3725 * t6;
    let t17821 = t17820 * t4952;
    let t17824 = t4947 * t2393;
    let t17825 = t3771 * t17824;
    let t17827 = t4951 * t1109 * t688;
    (t17819, t17821, t17825, t17827)
}
