//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 1997/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1997(t25310: f64, t25331: f64, t2435: f64, t25339: f64, t11064: f64, t7086: f64, t25604: f64, t995: f64, t357: f64, t988: f64, t355: f64, t1071: f64, t11239: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t93384 = t25310 * t25331;
    let t93391 = t2435 * t25339;
    let t93404 = t7086 * t11064;
    let t93436 = t995 * t25604;
    let t93437 = t357 * t988;
    let t93438 = t355 * t93437;
    let t93488 = t1071 * t11239;
    (t93384, t93391, t93404, t93436, t93438, t93488)
}
