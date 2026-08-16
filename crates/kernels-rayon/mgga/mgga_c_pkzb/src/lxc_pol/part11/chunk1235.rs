//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1235/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1235(t30313: f64, t30358: f64, t664: f64, t684: f64, t10868: f64, t17536: f64, t10892: f64, t5771: f64, t10777: f64, t663: f64, t685: f64, t17349: f64, t17351: f64, t20705: f64, t20845: f64, t25633: f64, t25636: f64, t261: f64, t30284: f64, t30287: f64) -> (f64, f64, f64, f64, f64) {
    let t30362 = 1.0_f64 * t664 * (t30313 + t30358) * t684;
    let t30364 = 0.51726012919273400301e3_f64 * t17536 * t10868;
    let t30366 = 6.0_f64 * t5771 * t10892;
    let t30367 = t10777 * t663;
    let t30369 = 1.0_f64 * t30367 * t685;
    let t30377 = (t17349 - 0.28842592592592592592e-1_f64 * t17351 - 0.86527777777777777779e-1_f64 * t20705 + t20845 + 0.37083333333333333333e-1_f64 * t25633 - 0.278125e-1_f64 * t25636 - 0.92708333333333333333e-2_f64 * t30284 + 0.278125e-1_f64 * t30287) * t261;
    (t30362, t30364, t30366, t30369, t30377)
}
