//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 589/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk589(t3262: f64, t3263: f64, t3264: f64, t106: f64, t494: f64, t97: f64) -> (f64, f64, f64) {
    let t3266 = t3262 * t3263 * t3264;
    let t3267 = 3.0_f64 / 4.0_f64 * t3266;
    let t3268 = t106 * t494;
    let t3269 = t97 * t3268;
    (t3267, t3268, t3269)
}
