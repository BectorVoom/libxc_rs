//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 942/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk942(t2132: f64, t322: f64, t7896: f64, t8103: f64, t2225: f64, t879: f64, t8099: f64, t2230: f64, t30009: f64, t3915: f64, t8347: f64, t2217: f64, t862: f64, t865: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t33015 = 0.52041769129231196772e1_f64 * t7896 * t2132 * t8103 * t322;
    let t33019 = 0.52041769129231196772e1_f64 * t7896 * t2132 * t2225 * t879;
    let t33028 = t7896 * t2132 * t8099 * t322;
    let t33031 = 0.52041769129231196772e1_f64 * t30009 * t2230;
    let t33037 = 0.39512695097613069591e1_f64 * t8347 * t3915;
    let t33047 = t862 * t2217 * t865;
    (t33015, t33019, t33028, t33031, t33037, t33047)
}
