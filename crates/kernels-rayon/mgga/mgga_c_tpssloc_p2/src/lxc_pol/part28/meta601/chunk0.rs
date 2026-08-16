//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1903/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1903(t22633: f64, t22635: f64, t26337: f64, t3911: f64, t26206: f64, t6883: f64, t1834: f64, t794: f64, t22892: f64, t6891: f64, t22704: f64, t26355: f64, t81326: f64) -> (f64, f64, f64, f64, f64) {
    let t90539 = t22633 * t22635 * t26337 * t3911;
    let t90541 = t6883 * t26206;
    let t90544 = t794 * t1834;
    let t90546 = t22892 * t90544 * t6891;
    let t90549 = t22704 * t81326 * t26355;
    (t90539, t90541, t90544, t90546, t90549)
}
