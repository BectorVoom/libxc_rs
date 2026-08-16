//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 612/1464 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk612(t169: f64, t3720: f64, t299: f64, t706: f64, t739: f64) -> (f64, f64, f64, f64) {
    let t3721 = t3720 * t169;
    let t3722 = t3721 * t299;
    let t3723 = t706 * t3722;
    let t3726 = t739 * t3720;
    (t3721, t3722, t3723, t3726)
}
