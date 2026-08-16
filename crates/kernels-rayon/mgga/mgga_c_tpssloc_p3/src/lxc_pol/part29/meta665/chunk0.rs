//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2210/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2210(t16052: f64, t1992: f64, t22897: f64, t26392: f64, t80670: f64, t16419: f64, t6976: f64, t22705: f64, t26422: f64, t81228: f64, t16040: f64, t22633: f64, t3807: f64) -> (f64, f64, f64, f64, f64) {
    let t90835 = t1992 * t22897 * t16052;
    let t90837 = t80670 * t26392;
    let t90840 = t1992 * t6976 * t16419;
    let t90844 = t81228 * t22705 * t26422;
    let t90845 = 0.16449340668482264365e-1_f64 * t90844;
    let t90848 = t22633 * t6976 * t16040 * t3807;
    (t90835, t90837, t90840, t90845, t90848)
}
