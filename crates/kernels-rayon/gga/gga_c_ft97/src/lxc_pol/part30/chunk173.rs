//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 173/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk173(t1212: f64, t852: f64, t192: f64, t1228: f64, t462: f64, t847: f64, t92: f64) -> (f64, f64) {
    let t1231 = t852 * t1212;
    let t1232 = t192 * t1231;
    let t1234 = -t847 - t462 * t1228 / 3.0_f64 - t92 * t1232;
    (t1232, t1234)
}
