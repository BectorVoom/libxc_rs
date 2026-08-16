//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1245/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1245(t5: f64, t108708: f64, t108727: f64, t108743: f64, t108763: f64, t112: f64, t105159: f64, t105201: f64, t106902: f64, t107504: f64, t19596: f64, t1983: f64, t20085: f64, t20296: f64, t2036: f64, t2075: f64, t2095: f64, t22425: f64, t22574: f64, t24432: f64, t26558: f64, t28030: f64, t28969: f64, t29205: f64, t29211: f64, t29222: f64, t29247: f64, t29377: f64, t29380: f64, t4028: f64, t510: f64, t5161: f64, t5450: f64, t5493: f64, t652: f64, t7170: f64, t74064: f64, t7458: f64, t7685: f64, t7802: f64, t7890: f64, t7940: f64, t91655: f64) -> (f64, f64) {
    let t7 = piecewise3(0.0_f64 < t5, t5, -t5);
    let t8 = -t7 <= -0.999999999999e0_f64;
    let t108766 = piecewise3(t8, 0.0_f64, t108708 + t108727 + t108743 + t108763);
    let t108767 = t108766 * t112;
    let t108780 = -9.0_f64 * t22574 * t24432 * t107504 + 6.0_f64 * t1983 * t7940 * t20085 + 9.0_f64 * t7685 * t28969 - 12.0_f64 * t4028 * t29205 - 6.0_f64 * t28030 * t7802 - 6.0_f64 * t7458 * t29211 - 3.0_f64 * t7685 * t29222 - 9.0_f64 * t22574 * t24432 * t106902 - 6.0_f64 * t1983 * t2095 * t74064 + 3.0_f64 * t1983 * t7170 * t105159 - 18.0_f64 * t91655 * t29247 + 18.0_f64 * t22574 * t26558 * t105201 - 6.0_f64 * t652 * t7890 * t5493 - 6.0_f64 * t20296 * t2075 - t108767 * t510 - 3.0_f64 * t5450 * t7890 - t2036 * t22425 - 3.0_f64 * t1983 * t7940 * t19596 + 18.0_f64 * t7685 * t29380 - 3.0_f64 * t1983 * t29377 * t5161;
    (t108767, t108780)
}
