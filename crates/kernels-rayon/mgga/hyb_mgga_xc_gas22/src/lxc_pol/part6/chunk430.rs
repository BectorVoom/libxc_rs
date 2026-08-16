//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 430/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk430(t1880: f64, t1913: f64, t1941: f64, t51: f64, t54: f64, t565: f64, t57: f64, t588: f64, t592: f64, t596: f64, t60: f64, t600: f64, t604: f64, t608: f64, t612: f64, t63: f64, t66: f64, t69: f64) -> f64 {
    let t1946 = t51 * t1880 / 6.0_f64 - t565 * t1913 / 18.0_f64 - t54 * t1880 / 48.0_f64 + t588 * t1913 / 240.0_f64 + t57 * t1880 / 640.0_f64 - t592 * t1913 / 4480.0_f64 - t60 * t1880 / 11520.0_f64 + t596 * t1913 / 103680.0_f64 + t63 * t1880 / 258048.0_f64 - t600 * t1913 / 2838528.0_f64 - t66 * t1880 / 6881280.0_f64 + t604 * t1913 / 89456640.0_f64 + t69 * t1880 / 0.21233664e9_f64 - t608 * t1913 / 0.31850496e10_f64 - t1941 * t1880 / 0.74317824e10_f64 + t612 * t1913 / 0.1263403008e12_f64;
    t1946
}
