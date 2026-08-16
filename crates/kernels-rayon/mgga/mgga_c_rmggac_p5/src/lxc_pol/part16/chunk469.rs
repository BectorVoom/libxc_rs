//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 469/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk469(t60: f64, t1805: f64, t921: f64, t5860: f64, t1403: f64, t284: f64, t5865: f64, t62: f64, t815: f64, t5864: f64, t277: f64, t352: f64, t570: f64, zeta_threshold: f64) -> (f64, f64, f64, f64) {
    let t61 = t60 <= zeta_threshold;
    let t5870 = t921 * t1805;
    let t5873 = -t5860;
    let t5877 = piecewise3(t61, 0.0_f64, -8.0_f64 / 27.0_f64 * t5865 * t284 - 16.0_f64 / 9.0_f64 * t1403 * t815 + 4.0_f64 / 9.0_f64 * t5870 * t284 + 4.0_f64 / 3.0_f64 * t62 * t5873);
    let t5878 = t5864 + t5877;
    let t5879 = t277 * t5878;
    let t5888 = t570 * t352;
    (t5873, t5878, t5879, t5888)
}
