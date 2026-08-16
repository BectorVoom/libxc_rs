//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 927/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk927(t13526: f64, t13530: f64, t13546: f64, t13552: f64, t13595: f64, t13598: f64, t13601: f64, t13605: f64, t13609: f64, t13612: f64, t13616: f64, t13630: f64, t13634: f64, t13636: f64) -> f64 {
    let t13734 = -0.66228e0_f64 * t13595 + 0.33114e0_f64 * t13598 - 0.99342e0_f64 * t13601 + 0.11038e0_f64 * t13605 - 0.73586666666666666666e-1_f64 * t13609 - 0.16557e0_f64 * t13612 - 0.5519e0_f64 * t13616 + 0.258925e1_f64 * t13630 - 0.412621875e-1_f64 * t13634 + 0.16504875e0_f64 * t13636 - 0.60384999999999999999e0_f64 * t13546 + 0.181155e1_f64 * t13552 - 0.40256666666666666668e0_f64 * t13526 + 0.20128333333333333333e0_f64 * t13530;
    t13734
}
