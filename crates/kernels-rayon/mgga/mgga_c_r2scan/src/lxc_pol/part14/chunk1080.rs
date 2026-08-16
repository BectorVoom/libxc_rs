//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1080/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1080(t22948: f64, t37945: f64, t38031: f64, t10868: f64, t6165: f64, t6166: f64, t254: f64, t259: f64, t277: f64, t37449: f64, t2080: f64, t3316: f64) -> (f64, f64, f64, f64) {
    let t38033 = t38031 * t37945 * t22948;
    let t38036 = t6165 * t10868 * t6166;
    let t38054 = t254 * t259 * t37449 * t277;
    let t38056 = t2080 * t3316;
    (t38033, t38036, t38054, t38056)
}
