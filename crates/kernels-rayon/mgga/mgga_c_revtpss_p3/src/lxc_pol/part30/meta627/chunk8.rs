//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2182/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2182(t892: f64, t99536: f64, t1940: f64, t1963: f64, t580: f64, t4343: f64, t605: f64, t27383: f64, t63164: f64, t2411: f64, t27363: f64, t25207: f64, t61102: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t99537 = t99536 * t892;
    let t99542 = t1940 * t1963 * t580;
    let t99543 = t605 * t4343;
    let t99550 = t27383 * t63164;
    let t99555 = t27363 * t2411;
    let t99558 = t25207 * t61102;
    (t99537, t99542, t99543, t99550, t99555, t99558)
}
