//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 259/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk259(t22: f64, t594: f64, t161: f64, t151: f64, t177: f64, t377: f64, t414: f64, t150: f64, t848: f64) -> (f64, f64, f64, f64, f64) {
    let t972 = 1.0_f64 / t22 / t594;
    let t973 = t161 * t972;
    let t974 = t151 * t973;
    let t976 = 0.56688979511669985553e-2_f64 * t974 * t177;
    let t977 = t377 * t414;
    let t979 = 0.20007875121765877254e-2_f64 * t977 * t177;
    let t980 = t848 * t150;
    (t972, t973, t976, t979, t980)
}
