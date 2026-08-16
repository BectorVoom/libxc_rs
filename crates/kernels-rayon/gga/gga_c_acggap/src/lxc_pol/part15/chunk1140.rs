//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1140/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1140(t2060: f64, t372: f64, t8927: f64, t9563: f64, t5694: f64, t8806: f64, t5698: f64, t7436: f64, t1839: f64, t322: f64, t1181: f64, t599: f64, t7346: f64) -> (f64, f64, f64, f64, f64) {
    let t39733 = t2060 * t8927 * t9563 * t372;
    let t39735 = t8806 * t5694;
    let t39737 = t7436 * t5698;
    let t39743 = t1839 * t322;
    let t39746 = t7346 * t1181 * t599 * t39743;
    (t39733, t39735, t39737, t39743, t39746)
}
