//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 1073/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk1073(t24586: f64, t7290: f64, t24350: f64, t1980: f64, t8774: f64, t2154: f64, t2936: f64, t6134: f64, t8792: f64, t1: f64, t106: f64, t316: f64, t8720: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t24741 = t7290 * t24586;
    let t24745 = t7290 * t24350;
    let t24751 = t1980 * t8774;
    let t24777 = t2154 * t2936;
    let t24784 = t6134 * t8792;
    let t24817 = t8720 * t1 * t106 * t316;
    (t24741, t24745, t24751, t24777, t24784, t24817)
}
