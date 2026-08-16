//! GGA_C_GAPC lxc pol — lxc_pol part 33 (v4rho2sigma2_12) CSE chunk 215/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part33_v4rho2sigma2_12_chunk215(t291: f64, t474: f64, t4: f64, t773: f64, t139: f64, t286: f64, t124: f64, t495: f64, t498: f64, t288: f64, t483: f64, t486: f64, t494: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t800 = t474 * t291;
    let t801 = t773 * t4;
    let t802 = t800 * t801;
    let t807 = t286 * t139;
    let t808 = t807 * t124;
    let t811 = t286 * t495;
    let t812 = t811 * t498;
    let t815 = t807 * t4;
    let t818 = -0.97071966386951317368e-2_f64 * t483 * t288 - 0.12133995798368914671e-2_f64 * t486 * t808 + 0.12133995798368914671e-3_f64 * t494 * t812 - 0.21574244529499930286e-3_f64 * t494 * t815;
    (t800, t801, t802, t808, t811, t812, t815, t818)
}
