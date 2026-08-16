//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1158/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1158(t2140: f64, t334: f64, t36951: f64, t209: f64, t7581: f64, t9220: f64, t7589: f64, t36958: f64, t73: f64, t9249: f64, t37000: f64, t7579: f64) -> (f64, f64, f64, f64, f64) {
    let t92223 = t36951 * t334 * t2140;
    let t92226 = t209 * t7581 * t9220;
    let t92227 = t7589 * t92226;
    let t92232 = t209 * t73 * t36958 * t9249;
    let t92233 = t37000 * t7579 * t92232;
    (t92223, t92226, t92227, t92232, t92233)
}
