//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 654/869 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk654<F: Float>(t1922: F, t447: F, t925: F, t1871: F, t3266: F, t499: F, t3103: F, t432: F, t110: F, t1755: F, t942: F, t1825: F, t3271: F, t452: F, t1882: F, t3263: F) -> (F, F, F, F, F, F) {
    let t11513 = t447 * t1922 * t925;
    let t11517 = t1871 * t499 * t3266;
    let t11520 = t3103 * t432;
    let t11522 = t1871 * t110 * t11520;
    let t11525 = t942 * t1755;
    let t11527 = t1871 * t110 * t11525;
    let t11531 = t452 * t1825 * t3271;
    let t11535 = 2.0 / 9.0 * t1882 * t3263;
    (t11513, t11517, t11522, t11527, t11531, t11535)
}
