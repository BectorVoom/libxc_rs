//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 589/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk589<F: Float>(t27: F, t3333: F, t23: F, t28: F, t3315: F, t3319: F, t3324: F, t3330: F, t7: F, t980: F, t984: F) -> (F, F) {
    let t3334 = t27 * t3333;
    let t3337 = 10.0 / 9.0 * t7 * t3315 + 5.0 / 3.0 * t7 * t3319 + 88.0 / 9.0 * t3324 * t28 - 80.0 / 9.0 * t980 * t984 + 10.0 / 9.0 * t23 * t3330 + 5.0 / 3.0 * t23 * t3334;
    (t3334, t3337)
}
