//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 868/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk868(t32: f64, t7911: f64, t8991: f64, t123: f64, t37993: f64, t532: f64, t120: f64, t1557: f64, t2264: f64, t341: f64, t17: f64, t8946: f64, t8947: f64) -> (f64, f64, f64, f64, f64) {
    let t39877 = t8991 / t32 / t7911;
    let t39889 = t123 / t532 / t37993;
    let t39912 = t120 * t1557;
    let t39922 = t341 * t2264;
    let t39926 = t8946 * t8947 * t17;
    (t39877, t39889, t39912, t39922, t39926)
}
