//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 922/1226 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk922(t11691: f64, t11757: f64, t11817: f64, t11866: f64, t493: f64, t3493: f64, t3612: f64, t1245: f64, t11812: f64, t1243: f64, t10471: f64, t11715: f64) -> (f64, f64, f64, f64, f64) {
    let t11868 = t11691 + t11757 + t11817 + t11866;
    let t11869 = t493 * t11868;
    let t11871 = t3612 * t3493;
    let t11872 = t1245 * t11871;
    let t11877 = t11812 * t1243;
    let t11880 = t10471 * t11715;
    (t11868, t11869, t11872, t11877, t11880)
}
