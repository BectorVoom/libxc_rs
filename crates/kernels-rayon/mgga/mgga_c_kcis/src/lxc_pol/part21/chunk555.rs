//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 555/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk555(t187: f64, t956: f64, t1233: f64, t2932: f64, t2935: f64, t2942: f64, t2983: f64, t2991: f64, t2998: f64, t3008: f64, t3027: f64, t3035: f64, t3596: f64, t972: f64) -> (f64, f64) {
    let t3600 = t187 * t956;
    let t3609 = -t2932 + t2935 - t2942 + t2983 + t2991 + t187 * t3596 + 0.19751789702565206229e-1_f64 * t187 * t2998 - 0.11696446794910408142e1_f64 * t3600 * t972 + 0.11696446794910408142e1_f64 * t1233 * t3008 - 0.58482233974552040708e0_f64 * t1233 * t3027 - 0.17315755899375863299e2_f64 * t1233 * t3035;
    (t3600, t3609)
}
