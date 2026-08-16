//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 826/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk826(t8742: f64, t8744: f64, t7465: f64, t7466: f64, t7469: f64, t7479: f64, t7481: f64, t7485: f64, t7489: f64, t7497: f64, t7500: f64, t8184: f64, t8185: f64, t8740: f64, t8748: f64) -> f64 {
    let t9277 = 0.4584375e-1_f64 * t8742;
    let t9278 = 0.305625e-1_f64 * t8744;
    let t9280 = t7465 - 0.56606566121287473723e-2_f64 * t7466 + t7469 + 0.1048269742986805069e-2_f64 * t7479 - 0.62896184579208304138e-3_f64 * t7481 + t7485 + t7489 - t7497 + 0.62896184579208304138e-3_f64 * t7500 + 0.62896184579208304138e-3_f64 * t8740 + t9277 + t9278 + t8184 - t8185 - 0.7640625e-2_f64 * t8748;
    t9280
}
