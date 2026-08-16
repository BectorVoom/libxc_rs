//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 591/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk591(t3495: f64, t439: f64, t3356: f64, t3413: f64, t1178: f64) -> (f64, f64, f64, f64) {
    let t3496 = t439 * t3495;
    let t3503 = 0.40256666666666666667e0_f64 * t3356;
    let t3510 = 0.137975e0_f64 * t3413;
    let t3519 = t1178 * t1178;
    (t3496, t3503, t3510, t3519)
}
