//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 890/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk890<F: Float>(t33034: F, t925: F, t2210: F, t33055: F, t574: F, t5935: F, t6639: F, t1391: F, t2185: F, t6630: F, t167: F, t34817: F, t9432: F) -> (F, F, F, F, F, F, F) {
    let t35090 = t33034 * t925;
    let t35091 = t2210 * t35090;
    let t35094 = t33055 * t925;
    let t35095 = t2210 * t35094;
    let t35099 = t574 * t5935 * t6639;
    let t35103 = t2185 * t1391 * t6630;
    let t35107 = t9432 * t167 * t34817;
    (t35090, t35091, t35094, t35095, t35099, t35103, t35107)
}
