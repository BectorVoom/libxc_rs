//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 681/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk681(t20045: f64, t378: f64, t92: f64, t11167: f64, t15734: f64, t15750: f64, t15760: f64, t20025: f64, t20029: f64, t20033: f64, t20037: f64, t20041: f64, t7945: f64) -> (f64, f64, f64) {
    let t20046 = t378 * t20045;
    let t20047 = t92 * t20046;
    let t20049 = -t7945 - 4.0_f64 / 9.0_f64 * t11167 + 2.0_f64 / 9.0_f64 * t15734 - 2.0_f64 / 3.0_f64 * t15750 + t15760 / 3.0_f64 - 10.0_f64 / 27.0_f64 * t20025 + 4.0_f64 / 3.0_f64 * t20029 - 2.0_f64 / 3.0_f64 * t20033 - 2.0_f64 * t20037 + 2.0_f64 * t20041 - t20047 / 3.0_f64;
    (t20046, t20047, t20049)
}
