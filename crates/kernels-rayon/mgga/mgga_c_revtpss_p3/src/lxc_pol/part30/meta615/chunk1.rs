//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2121/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2121(t1907: f64, t3829: f64, t28167: f64, t8717: f64, t25082: f64, t28197: f64, t73488: f64, t13625: f64, t33651: f64, t25090: f64, t7898: f64, t28187: f64, t7235: f64) -> (f64, f64, f64, f64, f64) {
    let t98519 = t1907 * t3829;
    let t98522 = 6.0_f64 * t28167 * t8717 * t98519;
    let t98525 = 6.0_f64 * t25082 * t28197 * t73488;
    let t98528 = 6.0_f64 * t25082 * t33651 * t13625;
    let t98530 = 3.0_f64 * t7898 * t25090;
    let t98532 = 2.0_f64 * t7235 * t28187;
    (t98522, t98525, t98528, t98530, t98532)
}
