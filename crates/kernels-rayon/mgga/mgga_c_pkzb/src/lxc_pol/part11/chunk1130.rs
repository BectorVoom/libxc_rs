//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1130/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1130(t1639: f64, t8770: f64, t1667: f64, t8717: f64, t501: f64, t8775: f64, t8777: f64, t46: f64, t552: f64, t8748: f64, t1545: f64, t3426: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t24606 = t8770 * t1639;
    let t24642 = t8717 * t1667;
    let t24651 = t501 * t8775;
    let t24653 = t501 * t8777;
    let t24662 = t8748 * t46 * t552;
    let t24671 = t1545 * t3426;
    (t24606, t24642, t24651, t24653, t24662, t24671)
}
