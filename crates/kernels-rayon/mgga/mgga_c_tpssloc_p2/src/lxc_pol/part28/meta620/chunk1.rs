//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1941/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1941(t22765: f64, t5289: f64, t22764: f64, t5234: f64, t1354: f64, t26298: f64, t80958: f64, t1307: f64, t1339: f64, t22827: f64, t5287: f64, t54068: f64, t550: f64) -> (f64, f64, f64, f64, f64) {
    let t91283 = t22765 * t5289;
    let t91285 = t5234 * t22764;
    let t91286 = t91285 * t1354;
    let t91290 = t80958 * t26298;
    let t91294 = t22827 * t1339 * t5287 * t1307;
    let t91298 = t22827 * t1339 * t54068 * t550;
    (t91283, t91286, t91290, t91294, t91298)
}
