//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 934/1049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk934(t32748: f64, t6897: f64, t794: f64, t32762: f64, t6883: f64, t214: f64, t7722: f64, t32761: f64, t114172: f64, t7700: f64, t22674: f64, t32697: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t120521 = t6897 * t794 * t32748;
    let t120532 = t6883 * t32762;
    let t120544 = t214 * t7722;
    let t120550 = t6897 * t794 * t32761;
    let t120568 = t6897 * t114172 * t7700;
    let t120576 = t6897 * t22674 * t32697;
    (t120521, t120532, t120544, t120550, t120568, t120576)
}
