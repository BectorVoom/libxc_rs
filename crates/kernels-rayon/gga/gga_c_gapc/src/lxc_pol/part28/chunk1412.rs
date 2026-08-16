//! GGA_C_GAPC lxc pol — lxc_pol part 28 (v4rho2sigma2_7) CSE chunk 1412/1429 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part28_v4rho2sigma2_7_chunk1412(t34942: f64, t34946: f64, t34951: f64, t34956: f64, t34958: f64, t34960: f64, t34965: f64, t34949: f64, t34954: f64, t34962: f64, t37134: f64, t34973: f64) -> (f64, f64) {
    let t37135 = 0.40518222710986570299e-5_f64 * t34942;
    let t37136 = 0.17679409834076461864e-7_f64 * t34946;
    let t37138 = 0.50603841145833333336e-5_f64 * t34951;
    let t37140 = 0.26519114751114692796e-6_f64 * t34956;
    let t37141 = 0.26519114751114692796e-6_f64 * t34958;
    let t37142 = 0.13259557375557346398e-6_f64 * t34960;
    let t37144 = 0.45020247456651744776e-7_f64 * t34965;
    let t37145 = t37134 - t37135 + t37136 - 0.18115908419564701085e-6_f64 * t34949 - t37138 - 0.98380106748709416168e-8_f64 * t34954 - t37140 - t37141 - t37142 - 0.18115908419564701085e-6_f64 * t34962 - t37144;
    let t37149 = 0.12141398358188788626e-5_f64 * t34973;
    (t37145, t37149)
}
