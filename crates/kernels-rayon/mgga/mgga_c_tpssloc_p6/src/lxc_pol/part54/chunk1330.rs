//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1330/1484 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1330(t120217: f64, t22704: f64, t32693: f64, t81326: f64, t22633: f64, t32704: f64, t80650: f64, t114285: f64, t26338: f64, t1992: f64, t22635: f64, t26226: f64) -> (f64, f64, f64, f64, f64) {
    let t120218 = 0.16449340668482264365e-1_f64 * t120217;
    let t120220 = t22704 * t81326 * t32693;
    let t120221 = 0.16449340668482264365e-1_f64 * t120220;
    let t120226 = 0.3289868133696452873e-1_f64 * t22633 * t80650 * t32704;
    let t120229 = 0.3289868133696452873e-1_f64 * t22633 * t114285 * t26338;
    let t120232 = 0.9869604401089358619e-1_f64 * t1992 * t22635 * t26226;
    (t120218, t120221, t120226, t120229, t120232)
}
