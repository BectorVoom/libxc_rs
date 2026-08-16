//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1402/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1402(t268: f64, t39644: f64, t556: f64, t561: f64, t8779: f64, t786: f64, t9656: f64, t4146: f64, t1892: f64, t9646: f64, t9648: f64, t1904: f64, t47567: f64) -> (f64, f64, f64, f64, f64) {
    let t47601 = 0.11638313500518478545e-4_f64 * t39644 * t556 * t561 * t8779 * t268;
    let t47603 = t786 * t556 * t9656;
    let t47671 = t4146 * t4146;
    let t47672 = 1.0_f64 / t47671;
    let t47764 = t9646 * t1892 * t9648;
    let t47772 = t47567 * t1904;
    (t47601, t47603, t47672, t47764, t47772)
}
