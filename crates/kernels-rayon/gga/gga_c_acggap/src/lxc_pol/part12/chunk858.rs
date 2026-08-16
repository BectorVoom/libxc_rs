//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 858/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk858(t2137: f64, t30028: f64, t2130: f64, t3035: f64, t3357: f64, t7741: f64, t3243: f64, t597: f64, t2100: f64, t7538: f64, t7544: f64, t1004: f64, t1979: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t30029 = t2137 * t30028;
    let t30032 = t3035 * t2130;
    let t30037 = t7741 * t3357;
    let t30044 = t3243 * t597;
    let t30045 = t30044 * t2100;
    let t30047 = t7538 * t7544;
    let t30049 = t1004 * t1979;
    (t30029, t30032, t30037, t30044, t30045, t30047, t30049)
}
