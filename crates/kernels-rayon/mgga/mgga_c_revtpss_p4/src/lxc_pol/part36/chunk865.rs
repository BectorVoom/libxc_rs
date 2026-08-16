//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 865/1378 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk865(t2496: f64, t5571: f64, t1317: f64, t5569: f64, t123: f64, t1856: f64, t2630: f64, t1857: f64, t3860: f64, t3863: f64, t1892: f64, t785: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t13652 = t5571 * t2496;
    let t13654 = t1317 * t5569;
    let t13665 = t1856 * t123;
    let t13666 = t13665 * t2630;
    let t13668 = t3860 * t1857;
    let t13670 = t3863 * t1857;
    let t13725 = t785 * t1892;
    (t13652, t13654, t13666, t13668, t13670, t13725)
}
