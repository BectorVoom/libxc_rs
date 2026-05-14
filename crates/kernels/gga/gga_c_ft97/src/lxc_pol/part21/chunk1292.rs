//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1292/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1292<F: Float>(t28: F, t3526: F, t586: F, t5890: F, t6615: F, t105847: F, t105849: F, t105856: F, t105863: F, t120055: F, t120059: F, t120062: F, t120066: F, t120070: F, t120074: F, t96119: F) -> (F, F) {
    let t120080 = t5890 * t28 * t586 * t6615 * t3526;
    let t120082 = 3.0 / 2.0 * t120055 - 6.0 * t120059 - t120062 + t105847 + t105849 - t120066 / 2.0 - 3.0 / 4.0 * t120070 + t96119 + 2.0 * t120074 - 4.0 / 9.0 * t105856 + t105863 + t120080 / 2.0;
    (t120080, t120082)
}
