//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 697/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk697(t1330: f64, t637: f64, t797: f64, t3093: f64, t35206: f64, t2048: f64, t6444: f64, t2044: f64, t25525: f64, t25640: f64, t3068: f64, t854: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t69270 = t797 * t1330 * t637;
    let t69272 = t3093 * t35206;
    let t69274 = t6444 * t2048;
    let t69276 = t25525 * t2044;
    let t69279 = t25640 * t3068;
    let t69287 = t854 * t1330 * t637;
    (t69270, t69272, t69274, t69276, t69279, t69287)
}
