//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 841/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk841(t15382: f64, t321: f64, t3352: f64, t515: f64, t7230: f64, t15252: f64, t3351: f64, t352: f64, t7231: f64, t875: f64, t118: f64, t2001: f64, t618: f64, t665: f64) -> (f64, f64, f64) {
    let t75115 = 0.3192344991997337955e-4_f64 * t7230 * t3352 * t515 * t15382 * t321;
    let t75119 = t3351 * t7231 * t875 * t15252 * t352;
    let t75123 = t2001 * t118 * t665 * t618;
    (t75115, t75119, t75123)
}
