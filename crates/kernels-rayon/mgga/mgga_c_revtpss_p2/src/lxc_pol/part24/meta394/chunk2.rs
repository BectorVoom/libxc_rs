//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1310/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1310(t234: f64, t251: f64, t268: f64, t39644: f64, t8779: f64, t39497: f64, t874: f64, t875: f64, t10529: f64, t2453: f64, t253: f64, t39552: f64) -> (f64, f64, f64, f64) {
    let t39649 = 0.11638313500518478545e-4_f64 * t39644 * t234 * t251 * t8779 * t268;
    let t39652 = 0.10118827226026589797e0_f64 * t874 * t875 * t39497;
    let t39680 = t2453 * t10529;
    let t39697 = 0.88356352675825229576e-3_f64 * t39552 * t253;
    (t39649, t39652, t39680, t39697)
}
