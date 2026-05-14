//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 736/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk736<F: Float>(t979: F, t28: F, t3315: F, t3319: F, t3330: F, t3334: F, t34: F, t38: F, t984: F, t991: F, tau1: F) -> (F, F) {
    let t3347 = tau1 * t979;
    let t3356 = 10.0 / 9.0 * t34 * t3315 + 5.0 / 3.0 * t34 * t3319 + 40.0 / 9.0 * t3347 * t28 - 50.0 / 9.0 * t991 * t984 + 10.0 / 9.0 * t38 * t3330 + 5.0 / 3.0 * t38 * t3334;
    (t3347, t3356)
}
