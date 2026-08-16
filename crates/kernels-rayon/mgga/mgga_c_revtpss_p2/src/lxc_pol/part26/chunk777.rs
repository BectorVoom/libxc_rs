//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 777/1225 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk777(t4018: f64, t9970: f64, t3930: f64, t4059: f64, t1386: f64, t2482: f64, t596: f64, t4021: f64, t1398: f64, t1412: f64, t3938: f64, t3992: f64) -> (f64, f64, f64, f64, f64) {
    let t9971 = t4018 * t9970;
    let t9973 = t3930 * t4059;
    let t9976 = t2482 * t1386 * t596;
    let t9977 = t9976 * t4021;
    let t9979 = t1412 * t1398;
    let t9980 = t9979 * t3938;
    let t9981 = t3992 * t9980;
    (t9971, t9973, t9977, t9980, t9981)
}
