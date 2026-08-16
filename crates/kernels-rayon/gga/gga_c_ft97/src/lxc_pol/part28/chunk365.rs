//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 365/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk365(t1307: f64, t473: f64, t469: f64, t28: f64, t5665: f64, t1317: f64, t1318: f64, t376: f64, t1316: f64, t92: f64) -> (f64, f64, f64, f64, f64) {
    let t5666 = t1307 * t473;
    let t5667 = t469 * t5666;
    let t5669 = t5665 * t28 * t5667;
    let t5672 = t1317 * t376 * t1318;
    let t5673 = t5672 / 18.0_f64;
    let t5674 = t1316 * t92;
    (t5667, t5669, t5672, t5673, t5674)
}
