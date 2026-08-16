//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1224/1422 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk1224(t19466: f64, t19476: f64, t1089: f64, t378: f64, t3302: f64, t357: f64, t4866: f64, t4893: f64, t1071: f64, t6299: f64, t1043: f64, t16560: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t19477 = t19466 + t19476;
    let t19479 = t378 * t19477 * t1089;
    let t19482 = t3302 * t357;
    let t19483 = t19482 * t4866;
    let t19484 = t4893 * t19483;
    let t19488 = t1071 * t6299 * t1089;
    let t19491 = t16560 * t1043;
    (t19477, t19479, t19482, t19484, t19488, t19491)
}
