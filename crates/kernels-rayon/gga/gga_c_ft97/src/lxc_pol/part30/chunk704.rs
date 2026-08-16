//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 704/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk704(t1476: f64, t4311: f64, t840: f64, t7033: f64, t8392: f64, t1901: f64, t24955: f64, t24960: f64, t29199: f64, t29204: f64, t29209: f64, t29212: f64, t29216: f64, t29219: f64, t29223: f64, t29226: f64, t29229: f64, t29232: f64, t446: f64) -> f64 {
    let t29235 = t840 * t4311 * t1476;
    let t29238 = t8392 * t7033;
    let t29241 = -2.0_f64 / 9.0_f64 * t1901 * t29199 - 2.0_f64 / 9.0_f64 * t1901 * t29204 + 2.0_f64 / 27.0_f64 * t1901 * t29209 - 2.0_f64 / 9.0_f64 * t1901 * t29212 - t1901 * t29216 / 9.0_f64 - 2.0_f64 / 9.0_f64 * t1901 * t29219 - t1901 * t29223 / 9.0_f64 - 2.0_f64 / 9.0_f64 * t1901 * t29226 + 2.0_f64 / 27.0_f64 * t1901 * t29229 + t29232 / 9.0_f64 - t446 * t29235 / 3.0_f64 - t24955 - t29238 / 27.0_f64 + t24960 / 9.0_f64;
    t29241
}
