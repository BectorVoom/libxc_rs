//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 645/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk645(t43: f64, t1211: f64, t1226: f64, t3067: f64, t3068: f64, t3108: f64, t616: f64, t635: f64, t72: f64, t88: f64, t29: f64, t125: f64, t26: f64) -> (f64, f64, f64, f64) {
    let t44 = 0.135e1_f64 <= t43;
    let t3112 = piecewise3(t44, t3067, -8.0_f64 / 3.0_f64 * t1211 * t635 - 8.0_f64 / 3.0_f64 * t616 * t1226 - 8.0_f64 / 3.0_f64 * t3068 * t88 - 8.0_f64 / 3.0_f64 * t72 * t3108);
    let t3113 = t29 * t3112;
    let t3114 = t3113 * t125;
    let t3115 = t26 * t3114;
    (t3112, t3113, t3114, t3115)
}
