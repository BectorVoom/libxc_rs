//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 911/1056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk911(t225: f64, t22942: f64, t22643: f64, t2752: f64, t606: f64, t1887: f64, t23069: f64, t229: f64, t268: f64, t6559: f64, t1902: f64, t2678: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t81319 = t22942 * t225;
    let t81326 = t22643 * t225;
    let t81547 = t2752 * t606;
    let t81591 = t23069 * t1887;
    let t81651 = t6559 * t229 * t268;
    let t82034 = t1902 * t2678;
    (t81319, t81326, t81547, t81591, t81651, t82034)
}
