//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 741/1226 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk741(t2645: f64, t2647: f64, t9626: f64, t210: f64, t2553: f64, t804: f64, t2631: f64, t828: f64, t232: f64, t819: f64, t820: f64, t2628: f64, t835: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9653 = t2645 * t9626 * t2647;
    let t9657 = t210 * t804 * t2553;
    let t9660 = t2631 * t828;
    let t9661 = t9660 * t232;
    let t9663 = t819 * t820 * t9661;
    let t9666 = t2628 * t835;
    (t9653, t9657, t9660, t9661, t9663, t9666)
}
