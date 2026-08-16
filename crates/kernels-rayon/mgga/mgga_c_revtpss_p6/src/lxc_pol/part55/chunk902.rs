//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 902/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk902(t1955: f64, t4469: f64, t72: f64, t7778: f64, t686: f64, t7064: f64, t1558: f64, t231: f64, t7048: f64, t7076: f64, t1949: f64, t4423: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t27275 = t1955 * t4469;
    let t27278 = t7778 * t72;
    let t27279 = t27278 * t686;
    let t27280 = t7064 * t27279;
    let t27286 = t7048 * t1558 * t231;
    let t27287 = t7076 * t27286;
    let t27291 = t1949 * t4423 * t231;
    (t27275, t27279, t27280, t27286, t27287, t27291)
}
