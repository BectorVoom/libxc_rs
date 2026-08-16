//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 824/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk824(t2572: f64, t7378: f64, t360: f64, t2195: f64, t2666: f64, t6343: f64, t938: f64, t551: f64, t549: f64, t1632: f64, t2719: f64, t2169: f64, t2731: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t7379 = t2572 * t7378;
    let t7380 = t360 * t7379;
    let t7383 = t2195 * t2666;
    let t7386 = t6343 * t938;
    let t7387 = t551 * t7386;
    let t7388 = t549 * t7387;
    let t7390 = t1632 * t2719;
    let t7391 = t551 * t7390;
    let t7393 = 0.23115257973478049502e0_f64 * t549 * t7391;
    let t7395 = 0.69345773920434148506e0_f64 * t2169 * t2731;
    (t7379, t7380, t7383, t7386, t7388, t7390, t7393, t7395)
}
