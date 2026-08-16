//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1405/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1405(t22: f64, t46389: f64, t543: f64, t5735: f64, t1432: f64, t5763: f64, t9288: f64, t14202: f64, t9303: f64, t14238: f64, t2453: f64, t10139: f64, t14219: f64, t9285: f64) -> (f64, f64, f64, f64, f64) {
    let t47967 = t46389 * t5735 * t543 * t22;
    let t47971 = t1432 * t5763 * t9288;
    let t48005 = t9303 * t14202;
    let t48007 = t2453 * t14238;
    let t48036 = t10139 * t14219 * t9285;
    (t47967, t47971, t48005, t48007, t48036)
}
