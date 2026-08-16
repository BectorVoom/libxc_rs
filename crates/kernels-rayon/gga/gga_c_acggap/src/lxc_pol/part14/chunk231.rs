//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 231/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk231(t863: f64, t865: f64, t315: f64, t441: f64, t323: f64, t322: f64, t463: f64, t449: f64) -> (f64, f64, f64, f64, f64) {
    let t867 = 0.13170898365871023197e1_f64 * t863 * t865;
    let t868 = t315 * t441;
    let t869 = t868 * t323;
    let t871 = t322 * t463;
    let t872 = t449 * t871;
    (t867, t868, t869, t871, t872)
}
