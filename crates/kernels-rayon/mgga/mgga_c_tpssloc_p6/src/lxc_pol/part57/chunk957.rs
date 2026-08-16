//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 957/1049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk957(t1516: f64, t32840: f64, t5628: f64, t8343: f64, t1880: f64, t25224: f64, t32866: f64, t118661: f64, t118663: f64, t23035: f64, t28298: f64, t30663: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t126339 = t32840 * t1516;
    let t126341 = t8343 * t5628;
    let t126349 = 0.3289868133696452873e-1_f64 * t1880 * t25224 * t32866;
    let t126352 = 0.3289868133696452873e-1_f64 * t118661;
    let t126353 = 0.15352717957250113407e0_f64 * t118663;
    let t126358 = 0.9869604401089358619e-1_f64 * t23035 * t30663 * t28298;
    (t126339, t126341, t126349, t126352, t126353, t126358)
}
