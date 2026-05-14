//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 727/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk727<F: Float>(t3455: F, t3578: F, t574: F, t12664: F, t3483: F, t144: F, t3478: F, t4790: F, t604: F) -> (F, F, F, F) {
    let t17398 = t574 * t3578 * t3455;
    let t17401 = t12664 * t3483;
    let t17402 = t144 * t17401;
    let t17406 = t574 * t3578 * t3478;
    let t17409 = t4790 * t604;
    (t17398, t17402, t17406, t17409)
}
