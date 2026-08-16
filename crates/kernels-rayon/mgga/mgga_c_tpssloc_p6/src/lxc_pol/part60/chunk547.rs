//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 547/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk547(t240: f64, t6612: f64, t812: f64, t1899: f64, t838: f64, t234: f64, t59: f64, t1903: f64, t225: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t6613 = t6612 * t240;
    let t6614 = t812 * t6613;
    let t6617 = t1899 * t838;
    let t6618 = 7.0_f64 / 2304.0_f64 * t6617;
    let t6619 = t234 * t59;
    let t6620 = t6619 * t240;
    let t6621 = t812 * t6620;
    let t6627 = t1903 * t225;
    (t6613, t6614, t6617, t6618, t6619, t6620, t6621, t6627)
}
