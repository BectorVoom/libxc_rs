//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 1814/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1814(t3630: f64, t7301: f64, t7300: f64, t1235: f64, t7299: f64, t7302: f64, t2123: f64, t3477: f64, t2127: f64, t23383: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t24563 = t7301 * t3630;
    let t24564 = t7300 * t24563;
    let t24567 = t7299 * t1235;
    let t24568 = t24567 * t7302;
    let t24571 = t3477 * t2123;
    let t24574 = t2127 * t23383;
    (t24563, t24564, t24567, t24568, t24571, t24574)
}
