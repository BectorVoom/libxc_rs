//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1245/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1245(t17487: f64, t20754: f64, t21055: f64, t21058: f64, t21059: f64, t30314: f64, t30316: f64, t30319: f64, t30322: f64, t30324: f64, t30326: f64, t30328: f64, t30331: f64, t30338: f64, t30342: f64, t30346: f64, t30350: f64, t30353: f64, t30356: f64) -> f64 {
    let t30587 = 0.94674375e0_f64 * t30314 + 0.94674375e0_f64 * t30316 + 0.31558125e0_f64 * t30319 - 0.6618234375e1_f64 * t30322 + 0.794188125e1_f64 * t30324 - 0.52945875e1_f64 * t30326 - 0.52945875e1_f64 * t30328 - 0.17648625e1_f64 * t30331 + t21055 + t21058 + t21059 - 0.27785333333333333333e1_f64 * t20754 + t17487 + 0.937755e0_f64 * t30338 + 0.312585e0_f64 * t30342 + 0.312585e0_f64 * t30346 + 0.937755e0_f64 * t30350 - 0.62517e0_f64 * t30353 - 0.20839e0_f64 * t30356;
    t30587
}
