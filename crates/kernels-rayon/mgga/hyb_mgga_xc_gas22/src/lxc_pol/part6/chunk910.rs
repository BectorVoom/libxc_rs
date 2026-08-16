//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 910/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk910(t43: f64, t7983: f64, t1941: f64, t3029: f64, t51: f64, t54: f64, t57: f64, t1913: f64, t3002: f64, t3032: f64, t3057: f64, t3062: f64, t565: f64, t584: f64, t588: f64, t592: f64, t596: f64, t600: f64, t604: f64, t608: f64, t612: f64) -> (f64, f64) {
    let t45 = 0.135e1_f64 < t43;
    let t7984 = piecewise3(t45, t7983, 0.0_f64);
    let t8003 = t1941 * t3029;
    let t8008 = t51 * t3029;
    let t8013 = t54 * t3029;
    let t8018 = t57 * t3029;
    let t8021 = t596 * t7984 / 103680.0_f64 - t600 * t7984 / 2838528.0_f64 + t604 * t7984 / 89456640.0_f64 - t608 * t7984 / 0.31850496e10_f64 + t612 * t7984 / 0.1263403008e12_f64 - t565 * t7984 / 18.0_f64 + t588 * t7984 / 240.0_f64 - t592 * t7984 / 4480.0_f64 + t3057 * t1913 / 0.21233664e9_f64 - t8003 * t584 / 0.37158912e10_f64 - t3062 * t1913 / 0.74317824e10_f64 + t8008 * t584 / 3.0_f64 + t3002 * t1913 / 6.0_f64 - t8013 * t584 / 24.0_f64 - t3032 * t1913 / 48.0_f64 + t8018 * t584 / 320.0_f64;
    (t7984, t8021)
}
