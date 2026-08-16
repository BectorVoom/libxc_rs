//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 916/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk916(t10615: f64, t3275: f64, t3277: f64, t3270: f64, t3348: f64, t3269: f64, t2259: f64, t797: f64, t3276: f64, t2330: f64, t6897: f64, t3263: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t10617 = t3275 * t10615 * t3277;
    let t10618 = 5.0_f64 / 8.0_f64 * t10617;
    let t10619 = t3270 * t3348;
    let t10620 = t3269 * t10619;
    let t10621 = t10620 / 2.0_f64;
    let t10622 = t797 * t2259;
    let t10624 = t3275 * t3276 * t10622;
    let t10625 = 5.0_f64 / 16.0_f64 * t10624;
    let t10626 = t6897 * t2330;
    let t10628 = t3275 * t3263 * t10626;
    (t10618, t10619, t10621, t10622, t10625, t10626, t10628)
}
