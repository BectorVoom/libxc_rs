//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 947/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk947(t22674: f64, t32697: f64, t6897: f64, t32694: f64, t6914: f64, t32735: f64, t6883: f64, t32769: f64, t33334: f64, t532: f64, t33409: f64, t6547: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t120576 = t6897 * t22674 * t32697;
    let t120605 = t6914 * t32694;
    let t120610 = t6883 * t32735;
    let t120632 = t6883 * t32769;
    let t120955 = t532 * t33334;
    let t121296 = t6547 * t33409;
    (t120576, t120605, t120610, t120632, t120955, t121296)
}
