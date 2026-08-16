//! GGA_C_GAPC lxc pol — lxc_pol part 34 (v4rho2sigma2_13) CSE chunk 646/1427 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part34_v4rho2sigma2_13_chunk646(t277: f64, t3768: f64, t334: f64, t1084: f64, t3687: f64, t1089: f64, t1026: f64) -> (f64, f64, f64, f64, f64) {
    let t3769 = t277 * t3768;
    let t3770 = t3769 * t334;
    let t3772 = t1084 * t3687;
    let t3773 = t3772 * t1089;
    let t3775 = t277 * t1026;
    (t3769, t3770, t3772, t3773, t3775)
}
