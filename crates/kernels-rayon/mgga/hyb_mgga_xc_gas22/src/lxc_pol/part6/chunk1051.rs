//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1051/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1051(t43: f64, t9914: f64, t3844: f64, t51: f64, t3827: f64, t592: f64, t54: f64, t596: f64, t57: f64, t3002: f64, t3029: f64, t3032: f64, t3037: f64, t565: f64, t584: f64, t588: f64, t600: f64, t604: f64, t608: f64, t612: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t45 = 0.135e1_f64 < t43;
    let t9915 = piecewise3(t45, t9914, 0.0_f64);
    let t9934 = t51 * t3844;
    let t9937 = t592 * t3827;
    let t9942 = t54 * t3844;
    let t9945 = t596 * t3827;
    let t9950 = t57 * t3844;
    let t9953 = -t592 * t9915 / 4480.0_f64 + t596 * t9915 / 103680.0_f64 - t600 * t9915 / 2838528.0_f64 + t604 * t9915 / 89456640.0_f64 - t608 * t9915 / 0.31850496e10_f64 + t612 * t9915 / 0.1263403008e12_f64 - t565 * t9915 / 18.0_f64 + t588 * t9915 / 240.0_f64 + t3002 * t3029 / 3.0_f64 + t9934 * t584 / 6.0_f64 + t9937 * t584 / 8.0_f64 - t3032 * t3029 / 24.0_f64 - t9942 * t584 / 48.0_f64 - t9945 * t584 / 80.0_f64 + t3037 * t3029 / 320.0_f64 + t9950 * t584 / 640.0_f64;
    (t9915, t9934, t9937, t9942, t9945, t9950, t9953)
}
