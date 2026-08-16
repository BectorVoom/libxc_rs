//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 1177/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1177(t1610: f64, t934: f64, t2874: f64, t1600: f64, t2880: f64, t918: f64, t2848: f64, t2884: f64, t4571: f64, t4576: f64, t4581: f64, t4585: f64) -> (f64, f64, f64, f64, f64) {
    let t4595 = t1610 * t934;
    let t4597 = 2.0_f64 * t2874 * t4595;
    let t4598 = t2880 * t1600;
    let t4599 = t4598 * t918;
    let t4606 = t2884 + t2848 / 9.0_f64 + t4571 / 9.0_f64 - 2.0_f64 / 9.0_f64 * t4576 + 2.0_f64 / 3.0_f64 * t4581 - t4585 / 3.0_f64;
    (t4595, t4597, t4598, t4599, t4606)
}
