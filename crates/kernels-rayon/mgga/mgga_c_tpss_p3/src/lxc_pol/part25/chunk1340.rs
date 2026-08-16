//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1340/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1340(t21116: f64, t62348: f64, t6077: f64, t67329: f64, t21123: f64, t5791: f64, t18670: f64, t21129: f64, t21133: f64, t19411: f64, t19414: f64, t19417: f64, t20282: f64, t21139: f64, t21756: f64, t5492: f64, t5794: f64, t6080: f64, t62277: f64, t6304: f64) -> f64 {
    let t71473 = t62348 * t21116;
    let t71475 = t67329 * t6077;
    let t71477 = t21123 * t5791;
    let t71479 = t18670 * t21129;
    let t71481 = t18670 * t21133;
    let t71487 = -2.0_f64 / 3.0_f64 * t21139 * t5794 - 4.0_f64 / 3.0_f64 * t19411 * t6304 - 4.0_f64 / 3.0_f64 * t19414 * t6304 - 4.0_f64 / 3.0_f64 * t19417 * t6304 - 4.0_f64 / 3.0_f64 * t6080 * t20282 - 80.0_f64 / 3.0_f64 * t71473 + 80.0_f64 / 9.0_f64 * t71475 + 32.0_f64 / 9.0_f64 * t71477 + 80.0_f64 / 9.0_f64 * t71479 + 40.0_f64 / 9.0_f64 * t71481 - 2.0_f64 / 3.0_f64 * t5492 * t21756 + 10.0_f64 * t62277 * t21116;
    t71487
}
