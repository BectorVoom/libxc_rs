//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 981/1034 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk981(t22633: f64, t31550: f64, t80650: f64, t22635: f64, t26331: f64, t31549: f64, t3734: f64, t22704: f64, t31559: f64, t81326: f64, t113931: f64, t113934: f64, t113941: f64, t115292: f64, t115294: f64, t115299: f64, t115303: f64, t115306: f64, t115308: f64, t22656: f64, t22670: f64, t24095: f64, t31642: f64, t31655: f64, t3758: f64, t6993: f64, t7214: f64, t90665: f64) -> f64 {
    let t115311 = t22633 * t80650 * t31550;
    let t115315 = t26331 * t22635 * t31549 * t3734;
    let t115318 = t22704 * t81326 * t31559;
    let t115322 = -2.0_f64 * t22656 * t7214 - 12.0_f64 * t90665 * t31655 - 2.0_f64 * t3758 * t31642 - t113931 - 2.0_f64 * t24095 * t6993 + t113934 + 0.38381794893125283518e-1_f64 * t115292 + 0.38381794893125283518e-1_f64 * t115294 + 0.3289868133696452873e-1_f64 * t115299 + 0.16449340668482264365e-1_f64 * t115303 - t115306 + 0.82246703342411321824e-2_f64 * t115308 + 0.3289868133696452873e-1_f64 * t115311 - 0.49348022005446793095e-1_f64 * t115315 - t113941 - 0.16449340668482264365e-1_f64 * t115318 - 2.0_f64 * t22670 * t7214;
    t115322
}
