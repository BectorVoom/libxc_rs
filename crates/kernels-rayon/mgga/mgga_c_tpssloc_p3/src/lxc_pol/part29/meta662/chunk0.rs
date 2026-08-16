//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2202/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2202(t22633: f64, t22635: f64, t26214: f64, t3719: f64, t225: f64, t26219: f64, t1985: f64, t7700: f64, t80707: f64, t214: f64, t5318: f64, t6888: f64, t6891: f64) -> (f64, f64, f64, f64, f64) {
    let t90728 = t22633 * t22635 * t26214 * t3719;
    let t90732 = t26219 * t225;
    let t90737 = t1985 * t80707 * t7700;
    let t90739 = t214 * t5318;
    let t90741 = t6888 * t90739 * t6891;
    (t90728, t90732, t90737, t90739, t90741)
}
