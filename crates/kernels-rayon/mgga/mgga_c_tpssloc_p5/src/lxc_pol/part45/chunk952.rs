//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 952/1056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk952(t552: f64, t6955: f64, t1307: f64, t6637: f64, t6888: f64, t31193: f64, t3719: f64, t22685: f64, t3734: f64, t1992: f64, t550: f64, t6976: f64, t81203: f64) -> (f64, f64, f64, f64) {
    let t114069 = t552 * t6955;
    let t114073 = 0.6579736267392905746e-1_f64 * t6888 * t6637 * t114069 * t1307;
    let t114077 = 0.3289868133696452873e-1_f64 * t6888 * t6637 * t31193 * t3719;
    let t114081 = 0.9869604401089358619e-1_f64 * t22685 * t6637 * t31193 * t3734;
    let t114085 = 0.16449340668482264365e-1_f64 * t1992 * t6976 * t81203 * t550;
    (t114073, t114077, t114081, t114085)
}
