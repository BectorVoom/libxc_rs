//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 793/1384 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk793(t33: f64, t9312: f64, t2769: f64, t73: f64, t2291: f64, t607: f64, t3241: f64, t76: f64, t2298: f64, t2250: f64, t634: f64, t638: f64, t9258: f64, t9288: f64) -> (f64, f64) {
    let t9313 = t33 * t9312;
    let t9321 = 1.0_f64 / t73 / t2769;
    let t9324 = t2291 * t607;
    let t9330 = 1.0_f64 / t76 / t3241;
    let t9333 = t2298 * t607;
    let t9338 = -280.0_f64 / 27.0_f64 * t9321 * t9288 + 28.0_f64 / 3.0_f64 * t9324 * t2250 - 4.0_f64 / 3.0_f64 * t634 * t9258 + 280.0_f64 / 27.0_f64 * t9330 * t9288 + 28.0_f64 / 3.0_f64 * t9333 * t2250 + 4.0_f64 / 3.0_f64 * t638 * t9258;
    (t9313, t9338)
}
