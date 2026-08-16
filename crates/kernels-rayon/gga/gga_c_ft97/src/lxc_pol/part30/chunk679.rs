//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 679/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk679(t193: f64, t28836: f64, t89: f64, t25042: f64, t25146: f64, t25163: f64, t25343: f64, t25351: f64, t28811: f64, t28814: f64, t28819: f64, t28824: f64, t28829: f64, t28833: f64) -> (f64, f64, f64) {
    let t28837 = t193 * t28836;
    let t28838 = t89 * t28837;
    let t28840 = -t25343 - t25042 / 27.0_f64 + t25146 / 18.0_f64 - t25351 - t28811 / 9.0_f64 - 2.0_f64 / 9.0_f64 * t28814 + t28819 / 12.0_f64 + t28824 / 12.0_f64 - t25163 / 54.0_f64 + 2.0_f64 / 3.0_f64 * t28829 + 2.0_f64 / 3.0_f64 * t28833 + 2.0_f64 / 3.0_f64 * t28838;
    (t28837, t28838, t28840)
}
