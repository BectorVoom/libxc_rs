//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2062/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2062(t246: f64, t41466: f64, t22715: f64, t268: f64, t271: f64, t10969: f64, t154: f64, t2769: f64, t885: f64, t9698: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t41467 = t41466 * t246;
    let t41654 = t268 * t22715 * t271;
    let t41655 = 0.18467901234567901234e0_f64 * t41654;
    let t41664 = t154 * t10969;
    let t41665 = t2769 * t2769;
    let t41666 = 1.0_f64 / t41665;
    let t41684 = t9698 * t885;
    (t41467, t41654, t41655, t41664, t41666, t41684)
}
