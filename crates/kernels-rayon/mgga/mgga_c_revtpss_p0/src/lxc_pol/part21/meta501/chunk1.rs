//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2112/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2112(t15837: f64, t247: f64, t3116: f64, t1066: f64, t15193: f64, t1062: f64, t4797: f64) -> (f64, f64, f64) {
    let t15839 = t247 * t3116 * t15837;
    let t15847 = t247 * t1066 * t15193;
    let t15850 = t4797 * t1062;
    (t15839, t15847, t15850)
}
