//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 490/1484 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk490(t2617: f64, t816: f64, t809: f64, t838: f64, t842: f64, t233: f64, t813: f64) -> (f64, f64, f64, f64) {
    let t2618 = t2617 * t816;
    let t2621 = t809 * t838;
    let t2623 = t2617 * t842;
    let t2627 = 1.0_f64 / t813 / t233;
    (t2618, t2621, t2623, t2627)
}
