//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 650/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk650(t1997: f64, t9222: f64, t2057: f64, t5055: f64, t1550: f64, t9005: f64, t1990: f64, t8571: f64, t2212: f64, t5928: f64, t2228: f64, t558: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9223 = t9222 * t1997;
    let t9225 = t5055 * t2057;
    let t9229 = t1550 * t9005;
    let t9236 = t8571 * t1990;
    let t9300 = t5928 * t2212;
    let t9302 = t2228 * t558;
    (t9223, t9225, t9229, t9236, t9300, t9302)
}
