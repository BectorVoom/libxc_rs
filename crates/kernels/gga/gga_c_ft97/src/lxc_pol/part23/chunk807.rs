//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 807/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk807<F: Float>(t24330: F, t6056: F, t6055: F, t444: F, t6041: F, t3789: F) -> (F, F, F, F) {
    let t24357 = t24330 * t6056;
    let t24358 = t6055 * t24357;
    let t24360 = t6041 * t444;
    let t24361 = t3789 * t24360;
    (t24357, t24358, t24360, t24361)
}
