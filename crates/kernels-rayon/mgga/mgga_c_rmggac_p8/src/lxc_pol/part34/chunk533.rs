//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 533/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk533(t14286: f64, t2079: f64, t262: f64, t3065: f64, t3851: f64, t328: f64, t3814: f64, t2566: f64, t14173: f64, t797: f64, t27: f64, t29: f64, t352: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t14288 = t2079 * t262 * t14286;
    let t14290 = t3851 * t3065;
    let t14291 = t14290 * t328;
    let t14293 = t3814 * t3065;
    let t14294 = t14293 * t2566;
    let t14296 = t797 * t14173;
    let t14298 = t27 * t29 * t352;
    (t14288, t14290, t14291, t14293, t14294, t14296, t14298)
}
