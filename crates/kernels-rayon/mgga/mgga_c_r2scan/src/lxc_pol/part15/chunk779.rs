//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 779/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk779(t2148: f64, t6541: f64, t2147: f64, t2155: f64, t5169: f64, t2132: f64, t2183: f64, t2262: f64, t797: f64, t296: f64, t297: f64, t306: f64, t307: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t6542 = t2148 * t6541;
    let t6543 = t2147 * t6542;
    let t6545 = t2155 * t5169;
    let t6583 = t2183 * t2132;
    let t6599 = t2262 * t797;
    let t6621 = 1.0_f64 / t297 / t296;
    let t6635 = 1.0_f64 / t307 / t306;
    (t6543, t6545, t6583, t6599, t6621, t6635)
}
