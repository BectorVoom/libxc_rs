//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 197/1439 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk197(t169: f64, t299: f64, t706: f64, t723: f64, t268: f64, t97: f64, t278: f64, t481: f64) -> (f64, f64, f64) {
    let t726 = t706 * t723 * t169 * t299;
    let t729 = t97 * t268;
    let t731 = t481 * t729 * t278;
    (t726, t729, t731)
}
