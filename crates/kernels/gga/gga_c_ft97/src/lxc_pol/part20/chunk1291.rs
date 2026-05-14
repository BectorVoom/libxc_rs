//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1291/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1291<F: Float>(t29424: F, t5: F, t14395: F, t14412: F, t14569: F, t14576: F, t14579: F, t14593: F, t25504: F, t29429: F, t2958: F, t2963: F, t4377: F, t4385: F, t4391: F, t4395: F, t6403: F, t911: F) -> (F,) {
    let t115081 = t5 * t29424;
    let t115093 = t25504 * t4377 / 2.0 + t6403 * t14412 / 4.0 + 3.0 / 2.0 * t6403 * t14593 + t25504 * t4391 / 2.0 - t25504 * t4395 + t6403 * t14579 / 4.0 + t29429 * t2963 / 4.0 + t115081 * t911 / 2.0 + t29429 * t2958 / 4.0 + t25504 * t4385 / 2.0 + t6403 * t14576 / 2.0 + t6403 * t14395 / 4.0 - t6403 * t14569;
    (t115093,)
}
