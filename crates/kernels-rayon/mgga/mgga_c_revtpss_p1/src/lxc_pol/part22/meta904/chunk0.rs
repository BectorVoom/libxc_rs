//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3101/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3101(t1011: f64, t3252: f64, t4574: f64, t697: f64, t1062: f64, t15887: f64, t11921: f64, t15837: f64, t247: f64, t4837: f64, t11267: f64, t4878: f64) -> (f64, f64, f64, f64) {
    let t54126 = t1011 * t697 * t3252 * t4574;
    let t54137 = t15887 * t1062;
    let t54142 = t4837 * t247 * t11921 * t15837;
    let t54144 = t4878 * t11267;
    (t54126, t54137, t54142, t54144)
}
