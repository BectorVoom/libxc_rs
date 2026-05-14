//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1415/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1415<F: Float>(t31962: F, t92: F, t10697: F, t112649: F, t126217: F, t126221: F, t126225: F, t126229: F, t126646: F, t128334: F, t128362: F, t128448: F, t25446: F, t2665: F, t312: F, t31835: F, t31841: F, t4969: F, t5305: F, t6216: F, t6219: F, t6391: F, t875: F) -> (F,) {
    let t128667 = t31962 * t92;
    let t128684 = 8.0 * t126646 + 8.0 * t126229 + 8.0 * t126225 - t112649 + 4.0 * t126221 + 4.0 * t126217 - 2.0 * t128362 - t128667 * t6219 / 18.0 - 12.0 * t10697 * t31841 * t875 + 2.0 * t128334 * t312 - t5305 * t6391 + t6216 * t2665 * t25446 * t4969 / 9.0 + 4.0 * t128448 - 24.0 * t10697 * t31835 * t875;
    (t128684,)
}
