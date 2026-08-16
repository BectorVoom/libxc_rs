//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 784/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk784(t1327: f64, t142: f64, t8888: f64, t599: f64, t8406: f64, t1181: f64, t7346: f64, t301: f64, t525: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t8889 = t142 * t1327;
    let t8890 = t8888 * t8889;
    let t8896 = t599 * t8406;
    let t8897 = t1181 * t8896;
    let t8898 = t7346 * t8897;
    let t8901 = t525 * t301;
    (t8889, t8890, t8896, t8897, t8898, t8901)
}
