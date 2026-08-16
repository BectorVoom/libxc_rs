//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 1787/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1787(t7537: f64, t865: f64, t2718: f64, t23204: f64, t7488: f64, t6562: f64, t23168: f64, t7480: f64, t6547: f64, t7489: f64, t23237: f64, t1880: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t25199 = t7537 * t865;
    let t25200 = t2718 * t25199;
    let t25205 = t23204 * t7488;
    let t25206 = t6562 * t25205;
    let t25209 = t23168 * t7480;
    let t25211 = t6547 * t7489;
    let t25213 = t23237 * t7488;
    let t25214 = t1880 * t25213;
    (t25200, t25205, t25206, t25209, t25211, t25213, t25214)
}
