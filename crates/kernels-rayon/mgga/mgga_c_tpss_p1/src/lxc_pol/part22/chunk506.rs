//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 506/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk506(t2045: f64, t77: f64, t1986: f64, t1994: f64, t1997: f64, t2026: f64, t583: f64, t603: f64, t616: f64, t71: f64, t85: f64) -> (f64, f64) {
    let t2046 = t77 * t2045;
    let t2049 = -t1986 * t85 / 12.0_f64 - t1994 * t85 / 12.0_f64 - t1997 * t85 / 6.0_f64 - t583 * t616 / 6.0_f64 + t2026 * t85 / 24.0_f64 + t603 * t616 / 12.0_f64 + t71 * t2046 / 24.0_f64;
    (t2046, t2049)
}
