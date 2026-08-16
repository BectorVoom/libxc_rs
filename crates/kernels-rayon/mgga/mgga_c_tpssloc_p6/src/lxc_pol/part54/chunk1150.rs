//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1150/1484 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1150(t2095: f64, t31035: f64, t1983: f64, t6999: f64, t8640: f64, t1307: f64, t2018: f64, t24432: f64, t22574: f64, t191: f64, t192: f64, t7166: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t31295 = t2095 * t31035;
    let t31296 = t1983 * t31295;
    let t31297 = t8640 * t6999;
    let t31298 = t1983 * t31297;
    let t31299 = t2018 * t1307;
    let t31300 = t24432 * t31299;
    let t31302 = 3.0_f64 * t22574 * t31300;
    let t31304 = t7166 * t191 * t192;
    (t31295, t31296, t31297, t31298, t31299, t31300, t31302, t31304)
}
