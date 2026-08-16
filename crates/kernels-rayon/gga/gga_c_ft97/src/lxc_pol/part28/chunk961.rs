//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 961/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk961(t1322: f64, t7151: f64, t8281: f64, t32399: f64, t5495: f64, t31998: f64, t1286: f64, t32391: f64, t376: f64, t32338: f64, t378: f64, t32379: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t137442 = 2.0_f64 / 27.0_f64 * t7151 * t8281 * t1322;
    let t137463 = t5495 * t32399;
    let t137471 = t5495 * t31998;
    let t137476 = t1286 * t376 * t32391;
    let t137488 = t378 * t32338;
    let t137497 = t1286 * t376 * t32379;
    (t137442, t137463, t137471, t137476, t137488, t137497)
}
