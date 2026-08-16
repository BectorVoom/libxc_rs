//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1272/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1272(t43: f64, t27288: f64, t27308: f64, t27348: f64, t27403: f64, t3029: f64, t1941: f64, t3002: f64, t51: f64, t54: f64, t565: f64, t57: f64, t584: f64, t588: f64, t592: f64, t596: f64, t60: f64, t600: f64, t604: f64, t608: f64, t612: f64, t63: f64, t66: f64, t69: f64, t7984: f64, t9915: f64) -> (f64, f64) {
    let t45 = 0.135e1_f64 < t43;
    let t27405 = t27288 + t27308 + t27348 + t27403;
    let t27406 = piecewise3(t45, t27405, 0.0_f64);
    let t27423 = t3029 * t3029;
    let t27440 = t3002 * t7984 / 3.0_f64 + t57 * t9915 * t584 / 320.0_f64 - t592 * t27406 / 4480.0_f64 + t596 * t27406 / 103680.0_f64 - t600 * t27406 / 2838528.0_f64 + t604 * t27406 / 89456640.0_f64 - t608 * t27406 / 0.31850496e10_f64 + t612 * t27406 / 0.1263403008e12_f64 - t565 * t27406 / 18.0_f64 + t588 * t27406 / 240.0_f64 - t54 * t27423 / 24.0_f64 + t57 * t27423 / 320.0_f64 - t60 * t27423 / 5760.0_f64 + t63 * t27423 / 129024.0_f64 - t66 * t27423 / 3440640.0_f64 + t69 * t27423 / 0.10616832e9_f64 - t1941 * t27423 / 0.37158912e10_f64 + t51 * t27423 / 3.0_f64;
    (t27405, t27440)
}
