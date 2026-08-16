//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1073/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1073(t28845: f64, t7289: f64, t689: f64, t8099: f64, t25904: f64, t25899: f64, t213: f64, t8085: f64, t1904: f64, t7492: f64, t27899: f64, t7515: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t28858 = t7289 * t28845;
    let t28894 = t8099 * t689;
    let t28895 = t25904 * t28894;
    let t28897 = t25899 * t28894;
    let t28899 = t213 * t8085;
    let t28902 = t7492 * t1904;
    let t28903 = t689 * t28902;
    let t28909 = t27899 * t7515;
    (t28858, t28894, t28895, t28897, t28899, t28902, t28903, t28909)
}
