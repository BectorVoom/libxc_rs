//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 661/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk661(t3073: f64, t450: f64, t1112: f64, t242: f64, t1108: f64, t2713: f64, t3050: f64) -> (f64, f64, f64) {
    let t3074 = t3073 * t450;
    let t3075 = t1112 * t3074;
    let t3076 = t242 * t3075;
    let t3080 = t2713 * t1108 * t3050;
    (t3074, t3076, t3080)
}
