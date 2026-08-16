//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2222/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2222(t28150: f64, t8143: f64, t108978: f64, t2122: f64, t108986: f64, t101230: f64, t104203: f64, t104208: f64, t104314: f64, t104332: f64, t108966: f64, t108975: f64, t108983: f64, t108990: f64, t25162: f64, t26792: f64, t26795: f64, t28147: f64, t28154: f64, t29380: f64) -> f64 {
    let t111665 = t8143 * t28150;
    let t111670 = t2122 * t108978;
    let t111675 = t2122 * t108986;
    let t111680 = -10.0_f64 / 3.0_f64 * t28154 * t104332 - 10.0_f64 * t104208 * t28147 - 10.0_f64 / 3.0_f64 * t28154 * t104314 - 10.0_f64 / 3.0_f64 * t101230 * t29380 - 10.0_f64 / 3.0_f64 * t108966 * t26795 - 10.0_f64 * t104203 * t28147 - 10.0_f64 / 3.0_f64 * t25162 * t111665 - 10.0_f64 * t26792 * t108975 - 10.0_f64 / 3.0_f64 * t25162 * t111670 - 5.0_f64 * t26792 * t108983 - 5.0_f64 / 3.0_f64 * t25162 * t111675 - 5.0_f64 / 3.0_f64 * t108990 * t26795;
    t111680
}
