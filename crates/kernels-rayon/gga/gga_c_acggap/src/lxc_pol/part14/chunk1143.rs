//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 1143/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk1143(t1815: f64, t406: f64, t1181: f64, t599: f64, t7413: f64, t1859: f64, t322: f64, t604: f64, t7493: f64, t301: f64, t8463: f64, t6405: f64, t7647: f64) -> (f64, f64, f64, f64, f64) {
    let t39794 = t1815 * t406;
    let t39797 = t7413 * t1181 * t599 * t39794;
    let t39802 = t7493 * t1181 * t604 * t1859 * t322;
    let t39807 = t8463 * t1181 * t604 * t1859 * t301;
    let t39809 = t7647 * t6405;
    (t39794, t39797, t39802, t39807, t39809)
}
