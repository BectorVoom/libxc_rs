//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 690/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk690(t20153: f64, t446: f64, t20044: f64, t359: f64, t356: f64, t89: f64, t11043: f64, t15891: f64, t15894: f64, t20126: f64, t20132: f64, t20136: f64, t20139: f64, t20143: f64, t20147: f64, t20151: f64) -> (f64, f64, f64, f64) {
    let t20154 = t446 * t20153;
    let t20157 = t359 * t20044;
    let t20159 = t89 * t356 * t20157;
    let t20161 = -5.0_f64 / 81.0_f64 * t20126 + t15891 / 6.0_f64 - t15894 / 3.0_f64 + t20132 / 9.0_f64 + 2.0_f64 / 9.0_f64 * t20136 - t20139 / 9.0_f64 + t20143 / 6.0_f64 + t20147 / 6.0_f64 - t20151 / 3.0_f64 + t20154 / 3.0_f64 - 2.0_f64 / 27.0_f64 * t11043 - t20159 / 18.0_f64;
    (t20154, t20157, t20159, t20161)
}
