//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 983/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk983(t3746: f64, t835: f64, t882: f64, t15138: f64, t296: f64, t1212: f64, t2894: f64, t840: f64, t10461: f64, t10463: f64, t15202: f64, t15206: f64, t15208: f64, t15212: f64, t15218: f64, t15222: f64, t15226: f64, t15230: f64, t15234: f64, t15238: f64, t1901: f64, t3281: f64, t446: f64) -> f64 {
    let t15242 = t835 * t882 * t3746;
    let t15245 = t296 * t15138;
    let t15249 = t840 * t2894 * t1212;
    let t15252 = -2.0_f64 / 9.0_f64 * t1901 * t15202 + t15206 + 2.0_f64 / 3.0_f64 * t446 * t15208 - t446 * t15212 / 3.0_f64 - 2.0_f64 / 27.0_f64 * t10461 - 2.0_f64 / 27.0_f64 * t10463 - 2.0_f64 / 3.0_f64 * t446 * t15218 - t446 * t15222 / 3.0_f64 - 2.0_f64 / 9.0_f64 * t1901 * t15226 - 4.0_f64 / 9.0_f64 * t1901 * t15230 - 2.0_f64 / 9.0_f64 * t446 * t15234 + 2.0_f64 / 3.0_f64 * t446 * t15238 + 4.0_f64 / 9.0_f64 * t3281 * t15242 - t446 * t15245 / 3.0_f64 - t446 * t15249 / 3.0_f64;
    t15252
}
