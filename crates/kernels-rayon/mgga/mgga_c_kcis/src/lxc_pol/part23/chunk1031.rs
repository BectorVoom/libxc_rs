//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1031/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1031(t27326: f64, t1299: f64, t1640: f64, t2233: f64, t4121: f64, t541: f64, t4125: f64, t303: f64, t1014: f64, t7932: f64, t7935: f64, t12231: f64, t1598: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t27327 = t27326 / 16.0_f64;
    let t27328 = t1299 * t1640;
    let t27329 = t2233 * t27328;
    let t27330 = t27329 / 8.0_f64;
    let t27331 = t541 * t4121;
    let t27332 = t27331 * t4125;
    let t27333 = t303 * t27332;
    let t27335 = t1014 * t7932;
    let t27337 = t1014 * t7935;
    let t27339 = t12231 * t1598;
    (t27327, t27328, t27330, t27331, t27332, t27333, t27335, t27337, t27339)
}
