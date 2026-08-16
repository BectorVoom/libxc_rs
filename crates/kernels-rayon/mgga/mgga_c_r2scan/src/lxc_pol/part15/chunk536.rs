//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 536/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk536(t113: f64, t285: f64, t498: f64, t983: f64, t792: f64, t537: f64, t910: f64, t2124: f64, t495: f64, t920: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t2534 = t113 * t285;
    let t2538 = t498 * t983;
    let t2541 = t983 * t792;
    let t2545 = t537 * t910;
    let t2547 = t2124 * t2545 * t495;
    let t2550 = t537 * t920;
    (t2534, t2538, t2541, t2545, t2547, t2550)
}
