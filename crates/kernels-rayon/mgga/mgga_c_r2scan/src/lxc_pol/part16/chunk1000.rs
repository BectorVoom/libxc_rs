//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 1000/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk1000(t12387: f64, t11496: f64, t986: f64, t3263: f64, t3262: f64, t3574: f64, t983: f64, t3276: f64, t3275: f64, t8601: f64, t9573: f64, t11479: f64, t2867: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t12388 = t12387 / 2.0_f64;
    let t12391 = t11496 * t986;
    let t12392 = t3263 * t12391;
    let t12393 = t3262 * t12392;
    let t12394 = 3.0_f64 / 2.0_f64 * t12393;
    let t12395 = t3574 * t983;
    let t12396 = t3276 * t12395;
    let t12397 = t3262 * t12396;
    let t12398 = 15.0_f64 / 8.0_f64 * t12397;
    let t12405 = t3275 * t3263 * t8601;
    let t12406 = t12405 / 4.0_f64;
    let t12409 = t3275 * t3263 * t9573;
    let t12410 = t12409 / 2.0_f64;
    let t12412 = t3275 * t11479 * t2867;
    (t12388, t12391, t12392, t12394, t12395, t12396, t12398, t12406, t12410, t12412)
}
