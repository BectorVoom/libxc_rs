//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 941/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk941(t32808: f64, t6562: f64, t794: f64, t112943: f64, t23164: f64, t7479: f64, t1437: f64, t31: f64, t22751: f64, t32731: f64, t1377: f64, t7749: f64) -> (f64, f64, f64, f64, f64) {
    let t118934 = t6562 * t794 * t32808;
    let t118940 = t23164 * t112943 * t7479;
    let t119878 = t1437 * t31;
    let t120179 = t22751 * t32731;
    let t120197 = t1377 * t7749;
    (t118934, t118940, t119878, t120179, t120197)
}
