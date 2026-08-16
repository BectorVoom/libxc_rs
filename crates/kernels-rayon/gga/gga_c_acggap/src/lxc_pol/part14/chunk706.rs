//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 706/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk706(t1983: f64, t407: f64, t7586: f64, t7585: f64, t130: f64, t413: f64, t577: f64) -> (f64, f64, f64, f64, f64) {
    let t7587 = t1983 * t407;
    let t7588 = t7586 * t7587;
    let t7589 = t7585 * t7588;
    let t7599 = t130 * t413;
    let t7600 = t7599 * t577;
    (t7587, t7588, t7589, t7599, t7600)
}
