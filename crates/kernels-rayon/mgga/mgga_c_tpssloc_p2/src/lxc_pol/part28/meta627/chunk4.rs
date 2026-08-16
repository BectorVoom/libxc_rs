//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1959/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1959(t23967: f64, t26067: f64, t2032: f64, t22519: f64, t23975: f64, t26055: f64, t26070: f64, t26090: f64, t26945: f64, t6495: f64, t7026: f64, t7035: f64, t7782: f64, t90150: f64, t90177: f64, t90334: f64, t90337: f64, t90340: f64, t90343: f64) -> f64 {
    let t91980 = 80.0_f64 / 9.0_f64 * t23967 * t26067;
    let t91993 = -4.0_f64 / 3.0_f64 * t26055 * t7035 - 10.0_f64 / 3.0_f64 * t23975 * t26090 - 4.0_f64 / 3.0_f64 * t22519 * t7782 - 10.0_f64 / 3.0_f64 * t7026 * t90177 - 4.0_f64 / 3.0_f64 * t6495 * t26945 + t91980 - 5.0_f64 / 3.0_f64 * t7026 * t90334 - 2.0_f64 / 3.0_f64 * t90337 * t2032 - 4.0_f64 / 3.0_f64 * t90340 * t2032 - 4.0_f64 / 3.0_f64 * t90343 * t2032 - 4.0_f64 / 3.0_f64 * t26070 * t7035 - 2.0_f64 / 3.0_f64 * t90150 * t2032;
    t91993
}
