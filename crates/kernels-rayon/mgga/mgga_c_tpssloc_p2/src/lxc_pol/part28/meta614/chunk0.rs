//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1929/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1929(t16261: f64, t26309: f64, t22832: f64, t5234: f64, t3809: f64, t16405: f64, t22833: f64, t16387: f64, t16275: f64, t16271: f64, t1336: f64, t22759: f64, t5252: f64, t836: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t91098 = t26309 * t16261;
    let t91100 = t5234 * t22832;
    let t91101 = t91100 * t3809;
    let t91103 = t22833 * t16405;
    let t91105 = t26309 * t16387;
    let t91107 = t22833 * t16275;
    let t91109 = t22833 * t16271;
    let t91113 = t1336 * t22759 * t836 * t5252;
    (t91098, t91101, t91103, t91105, t91107, t91109, t91113)
}
