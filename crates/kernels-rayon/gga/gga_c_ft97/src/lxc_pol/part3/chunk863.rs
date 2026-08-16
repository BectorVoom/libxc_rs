//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 863/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk863(t17409: f64, t609: f64, t144: f64, t2185: f64, t4668: f64, t616: f64, t4724: f64, t9276: f64, t1882: f64, t4811: f64, t13152: f64, t13187: f64, t13190: f64, t17377: f64, t17381: f64, t17385: f64, t17390: f64, t17394: f64, t17398: f64, t17402: f64, t17406: f64, t1901: f64, t446: f64) -> (f64, f64, f64) {
    let t17410 = t17409 * t609;
    let t17411 = t144 * t17410;
    let t17415 = t2185 * t616 * t4668;
    let t17418 = t9276 * t4724;
    let t17419 = t144 * t17418;
    let t17422 = t1882 * t4811;
    let t17425 = -4.0_f64 / 9.0_f64 * t1901 * t17377 + 4.0_f64 / 27.0_f64 * t1901 * t17381 - 2.0_f64 / 9.0_f64 * t1901 * t17385 - t13152 + t446 * t17390 / 3.0_f64 + 4.0_f64 / 3.0_f64 * t446 * t17394 + 2.0_f64 / 3.0_f64 * t446 * t17398 + 4.0_f64 / 3.0_f64 * t446 * t17402 + 2.0_f64 / 3.0_f64 * t446 * t17406 - t446 * t17411 / 3.0_f64 + 2.0_f64 / 3.0_f64 * t446 * t17415 + 2.0_f64 / 3.0_f64 * t446 * t17419 + 2.0_f64 / 9.0_f64 * t17422 - 8.0_f64 / 27.0_f64 * t13187 + t13190;
    (t17410, t17418, t17425)
}
