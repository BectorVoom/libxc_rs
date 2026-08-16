//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 1481/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1481(t18281: f64, t190: f64, t706: f64, t14441: f64, t10593: f64, t10597: f64, t189: f64, t5819: f64, t606: f64, t14330: f64, t10608: f64, t4308: f64, t4311: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t18569 = t190 * t18281;
    let t18571 = 4.0_f64 * t706 * t18569;
    let t18572 = 8.0_f64 * t14441;
    let t18573 = 0.5848223622634646207e0_f64 * t10593;
    let t18574 = 0.17315859105681463759e2_f64 * t10597;
    let t18575 = t189 * t5819;
    let t18576 = t18575 * t606;
    let t18578 = 24.0_f64 * t14330 * t18576;
    let t18579 = 0.11696447245269292414e1_f64 * t10608;
    let t18581 = 8.0_f64 * t4311 * t4308;
    (t18571, t18572, t18573, t18574, t18578, t18579, t18581)
}
