//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 1664/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1664(t12297: f64, t12299: f64, t12301: f64, t12303: f64, t16706: f64, t16727: f64, t16748: f64, t16869: f64, t16871: f64, t16873: f64, t16876: f64, t1145: f64, t16742: f64) -> (f64, f64) {
    let t16883 = -t16869 + 0.82156666666666666667e-1_f64 * t16871 - t16873 + 0.29896666666666666667e0_f64 * t16748 + 0.13287407407407407408e0_f64 * t16706 + 0.91285185185185185185e-1_f64 * t16876 + 0.66437037037037037038e-1_f64 * t12299 + 0.26574814814814814816e0_f64 * t12297 - 0.19931111111111111111e0_f64 * t12301 - 0.99655555555555555557e-1_f64 * t12303 - 0.39862222222222222222e0_f64 * t16727;
    let t16886 = t1145 * t16742;
    (t16883, t16886)
}
