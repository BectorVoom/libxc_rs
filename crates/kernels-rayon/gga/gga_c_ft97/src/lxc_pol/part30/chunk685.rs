//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 685/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk685(t25036: f64, t25042: f64, t25146: f64, t25154: f64, t25163: f64, t28811: f64, t28814: f64, t28819: f64, t28824: f64, t28829: f64, t28833: f64, t28838: f64) -> f64 {
    let t28922 = -t25036 - t25042 / 9.0_f64 + t25146 / 6.0_f64 - t25154 - t28811 / 3.0_f64 - 2.0_f64 / 3.0_f64 * t28814 + t28819 / 4.0_f64 + t28824 / 4.0_f64 - t25163 / 18.0_f64 + 2.0_f64 * t28829 + 2.0_f64 * t28833 + 2.0_f64 * t28838;
    t28922
}
