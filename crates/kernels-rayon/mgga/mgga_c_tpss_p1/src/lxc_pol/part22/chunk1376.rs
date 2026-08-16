//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1376/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1376(t19411: f64, t5791: f64, t19414: f64, t19417: f64, t1792: f64, t18649: f64, t18663: f64, t19388: f64, t5785: f64, t5794: f64, t6080: f64, t65234: f64, t65237: f64, t65244: f64, t65321: f64, t65325: f64) -> f64 {
    let t67429 = 32.0_f64 / 9.0_f64 * t19411 * t5791;
    let t67431 = 32.0_f64 / 9.0_f64 * t19414 * t5791;
    let t67433 = 32.0_f64 / 9.0_f64 * t19417 * t5791;
    let t67434 = -2.0_f64 / 3.0_f64 * t65234 * t1792 - 4.0_f64 / 3.0_f64 * t65237 * t1792 - 4.0_f64 / 3.0_f64 * t19414 * t5794 - 2.0_f64 / 3.0_f64 * t65244 * t1792 - 4.0_f64 / 3.0_f64 * t19417 * t5794 - 2.0_f64 / 3.0_f64 * t6080 * t18663 - 10.0_f64 / 3.0_f64 * t18649 * t19388 - 10.0_f64 / 3.0_f64 * t5785 * t65321 - 5.0_f64 / 3.0_f64 * t5785 * t65325 + t67429 + t67431 + t67433;
    t67434
}
