//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 1292/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1292(t1419: f64, t4086: f64, t786: f64, t555: f64, t5744: f64, t2435: f64, t4093: f64, t4083: f64, t9303: f64, t2777: f64, t4092: f64, t2439: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10013 = t4086 * t1419;
    let t10014 = t786 * t10013;
    let t10022 = t5744 * t555;
    let t10023 = t786 * t10022;
    let t10032 = t2435 * t4093;
    let t10035 = 0.26019841438354088051e-2_f64 * t9303 * t4083;
    let t10043 = t2777 * t4092;
    let t10044 = t2439 * t10043;
    (t10014, t10022, t10023, t10032, t10035, t10044)
}
