//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 681/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk681(t13077: f64, t9824: f64, t3427: f64, t871: f64, t1020: f64, t3113: f64, t10628: f64, t2365: f64, t6111: f64, t10893: f64, t959: f64, t10012: f64, t1022: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t13078 = t13077 * t9824;
    let t13088 = t3427 * t871;
    let t13089 = t1020 * t3113;
    let t13118 = t2365 * t10628;
    let t13119 = t6111 * t13118;
    let t13121 = t10893 * t959;
    let t13141 = t10012 * t1022;
    (t13078, t13088, t13089, t13118, t13119, t13121, t13141)
}
