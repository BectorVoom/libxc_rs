//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 582/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk582(t2525: f64, t866: f64, t846: f64, t844: f64, t269: f64) -> (f64, f64, f64, f64, f64) {
    let t2526 = t2525 * t866;
    let t2528 = 1.0_f64 * t846 * t2526;
    let t2529 = t844 * t844;
    let t2530 = 1.0_f64 / t2529;
    let t2531 = t269 * t2530;
    (t2526, t2528, t2529, t2530, t2531)
}
