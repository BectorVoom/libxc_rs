//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1957/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1957(t23967: f64, t26063: f64, t7432: f64, t84241: f64, t2032: f64, t22493: f64, t24001: f64, t26009: f64, t26028: f64, t26073: f64, t32332: f64, t7035: f64, t7428: f64, t7782: f64, t84222: f64, t84224: f64, t84229: f64, t84245: f64, t90205: f64, t9239: f64) -> f64 {
    let t91921 = 80.0_f64 / 9.0_f64 * t23967 * t26063;
    let t91922 = t84241 * t7432;
    let t91938 = -2.0_f64 / 3.0_f64 * t90205 * t2032 - 4.0_f64 / 3.0_f64 * t26073 * t7035 + t91921 - 440.0_f64 / 27.0_f64 * t91922 + 10.0_f64 / 3.0_f64 * t84245 * t7432 - 8.0_f64 / 9.0_f64 * t84222 - 16.0_f64 / 9.0_f64 * t84224 + 176.0_f64 / 27.0_f64 * t84229 - 40.0_f64 * t9239 * t32332 * t26009 + 2.0_f64 / 3.0_f64 * t26028 * t7035 + t7428 * t24001 / 3.0_f64 + t22493 * t7782 / 3.0_f64;
    t91938
}
