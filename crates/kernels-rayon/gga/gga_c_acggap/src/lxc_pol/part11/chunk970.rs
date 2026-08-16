//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 970/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk970(t1960: f64, t3909: f64, t315: f64, t323: f64, t7877: f64, t3035: f64, t3923: f64, t609: f64, t30028: f64, t7966: f64, t29997: f64, t7963: f64, t7965: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t32084 = t1960 * t3909;
    let t32087 = t315 * t7877 * t323;
    let t32091 = 0.39512695097613069591e1_f64 * t3035 * t609 * t3923;
    let t32092 = t315 * t30028;
    let t32093 = t32092 * t7966;
    let t32096 = t7963 * t29997 * t7965;
    (t32084, t32087, t32091, t32092, t32093, t32096)
}
