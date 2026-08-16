//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 613/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk613(t837: f64, t949: f64, t2741: f64, t357: f64, t956: f64, t339: f64, t349: f64) -> (f64, f64, f64) {
    let t2742 = t949 * t837;
    let t2743 = t2741 * t2742;
    let t2746 = t956 * t357;
    let t2748 = t339 * t349 * t2746;
    (t2742, t2743, t2748)
}
