//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3930/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3930(t2289: f64, t5892: f64, t21821: f64, t625: f64, t21824: f64, t10208: f64, t21829: f64, t2339: f64, t2340: f64, t2366: f64, t28036: f64, t31035: f64, t4287: f64, t46144: f64, t46146: f64, t46148: f64, t5915: f64, t655: f64, t69: f64, t75536: f64, t75540: f64, t75542: f64, t75585: f64, t75634: f64) -> f64 {
    let t75639 = t2289 * t5892;
    let t75641 = t625 * t21821;
    let t75643 = t625 * t21824;
    let t75655 = t69 * t21829 * t2366 / 4.0_f64 + t69 * t2339 * t75536 / 2.0_f64 - 11.0_f64 / 9.0_f64 * t75540 + 2.0_f64 / 3.0_f64 * t75542 - t69 * t655 * (t75585 + t75634) / 8.0_f64 + 22.0_f64 / 9.0_f64 * t75639 + 4.0_f64 * t75641 - 8.0_f64 / 3.0_f64 * t75643 - 3.0_f64 / 4.0_f64 * t69 * t10208 * t5915 * t2340 - 3.0_f64 * t31035 * t28036 * t4287 + 308.0_f64 / 27.0_f64 * t46144 + 22.0_f64 / 9.0_f64 * t46146 - 11.0_f64 / 9.0_f64 * t46148;
    t75655
}
