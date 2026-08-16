//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 799/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk799(t532: f64, t7605: f64, t1569: f64, t2001: f64, t1967: f64, t2327: f64, t5616: f64, t604: f64, t1181: f64, t2068: f64, t7380: f64, t8544: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t8718 = t7605 * t532;
    let t8720 = t2001 * t1569;
    let t8722 = t1967 * t2327;
    let t8738 = t604 * t5616;
    let t8739 = t1181 * t8738;
    let t8740 = t2068 * t8739;
    let t8742 = t7380 * t8544;
    (t8718, t8720, t8722, t8738, t8739, t8740, t8742)
}
