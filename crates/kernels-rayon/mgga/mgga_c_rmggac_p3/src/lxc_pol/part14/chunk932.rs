//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 932/1089 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk932(t2298: f64, t24363: f64, t1614: f64, t3351: f64, t498: f64, t511: f64, t7231: f64, t34724: f64, t8626: f64, t504: f64, t8629: f64, t8632: f64) -> (f64, f64, f64, f64) {
    let t40050 = t24363 * t2298;
    let t40055 = t3351 * t7231 * t511 * t1614 * t498;
    let t40057 = t34724 * t8626;
    let t40060 = t504 * t8629 * t8632;
    (t40050, t40055, t40057, t40060)
}
