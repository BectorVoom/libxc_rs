//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 123/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk123(t155: f64, t389: f64, t163: f64, t158: f64, t247: f64, t250: f64, t369: f64, t374: f64, t166: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t390 = t155 * t389;
    let t394 = t163 * t163;
    let t395 = 1.0_f64 / t394;
    let t396 = t158 * t395;
    let t401 = -0.1176575e1_f64 * t247 - 0.516475e0_f64 * t250 - 0.2103875e0_f64 * t369 - 0.104195e0_f64 * t374;
    let t402 = 1.0_f64 / t166;
    (t390, t394, t395, t396, t401, t402)
}
