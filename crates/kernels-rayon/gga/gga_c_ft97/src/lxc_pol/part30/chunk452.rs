//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 452/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk452(t263: f64, t7441: f64, t193: f64, t1410: f64, t203: f64) -> (f64, f64, f64, f64) {
    let t7442 = t7441 * t263;
    let t7443 = t193 * t7442;
    let t7446 = t1410 * t1410;
    let t7447 = t203 * t7446;
    (t7442, t7443, t7446, t7447)
}
