//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 1751/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1751(t1230: f64, t5390: f64, t12879: f64, t1715: f64, t247: f64, t1261: f64, t12916: f64, t5342: f64, t5340: f64, t12702: f64, t5330: f64, t12744: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t17412 = t1230 * t5390;
    let t17416 = t247 * t12879 * t1715;
    let t17417 = t1261 * t17416;
    let t17423 = t12916 * t5342;
    let t17425 = 0.57165357490759649296e-3_f64 * t5340 * t17423;
    let t17426 = t12702 * t5330;
    let t17429 = t12744 * t5330;
    (t17412, t17416, t17417, t17423, t17425, t17426, t17429)
}
