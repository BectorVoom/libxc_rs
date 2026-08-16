//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 1093/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk1093(t1165: f64, t2068: f64, t25742: f64, t7351: f64, t6271: f64, t7561: f64, t6396: f64, t7822: f64, t6400: f64, t30148: f64, t6841: f64, t7585: f64, t7842: f64) -> (f64, f64, f64, f64, f64) {
    let t39141 = t2068 * t1165 * t7351 * t25742;
    let t39143 = t7561 * t6271;
    let t39145 = t7822 * t6396;
    let t39147 = t7822 * t6400;
    let t39151 = t7585 * t7842 * t30148 * t6841;
    (t39141, t39143, t39145, t39147, t39151)
}
