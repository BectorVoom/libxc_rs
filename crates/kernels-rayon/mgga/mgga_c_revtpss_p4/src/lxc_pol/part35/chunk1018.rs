//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1018/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1018(t1211: f64, t24713: f64, t1828: f64, t6587: f64, t1277: f64, t6573: f64, t24543: f64, t487: f64, t13143: f64, t24864: f64, t489: f64, t1287: f64, t1794: f64, t6695: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t24892 = t1211 * t24713;
    let t24899 = t6587 * t1828;
    let t24900 = t1277 * t24899;
    let t24906 = t1277 * t6573 * t1828;
    let t24911 = t487 * t24543;
    let t24912 = t24911 * t13143;
    let t24915 = t489 * t24864;
    let t24919 = t6695 * t1794 * t1287;
    (t24892, t24900, t24906, t24911, t24912, t24915, t24919)
}
