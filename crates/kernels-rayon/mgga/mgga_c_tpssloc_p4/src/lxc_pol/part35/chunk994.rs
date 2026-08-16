//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 994/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk994(t21033: f64, t858: f64, t20936: f64, t252: f64, t1492: f64, t5631: f64, t1527: f64, t5636: f64, t10110: f64, t5657: f64, t2718: f64, t1519: f64, t5558: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t21034 = t858 * t21033;
    let t21036 = t20936 * t252;
    let t21038 = t1492 * t5631;
    let t21049 = t5636 * t1527;
    let t21050 = t10110 * t21049;
    let t21053 = t1527 * t5657;
    let t21054 = t2718 * t21053;
    let t21061 = t5558 * t1519;
    (t21034, t21036, t21038, t21049, t21050, t21053, t21054, t21061)
}
