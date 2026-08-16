//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1379/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1379(t19396: f64, t5791: f64, t18646: f64, t6073: f64, t6077: f64, t62306: f64, t6080: f64, t1792: f64, t18356: f64, t18363: f64, t18649: f64, t19408: f64, t20246: f64, t5785: f64, t5794: f64, t6304: f64, t65214: f64, t65289: f64, t65400: f64, t65403: f64) -> f64 {
    let t67491 = 32.0_f64 / 9.0_f64 * t19396 * t5791;
    let t67496 = t6073 * t18646;
    let t67510 = t62306 * t6077;
    let t67512 = t6080 * t18646;
    let t67514 = t67491 - 2.0_f64 / 3.0_f64 * t65403 * t1792 - 2.0_f64 / 3.0_f64 * t18363 * t6304 + 88.0_f64 / 27.0_f64 * t67496 - 2.0_f64 / 3.0_f64 * t65214 * t1792 - 4.0_f64 / 3.0_f64 * t65400 * t1792 - 10.0_f64 / 3.0_f64 * t20246 * t18356 - 4.0_f64 / 3.0_f64 * t19396 * t5794 - 10.0_f64 / 3.0_f64 * t18649 * t19408 - 10.0_f64 / 3.0_f64 * t5785 * t65289 - 440.0_f64 / 27.0_f64 * t67510 - 176.0_f64 / 27.0_f64 * t67512;
    t67514
}
