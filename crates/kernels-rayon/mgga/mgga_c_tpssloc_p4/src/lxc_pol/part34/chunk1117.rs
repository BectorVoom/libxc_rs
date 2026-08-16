//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1117/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1117(t1408: f64, t2752: f64, t25345: f64, t82038: f64, t1519: f64, t213: f64, t225: f64, t794: f64, t7480: f64, t81632: f64, t23030: f64, t25035: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t86721 = t2752 * t1408;
    let t86870 = t82038 * t25345;
    let t86873 = t213 * t1519 * t225;
    let t86893 = t794 * t1519;
    let t86903 = t81632 * t7480;
    let t86911 = t23030 * t25035;
    (t86721, t86870, t86873, t86893, t86903, t86911)
}
