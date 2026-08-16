//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 934/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk934(t31228: f64, t7556: f64, t30984: f64, t7495: f64, t30543: f64, t7871: f64, t2009: f64, t968: f64, t151: f64, t394: f64, t592: f64, t7510: f64) -> (f64, f64, f64, f64, f64) {
    let t31295 = t31228 * t7556;
    let t31296 = 0.94344276868812456204e-3_f64 * t31295;
    let t31297 = t30984 * t7495;
    let t31299 = t30543 * t7871;
    let t31305 = t2009 * t968;
    let t31309 = t151 * t394 * t592 * t7510;
    (t31296, t31297, t31299, t31305, t31309)
}
