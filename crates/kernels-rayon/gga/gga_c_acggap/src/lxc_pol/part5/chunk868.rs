//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 868/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk868(t3372: f64, t3427: f64, t1113: f64, t3770: f64, t1108: f64, t1089: f64, t175: f64, t384: f64, t839: f64, t879: f64, t1036: f64, t1077: f64, t368: f64, t398: f64, t864: f64) -> (f64, f64, f64, f64, f64) {
    let t12478 = t3372 * t3427;
    let t12498 = t3770 * t1113;
    let t12511 = t3770 * t1108;
    let t12516 = t384 * t1089 * t175 * t839 * t879;
    let t12529 = t1036 * t398 * t368 * t864 * t1077;
    (t12478, t12498, t12511, t12516, t12529)
}
