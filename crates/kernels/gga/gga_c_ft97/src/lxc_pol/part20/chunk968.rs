//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 968/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk968<F: Float>(t299: F, t28927: F, t28981: F, t29029: F, t29422: F, t332: F, t5: F, t7137: F, t113: F, t1275: F, t25504: F, t4377: F, t4382: F, t4385: F, t4391: F, t4395: F, t505: F, t6400: F, t6403: F, t7138: F, t911: F, t992: F) -> (F, F, F, F) {
    let t300 = 10000000.0 <= t299;
    let t29424 = t28927 + t28981 + t29029 + t29422;
    let t29425 = t29424 * t332;
    let t29429 = t5 * t7137;
    let t29451 = piecewise3(t300, 0.0, t5 * t29425 * t113 / 4.0 + t29429 * t911 / 4.0 + t5 * t7138 * t505 / 4.0 + t25504 * t1275 / 4.0 + t6403 * t4377 / 4.0 + t6403 * t4382 / 4.0 + t6403 * t4385 / 4.0 + t5 * t6400 * t992 / 4.0 + t6403 * t4391 / 4.0 - t6403 * t4395 / 2.0);
    (t29424, t29425, t29429, t29451)
}
