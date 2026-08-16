//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 656/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk656(t302: f64, t7350: f64, t22: f64, t4616: f64, t2078: f64, t26: f64, t3814: f64, t265: f64, t874: f64, t507: f64, t7191: f64, t3924: f64, t504: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t35718 = t7350 * t302;
    let t35928 = t4616 * t22;
    let t35959 = t2078 * t26;
    let t35960 = t3814 * t35959;
    let t36292 = t874 * t265;
    let t36471 = t507 * t7191;
    let t36596 = t504 * t3924;
    (t35718, t35928, t35959, t35960, t36292, t36471, t36596)
}
