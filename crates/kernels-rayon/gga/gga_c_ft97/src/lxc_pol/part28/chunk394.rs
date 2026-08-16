//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 394/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk394(t167: f64, t2185: f64, t5860: f64, t1359: f64, t574: f64, t616: f64, t609: f64, t605: f64, t5842: f64, t1380: f64, t376: f64, t89: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t5862 = t2185 * t167 * t5860;
    let t5866 = t574 * t616 * t1359;
    let t5869 = t1359 * t609;
    let t5871 = t574 * t605 * t5869;
    let t5875 = t574 * t167 * t5842;
    let t5880 = t89 * t376 * t1380 / 9.0_f64;
    (t5862, t5866, t5869, t5871, t5875, t5880)
}
