//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 968/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk968(t23097: f64, t28395: f64, t6612: f64, t1516: f64, t32840: f64, t5628: f64, t8343: f64, t1880: f64, t25224: f64, t32866: f64, t118661: f64, t118663: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t126337 = t23097 * t6612 * t28395;
    let t126339 = t32840 * t1516;
    let t126341 = t8343 * t5628;
    let t126349 = 0.3289868133696452873e-1_f64 * t1880 * t25224 * t32866;
    let t126352 = 0.3289868133696452873e-1_f64 * t118661;
    let t126353 = 0.15352717957250113407e0_f64 * t118663;
    (t126337, t126339, t126341, t126349, t126352, t126353)
}
