//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1236/1341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1236(t15822: f64, t25508: f64, t25516: f64, t4954: f64, t25504: f64, t4857: f64, t7131: f64, t3201: f64, t7801: f64, t15670: f64, t1972: f64, t15749: f64, t7117: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t100063 = t15822 * t25508;
    let t100146 = t4954 * t25516;
    let t100173 = t15822 * t25504;
    let t100255 = t4857 * t7131;
    let t100272 = t7801 * t3201;
    let t100321 = t15670 * t1972;
    let t100329 = t7117 * t15749;
    (t100063, t100146, t100173, t100255, t100272, t100321, t100329)
}
