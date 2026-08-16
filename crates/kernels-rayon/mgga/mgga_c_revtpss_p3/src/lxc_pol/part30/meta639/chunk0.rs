//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2216/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2216(t101218: f64, t2122: f64, t101204: f64, t101234: f64, t101237: f64, t101240: f64, t101243: f64, t101252: f64, t101360: f64, t10309: f64, t2121: f64, t2123: f64, t25162: f64, t26792: f64, t26795: f64, t28093: f64, t28147: f64, t28154: f64, t607: f64, t7576: f64, t7579: f64, t96752: f64, t96757: f64, t96804: f64) -> f64 {
    let t104332 = t2122 * t101218;
    let t104359 = -10.0_f64 / 3.0_f64 * t25162 * t104332 - 10.0_f64 / 3.0_f64 * t101237 * t26795 - 10.0_f64 / 3.0_f64 * t101240 * t26795 - 10.0_f64 / 3.0_f64 * t101243 * t26795 - 10.0_f64 / 3.0_f64 * t28154 * t96757 - 5.0_f64 * t26792 * t101204 - t101360 * t2123 / 6.0_f64 - t28093 * t7576 / 3.0_f64 - t28093 * t7579 / 3.0_f64 + 35.0_f64 * t96804 * t101234 + 10.0_f64 * t101252 * t96752 + 20.0_f64 * t10309 * t607 * t2121 * t28147;
    t104359
}
