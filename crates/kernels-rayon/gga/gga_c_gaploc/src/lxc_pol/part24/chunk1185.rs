//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1185/1439 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1185(t31501: f64, t475: f64, t23609: f64, t3327: f64, t10546: f64, t31548: f64, t4807: f64, t10156: f64, t10157: f64, t1064: f64, t1266: f64, t2268: f64, t31766: f64, t31772: f64, t31777: f64, t31783: f64, t31786: f64, t31788: f64, t31790: f64, t31792: f64, t31796: f64, t31799: f64, t6305: f64) -> (f64, f64) {
    let t31800 = t31501 * t475;
    let t31805 = 0.12646669615856066488e-1_f64 * t23609 * t3327;
    let t31811 = 0.39837009289946609438e0_f64 * t31548 * t10546 * t4807;
    let t31814 = t31766 - t31772 + t31777 - t31783 + t31786 - t31788 - t31790 - t31792 - t31796 + t31799 - 0.1707300398140568976e0_f64 * t2268 * t1064 * t31800 + t31805 - 0.85365019907028448797e-1_f64 * t2268 * t10156 * t1266 + t31811 - 0.1707300398140568976e0_f64 * t6305 * t10157;
    (t31800, t31814)
}
