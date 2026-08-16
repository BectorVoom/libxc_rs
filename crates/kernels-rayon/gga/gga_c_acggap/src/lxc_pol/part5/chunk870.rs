//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 870/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk870(t12586: f64, t3459: f64, t1172: f64, t4180: f64, t1092: f64, t3670: f64, t1098: f64, t1108: f64, t3700: f64, t1426: f64, t175: f64, t384: f64, t879: f64, t922: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t12587 = t12586 * t3459;
    let t12589 = t4180 * t1172;
    let t12599 = t3670 * t1092;
    let t12601 = t3670 * t1098;
    let t12603 = t3700 * t1108;
    let t12608 = t384 * t1426 * t175 * t922 * t879;
    (t12587, t12589, t12599, t12601, t12603, t12608)
}
