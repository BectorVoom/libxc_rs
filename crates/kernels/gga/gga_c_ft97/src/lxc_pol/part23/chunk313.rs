//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 313/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk313<F: Float>(t2360: F, t327: F, t326: F, t2400: F, t14: F, t1576: F, t17: F) -> (F, F, F, F, F, F) {
    let t2928 = t327 * t2360;
    let t2937 = t326 * t326;
    let t2938 = 1.0 / t2937;
    let t2946 = 0.19257444444444444444e0 * t2400;
    let t2998 = 1.0 / t14 / t1576;
    let t2999 = t2998 * t17;
    (t2928, t2937, t2938, t2946, t2998, t2999)
}
