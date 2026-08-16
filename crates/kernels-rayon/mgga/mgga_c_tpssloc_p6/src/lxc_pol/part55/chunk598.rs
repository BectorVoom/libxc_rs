//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 598/1304 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk598(t1088: f64, t4733: f64, t123: f64, t3237: f64, t3238: f64, t4721: f64, t4726: f64, t4731: f64, t423: f64, t1098: f64, t1657: f64, t1119: f64) -> (f64, f64, f64) {
    let t4734 = t1088 * t4733;
    let t4735 = t123 * t4734;
    let t4737 = t3237 - 0.5936111111111111111e-2_f64 * t3238 - 0.5936111111111111111e-2_f64 * t4721 - 0.11872222222222222222e-1_f64 * t4726 + 0.35616666666666666666e-1_f64 * t4731 + 0.17808333333333333333e-1_f64 * t4735;
    let t4739 = 0.621814e-1_f64 * t4737 * t423;
    let t4740 = t1657 * t1098;
    let t4742 = 1.0_f64 * t4740 * t1119;
    (t4735, t4739, t4742)
}
