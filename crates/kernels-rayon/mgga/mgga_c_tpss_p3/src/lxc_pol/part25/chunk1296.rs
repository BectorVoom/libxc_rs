//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1296/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1296(t19352: f64, t5791: f64, t18660: f64, t6073: f64, t19411: f64, t19414: f64, t19417: f64, t6080: f64, t18670: f64, t19388: f64, t42178: f64, t5784: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t67389 = 16.0_f64 / 9.0_f64 * t19352 * t5791;
    let t67391 = 16.0_f64 / 9.0_f64 * t6073 * t18660;
    let t67429 = 32.0_f64 / 9.0_f64 * t19411 * t5791;
    let t67431 = 32.0_f64 / 9.0_f64 * t19414 * t5791;
    let t67433 = 32.0_f64 / 9.0_f64 * t19417 * t5791;
    let t67436 = 32.0_f64 / 9.0_f64 * t6080 * t18660;
    let t67440 = 80.0_f64 / 9.0_f64 * t18670 * t19388;
    let t67441 = t42178 * t5784;
    (t67389, t67391, t67429, t67431, t67433, t67436, t67440, t67441)
}
