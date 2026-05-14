//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1211/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1211<F: Float>(t29923: F, t8392: F, t102848: F, t103068: F, t103510: F, t103625: F, t103632: F, t103640: F, t103647: F, t103649: F, t11490: F, t115271: F, t116099: F, t16328: F, t1852: F, t1871: F, t1901: F, t29986: F, t3205: F, t3271: F, t446: F, t452: F, t4551: F, t4623: F, t47659: F, t5617: F, t5635: F, t83: F, t8466: F, t92062: F) -> (F,) {
    let t117929 = t8392 * t29923;
    let t117952 = -4.0 / 3.0 * t1901 * t11490 * t103068 * t3271 + 4.0 / 81.0 * t92062 + t103625 - t103632 + 2.0 / 3.0 * t446 * t1871 * t4623 * t5635 + 4.0 / 9.0 * t117929 - 2.0 / 3.0 * t446 * t83 * t116099 + t103640 + t103647 + t103649 - 2.0 / 3.0 * t446 * t452 * t8466 * t29986 - 2.0 / 3.0 * t446 * t452 * t1852 * t5617 * t4551 + 2.0 / 3.0 * t446 * t83 * t115271 + 2.0 / 9.0 * t1901 * t102848 * t3205 + 4.0 / 9.0 * t47659 * t103510 * t16328;
    (t117952,)
}
