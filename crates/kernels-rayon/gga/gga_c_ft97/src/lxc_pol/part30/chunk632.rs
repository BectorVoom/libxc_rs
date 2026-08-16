//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 632/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk632(t10007: f64, t28187: f64, t1882: f64, t6927: f64, t11593: f64, t1901: f64, t24590: f64, t24592: f64, t28150: f64, t28154: f64, t28158: f64, t28163: f64, t28167: f64, t28171: f64, t28175: f64, t28178: f64, t28181: f64, t28184: f64, t446: f64) -> f64 {
    let t28188 = t10007 * t28187;
    let t28191 = t1882 * t6927;
    let t28193 = t1901 * t28150 / 9.0_f64 + t1901 * t28154 / 9.0_f64 - 2.0_f64 / 9.0_f64 * t11593 * t28158 - 2.0_f64 / 9.0_f64 * t24590 - t24592 / 9.0_f64 - 2.0_f64 / 9.0_f64 * t1901 * t28163 - t446 * t28167 / 3.0_f64 - t446 * t28171 / 3.0_f64 - t446 * t28175 / 3.0_f64 - t446 * t28178 / 3.0_f64 + 2.0_f64 / 27.0_f64 * t1901 * t28181 - 2.0_f64 / 9.0_f64 * t1901 * t28184 - t1901 * t28188 / 9.0_f64 + t28191 / 9.0_f64;
    t28193
}
