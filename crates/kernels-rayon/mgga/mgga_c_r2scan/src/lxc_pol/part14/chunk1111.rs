//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1111/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1111(t3308: f64, t37782: f64, t8111: f64, t574: f64, t7453: f64, t2650: f64, t546: f64, t10777: f64, t565: f64, t10773: f64, t11802: f64, t37685: f64) -> (f64, f64, f64, f64, f64) {
    let t39370 = t37782 * t3308 * t8111;
    let t39373 = t574 * t3308 * t7453;
    let t39375 = t546 * t2650;
    let t39376 = t39375 * t10777;
    let t39378 = t565 * t2650;
    let t39379 = t39378 * t10773;
    let t39381 = t37685 * t11802;
    (t39370, t39373, t39376, t39379, t39381)
}
