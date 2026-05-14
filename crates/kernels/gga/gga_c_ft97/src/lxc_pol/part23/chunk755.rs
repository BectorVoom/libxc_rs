//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 755/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk755<F: Float>(t1091: F, t4129: F, t2665: F, t446: F, t1212: F, t3746: F, t3281: F, t4965: F, t824: F, t10409: F, t17766: F, t2857: F, t1882: F, t5214: F, t2680: F, t5299: F) -> (F, F, F, F, F, F, F, F, F) {
    let t19263 = t1091 * t4129;
    let t19264 = t2665 * t19263;
    let t19265 = t446 * t19264;
    let t19267 = t3746 * t1212;
    let t19268 = t2665 * t19267;
    let t19269 = t3281 * t19268;
    let t19271 = t4965 * t824;
    let t19272 = t10409 * t19271;
    let t19273 = t446 * t19272;
    let t19275 = t2857 * t17766;
    let t19276 = t446 * t19275;
    let t19278 = t1882 * t5214;
    let t19280 = t2680 * t5299;
    (t19263, t19265, t19267, t19269, t19271, t19273, t19276, t19278, t19280)
}
