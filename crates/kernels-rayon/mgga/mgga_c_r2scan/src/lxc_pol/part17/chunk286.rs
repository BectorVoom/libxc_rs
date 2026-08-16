//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 286/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk286(t471: f64, t910: f64, t97: f64, t108: f64, t381: f64, t404: f64, t408: f64, t412: f64, t459: f64, t466: f64, t470: f64, t900: f64, t902: f64) -> (f64, f64) {
    let t912 = t97 * t471 * t910;
    let t913 = 3.0_f64 * t912;
    let t915 = (t381 + t404 - t408 - t412 + t900 + t459 + t902 - t466 - t470) * t108;
    (t913, t915)
}
