//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 669/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk669(t2444: f64, t780: f64, t689: f64, t779: f64, t887: f64, t211: f64, t784: f64) -> (f64, f64, f64, f64, f64) {
    let t2445 = t2444 * t780;
    let t2446 = t689 * t2445;
    let t2448 = t779 * t887;
    let t2449 = t689 * t2448;
    let t2452 = 1.0_f64 / t784 / t211;
    (t2445, t2446, t2448, t2449, t2452)
}
