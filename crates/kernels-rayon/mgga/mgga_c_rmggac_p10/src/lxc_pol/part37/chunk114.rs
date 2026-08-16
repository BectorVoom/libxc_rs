//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 114/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk114(t215: f64, t28: f64, t465: f64, t140: f64, t217: f64, t219: f64, t205: f64, t449: f64, t23: f64, t453: f64, t446: f64, t206: f64, t207: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t466 = t215 * t28;
    let t467 = t465 * t466;
    let t469 = t217 * t140 * t219;
    let t470 = t449 * t205;
    let t472 = t23 * t453;
    let t473 = t472 * t446;
    let t476 = 3.0_f64 * t206 * t473 - t207 * t470;
    (t466, t467, t469, t470, t472, t473, t476)
}
