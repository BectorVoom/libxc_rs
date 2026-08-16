//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1773/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1773(t4066: f64, t4086: f64, t786: f64, t4104: f64, t2782: f64, t4100: f64, t46433: f64, t10022: f64, t2453: f64, t281: f64, t4003: f64, t46507: f64) -> (f64, f64, f64) {
    let t47423 = t786 * t4086 * t4066;
    let t47424 = t47423 * t4104;
    let t47427 = t2782 * t4100 * t46433;
    let t47429 = t2453 * t10022;
    let t47432 = t47429 * t281 * t46507 * t4003;
    (t47424, t47427, t47432)
}
