//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1074/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1074(t2385: f64, t323: f64, t851: f64, t7990: f64, t9154: f64, t862: f64, t865: f64, t32092: f64, t9168: f64, t33323: f64, t557: f64, t33092: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t38285 = t851 * t2385 * t323;
    let t38293 = 0.34694512752820797848e1_f64 * t7990 * t9154;
    let t38309 = t862 * t2385 * t865;
    let t38315 = 0.17347256376410398924e1_f64 * t32092 * t9168;
    let t38319 = 0.13170898365871023197e1_f64 * t33323 * t557;
    let t38321 = t33092 * t557;
    (t38285, t38293, t38309, t38315, t38319, t38321)
}
