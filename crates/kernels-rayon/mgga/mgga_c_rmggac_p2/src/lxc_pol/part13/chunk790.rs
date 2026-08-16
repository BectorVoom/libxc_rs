//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 790/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk790(t1249: f64, t880: f64, t1338: f64, t2039: f64, t303: f64, t638: f64, t4601: f64, t7769: f64, t132: f64, t26078: f64, t36: f64, t4787: f64, t71: f64) -> (f64, f64, f64, f64) {
    let t36669 = t1249 * t880;
    let t36674 = t638 * t2039 * t303 * t1338;
    let t36680 = t4601 * t7769;
    let t36700 = t638 * t36 * t26078 * t71 * t132 * t4787;
    (t36669, t36674, t36680, t36700)
}
