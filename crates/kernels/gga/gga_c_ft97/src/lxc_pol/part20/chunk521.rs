//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 521/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk521<F: Float>(t7045: F, t840: F, t871: F, t319: F, t7021: F, t1091: F, t1508: F, t835: F, t1212: F, t1234: F, t1476: F) -> (F, F, F, F, F) {
    let t7047 = t840 * t871 * t7045;
    let t7051 = t840 * t319 * t7021;
    let t7055 = t835 * t1508 * t1091;
    let t7059 = t840 * t1508 * t1212;
    let t7062 = t1476 * t1234;
    (t7047, t7051, t7055, t7059, t7062)
}
