//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 1838/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1838(t24826: f64, t7378: f64, t2147: f64, t3590: f64, t462: f64, t7319: f64, t7327: f64) -> (f64, f64, f64, f64) {
    let t24827 = t24826 * t7378;
    let t24829 = t2147 * t3590;
    let t24830 = t462 * t24829;
    let t24833 = t7319 * t7327;
    (t24827, t24829, t24830, t24833)
}
