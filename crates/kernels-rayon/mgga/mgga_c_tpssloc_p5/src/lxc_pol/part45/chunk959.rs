//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 959/1056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk959(t114226: f64, t1307: f64, t22633: f64, t22635: f64, t31099: f64, t3719: f64, t31100: f64, t81228: f64, t81326: f64, t31109: f64, t6883: f64, t1992: f64, t26225: f64, t3888: f64) -> (f64, f64, f64, f64, f64) {
    let t114230 = 0.6579736267392905746e-1_f64 * t22633 * t22635 * t114226 * t1307;
    let t114234 = 0.3289868133696452873e-1_f64 * t22633 * t22635 * t31099 * t3719;
    let t114240 = t81228 * t81326 * t31100;
    let t114241 = 0.3289868133696452873e-1_f64 * t114240;
    let t114242 = t6883 * t31109;
    let t114243 = 0.76763589786250567036e-1_f64 * t114242;
    let t114247 = 0.9869604401089358619e-1_f64 * t1992 * t22635 * t26225 * t3888;
    (t114230, t114234, t114241, t114243, t114247)
}
