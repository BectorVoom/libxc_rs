//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2300/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2300(t23384: f64, t28492: f64, t28500: f64, t1599: f64, t1625: f64, t18071: f64, t23327: f64, t23346: f64, t25429: f64, t25431: f64, t25712: f64, t28684: f64, t28691: f64, t343: f64, t6687: f64, t6690: f64, t6771: f64, t7553: f64, t83444: f64, t88050: f64, t88105: f64, t89630: f64, t89648: f64, t89653: f64) -> f64 {
    let t99948 = t23384 * t28492;
    let t99956 = t23384 * t28500;
    let t99959 = 0.43864908449286038307e-1_f64 * t23346 * t28684 + 0.16449340668482264365e-1_f64 * t6687 * t1599 * t89648 + t89630 - 0.16449340668482264365e-1_f64 * t6687 * t25712 * t343 * t1625 * t6690 + 0.21932454224643019153e-1_f64 * t23346 * t28691 - 6.0_f64 * t6771 * t18071 - 0.97477574331746751795e-2_f64 * t23346 * t28492 + 0.12184696791468343974e-2_f64 * t99948 + 0.73108180748810063845e-2_f64 * t25429 * t88050 * t25431 - 0.54831135561607547884e-2_f64 * t23327 * t88105 * t7553 - 0.18277045187202515961e-2_f64 * t99956 - 0.18277045187202515961e-2_f64 * t83444 - t89653;
    t99959
}
