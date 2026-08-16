//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1063/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1063(t14570: f64, t866: f64, t846: f64, t4879: f64, t8595: f64, t4838: f64, t845: f64, t867: f64, t2814: f64, t5039: f64, t3894: f64, t3904: f64) -> (f64, f64, f64, f64, f64) {
    let t14571 = t14570 * t866;
    let t14573 = 1.0_f64 * t846 * t14571;
    let t14575 = 0.16081979498692535067e2_f64 * t8595 * t4879;
    let t14576 = t4838 * t845;
    let t14578 = 1.0_f64 * t14576 * t867;
    let t14579 = t5039 * t2814;
    let t14583 = 0.11696447245269292414e1_f64 * t3894 * t3904;
    (t14573, t14575, t14578, t14579, t14583)
}
