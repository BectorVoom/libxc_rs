//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 76/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk76(t243: f64, t245: f64, t242: f64, t7: f64, t5: f64, t240: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t246 = t243 * t245;
    let t247 = t242 * t246;
    let t249 = t7 * t245;
    let t250 = t5 * t249;
    let t252 = -0.74083333333333333333e-2_f64 * t247 - 0.1046175e-1_f64 * t250;
    let t253 = t240 * t252;
    (t246, t247, t249, t250, t252, t253)
}
