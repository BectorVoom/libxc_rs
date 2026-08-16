//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1320/1360 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1320(t94570: f64, t94534: f64, t94537: f64, t94540: f64, t94542: f64, t94546: f64, t94548: f64, t94552: f64, t94554: f64, t94557: f64, t94559: f64, t94561: f64, t94565: f64, t94569: f64) -> f64 {
    let t94571 = 0.14450132032386466905e-2_f64 * t94570;
    let t94572 = -0.85748036236139473943e-3_f64 * t94534 + 0.15246000842785598468e-4_f64 * t94537 - 0.1084295579938911763e-3_f64 * t94540 - 0.30492001685571196935e-3_f64 * t94542 - 0.13605355082800796533e0_f64 * t94546 + 0.24009450146119052704e-1_f64 * t94548 - 0.85748036236139473944e-4_f64 * t94552 - 0.45732285992607719437e-3_f64 * t94554 + 0.42874018118069736972e-4_f64 * t94557 - 0.12004725073059526352e0_f64 * t94559 + 0.15246000842785598468e-2_f64 * t94561 - 0.27107389498472794076e-4_f64 * t94565 - t94569 - t94571;
    t94572
}
