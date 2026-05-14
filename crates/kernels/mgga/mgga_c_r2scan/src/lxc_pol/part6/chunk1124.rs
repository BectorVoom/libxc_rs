//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1124/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1124<F: Float>(t20342: F, t2115: F, t1604: F, t1577: F, t1632: F, t551: F, t6370: F, t6449: F, t6450: F, t2196: F, t6334: F, t5100: F, t5169: F, t1610: F, t2201: F, t5128: F) -> (F, F, F, F, F, F, F) {
    let t20343 = t2115 * t20342;
    let t20344 = t1604 * t20343;
    let t20348 = t1577 * t551 * t1632 * t6370;
    let t20357 = t6449 * t551 * t1632 * t6450;
    let t20361 = t2196 * t551 * t1632 * t6334;
    let t20363 = t5100 * t5169;
    let t20366 = t2201 * t1610 * t5128;
    (t20343, t20344, t20348, t20357, t20361, t20363, t20366)
}
