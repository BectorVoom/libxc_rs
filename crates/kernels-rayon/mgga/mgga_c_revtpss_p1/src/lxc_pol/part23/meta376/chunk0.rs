//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 1709/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1709(t11922: f64, t4895: f64, t4892: f64, t140: f64, t4886: f64, t1011: f64, t3241: f64, t4924: f64, t12047: f64, t15905: f64) -> (f64, f64, f64, f64, f64) {
    let t16055 = t11922 * t4895;
    let t16057 = 0.57165357490759649296e-3_f64 * t4892 * t16055;
    let t16060 = t140 * t4886;
    let t16062 = t1011 * t16060 / 432.0_f64;
    let t16064 = t3241 * t4924 / 162.0_f64;
    let t16067 = t12047 * t15905;
    (t16055, t16057, t16062, t16064, t16067)
}
