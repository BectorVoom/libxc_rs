//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 369/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk369<F: Float>(t433: F, t1426: F, t1432: F, t1437: F, t1441: F, t16: F, t34: F, t38: F, t441: F, t454: F, tau0: F) -> (F, F) {
    let t1453 = tau0 * t433;
    let t1466 = 40.0 / 9.0 * t1453 * t16 - 50.0 / 9.0 * t454 * t441 + 10.0 / 9.0 * t34 * t1426 + 5.0 / 3.0 * t34 * t1432 + 10.0 / 9.0 * t38 * t1437 + 5.0 / 3.0 * t38 * t1441;
    (t1453, t1466)
}
