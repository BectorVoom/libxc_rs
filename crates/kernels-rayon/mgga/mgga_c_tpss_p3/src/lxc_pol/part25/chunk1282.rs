//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1282/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1282(t2: f64, t823: f64, t1288: f64, t2436: f64, t3678: f64, t61033: f64, t3638: f64, t17954: f64, t339: f64, t3632: f64, t790: f64, t236: f64, t61038: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t63783 = t823 * t2;
    let t63840 = t2436 * t1288;
    let t63907 = t61033 * t3678;
    let t63913 = t61033 * t3638;
    let t63917 = t339 * t17954 * t790 * t3632;
    let t63920 = t339 * t61038 * t236;
    (t63783, t63840, t63907, t63913, t63917, t63920)
}
