//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 715/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk715(t1983: f64, t407: f64, t7586: f64, t7585: f64, t1131: f64, t599: f64, t336: f64, t578: f64, t1198: f64, t137: f64, t130: f64, t413: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t7587 = t1983 * t407;
    let t7588 = t7586 * t7587;
    let t7589 = t7585 * t7588;
    let t7590 = 0.14291339372689912324e-3_f64 * t7589;
    let t7591 = t599 * t1131;
    let t7592 = t336 * t7591;
    let t7593 = t578 * t7592;
    let t7596 = t336 * t1198 * t137;
    let t7597 = t578 * t7596;
    let t7599 = t130 * t413;
    (t7587, t7588, t7589, t7590, t7592, t7593, t7596, t7597, t7599)
}
