//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1377/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1377(t18660: f64, t6080: f64, t18670: f64, t19388: f64, t42178: f64, t5784: f64, t20275: f64, t5483: f64, t1675: f64, t19380: f64, t5790: f64, t1791: f64, t1792: f64, t18305: f64, t18663: f64, t18666: f64, t19352: f64, t20282: f64, t5489: f64, t5794: f64, t6073: f64, t6304: f64, t65217: f64, t65396: f64, t65410: f64) -> f64 {
    let t67436 = 32.0_f64 / 9.0_f64 * t6080 * t18660;
    let t67440 = 80.0_f64 / 9.0_f64 * t18670 * t19388;
    let t67441 = t42178 * t5784;
    let t67451 = 16.0_f64 / 9.0_f64 * t5483 * t20275;
    let t67454 = 16.0_f64 / 9.0_f64 * t1675 * t5790 * t19380;
    let t67462 = t67436 + 10.0_f64 * t18666 * t65410 + t67440 - 10.0_f64 / 3.0_f64 * t67441 * t5489 + t65217 * t1792 / 3.0_f64 + 2.0_f64 / 3.0_f64 * t19352 * t5794 + t6073 * t18663 / 3.0_f64 - t67451 - t67454 + t18305 * t6304 / 3.0_f64 + 2.0_f64 / 3.0_f64 * t5483 * t20282 + t1675 * t1791 * t65396 / 3.0_f64;
    t67462
}
