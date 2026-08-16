//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1329/1475 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1329(t1377: f64, t7749: f64, t1307: f64, t22633: f64, t22635: f64, t1992: f64, t32693: f64, t80650: f64, t31100: f64, t90566: f64, t32704: f64, t81228: f64, t81326: f64) -> (f64, f64, f64, f64) {
    let t120197 = t1377 * t7749;
    let t120201 = 0.3289868133696452873e-1_f64 * t22633 * t22635 * t120197 * t1307;
    let t120209 = 0.3289868133696452873e-1_f64 * t1992 * t80650 * t32693;
    let t120213 = 0.3289868133696452873e-1_f64 * t22633 * t90566 * t31100;
    let t120217 = t81228 * t81326 * t32704;
    (t120201, t120209, t120213, t120217)
}
