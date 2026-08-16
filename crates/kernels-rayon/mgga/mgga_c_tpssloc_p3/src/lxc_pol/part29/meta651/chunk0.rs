//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2175/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2175(t5107: f64, t652: f64, t6534: f64, t22574: f64, t56198: f64, t8643: f64, t26162: f64, t57802: f64, t22597: f64, t7685: f64, t2018: f64, t3734: f64) -> (f64, f64, f64, f64, f64) {
    let t90051 = 4.0_f64 * t652 * t5107 * t6534;
    let t90059 = 6.0_f64 * t22574 * t8643 * t56198;
    let t90062 = 6.0_f64 * t22574 * t26162 * t57802;
    let t90064 = 6.0_f64 * t7685 * t22597;
    let t90065 = t3734 * t2018;
    (t90051, t90059, t90062, t90064, t90065)
}
