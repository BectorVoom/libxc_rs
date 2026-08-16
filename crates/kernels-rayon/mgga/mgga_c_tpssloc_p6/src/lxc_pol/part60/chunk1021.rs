//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 1021/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk1021(t652: f64, t7467: f64, t7890: f64, t33214: f64, t7802: f64, t29211: f64, t8526: f64, t115262: f64, t1983: f64, t28826: f64, t120955: f64, t7687: f64) -> (f64, f64, f64, f64, f64) {
    let t128418 = 4.0_f64 * t652 * t7890 * t7467;
    let t128420 = 4.0_f64 * t33214 * t7802;
    let t128422 = 2.0_f64 * t8526 * t29211;
    let t128429 = 6.0_f64 * t1983 * t115262 * t28826;
    let t128438 = 6.0_f64 * t1983 * t120955 * t7687;
    (t128418, t128420, t128422, t128429, t128438)
}
