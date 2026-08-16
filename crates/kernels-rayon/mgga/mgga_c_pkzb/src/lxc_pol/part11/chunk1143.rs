//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1143/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1143(t2099: f64, t757: f64, t9541: f64, t17848: f64, t2104: f64, t9288: f64, t5974: f64, t9558: f64, t2899: f64, t774: f64, t9563: f64, t179: f64, t2068: f64, t299: f64, t9161: f64) -> (f64, f64, f64, f64, f64) {
    let t26413 = t757 * t2099 * t9541;
    let t26423 = t2104 * t17848 * t9288;
    let t26426 = t2104 * t5974 * t9558;
    let t26430 = t2899 * t774 * t9563;
    let t26440 = t299 * t179 * t2068 * t9161;
    (t26413, t26423, t26426, t26430, t26440)
}
