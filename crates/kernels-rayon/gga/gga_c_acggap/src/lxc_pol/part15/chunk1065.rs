//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1065/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1065(t2934: f64, t633: f64, t1614: f64, t8114: f64, t556: f64, t8306: f64) -> (f64, f64, f64) {
    let t38040 = t2934 * t633;
    let t38051 = 0.13170898365871023197e1_f64 * t8114 * t1614;
    let t38052 = t8306 * t556;
    (t38040, t38051, t38052)
}
