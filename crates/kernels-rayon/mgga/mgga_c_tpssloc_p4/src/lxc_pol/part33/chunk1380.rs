//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1380/1415 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1380(t22633: f64, t22635: f64, t26337: f64, t6460: f64, t1985: f64, t7700: f64, t97511: f64, t1842: f64, t26331: f64, t96922: f64, t1992: f64, t26354: f64) -> (f64, f64, f64, f64) {
    let t106982 = t22633 * t22635 * t26337 * t6460;
    let t106986 = t1985 * t97511 * t7700;
    let t106991 = t26331 * t22635 * t96922 * t1842;
    let t107007 = t1992 * t22635 * t26354 * t6460;
    (t106982, t106986, t106991, t107007)
}
