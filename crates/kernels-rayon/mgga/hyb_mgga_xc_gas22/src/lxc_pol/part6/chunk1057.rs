//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1057/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1057(t43: f64, t10076: f64, t1211: f64, t1226: f64, t3068: f64, t3108: f64, t3876: f64, t3912: f64, t616: f64, t635: f64, t72: f64, t88: f64, t9953: f64, t9997: f64, t9999: f64) -> f64 {
    let t44 = 0.135e1_f64 <= t43;
    let t10080 = piecewise3(t44, t9953 + t9997, -8.0_f64 / 3.0_f64 * t9999 * t88 - 8.0_f64 / 3.0_f64 * t3876 * t635 - 16.0_f64 / 3.0_f64 * t3068 * t1226 - 16.0_f64 / 3.0_f64 * t1211 * t3108 - 8.0_f64 / 3.0_f64 * t616 * t3912 - 8.0_f64 / 3.0_f64 * t72 * t10076);
    t10080
}
