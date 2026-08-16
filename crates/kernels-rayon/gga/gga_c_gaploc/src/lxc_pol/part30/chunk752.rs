//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 752/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk752(t169: f64, t299: f64, t7112: f64, t706: f64, t739: f64, t738: f64, t278: f64, t481: f64, t5286: f64) -> (f64, f64, f64, f64) {
    let t7114 = t7112 * t169 * t299;
    let t7115 = t706 * t7114;
    let t7124 = t739 * t7112;
    let t7125 = t738 * t7124;
    let t7129 = t481 * t5286 * t278;
    (t7115, t7124, t7125, t7129)
}
