//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 904/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk904(t6090: f64, t6161: f64, t7931: f64, t7955: f64, t9772: f64, t9774: f64, t9777: f64, t9782: f64, t9797: f64, t9799: f64, t9806: f64, t9808: f64) -> f64 {
    let t9810 = 0.142419375e1_f64 * t9772 - 0.1898925e1_f64 * t9774 - 0.9494625e0_f64 * t9777 + 0.1898925e1_f64 * t9799 - t6161 + 0.39862222222222222223e0_f64 * t6090 + 0.79724444444444444445e0_f64 * t7955 - t7931 - 0.29896666666666666667e0_f64 * t9782 + 0.8969e0_f64 * t9797 - 0.76790625e-1_f64 * t9806 + 0.3071625e0_f64 * t9808;
    t9810
}
