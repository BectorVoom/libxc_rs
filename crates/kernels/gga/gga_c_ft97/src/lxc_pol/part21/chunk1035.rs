//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1035/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1035<F: Float>(t1642: F, t5507: F, t1286: F, t1309: F, t7943: F, t1637: F, t5623: F, t458: F, t5494: F, t1285: F, t1771: F, t5504: F, t22873: F, t378: F, t22870: F, t5495: F) -> (F, F, F, F, F, F, F, F) {
    let t93871 = t1642 * t5507;
    let t93946 = 14.0 / 81.0 * t1286 * t7943 * t1309;
    let t94024 = t1286 * t1637 * t5623;
    let t94032 = t5494 * t458;
    let t94035 = t1285 * t1771;
    let t94036 = t94035 * t5504;
    let t94038 = t378 * t22873;
    let t94046 = t5495 * t22870;
    (t93871, t93946, t94024, t94032, t94035, t94036, t94038, t94046)
}
