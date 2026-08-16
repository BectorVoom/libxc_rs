//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2099/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2099(t1459: f64, t30188: f64, t116: f64, t30004: f64, t572: f64, t670: f64, t1518: f64, t1936: f64, t4292: f64, t6941: f64, t7334: f64, t30194: f64) -> (f64, f64, f64, f64, f64) {
    let t105818 = 12.0_f64 * t1459 * t30188;
    let t105819 = t116 * t30004;
    let t105822 = 6.0_f64 * t572 * t105819 * t670;
    let t105823 = t1518 * t1936;
    let t105826 = 12.0_f64 * t572 * t105823 * t4292;
    let t105830 = 3.0_f64 * t6941 * t7334;
    let t105834 = 3.0_f64 * t1459 * t30194;
    (t105818, t105822, t105826, t105830, t105834)
}
