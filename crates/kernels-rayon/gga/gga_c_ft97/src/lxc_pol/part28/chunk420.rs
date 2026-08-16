//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 420/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk420(t28: f64, t5665: f64, t6496: f64, t1564: f64, t5675: f64, t925: f64, t5674: f64, t1800: f64, t6469: f64, t1317: f64, t469: f64, t6454: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t6498 = t5665 * t28 * t6496;
    let t6501 = t1564 * t5675 * t925;
    let t6502 = t5674 * t6501;
    let t6504 = t1800 * t6469;
    let t6506 = t1317 * t28 * t6504;
    let t6508 = t469 * t6454;
    (t6498, t6501, t6502, t6504, t6506, t6508)
}
