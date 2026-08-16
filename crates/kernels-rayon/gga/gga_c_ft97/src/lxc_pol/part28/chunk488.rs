//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 488/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk488(t2179: f64, t7400: f64, t144: f64, t7372: f64, t7376: f64, t7380: f64, t7384: f64, t7388: f64) -> (f64, f64, f64) {
    let t7401 = t2179 * t7400;
    let t7402 = t144 * t7401;
    let t7407 = -t7372 + t7376 - t7380 / 2.0_f64 + 2.0_f64 * t7384 - t7388;
    (t7401, t7402, t7407)
}
