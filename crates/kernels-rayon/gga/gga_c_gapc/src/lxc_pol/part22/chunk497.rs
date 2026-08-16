//! GGA_C_GAPC lxc pol — lxc_pol part 22 (v4rho2sigma2_1) CSE chunk 497/1426 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part22_v4rho2sigma2_1_chunk497(t385: f64, t991: f64, t426: f64, t118: f64, t632: f64, t61: f64, t126: f64, t144: f64) -> (f64, f64, f64, f64, f64) {
    let t2874 = t385 * t991;
    let t2876 = t426 * t991;
    let t2878 = t632 * t118;
    let t2879 = t61 * t2878;
    let t2880 = t126 * t144;
    (t2874, t2876, t2878, t2879, t2880)
}
