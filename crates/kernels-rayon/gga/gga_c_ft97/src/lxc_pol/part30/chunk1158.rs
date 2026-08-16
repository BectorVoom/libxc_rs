//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1158/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1158(t143177: f64, t143180: f64, t152899: f64, t152902: f64, t152905: f64, t152907: f64, t152913: f64, t152917: f64, t152920: f64, t152924: f64, t152927: f64, t152931: f64, t152934: f64, t152937: f64, t152940: f64, t152943: f64) -> f64 {
    let t154173 = -8.0_f64 / 3.0_f64 * t152899 + 2.0_f64 * t152902 - 2.0_f64 / 3.0_f64 * t152905 - t152907 / 18.0_f64 + t143177 / 6.0_f64 + t143180 - 6.0_f64 * t152913 + t152917 / 6.0_f64 + t152920 / 6.0_f64 + 2.0_f64 * t152924 - t152927 / 3.0_f64 + t152931 / 3.0_f64 - t152934 / 12.0_f64 + 4.0_f64 / 3.0_f64 * t152937 - 8.0_f64 / 3.0_f64 * t152940 - t152943;
    t154173
}
