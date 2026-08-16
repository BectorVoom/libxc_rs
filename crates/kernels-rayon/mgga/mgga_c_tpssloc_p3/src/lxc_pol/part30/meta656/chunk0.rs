//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2073/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2073(t46104: f64, t6489: f64, t12571: f64, t22522: f64, t26083: f64, t9239: f64, t645: f64, t7445: f64, t1863: f64, t22550: f64, t7441: f64, t9231: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t90182 = t46104 * t6489;
    let t90185 = t12571 * t22522;
    let t90192 = t9239 * t26083;
    let t90247 = t7445 * t645;
    let t90248 = t1863 * t90247;
    let t90251 = t7441 * t22550;
    let t90308 = t9231 * t26083;
    (t90182, t90185, t90192, t90248, t90251, t90308)
}
