//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 825/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk825(t6243: f64, t7406: f64, t1604: f64, t2122: f64, t2198: f64, t5117: f64, t5121: f64, t6106: f64, t6139: f64, t7367: f64, t7369: f64, t7373: f64, t7377: f64, t7380: f64, t7383: f64, t7388: f64, t7393: f64, t7395: f64, t7397: f64, t7399: f64, t7401: f64, t7405: f64) -> (f64, f64) {
    let t7407 = t6243 * t7406;
    let t7408 = t1604 * t7407;
    let t7412 = -t7367 - 0.2600466522016280569e0_f64 * t6139 * t7369 + 0.54878743191129263322e-1_f64 * t2122 * t7373 - t7377 - 0.5200933044032561138e0_f64 * t6106 * t7380 + 0.5200933044032561138e0_f64 * t7383 * t2198 - 0.42377972951376424087e0_f64 * t7388 + t7393 + t7395 + t7397 + t7399 + t7401 - t7405 - 0.32927245914677557994e-1_f64 * t7408 + 0.27439371595564631661e-2_f64 * t5117 + 0.29272321618148349056e-1_f64 * t5121;
    (t7407, t7412)
}
