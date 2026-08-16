//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 667/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk667(t5393: f64, t870: f64, t312: f64, t5299: f64, t2842: f64, t5309: f64, t5376: f64, t681: f64, t89: f64, t1775: f64, t5346: f64, t458: f64, t5360: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t19571 = t870 * t5393;
    let t19576 = t312 * t5299;
    let t19585 = t2842 * t5309;
    let t19635 = t89 * t681 * t5376;
    let t19649 = t1775 * t5346;
    let t19651 = t458 * t5360;
    (t19571, t19576, t19585, t19635, t19649, t19651)
}
