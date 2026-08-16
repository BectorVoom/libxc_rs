//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1413/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1413(t10111: f64, t22: f64, t5759: f64, t14159: f64, t3964: f64, t9285: f64, t5600: f64, t9292: f64, t1893: f64, t4075: f64, t786: f64, t10115: f64, t1894: f64) -> (f64, f64, f64, f64, f64) {
    let t49361 = t10111 * t5759 * t22;
    let t49432 = t3964 * t14159 * t9285;
    let t49468 = t9292 * t5600;
    let t49471 = t786 * t1893 * t4075;
    let t49474 = t10115 * t1894;
    (t49361, t49432, t49468, t49471, t49474)
}
