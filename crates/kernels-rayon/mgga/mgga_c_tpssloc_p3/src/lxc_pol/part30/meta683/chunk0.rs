//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2152/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2152(t28160: f64, t6883: f64, t19873: f64, t26309: f64, t19966: f64, t6396: f64, t80816: f64, t19951: f64, t22833: f64, t19972: f64, t19976: f64, t5259: f64, t91100: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t97200 = t6883 * t28160;
    let t97202 = t26309 * t19873;
    let t97204 = t26309 * t19966;
    let t97206 = t80816 * t6396;
    let t97208 = t22833 * t19951;
    let t97210 = t22833 * t19972;
    let t97212 = t22833 * t19976;
    let t97214 = t91100 * t5259;
    (t97200, t97202, t97204, t97206, t97208, t97210, t97212, t97214)
}
