//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 1601/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1601(t18710: f64, t449: f64, t11137: f64, t11247: f64, t14702: f64, t14721: f64, t14723: f64, t14724: f64, t18203: f64, t18208: f64, t18213: f64, t18217: f64, t18219: f64, t18223: f64, t18227: f64, t18229: f64, t18234: f64, t18239: f64, t18243: f64) -> (f64, f64) {
    let t18711 = t18710 * t449;
    let t18730 = -t11247 + 4.0_f64 / 27.0_f64 * t11137 + 8.0_f64 / 27.0_f64 * t14702 + t14721 - t14723 - t14724 + 2.0_f64 / 27.0_f64 * t18203 + 10.0_f64 / 27.0_f64 * t18208 - 4.0_f64 / 3.0_f64 * t18213 - 4.0_f64 / 9.0_f64 * t18217 - 2.0_f64 / 9.0_f64 * t18219 + 2.0_f64 * t18223 + 4.0_f64 / 3.0_f64 * t18227 - t18229 / 9.0_f64 - 2.0_f64 / 9.0_f64 * t18234 + 2.0_f64 / 3.0_f64 * t18239 + t18243 / 3.0_f64;
    (t18711, t18730)
}
