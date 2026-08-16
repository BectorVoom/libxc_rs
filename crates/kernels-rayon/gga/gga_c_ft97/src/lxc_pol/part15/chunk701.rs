//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 701/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk701(t452: f64, t4623: f64, t942: f64, t3119: f64, t4533: f64, t91: f64, t11043: f64, t15606: f64, t15609: f64, t15612: f64, t15891: f64, t15894: f64, t15899: f64, t20101: f64, t20116: f64, t20136: f64, t20151: f64, t20159: f64) -> (f64, f64, f64) {
    let t20307 = t452 * t4623 * t942;
    let t20316 = t91 * t3119 * t4533;
    let t20322 = 2.0_f64 / 9.0_f64 * t15606 - 2.0_f64 / 3.0_f64 * t15609 + t15612 / 3.0_f64 - t20101 - 6.0_f64 * t20116 + t15891 - 2.0_f64 * t15894 - 2.0_f64 / 3.0_f64 * t15899 - 3.0_f64 / 4.0_f64 * t20316 + 4.0_f64 / 3.0_f64 * t20136 - 2.0_f64 * t20151 - t20159 / 3.0_f64 - 4.0_f64 / 9.0_f64 * t11043;
    (t20307, t20316, t20322)
}
