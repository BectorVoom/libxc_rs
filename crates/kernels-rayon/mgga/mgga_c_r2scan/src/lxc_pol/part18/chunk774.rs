//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 774/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk774(t1632: f64, t2719: f64, t551: f64, t549: f64, t2169: f64, t2731: f64, t2236: f64, t2727: f64, t2219: f64, t2670: f64, t2177: f64, t2699: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t7390 = t1632 * t2719;
    let t7391 = t551 * t7390;
    let t7393 = 0.23115257973478049502e0_f64 * t549 * t7391;
    let t7395 = 0.69345773920434148506e0_f64 * t2169 * t2731;
    let t7397 = 0.23115257973478049502e0_f64 * t2236 * t2727;
    let t7399 = 0.69345773920434148506e0_f64 * t2670 * t2219;
    let t7401 = 0.25610080155860322884e0_f64 * t2177 * t2699;
    (t7390, t7393, t7395, t7397, t7399, t7401)
}
