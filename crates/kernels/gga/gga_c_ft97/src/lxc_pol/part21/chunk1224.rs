//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1224/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1224<F: Float>(t101983: F, t103: F, t104006: F, t117226: F, t118114: F, t118150: F, t1337: F, t15599: F, t1564: F, t15968: F, t16261: F, t25553: F, t25558: F, t25861: F, t25863: F, t29748: F, t3266: F, t379: F, t4415: F, t5501: F, t5502: F, t5748: F, t6414: F, t8411: F, t91504: F, t925: F) -> (F,) {
    let t118462 = t5501 * t8411 * t5502 * t16261 - t5501 * t91504 * t29748 * t379 / 3.0 - t25558 * t25863 / 9.0 + 4.0 * t118114 - t5501 * t1564 * t101983 * t925 / 9.0 - t104006 + 2.0 * t5501 * t8411 * t25861 * t3266 + 2.0 * t117226 * t103 - t4415 * t5748 - t15968 * t1337 - t15599 * t1337 + t6414 * t25553 / 3.0 - 4.0 * t118150;
    (t118462,)
}
