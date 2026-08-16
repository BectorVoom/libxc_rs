//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 482/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk482(t116: f64, t206: f64, t212: f64, t2586: f64, t154: f64, t2559: f64, t222: f64, t233: f64, t813: f64) -> (f64, f64, f64, f64, f64) {
    let t2587 = t206 * t116;
    let t2588 = t2587 * t212;
    let t2590 = 0.83333333333333333332e-3_f64 * t2586 * t2588;
    let t2600 = t2559 * t154;
    let t2602 = 35.0_f64 / 432.0_f64 * t2600 * t222;
    let t2627 = 1.0_f64 / t813 / t233;
    (t2588, t2590, t2600, t2602, t2627)
}
