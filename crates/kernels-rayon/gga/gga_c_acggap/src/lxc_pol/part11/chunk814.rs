//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 814/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk814(t527: f64, t7685: f64, t1426: f64, t2085: f64, t535: f64, t598: f64, t537: f64, t7605: f64, t1576: f64, t2001: f64, t1581: f64, t542: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t8835 = t7685 * t527;
    let t8838 = t1426 * t535 * t2085;
    let t8839 = t598 * t8838;
    let t8841 = t7605 * t537;
    let t8843 = t2001 * t1576;
    let t8845 = t2001 * t1581;
    let t8847 = t7605 * t542;
    (t8835, t8838, t8839, t8841, t8843, t8845, t8847)
}
