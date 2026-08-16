//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 855/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk855(t14651: f64, t159: f64, t448: f64, t7911: f64, t2137: f64, t2130: f64, t3035: f64, t2132: f64, t309: f64, t7886: f64, t3357: f64, t7741: f64) -> (f64, f64, f64, f64, f64) {
    let t30023 = t14651 * t159;
    let t30028 = t7911 * t448;
    let t30029 = t2137 * t30028;
    let t30032 = t3035 * t2130;
    let t30036 = 0.15612530738769359031e2_f64 * t30032 * t2132 * t7886 * t309;
    let t30037 = t7741 * t3357;
    (t30023, t30028, t30029, t30036, t30037)
}
