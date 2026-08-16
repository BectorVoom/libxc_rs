//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2054/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2054(t7291: f64, t85660: f64, t24564: f64, t24574: f64, t11605: f64, t225: f64, t3597: f64, t3599: f64, t2122: f64, t7303: f64, t3590: f64, t7299: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t85661 = t85660 * t7291;
    let t85669 = t24574 * t24564;
    let t85674 = t225 * t11605;
    let t85687 = t3597 * t3599;
    let t85688 = t2122 * t85687;
    let t85701 = t85660 * t7303;
    let t85707 = t7299 * t3590;
    (t85661, t85669, t85674, t85688, t85701, t85707)
}
