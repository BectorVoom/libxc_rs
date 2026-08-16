//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 225/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk225(t167: f64, t960: f64, t965: f64, t967: f64, t970: f64) -> f64 {
    let t972 = 0.59778596625315888114e-2_f64 * t167 - 0.17565e-2_f64 * t960 + 0.39625e-3_f64 * t965 - 0.1294884726949076719e-4_f64 * t967 + 0.1260328125e-5_f64 * t970;
    t972
}
