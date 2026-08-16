//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 208/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk208(t43: f64, t565: f64, t584: f64, t588: f64, t592: f64, t596: f64, t600: f64, t604: f64, t608: f64, t612: f64, t616: f64, t635: f64, t72: f64, t88: f64) -> f64 {
    let t44 = 0.135e1_f64 <= t43;
    let t639 = piecewise3(t44, -t565 * t584 / 18.0_f64 + t588 * t584 / 240.0_f64 - t592 * t584 / 4480.0_f64 + t596 * t584 / 103680.0_f64 - t600 * t584 / 2838528.0_f64 + t604 * t584 / 89456640.0_f64 - t608 * t584 / 0.31850496e10_f64 + t612 * t584 / 0.1263403008e12_f64, -8.0_f64 / 3.0_f64 * t616 * t88 - 8.0_f64 / 3.0_f64 * t72 * t635);
    t639
}
