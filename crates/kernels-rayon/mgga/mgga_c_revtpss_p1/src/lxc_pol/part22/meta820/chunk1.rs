//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2934/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2934(t14082: f64, t3920: f64, t14078: f64, t2470: f64, t3915: f64, t13735: f64, t2435: f64, t10119: f64, t14114: f64, t10115: f64, t1900: f64, t14189: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t47944 = t14082 * t3920;
    let t47947 = t3915 * t14078 * t2470;
    let t47952 = t2435 * t13735;
    let t47957 = t14114 * t10119;
    let t47961 = t10115 * t1900;
    let t47963 = t2435 * t14189;
    (t47944, t47947, t47952, t47957, t47961, t47963)
}
