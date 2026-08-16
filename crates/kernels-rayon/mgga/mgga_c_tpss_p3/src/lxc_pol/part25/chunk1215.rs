//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1215/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1215(t5791: f64, t6080: f64, t18670: f64, t6077: f64, t1792: f64, t18649: f64, t19388: f64, t19396: f64, t19404: f64, t19408: f64, t19411: f64, t19414: f64, t19417: f64, t20246: f64, t5489: f64, t5492: f64, t5785: f64, t5794: f64, t6304: f64) -> (f64, f64, f64) {
    let t20255 = t6080 * t5791;
    let t20257 = t18670 * t6077;
    let t20259 = -5.0_f64 / 3.0_f64 * t5785 * t19408 - 2.0_f64 / 3.0_f64 * t19411 * t1792 - 2.0_f64 / 3.0_f64 * t19414 * t1792 - 2.0_f64 / 3.0_f64 * t19417 * t1792 - 2.0_f64 / 3.0_f64 * t6080 * t5794 - 5.0_f64 / 3.0_f64 * t5785 * t19388 - 2.0_f64 / 3.0_f64 * t5492 * t6304 - 5.0_f64 / 3.0_f64 * t20246 * t5489 - 2.0_f64 / 3.0_f64 * t19396 * t1792 - 5.0_f64 / 3.0_f64 * t18649 * t6077 - 5.0_f64 / 3.0_f64 * t5785 * t19404 + 16.0_f64 / 9.0_f64 * t20255 + 40.0_f64 / 9.0_f64 * t20257;
    (t20255, t20257, t20259)
}
