//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 748/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk748(t4210: f64, t8306: f64, t7942: f64, t7965: f64, t7963: f64, t119: f64, t2217: f64, t2219: f64, t310: f64, t635: f64, t848: f64, t633: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t8310 = t8306 * t4210;
    let t8311 = t7942 * t8310;
    let t8313 = t8306 * t7965;
    let t8314 = t7963 * t8313;
    let t8316 = t119 * t2217;
    let t8319 = t310 * t2219;
    let t8330 = 0.65854491829355115987e0_f64 * t848 * t635;
    let t8331 = t310 * t633;
    (t8310, t8311, t8313, t8314, t8316, t8319, t8330, t8331)
}
