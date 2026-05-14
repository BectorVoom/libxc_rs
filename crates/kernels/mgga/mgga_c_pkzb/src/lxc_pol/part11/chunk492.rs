//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 492/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk492<F: Float>(t23: F, t2490: F, t2494: F, t2500: F, t2504: F, t434: F, t445: F, t7: F, t974: F, t980: F) -> (F,) {
    let t2507 = -40.0 / 9.0 * t434 * t974 + 10.0 / 9.0 * t7 * t2490 + 5.0 / 3.0 * t7 * t2494 - 40.0 / 9.0 * t980 * t445 + 10.0 / 9.0 * t23 * t2500 - 5.0 / 3.0 * t23 * t2504;
    (t2507,)
}
