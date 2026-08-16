//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 956/1056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk956(t22674: f64, t31123: f64, t6897: f64, t1992: f64, t22635: f64, t31090: f64, t3911: f64, t214: f64, t6955: f64, t1985: f64, t6907: f64, t80707: f64, t8458: f64) -> (f64, f64, f64, f64, f64) {
    let t114154 = t6897 * t22674 * t31123;
    let t114155 = 0.16449340668482264365e-1_f64 * t114154;
    let t114159 = 0.3289868133696452873e-1_f64 * t1992 * t22635 * t31090 * t3911;
    let t114160 = t214 * t6955;
    let t114163 = 0.3289868133696452873e-1_f64 * t1985 * t114160 * t6907;
    let t114168 = 0.16449340668482264365e-1_f64 * t1985 * t80707 * t8458;
    (t114155, t114159, t114160, t114163, t114168)
}
