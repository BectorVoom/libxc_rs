//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 763/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk763<F: Float>(t10683: F, t19399: F, t319: F, t5225: F, t875: F, t2862: F, t871: F, t1212: F, t4129: F) -> (F, F, F, F) {
    let t19401 = t10683 * t319 * t19399;
    let t19404 = t5225 * t875;
    let t19406 = t2862 * t871 * t19404;
    let t19409 = t1212 * t4129;
    (t19401, t19404, t19406, t19409)
}
