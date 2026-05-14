//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1227/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1227<F: Float>(t1395: F, t15625: F, t16586: F, t16591: F, t17539: F, t17542: F, t17545: F, t24157: F, t27440: F, t30566: F, t363: F, t3660: F, t3665: F, t3668: F, t4431: F, t4890: F, t5: F, t5982: F, t5985: F) -> (F,) {
    let t118536 = t5985 * t16586 / 4.0 + t5985 * t17539 / 2.0 + t5985 * t17542 + t5 * t5982 * t4431 / 4.0 + t5985 * t17545 / 4.0 + t27440 * t3665 / 2.0 + t5 * t1395 * t15625 / 4.0 + t5985 * t16591 / 2.0 + t5 * t30566 * t363 / 4.0 + t24157 * t4890 / 4.0 + t27440 * t3660 / 2.0 + t27440 * t3668 / 2.0;
    (t118536,)
}
