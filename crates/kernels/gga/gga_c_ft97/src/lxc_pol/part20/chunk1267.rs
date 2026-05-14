//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1267/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1267<F: Float>(t29055: F, t56110: F, t113060: F, t113035: F, t113039: F, t113043: F, t113046: F, t113049: F, t113053: F, t113058: F, t113064: F, t113068: F, t113073: F, t113105: F, t113078: F, t113082: F, t113086: F, t113090: F, t113095: F, t113099: F, t113103: F, t113110: F, t113114: F, t99313: F, t99315: F) -> (F, F, F) {
    let t114271 = t56110 * t29055;
    let t114282 = 2.0 / 27.0 * t113060;
    let t114285 = -t113035 / 18.0 - t113039 / 18.0 - t113043 / 9.0 - t113046 / 9.0 - 2.0 / 9.0 * t113049 + t113053 / 27.0 + 5.0 / 81.0 * t113058 + t114282 + 2.0 / 9.0 * t113064 - 2.0 / 9.0 * t113068 + t113073;
    let t114292 = 2.0 / 3.0 * t113105;
    let t114297 = 8.0 / 9.0 * t113078 - 8.0 / 27.0 * t113082 - t113086 - t113090 / 4.0 - t113095 / 4.0 - t113099 / 8.0 + 4.0 / 3.0 * t113103 + t114292 + t99313 / 9.0 + 4.0 / 81.0 * t99315 - 2.0 / 9.0 * t113110 + t113114 / 18.0;
    (t114271, t114285, t114297)
}
