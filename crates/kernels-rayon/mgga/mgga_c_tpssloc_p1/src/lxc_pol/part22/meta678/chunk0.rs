//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2239/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2239(t17659: f64, t3117: f64, t1041: f64, t17187: f64, t248: f64, t3051: f64, t10422: f64, t17704: f64, t3070: f64, t17680: f64, t13969: f64, t17692: f64) -> (f64, f64, f64, f64, f64) {
    let t61977 = t3117 * t17659;
    let t61981 = t1041 * t248 * t3051 * t17187;
    let t62013 = t3070 * t10422 * t17704;
    let t62032 = t3070 * t10422 * t17680;
    let t62038 = t1041 * t13969 * t17692;
    (t61977, t61981, t62013, t62032, t62038)
}
