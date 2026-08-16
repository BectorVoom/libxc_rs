//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 970/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk970(t1882: f64, t32622: f64, t23405: f64, t32719: f64, t1359: f64, t5973: f64, t614: f64, t7312: f64, t32709: f64, t378: f64, t1389: f64, t358: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t138367 = t1882 * t32622;
    let t138411 = t23405 * t32719;
    let t138415 = t1359 * t5973;
    let t138420 = t7312 * t614;
    let t138425 = t378 * t32709;
    let t138433 = t1389 * t358;
    (t138367, t138411, t138415, t138420, t138425, t138433)
}
