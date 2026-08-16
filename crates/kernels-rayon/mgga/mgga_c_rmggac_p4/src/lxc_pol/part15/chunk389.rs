//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 389/1110 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk389(t574: f64, t640: f64, t558: f64, t649: f64, t27: f64, t570: f64, t534: f64, t71: f64) -> (f64, f64, f64, f64) {
    let t2323 = t640 * t574;
    let t2328 = t649 * t558;
    let t2329 = t27 * t2328;
    let t2332 = t649 * t570;
    let t2333 = t27 * t2332;
    let t2338 = t71 * t534;
    (t2323, t2329, t2333, t2338)
}
