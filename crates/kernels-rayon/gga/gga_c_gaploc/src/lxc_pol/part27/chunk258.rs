//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 258/1468 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk258(t738: f64, t948: f64, t270: f64, t938: f64, t946: f64, t314: f64, t935: f64) -> (f64, f64, f64) {
    let t949 = t738 * t948;
    let t952 = 0.76905262301422242837e-2_f64 * t270 * t938 + 0.32043859292259267849e-3_f64 * t946 - 0.76905262301422242837e-2_f64 * t270 * t949;
    let t954 = t314 * t935;
    (t949, t952, t954)
}
