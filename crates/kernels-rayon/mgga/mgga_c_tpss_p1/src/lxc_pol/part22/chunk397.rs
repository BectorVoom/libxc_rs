//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 397/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk397(t1289: f64, t60: f64, t1294: f64, t1300: f64, t44: f64, t56: f64, t601: f64, t61: f64, t38: f64, t608: f64, t612: f64) -> (f64, f64, f64, f64, f64) {
    let t1303 = t60 * t1289;
    let t1306 = 5.0_f64 / 6.0_f64 * t44 * t1294 - 8.0_f64 / 3.0_f64 * t1300 * t61 - 5.0_f64 / 6.0_f64 * t56 * t1303 + t601;
    let t1307 = t38 * t1306;
    let t1310 = t608 * t1289;
    let t1311 = t612 * t1289;
    let t1313 = -4.0_f64 / 3.0_f64 * t1310 + 4.0_f64 / 3.0_f64 * t1311;
    (t1306, t1307, t1310, t1311, t1313)
}
