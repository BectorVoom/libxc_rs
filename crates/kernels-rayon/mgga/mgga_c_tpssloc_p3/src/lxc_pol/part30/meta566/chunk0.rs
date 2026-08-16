//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 1934/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1934(t1599: f64, t25784: f64, t225: f64, t387: f64, t5914: f64, t345: f64, t5943: f64, t6705: f64, t6704: f64, t1634: f64, t7624: f64, t3174: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t28470 = t1599 * t25784;
    let t28474 = t5914 * t225 * t387;
    let t28475 = t345 * t28474;
    let t28480 = t6705 * t5943;
    let t28481 = t6704 * t28480;
    let t28484 = t7624 * t1634;
    let t28485 = t3174 * t28484;
    (t28470, t28474, t28475, t28480, t28481, t28485)
}
