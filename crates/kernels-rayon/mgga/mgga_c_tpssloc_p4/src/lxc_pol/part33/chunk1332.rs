//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1332/1415 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1332(t22986: f64, t25249: f64, t5612: f64, t6646: f64, t1510: f64, t98389: f64, t98422: f64, t20756: f64, t6637: f64, t6638: f64, t81984: f64, t1888: f64, t22996: f64, t2632: f64, t67358: f64) -> (f64, f64, f64, f64, f64) {
    let t105661 = t22986 * t6646 * t25249 * t5612;
    let t105665 = t22986 * t6646 * t98389 * t1510;
    let t105669 = t22986 * t6646 * t98422 * t1510;
    let t105674 = t81984 * t6637 * t6638 * t20756;
    let t105685 = t1888 * t22996 * t67358 * t2632;
    (t105661, t105665, t105669, t105674, t105685)
}
