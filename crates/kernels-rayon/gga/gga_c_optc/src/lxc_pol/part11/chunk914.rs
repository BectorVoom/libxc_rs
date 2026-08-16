//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 914/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk914(t16917: f64, t2596: f64, t894: f64, t2648: f64, t17045: f64, t287: f64, t297: f64, t914: f64, t312: f64, t14339: f64, t14640: f64, t14525: f64, t4947: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t17175 = t2596 * t16917;
    let t17176 = t894 * t17175;
    let t17180 = t2648 * t16917;
    let t17181 = t894 * t17180;
    let t17185 = t287 * t17045 * t297;
    let t17186 = t914 * t17185;
    let t17190 = t312 * t17045 * t297;
    let t17191 = t894 * t17190;
    let t17196 = t14640 * t14339;
    let t17201 = t14525 * t4947;
    (t17175, t17176, t17180, t17181, t17185, t17186, t17190, t17191, t17196, t17201)
}
