//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 488/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk488(t221: f64, t446: f64, t6172: f64, t1888: f64, t476: f64, t209: f64, t1867: f64) -> (f64, f64, f64, f64) {
    let t6174 = t221 * t6172 * t446;
    let t6177 = t1888 * t476;
    let t6178 = t6177 * t209;
    let t6179 = t221 * t6178;
    let t6182 = t1867 * t209;
    (t6174, t6178, t6179, t6182)
}
