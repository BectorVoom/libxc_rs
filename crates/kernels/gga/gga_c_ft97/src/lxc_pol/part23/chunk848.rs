//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 848/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk848<F: Float>(t25462: F, t6219: F, t317: F, t6260: F, t1478: F, t2399: F, t1466: F, t6262: F, t681: F, t6266: F, t5: F, t6399: F, t1434: F, t6891: F, t668: F, t6837: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t25463 = t25462 * t6219;
    let t25465 = t6260 * t317;
    let t25485 = t2399 * t1478;
    let t25487 = 2.0 / 27.0 * t1466 * t25485;
    let t25488 = t681 * t6262;
    let t25489 = t1466 * t25488;
    let t25491 = t681 * t6266;
    let t25492 = t1466 * t25491;
    let t25504 = t5 * t6399;
    let t27466 = t1434 * t681 * t6891;
    let t27468 = t6837 * t668;
    (t25463, t25465, t25485, t25487, t25488, t25489, t25491, t25492, t25504, t27466, t27468)
}
