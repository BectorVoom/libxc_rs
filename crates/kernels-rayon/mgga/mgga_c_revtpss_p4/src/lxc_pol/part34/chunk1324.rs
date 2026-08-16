//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1324/1341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1324(t22860: f64, t94493: f64, t22854: f64, t7271: f64, t22956: f64, t7264: f64, t22822: f64, t22815: f64, t108604: f64, t108608: f64, t108623: f64, t108625: f64, t108627: f64, t108629: f64, t94569: f64, t94571: f64, t98285: f64) -> f64 {
    let t114573 = t94493 * t22860;
    let t114575 = t7271 * t22854;
    let t114577 = t7264 * t22956;
    let t114584 = t7271 * t22822;
    let t114586 = t7271 * t22815;
    let t114588 = -0.85748036236139473944e-4_f64 * t108604 - 0.30492001685571196935e-3_f64 * t108608 - 0.25724410870841842183e-2_f64 * t114573 + 0.25724410870841842184e-1_f64 * t114575 - 0.42874018118069736972e-3_f64 * t114577 - t94569 - t94571 - 0.1084295579938911763e-3_f64 * t98285 + 0.42874018118069736972e-4_f64 * t108623 + 0.15246000842785598468e-2_f64 * t108625 - 0.12004725073059526352e0_f64 * t108627 + 0.24009450146119052704e-1_f64 * t108629 - 0.17149607247227894789e-2_f64 * t114584 - 0.51448821741683684367e-1_f64 * t114586;
    t114588
}
