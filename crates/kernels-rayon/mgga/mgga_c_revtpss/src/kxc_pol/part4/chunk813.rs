//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 813/1428 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk813(t231: f64, t2783: f64, t4494: f64, t2782: f64, t1559: f64, t72: f64, t686: f64, t2798: f64, t225: f64, t2718: f64, t213: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t4496 = t2783 * t4494 * t231;
    let t4497 = t2782 * t4496;
    let t4499 = t1559 * t72;
    let t4500 = t4499 * t686;
    let t4501 = t2798 * t4500;
    let t4503 = t225 * t2718;
    let t4504 = t213 * t4503;
    let t4514 = t213 * t2783;
    (t4496, t4497, t4499, t4500, t4501, t4503, t4504, t4514)
}
