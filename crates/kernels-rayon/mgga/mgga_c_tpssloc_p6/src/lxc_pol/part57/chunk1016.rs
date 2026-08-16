//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 1016/1049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk1016(t5493: f64, t652: f64, t8595: f64, t33620: f64, t4028: f64, t22574: f64, t33357: f64, t33899: f64, t1983: f64, t33136: f64, t7940: f64, t28817: f64, t8607: f64) -> (f64, f64, f64, f64, f64) {
    let t128452 = 2.0_f64 * t652 * t8595 * t5493;
    let t128454 = 4.0_f64 * t4028 * t33620;
    let t128457 = 6.0_f64 * t22574 * t33899 * t33357;
    let t128460 = 2.0_f64 * t1983 * t7940 * t33136;
    let t128464 = 6.0_f64 * t8607 * t28817;
    (t128452, t128454, t128457, t128460, t128464)
}
