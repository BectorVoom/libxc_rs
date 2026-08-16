//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 888/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk888(t1554: f64, t2562: f64, t360: f64, t2567: f64, t113: f64, t2185: f64, t2572: f64, t2719: f64, t560: f64, t551: f64, t552: f64, t538: f64, t920: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t8102 = t2562 * t1554;
    let t8103 = t360 * t8102;
    let t8106 = t2567 * t1554;
    let t8107 = t360 * t8106;
    let t8110 = t113 * t2185;
    let t8111 = t2572 * t8110;
    let t8112 = t360 * t8111;
    let t8117 = t2719 * t560;
    let t8119 = t551 * t552 * t8117;
    let t8123 = t538 * t920;
    (t8102, t8103, t8106, t8107, t8111, t8112, t8119, t8123)
}
