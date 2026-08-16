//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1348/1384 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1348(t11605: f64, t225: f64, t1184: f64, t3470: f64, t3597: f64, t3599: f64, t2122: f64, t7303: f64, t85660: f64, t3590: f64, t7299: f64, t24571: f64, t24574: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t85674 = t225 * t11605;
    let t85683 = t3470 * t1184;
    let t85687 = t3597 * t3599;
    let t85688 = t2122 * t85687;
    let t85701 = t85660 * t7303;
    let t85707 = t7299 * t3590;
    let t85711 = t24574 * t24571;
    (t85674, t85683, t85687, t85688, t85701, t85707, t85711)
}
