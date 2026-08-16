//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 1023/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk1023(t22574: f64, t33357: f64, t33899: f64, t1983: f64, t33136: f64, t7940: f64, t28817: f64, t8607: f64, t28823: f64, t127162: f64, t26161: f64, t26558: f64) -> (f64, f64, f64, f64, f64) {
    let t128457 = 6.0_f64 * t22574 * t33899 * t33357;
    let t128460 = 2.0_f64 * t1983 * t7940 * t33136;
    let t128464 = 6.0_f64 * t8607 * t28817;
    let t128466 = 2.0_f64 * t8607 * t28823;
    let t128474 = 4.0_f64 * t26161 * t26558 * t127162;
    (t128457, t128460, t128464, t128466, t128474)
}
