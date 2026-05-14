//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 381/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk381<F: Float>(t1419: F, t1426: F, t1432: F, t1437: F, t1441: F, t16: F, t23: F, t434: F, t441: F, t7: F) -> (F,) {
    let t1444 = 88.0 / 9.0 * t1419 * t16 - 80.0 / 9.0 * t434 * t441 + 10.0 / 9.0 * t7 * t1426 + 5.0 / 3.0 * t7 * t1432 + 10.0 / 9.0 * t23 * t1437 + 5.0 / 3.0 * t23 * t1441;
    (t1444,)
}
