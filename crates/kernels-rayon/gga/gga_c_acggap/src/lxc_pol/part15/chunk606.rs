//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 606/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk606(t1748: f64, t360: f64, t1181: f64, t1532: f64, t372: f64, t1165: f64, t1552: f64, t407: f64, t495: f64) -> (f64, f64, f64) {
    let t5710 = t1748 * t360;
    let t5712 = t1181 * t1532 * t5710;
    let t5715 = t1748 * t372;
    let t5717 = t1165 * t1552 * t5715;
    let t5720 = t407 * t495;
    (t5712, t5717, t5720)
}
