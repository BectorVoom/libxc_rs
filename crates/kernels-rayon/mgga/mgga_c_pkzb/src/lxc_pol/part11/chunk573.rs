//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 573/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk573(t1197: f64, t870: f64, t2175: f64, t2224: f64, t2264: f64, t2269: f64, t3017: f64, t3028: f64, t3042: f64, t3047: f64, t3053: f64, t3055: f64, t3059: f64, t3063: f64, t3067: f64) -> (f64, f64) {
    let t3088 = t1197 * t870;
    let t3102 = -0.17648625e1_f64 * t3042 + 0.3529725e1_f64 * t3047 + t2264 - 0.516475e0_f64 * t2175 - 0.516475e0_f64 * t3017 + 0.1549425e1_f64 * t3028 + 0.31558125e0_f64 * t3053 + 0.6311625e0_f64 * t3055 + t2269 - 0.20839e0_f64 * t2224 - 0.20839e0_f64 * t3059 + 0.312585e0_f64 * t3063 + 0.312585e0_f64 * t3067;
    (t3088, t3102)
}
