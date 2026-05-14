//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 623/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk623<F: Float>(t23443: F, t3425: F, t569: F, t5975: F, t925: F, t3578: F, t574: F, t5869: F, t2142: F, t6639: F, t1053: F, t5842: F, t605: F, t1359: F, t3408: F, t167: F, t2185: F) -> (F, F, F, F, F, F, F, F) {
    let t26868 = t23443 * t3425;
    let t26872 = t569 * t5975 * t925;
    let t26876 = t574 * t3578 * t5869;
    let t26880 = t574 * t2142 * t6639;
    let t26883 = t5842 * t1053;
    let t26885 = t574 * t605 * t26883;
    let t26888 = t1359 * t3408;
    let t26890 = t2185 * t167 * t26888;
    (t26868, t26872, t26876, t26880, t26883, t26885, t26888, t26890)
}
