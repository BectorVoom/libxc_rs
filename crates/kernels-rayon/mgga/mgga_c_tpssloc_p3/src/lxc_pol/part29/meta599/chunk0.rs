//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2031/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2031(t23041: f64, t2686: f64, t59: f64, t9971: f64, t6613: f64, t9612: f64, t23040: f64, t2617: f64, t831: f64, t23061: f64, t6604: f64, t23099: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t81810 = t23041 * t2686;
    let t81816 = t9971 * t59;
    let t81821 = t9612 * t6613;
    let t81824 = t2617 * t23040;
    let t81825 = t81824 * t831;
    let t81835 = t23061 * t6604;
    let t81836 = t81835 * t23099;
    (t81810, t81816, t81821, t81824, t81825, t81835, t81836)
}
