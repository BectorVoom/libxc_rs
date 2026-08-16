//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2335/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2335(t22: f64, t251: f64, t39698: f64, t837: f64, t2722: f64, t860: f64, t231: f64, t2782: f64, t2783: f64, t10665: f64, t2723: f64, t4503: f64) -> (f64, f64, f64, f64, f64) {
    let t39701 = t39698 * t251 * t22 * t837;
    let t39704 = t860 * t2722;
    let t39707 = t2782 * t2783 * t39704 * t231;
    let t39709 = t251 * t10665;
    let t39712 = t2782 * t4503 * t39709 * t2723;
    (t39701, t39704, t39707, t39709, t39712)
}
