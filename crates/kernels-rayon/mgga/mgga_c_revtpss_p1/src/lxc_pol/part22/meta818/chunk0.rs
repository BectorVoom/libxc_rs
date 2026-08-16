//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2929/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2929(t14085: f64, t2435: f64, t14104: f64, t47520: f64, t10069: f64, t13731: f64, t137: f64, t14103: f64, t47480: f64, t9675: f64, t14099: f64, t2453: f64) -> (f64, f64, f64, f64, f64) {
    let t47834 = t2435 * t14085;
    let t47837 = t47520 * t14104;
    let t47839 = t10069 * t13731;
    let t47844 = t47480 * t14103 * t137 * t9675;
    let t47856 = t2453 * t14099;
    (t47834, t47837, t47839, t47844, t47856)
}
