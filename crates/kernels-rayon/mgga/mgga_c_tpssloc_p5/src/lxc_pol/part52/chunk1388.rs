//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1388/1400 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1388(t25010: f64, t8690: f64, t116135: f64, t25971: f64, t120678: f64, t120680: f64, t120683: f64, t120687: f64, t120691: f64, t120692: f64, t120697: f64, t120699: f64, t120702: f64, t1442: f64, t27293: f64, t31829: f64, t6517: f64) -> f64 {
    let t123228 = t8690 * t25010;
    let t123229 = t116135 * t25971;
    let t123232 = -t1442 * t31829 - 2.0_f64 * t27293 * t6517 - 2.0_f64 * t120678 - 2.0_f64 * t120680 - t120683 - t120687 - t120691 + 3.0_f64 * t120692 + t120697 + t120699 + t120702 - t123228 - 3.0_f64 * t123229;
    t123232
}
