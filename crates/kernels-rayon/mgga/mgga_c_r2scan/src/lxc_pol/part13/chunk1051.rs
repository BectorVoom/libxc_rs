//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 1051/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk1051(t10677: f64, t37400: f64, t2304: f64, t57: f64, t10978: f64, t3439: f64, t875: f64, t10647: f64, t10649: f64, t2049: f64, t3438: f64, t357: f64, t6806: f64) -> (f64, f64, f64) {
    let t37401 = t37400 * t10677;
    let t37403 = t57 * t2304;
    let t37406 = t10978 * t37403 * t875 * t3439;
    let t37407 = 0.5854811038705731867e-3_f64 * t37406;
    let t37412 = t6806 * t357 * t10647 * t10649 * t3438 * t2049;
    (t37401, t37407, t37412)
}
