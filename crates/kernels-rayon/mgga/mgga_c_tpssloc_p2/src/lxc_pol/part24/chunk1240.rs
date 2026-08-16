//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1240/1438 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1240(t22579: f64, t6876: f64, t26161: f64, t26162: f64, t55173: f64, t24995: f64, t53789: f64, t8643: f64, t1983: f64, t22948: f64, t6999: f64, t23831: f64, t4034: f64) -> (f64, f64, f64, f64, f64) {
    let t80611 = 3.0_f64 * t6876 * t22579;
    let t80614 = 6.0_f64 * t26161 * t26162 * t55173;
    let t80617 = 18.0_f64 * t24995 * t8643 * t53789;
    let t80620 = 3.0_f64 * t1983 * t22948 * t6999;
    let t80622 = 6.0_f64 * t4034 * t23831;
    (t80611, t80614, t80617, t80620, t80622)
}
