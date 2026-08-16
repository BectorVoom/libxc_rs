//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 911/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk911(t1193: f64, t6004: f64, t3029: f64, t60: f64, t63: f64, t66: f64, t69: f64, t1196: f64, t1198: f64, t1200: f64, t1202: f64, t1204: f64, t1206: f64, t1208: f64, t1880: f64, t1913: f64, t3037: f64, t3042: f64, t3047: f64, t3052: f64, t584: f64) -> (f64, f64) {
    let t8036 = t6004 * t1193;
    let t8041 = t60 * t3029;
    let t8046 = t63 * t3029;
    let t8051 = t66 * t3029;
    let t8056 = t69 * t3029;
    let t8059 = -2.0_f64 / 3.0_f64 * t1196 * t1880 + t1198 * t1880 / 8.0_f64 - t1200 * t1880 / 80.0_f64 + t1202 * t1880 / 1152.0_f64 - t1204 * t1880 / 21504.0_f64 + t1206 * t1880 / 491520.0_f64 - t1208 * t1880 / 13271040.0_f64 + t8036 * t1880 / 412876800.0_f64 + t3037 * t1913 / 640.0_f64 - t8041 * t584 / 5760.0_f64 - t3042 * t1913 / 11520.0_f64 + t8046 * t584 / 129024.0_f64 + t3047 * t1913 / 258048.0_f64 - t8051 * t584 / 3440640.0_f64 - t3052 * t1913 / 6881280.0_f64 + t8056 * t584 / 0.10616832e9_f64;
    (t8036, t8059)
}
