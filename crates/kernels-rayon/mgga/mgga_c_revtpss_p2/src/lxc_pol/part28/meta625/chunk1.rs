//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2224/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2224(t27479: f64, t3215: f64, t100315: f64, t100321: f64, t100324: f64, t100327: f64, t100329: f64, t100332: f64, t100334: f64, t1028: f64, t15606: f64, t15975: f64, t27498: f64, t27528: f64, t27532: f64, t3208: f64, t93548: f64, t93813: f64) -> f64 {
    let t100336 = 0.57165357490759649296e-3_f64 * t27479 * t3215;
    let t100337 = -0.28582678745379824648e-3_f64 * t27498 * t15975 + 0.85748036236139473944e-3_f64 * t93548 * t15606 + t100315 * t27528 / 27.0_f64 - 2.0_f64 / 81.0_f64 * t100315 * t27532 - t93813 / 432.0_f64 + 0.85748036236139473944e-3_f64 * t100321 * t3208 + 0.45732285992607719436e-2_f64 * t100324 * t1028 + 0.30488190661738479624e-2_f64 * t100327 + 0.95275595817932748827e-4_f64 * t100329 - t100332 - t100334 - t100336;
    t100337
}
