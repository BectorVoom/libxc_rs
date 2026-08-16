//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1280/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1280(t18646: f64, t5492: f64, t18351: f64, t5790: f64, t31464: f64, t5784: f64, t18669: f64, t7690: f64, t60684: f64, t60722: f64, t1219: f64, t5918: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t62309 = t5492 * t18646;
    let t62342 = t5790 * t18351;
    let t62345 = t31464 * t5784;
    let t62348 = t7690 * t18669;
    let t62375 = 595.0_f64 / 2592.0_f64 * t60684;
    let t62390 = 455.0_f64 / 648.0_f64 * t60722;
    let t62508 = t1219 * t5918;
    (t62309, t62342, t62345, t62348, t62375, t62390, t62508)
}
