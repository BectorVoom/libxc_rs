//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1053/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1053(t150045: f64, t446: f64, t9770: f64, t141370: f64, t141384: f64, t151001: f64, t151004: f64, t151008: f64, t151011: f64, t151014: f64, t151017: f64, t151020: f64, t151025: f64, t151027: f64, t151030: f64, t151033: f64, t151035: f64, t151040: f64) -> (f64, f64) {
    let t151043 = t446 * t9770 * t150045;
    let t151047 = 4.0_f64 * t151001 - 4.0_f64 / 3.0_f64 * t151004 + 2.0_f64 * t151008 - t151011 / 3.0_f64 + t151014 / 9.0_f64 + t151017 / 6.0_f64 + 2.0_f64 * t151020 + t151025 - t151027 / 3.0_f64 - 8.0_f64 / 3.0_f64 * t151030 - t151033 - t151035 / 12.0_f64 + t151040 / 4.0_f64 - 4.0_f64 / 3.0_f64 * t151043 - 4.0_f64 / 9.0_f64 * t141370 + t141384 / 9.0_f64;
    (t151043, t151047)
}
