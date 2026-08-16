//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 780/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk780(t598: f64, t8838: f64, t537: f64, t7605: f64, t1576: f64, t2001: f64, t1581: f64, t542: f64, t1588: f64, t1988: f64, t2327: f64, t1487: f64, t6: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t8839 = t598 * t8838;
    let t8841 = t7605 * t537;
    let t8843 = t2001 * t1576;
    let t8845 = t2001 * t1581;
    let t8847 = t7605 * t542;
    let t8849 = t2001 * t1588;
    let t8851 = t1988 * t2327;
    let t8853 = t6 * t1487;
    (t8839, t8841, t8843, t8845, t8847, t8849, t8851, t8853)
}
