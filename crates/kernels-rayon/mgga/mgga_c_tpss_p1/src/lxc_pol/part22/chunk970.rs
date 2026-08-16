//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 970/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk970(t1289: f64, t7771: f64, t2033: f64, t3431: f64, t7780: f64, t2040: f64, t10353: f64, t1985: f64, t1992: f64, t3472: f64, t3477: f64, t581: f64, t608: f64, t612: f64) -> f64 {
    let t10388 = t7771 * t1289;
    let t10391 = t2033 * t3431;
    let t10398 = t7780 * t1289;
    let t10401 = t2040 * t3431;
    let t10408 = -280.0_f64 / 27.0_f64 * t10388 * t1985 + 56.0_f64 / 9.0_f64 * t10391 * t581 + 28.0_f64 / 9.0_f64 * t3472 * t1992 - 4.0_f64 / 3.0_f64 * t608 * t10353 + 280.0_f64 / 27.0_f64 * t10398 * t1985 + 56.0_f64 / 9.0_f64 * t10401 * t581 + 28.0_f64 / 9.0_f64 * t3477 * t1992 + 4.0_f64 / 3.0_f64 * t612 * t10353;
    t10408
}
