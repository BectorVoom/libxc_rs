//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 991/1384 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk991(t3395: f64, t3400: f64, t4883: f64, t1164: f64, t11194: f64, t11272: f64, t11280: f64, t11288: f64, t11290: f64, t11296: f64, t11472: f64, t11475: f64, t11480: f64, t11482: f64, t11484: f64) -> (f64, f64) {
    let t11634 = t3400 * t3395 * t4883;
    let t11636 = 0.51947577317044391277e2_f64 * t1164 * t11634;
    let t11637 = -t11194 + t11272 + t11280 - t11288 + t11290 + t11296 - t11480 - t11482 - t11484 - t11472 + t11475 - t11636;
    (t11636, t11637)
}
