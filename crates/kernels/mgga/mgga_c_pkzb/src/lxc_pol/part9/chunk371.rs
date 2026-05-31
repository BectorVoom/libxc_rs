//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 371/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk371<F: Float>(t433: F, t1426: F, t1432: F, t1437: F, t1441: F, t16: F, t34: F, t38: F, t441: F, t454: F, tau0: F) -> (F, F) {
    let t1453 = tau0 * t433;
    let t1466 = F::cast_from(40.0_f64) / F::cast_from(9.0_f64) * t1453 * t16 - F::cast_from(50.0_f64) / F::cast_from(9.0_f64) * t454 * t441 + F::cast_from(10.0_f64) / F::cast_from(9.0_f64) * t34 * t1426 + F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t34 * t1432 + F::cast_from(10.0_f64) / F::cast_from(9.0_f64) * t38 * t1437 + F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t38 * t1441;
    (t1453, t1466)
}
