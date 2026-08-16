//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 724/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk724<F: Float>(t11401: F, t355: F, t3001: F, t89: F, t1755: F, t3013: F, t28: F, t1586: F, t3103: F, t432: F, t3014: F, t376: F) -> (F, F, F, F, F) {
    let t11402 = t11401 * t355;
    let t11404 = t89 * t11402 * t3001;
    let t11406 = t3013 * t1755;
    let t11408 = t89 * t28 * t11406;
    let t11410 = t1586 * t3103;
    let t11411 = t11410 * t432;
    let t11413 = t89 * t28 * t11411;
    let t11416 = t89 * t376 * t3014;
    (t11402, t11404, t11408, t11413, t11416)
}
