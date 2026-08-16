//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 421/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk421(t1509: f64, t252: f64, t1519: f64, t814: f64, t1530: f64, t870: f64, t193: f64, t200: f64) -> (f64, f64, f64, f64) {
    let t4282 = t252 * t1509;
    let t4295 = t814 * t1519;
    let t4310 = t1530 * t870;
    let t4314 = t193 * t200;
    (t4282, t4295, t4310, t4314)
}
