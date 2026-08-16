//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1375/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1375(t19352: f64, t5791: f64, t18660: f64, t6073: f64, t1792: f64, t18673: f64, t19411: f64, t5794: f64, t62307: f64, t62309: f64, t62314: f64, t62343: f64, t62349: f64, t65189: f64, t65296: f64, t65299: f64, t65302: f64) -> f64 {
    let t67389 = 16.0_f64 / 9.0_f64 * t19352 * t5791;
    let t67391 = 16.0_f64 / 9.0_f64 * t6073 * t18660;
    let t67407 = -t67389 - t67391 - 880.0_f64 / 27.0_f64 * t62307 - 352.0_f64 / 27.0_f64 * t62309 + 32.0_f64 / 9.0_f64 * t62314 - 160.0_f64 / 9.0_f64 * t62343 - 80.0_f64 / 3.0_f64 * t62349 + 20.0_f64 / 3.0_f64 * t65189 * t18673 - 2.0_f64 / 3.0_f64 * t65296 * t1792 - 4.0_f64 / 3.0_f64 * t65299 * t1792 - 4.0_f64 / 3.0_f64 * t65302 * t1792 - 4.0_f64 / 3.0_f64 * t19411 * t5794;
    t67407
}
