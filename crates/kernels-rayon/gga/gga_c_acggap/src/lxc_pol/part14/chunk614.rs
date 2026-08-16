//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 614/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk614(t1083: f64, t398: f64, t5814: f64, t1524: f64, t506: f64, t1713: f64, t322: f64, t1426: f64, t175: f64, t384: f64, t1841: f64, t935: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t5816 = t398 * t1083 * t5814;
    let t5819 = t506 * t1524;
    let t5821 = t398 * t1083 * t5819;
    let t5824 = t1713 * t322;
    let t5826 = t1426 * t175 * t5824;
    let t5827 = t384 * t5826;
    let t5829 = t935 * t1841;
    (t5816, t5819, t5821, t5824, t5826, t5827, t5829)
}
