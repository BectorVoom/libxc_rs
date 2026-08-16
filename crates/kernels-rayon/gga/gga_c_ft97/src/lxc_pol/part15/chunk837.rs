//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 837/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk837(t21949: f64, t2771: f64, t192: f64, t22161: f64, t852: f64, t10613: f64, t21958: f64, t21602: f64, t2766: f64, t21196: f64, t4199: f64, t21969: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t22302 = t2771 * t21949;
    let t22306 = t192 * t852 * t22161;
    let t22310 = t10613 * t21958;
    let t22313 = t2766 * t21602;
    let t22316 = t4199 * t21196;
    let t22319 = t2771 * t21969;
    (t22302, t22306, t22310, t22313, t22316, t22319)
}
