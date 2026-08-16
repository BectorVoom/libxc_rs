//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1230/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1230(t1805: f64, t226: f64, t3664: f64, t5577: f64, t18770: f64, t19781: f64, t5572: f64, t6337: f64, t818: f64, t782: f64, t1708: f64, t20446: f64, t228: f64) -> (f64, f64, f64, f64, f64) {
    let t20492 = t5577 * t1805 * t3664 * t226;
    let t20494 = t18770 * t19781;
    let t20498 = t5572 * t6337 * t818;
    let t20502 = t6337 * t782 * t226;
    let t20503 = t5577 * t20502;
    let t20506 = t1708 * t228 * t20446;
    (t20492, t20494, t20498, t20503, t20506)
}
