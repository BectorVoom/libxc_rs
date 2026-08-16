//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 380/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk380(t212: f64, t2587: f64, t2586: f64, t154: f64, t2559: f64, t222: f64, t233: f64, t813: f64) -> (f64, f64, f64, f64) {
    let t2588 = t2587 * t212;
    let t2590 = 0.83333333333333333332e-3_f64 * t2586 * t2588;
    let t2600 = t2559 * t154;
    let t2602 = 35.0_f64 / 432.0_f64 * t2600 * t222;
    let t2627 = 1.0_f64 / t813 / t233;
    (t2590, t2600, t2602, t2627)
}
