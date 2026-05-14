//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1419/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1419<F: Float>(t299: F, t125442: F, t128779: F, t107750: F, t107751: F, t125415: F, t13: F, t31334: F, t31992: F) -> (F,) {
    let t300 = 10000000.0 <= t299;
    let t128781 = piecewise3(t300, 0.0, t125442 + t128779);
    let tv4rho3sigma8 = t107750 + t107751 + t31334 + t31992 + t13 * (t125415 + t128781);
    (tv4rho3sigma8,)
}
