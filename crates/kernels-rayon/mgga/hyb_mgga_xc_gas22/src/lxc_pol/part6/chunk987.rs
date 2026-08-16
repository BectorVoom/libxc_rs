//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 987/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk987(t9011: f64, t7037: f64, t7040: f64, t7043: f64, t7082: f64, t7089: f64, t9008: f64, t9029: f64, t9149: f64, t9152: f64, t9155: f64, t9159: f64) -> (f64, f64) {
    let t9161 = 0.59793333333333333334e0_f64 * t9011;
    let t9166 = -0.1898925e1_f64 * t9149 + 0.142419375e1_f64 * t9152 - 0.76790625e-1_f64 * t9155 + 0.39862222222222222223e0_f64 * t9008 + 0.27385555555555555555e0_f64 * t9159 - t9161 + 0.8969e0_f64 * t9029 - t7082 - t7089 + 0.54771111111111111111e0_f64 * t7037 - 0.16431333333333333333e0_f64 * t7040 - 0.16431333333333333333e0_f64 * t7043;
    (t9161, t9166)
}
