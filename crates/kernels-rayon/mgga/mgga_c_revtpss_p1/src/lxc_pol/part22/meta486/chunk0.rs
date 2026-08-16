//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2205/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2205(t1651: f64, t3133: f64, t1045: f64, t3117: f64, t12167: f64, t15905: f64) -> (f64, f64, f64, f64) {
    let t16076 = t1651 * t3133;
    let t16077 = t16076 * t1045;
    let t16078 = t3117 * t16077;
    let t16081 = t12167 * t15905;
    (t16076, t16077, t16078, t16081)
}
