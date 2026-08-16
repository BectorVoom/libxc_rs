//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1142/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1142(t2957: f64, t5145: f64, t1061: f64, t4142: f64, t4146: f64, t5129: f64, t9467: f64, t1080: f64, t5162: f64, t1543: f64, t4180: f64, t5181: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t15736 = t5145 * t2957;
    let t15737 = t15736 * t1061;
    let t15740 = t4146 * t4142;
    let t15743 = t5129 * t9467;
    let t15744 = t15743 * t1061;
    let t15751 = t5162 * t1080;
    let t15754 = t1543 * t4180;
    let t15757 = t5181 * t1080;
    (t15737, t15740, t15744, t15751, t15754, t15757)
}
