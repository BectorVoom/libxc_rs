//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 913/1378 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk913(t1041: f64, t19658: f64, t1065: f64, t6258: f64, t1032: f64, t6235: f64, t1040: f64, t19463: f64, t366: f64, t11710: f64, t6267: f64, t3091: f64) -> (f64, f64, f64, f64, f64) {
    let t19659 = t1041 * t19658;
    let t19675 = t1065 * t6258;
    let t19696 = t6235 * t1032;
    let t19697 = t19696 * t1040;
    let t19773 = t19463 * t366;
    let t19785 = t11710 * t6267;
    let t19786 = t3091 * t19785;
    (t19659, t19675, t19697, t19773, t19786)
}
