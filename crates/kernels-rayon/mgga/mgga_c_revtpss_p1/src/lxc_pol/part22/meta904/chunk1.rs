//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3102/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3102(t11263: f64, t4879: f64, t11773: f64, t3278: f64, t11875: f64, t11922: f64, t15898: f64, t15728: f64, t15827: f64, t11672: f64, t15984: f64, t16052: f64, t16055: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t54147 = t4879 * t11263;
    let t54166 = t3278 * t11773;
    let t54187 = t11875 * t11922 * t15898;
    let t54198 = t15728 * t15827;
    let t54222 = t11672 * t15984;
    let t54259 = t16052 * t16055;
    (t54147, t54166, t54187, t54198, t54222, t54259)
}
