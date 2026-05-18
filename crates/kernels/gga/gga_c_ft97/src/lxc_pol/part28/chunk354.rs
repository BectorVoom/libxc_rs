//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 354/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk354<F: Float>(t428: F, t72: F, t5579: F, t391: F, t444: F, t1302: F, t173: F, t71: F) -> (F, F, F, F, F) {
    let t5599 = t72 * t428;
    let t5600 = t5579 * t5599;
    let t5603 = t444 * t391;
    let t5604 = t5603 * t1302;
    let t5607 = t173 * t71;
    (t5599, t5600, t5603, t5604, t5607)
}
