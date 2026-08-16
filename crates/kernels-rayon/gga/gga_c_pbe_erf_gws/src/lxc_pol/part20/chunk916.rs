//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 916/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk916(t4785: f64, t4791: f64, t4796: f64, t8004: f64, t10018: f64, t10019: f64, t10022: f64, t4652: f64, t4664: f64, t4751: f64, t4784: f64, t4790: f64, t4799: f64, t4803: f64, t4807: f64, t7994: f64) -> (f64, f64, f64, f64, f64) {
    let t10248 = 0.58482233974552040708e0_f64 * t4785;
    let t10249 = 0.17315755899375863299e2_f64 * t4791;
    let t10250 = 0.11696446794910408142e1_f64 * t4796;
    let t10251 = 0.48830813431341759843e-3_f64 * t8004;
    let t10252 = t4751 + t4652 - t7994 + t10018 + t4664 + t10019 - t10022 - t4784 - t10248 - t4790 - t10249 + t10250 + t10251 - t4799 - t4803 + t4807;
    (t10248, t10249, t10250, t10251, t10252)
}
