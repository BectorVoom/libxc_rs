//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2069/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2069(t7607: f64, t82573: f64, t1920: f64, t25766: f64, t968: f64, t23384: f64, t25739: f64, t25751: f64, t82431: f64, t4657: f64, t6703: f64, t7554: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t89546 = 0.14621636149762012769e-1_f64 * t82573 * t7607;
    let t89561 = 0.54831135561607547884e-2_f64 * t1920 * t968 * t25766;
    let t89583 = 0.10966227112321509577e-1_f64 * t23384 * t25739;
    let t89597 = 0.18277045187202515961e-2_f64 * t82431 * t25751;
    let t89598 = t6703 * t4657;
    let t89609 = t82573 * t7554;
    (t89546, t89561, t89583, t89597, t89598, t89609)
}
