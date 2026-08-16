//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1154/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1154(t2001: f64, t5681: f64, t6106: f64, t6110: f64, t1896: f64, t7605: f64, t1992: f64, t6847: f64, t7585: f64, t7586: f64, t1181: f64, t2068: f64, t25727: f64, t604: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t39967 = t2001 * t5681;
    let t39969 = t2001 * t6106;
    let t39971 = t2001 * t6110;
    let t39973 = t7605 * t1896;
    let t39977 = t7585 * t7586 * t1992 * t6847;
    let t39981 = t2068 * t1181 * t604 * t25727;
    (t39967, t39969, t39971, t39973, t39977, t39981)
}
