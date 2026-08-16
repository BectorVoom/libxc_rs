//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1352/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1352(t1799: f64, t6324: f64, t22574: f64, t26162: f64, t1873: f64, t22425: f64, t652: f64, t28827: f64, t7685: f64, t23035: f64, t25224: f64, t28298: f64) -> (f64, f64, f64, f64) {
    let t105201 = t1799 * t6324;
    let t105204 = 18.0_f64 * t22574 * t26162 * t105201;
    let t105207 = 2.0_f64 * t652 * t22425 * t1873;
    let t105213 = 18.0_f64 * t7685 * t28827;
    let t105223 = t23035 * t25224 * t28298;
    (t105204, t105207, t105213, t105223)
}
