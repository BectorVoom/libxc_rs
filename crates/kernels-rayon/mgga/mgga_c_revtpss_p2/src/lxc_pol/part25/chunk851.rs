//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 851/1360 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk851(t10019: f64, t4101: f64, t555: f64, t5744: f64, t786: f64, t3923: f64, t675: f64, t268: f64, t4003: f64, t2435: f64, t4093: f64, t4083: f64, t9303: f64) -> (f64, f64, f64, f64, f64) {
    let t10020 = t4101 * t10019;
    let t10022 = t5744 * t555;
    let t10023 = t786 * t10022;
    let t10024 = t675 * t3923;
    let t10026 = t268 * t10024 * t4003;
    let t10027 = t10023 * t10026;
    let t10032 = t2435 * t4093;
    let t10035 = 0.26019841438354088051e-2_f64 * t9303 * t4083;
    (t10020, t10024, t10027, t10032, t10035)
}
