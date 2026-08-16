//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 1718/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1718(t1983: f64, t22585: f64, t532: f64, t6995: f64, t6879: f64, t2018: f64, t531: f64, t1390: f64, t3734: f64, t6876: f64, t6997: f64, t191: f64, t192: f64, t3660: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t22587 = 3.0_f64 * t1983 * t22585;
    let t22591 = t532 * t6995;
    let t22592 = t22591 * t6879;
    let t22594 = 6.0_f64 * t1983 * t22592;
    let t22595 = t531 * t2018;
    let t22596 = t1390 * t3734;
    let t22597 = t22595 * t22596;
    let t22599 = 6.0_f64 * t1983 * t22597;
    let t22605 = 2.0_f64 * t6876 * t6997;
    let t22607 = t3660 * t191 * t192;
    (t22587, t22591, t22592, t22594, t22596, t22597, t22599, t22605, t22607)
}
