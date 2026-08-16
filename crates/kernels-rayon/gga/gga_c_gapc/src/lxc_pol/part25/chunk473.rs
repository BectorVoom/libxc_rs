//! GGA_C_GAPC lxc pol — lxc_pol part 25 (v4rho2sigma2_4) CSE chunk 473/1444 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part25_v4rho2sigma2_4_chunk473(t2232: f64, t2598: f64, t2597: f64, t604: f64, t924: f64, t819: f64, t923: f64, t181: f64, t891: f64, t2299: f64, t314: f64, t298: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t2599 = t2598 * t2232;
    let t2600 = t2597 * t2599;
    let t2603 = t604 * t924;
    let t2606 = t819 * t923;
    let t2607 = t181 * t2606;
    let t2610 = t819 * t891;
    let t2611 = t181 * t2610;
    let t2614 = t314 * t2299;
    let t2615 = t298 * t2614;
    (t2599, t2600, t2603, t2607, t2611, t2614, t2615)
}
