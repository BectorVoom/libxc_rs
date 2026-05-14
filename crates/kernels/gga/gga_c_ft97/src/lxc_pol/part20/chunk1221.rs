//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1221/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1221<F: Float>(t113078: F, t113082: F, t113086: F, t113090: F, t113095: F, t113099: F, t113103: F, t113106: F, t113110: F, t113114: F, t99315: F, t99735: F, t10409: F, t2405: F, t28746: F, t6317: F) -> (F, F) {
    let t113116 = 8.0 / 3.0 * t113078 - 8.0 / 9.0 * t113082 - 3.0 * t113086 - 3.0 / 4.0 * t113090 - 3.0 / 4.0 * t113095 - 3.0 / 8.0 * t113099 + 4.0 * t113103 + t113106 + t99735 + 4.0 / 27.0 * t99315 - 2.0 / 3.0 * t113110 + t113114 / 6.0;
    let t113120 = t6317 * t10409 * t28746 * t2405;
    (t113116, t113120)
}
