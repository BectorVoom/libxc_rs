//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 751/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk751(t10153: f64, t766: f64, t242: f64, t192: f64, t7514: f64, t265: f64, t9708: f64, t10090: f64, t10094: f64, t10123: f64, t10126: f64, t10128: f64, t10131: f64, t10134: f64, t10137: f64, t10140: f64, t10143: f64, t10146: f64, t10148: f64, t10151: f64, t1901: f64, t446: f64) -> (f64, f64, f64, f64, f64) {
    let t10154 = t10153 * t766;
    let t10155 = t242 * t10154;
    let t10157 = t192 * t7514;
    let t10159 = t10157 * t265 * t9708;
    let t10162 = -2.0_f64 / 9.0_f64 * t10090 + t1901 * t10094 / 3.0_f64 - t446 * t10123 / 3.0_f64 + t10126 / 9.0_f64 + 2.0_f64 / 27.0_f64 * t10128 - t446 * t10131 / 9.0_f64 - 4.0_f64 / 27.0_f64 * t10134 - t446 * t10137 / 3.0_f64 + 2.0_f64 / 9.0_f64 * t10140 + 2.0_f64 / 3.0_f64 * t446 * t10143 - 2.0_f64 / 9.0_f64 * t10146 - 2.0_f64 / 3.0_f64 * t10148 - t446 * t10151 - t446 * t10155 - 2.0_f64 * t446 * t10159;
    (t10154, t10155, t10157, t10159, t10162)
}
