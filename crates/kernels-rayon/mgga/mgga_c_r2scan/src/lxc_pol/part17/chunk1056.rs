//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1056/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1056(t10648: f64, t2317: f64, t3314: f64, t3448: f64, t2104: f64, t2302: f64, t2304: f64, t10677: f64, t57: f64, t10978: f64, t3439: f64, t875: f64) -> (f64, f64, f64, f64) {
    let t37397 = t10648 * t3314 * t2317 * t3448;
    let t37400 = t2302 * t2104 * t2304;
    let t37401 = t37400 * t10677;
    let t37403 = t57 * t2304;
    let t37406 = t10978 * t37403 * t875 * t3439;
    (t37397, t37400, t37401, t37406)
}
