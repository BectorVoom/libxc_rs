//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1418/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1418(t1992: f64, t20638: f64, t22897: f64, t20416: f64, t6637: f64, t6888: f64, t6968: f64, t22633: f64, t26421: f64, t6388: f64, t1825: f64, t26331: f64, t6976: f64, t97011: f64) -> (f64, f64, f64, f64) {
    let t107367 = t1992 * t22897 * t20638;
    let t107377 = t6888 * t6637 * t6968 * t20416;
    let t107381 = t22633 * t22897 * t26421 * t6388;
    let t107385 = t26331 * t6976 * t97011 * t1825;
    (t107367, t107377, t107381, t107385)
}
