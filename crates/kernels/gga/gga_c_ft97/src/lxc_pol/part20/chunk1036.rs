//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1036/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1036<F: Float>(t25180: F, t99312: F, t1485: F, t1771: F, t6320: F, t6336: F, t8232: F, t1882: F, t25142: F, t1476: F, t9577: F, t25039: F, t25149: F, t1486: F, t25136: F, t681: F) -> (F, F, F, F, F, F, F, F, F) {
    let t99313 = t99312 * t25180;
    let t99314 = t1485 * t1771;
    let t99315 = t99314 * t6320;
    let t99317 = t8232 * t6336;
    let t99320 = t1882 * t25142;
    let t99322 = t1476 * t9577;
    let t99327 = t1882 * t25039;
    let t99329 = t1882 * t25149;
    let t99332 = t1486 * t681 * t25136;
    (t99313, t99314, t99315, t99317, t99320, t99322, t99327, t99329, t99332)
}
