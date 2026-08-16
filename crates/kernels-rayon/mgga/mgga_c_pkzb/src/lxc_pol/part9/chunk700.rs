//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 700/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk700(t1676: f64, t637: f64, t13: f64, t25: f64, t1410: f64, t452: f64, t1448: f64, t30: f64, t1450: f64, t448: f64, t1444: f64, t459: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t4025 = t1676 * t637;
    let t4494 = t13 * t13;
    let t4635 = t25 * t25;
    let t4769 = t1410 * t452;
    let t4772 = t30 * t1448;
    let t4773 = t448 * t1450;
    let t4776 = t1444 * t459;
    (t4025, t4494, t4635, t4769, t4772, t4773, t4776)
}
