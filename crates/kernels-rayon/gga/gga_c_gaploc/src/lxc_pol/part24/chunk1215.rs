//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1215/1439 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1215(t10770: f64, t7137: f64, t2958: f64, t7112: f64, t2508: f64, t2580: f64, t3431: f64, t723: f64) -> (f64, f64, f64, f64) {
    let t32207 = 0.20508069947045931424e-1_f64 * t7137 * t10770;
    let t32210 = t2958 * t7112;
    let t32213 = 0.15381052460284448567e-1_f64 * t2508 * t2580 * t32210;
    let t32214 = t3431 * t723;
    (t32207, t32210, t32213, t32214)
}
