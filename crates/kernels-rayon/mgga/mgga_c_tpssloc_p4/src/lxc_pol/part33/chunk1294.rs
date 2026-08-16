//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1294/1415 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1294(t25577: f64, t4630: f64, t25580: f64, t4571: f64, t17906: f64, t6765: f64, t17884: f64, t17655: f64, t23541: f64, t18029: f64, t6754: f64, t1036: f64, t28572: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t99495 = t25577 * t4630;
    let t99497 = t25580 * t4571;
    let t99501 = t6765 * t17906;
    let t99507 = t6765 * t17884;
    let t99509 = t23541 * t17655;
    let t99539 = t18029 * t6754;
    let t99590 = t28572 * t1036;
    (t99495, t99497, t99501, t99507, t99509, t99539, t99590)
}
