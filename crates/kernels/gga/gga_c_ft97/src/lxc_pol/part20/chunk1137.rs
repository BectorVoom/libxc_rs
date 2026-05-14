//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1137/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1137<F: Float>(t108086: F, t108091: F, t108095: F, t108099: F, t108104: F, t108107: F, t108112: F, t108118: F, t110067: F, t110068: F, t110069: F, t110077: F, t108138: F, t108157: F, t108160: F, t108122: F, t108126: F, t108130: F, t108134: F, t108137: F, t108140: F, t108145: F, t108150: F, t108155: F) -> (F, F) {
    let t110079 = -t110067 - t110068 - t110069 + 4.0 / 9.0 * t108086 + t108091 / 3.0 + 2.0 / 3.0 * t108095 + 2.0 / 3.0 * t108099 + t108104 / 3.0 - t108107 / 9.0 + 2.0 / 9.0 * t108112 - t110077 - t108118 / 3.0;
    let t110085 = t108138 / 27.0;
    let t110089 = t108157 / 18.0;
    let t110090 = t108160 / 18.0;
    let t110091 = 8.0 / 9.0 * t108122 - t108126 / 3.0 - t108130 / 3.0 - t108134 / 3.0 - t108137 / 3.0 - t110085 + 4.0 / 81.0 * t108140 - 2.0 / 9.0 * t108145 - t108150 + t108155 / 3.0 - t110089 - t110090;
    (t110079, t110091)
}
