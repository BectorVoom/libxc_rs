//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1354/1400 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1354(t33157: f64, t6876: f64, t120145: f64, t1874: f64, t120148: f64, t31036: f64, t7685: f64, t40611: f64, t8492: f64, t26161: f64, t26163: f64, t31086: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t120677 = t6876 * t33157;
    let t120678 = t120145 * t1874;
    let t120680 = t120148 * t1874;
    let t120683 = 2.0_f64 * t7685 * t31036;
    let t120684 = t8492 * t40611;
    let t120687 = 6.0_f64 * t26161 * t120684 * t26163;
    let t120691 = 3.0_f64 * t7685 * t31086;
    (t120677, t120678, t120680, t120683, t120687, t120691)
}
