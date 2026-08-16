//! GGA_C_GAPC lxc pol — lxc_pol part 30 (v4rho2sigma2_9) CSE chunk 1138/1331 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part30_v4rho2sigma2_9_chunk1138<F: Float>(t11849: F, t1952: F, t919: F, t11761: F, t34005: F, t3775: F, t9586: F, t11428: F, t667: F, t3326: F, t29576: F, t29582: F) -> (F, F, F, F, F, F) {
    let t34033 = t11849 * t1952 * t919;
    let t34036 = t34005 * t11761;
    let t34038 = t3775 * t9586;
    let t34040 = t667 * t11428;
    let t34041 = t34040 * t3326;
    let t34043 = t29576 * t34041 * t29582;
    (t34033, t34036, t34038, t34040, t34041, t34043)
}
