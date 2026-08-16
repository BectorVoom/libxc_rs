//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1271/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1271(t1268: f64, t5371: f64, t1206: f64, t5451: f64, t1625: f64, t4519: f64, t4706: f64, t821: f64, t16264: f64, t782: f64, t4701: f64, t1364: f64, t3724: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t51631 = t5371 * t1268;
    let t51635 = t5451 * t1206;
    let t51642 = t1625 * t4519;
    let t51664 = t5451 * t1268;
    let t51780 = t4706 * t821;
    let t52460 = t16264 * t782;
    let t52613 = t4701 * t821;
    let t52639 = t1364 * t3724;
    (t51631, t51635, t51642, t51664, t51780, t52460, t52613, t52639)
}
