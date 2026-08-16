//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1152/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1152(t11883: f64, t642: f64, t10761: f64, t1679: f64, t560: f64, t32262: f64, t495: f64, t694: f64, t9455: f64, t9449: f64, t96: f64, t1674: f64, t9108: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t36729 = t642 * t11883;
    let t36744 = 2.0_f64 * t1679 * t10761 * t560;
    let t36747 = 6.0_f64 * t694 * t32262 * t495;
    let t36750 = 6.0_f64 * t694 * t9455;
    let t36753 = 2.0_f64 * t96 * t9449;
    let t36755 = 12.0_f64 * t1674 * t9108;
    (t36729, t36744, t36747, t36750, t36753, t36755)
}
