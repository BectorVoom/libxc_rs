//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 984/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk984(t1636: f64, t2076: f64, t89: f64, t375: f64, t9008: f64, t9018: f64, t1987: f64, t40301: f64, t40306: f64, t40309: f64, t40312: f64, t40315: f64, t40318: f64, t40321: f64, t40490: f64, t40494: f64, t40497: f64, t40500: f64) -> (f64, f64, f64, f64, f64) {
    let t40503 = t89 * t1636 * t2076;
    let t40506 = t89 * t375 * t9008;
    let t40509 = t89 * t375 * t9018;
    let t40512 = t89 * t1636 * t1987;
    let t40514 = -16.0_f64 / 9.0_f64 * t40301 + 8.0_f64 * t40306 - 8.0_f64 * t40309 - 8.0_f64 / 9.0_f64 * t40312 - 16.0_f64 / 27.0_f64 * t40315 + 4.0_f64 / 9.0_f64 * t40318 + 40.0_f64 / 81.0_f64 * t40321 + t40490 / 2.0_f64 - 3.0_f64 / 4.0_f64 * t40494 + 112.0_f64 / 81.0_f64 * t40497 + 16.0_f64 / 9.0_f64 * t40500 - 8.0_f64 / 3.0_f64 * t40503 + 4.0_f64 / 3.0_f64 * t40506 + 8.0_f64 * t40509 + 16.0_f64 / 3.0_f64 * t40512;
    (t40503, t40506, t40509, t40512, t40514)
}
