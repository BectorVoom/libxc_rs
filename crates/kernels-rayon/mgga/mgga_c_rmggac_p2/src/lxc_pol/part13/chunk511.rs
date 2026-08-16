//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 511/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk511(t221: f64, t446: f64, t5605: f64, t1475: f64, t998: f64, t1494: f64, t476: f64, t209: f64, t1184: f64, t1212: f64, t1468: f64, t1515: f64, t1516: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t5607 = t221 * t5605 * t446;
    let t5611 = t221 * t1475 * t998;
    let t5614 = t1494 * t476;
    let t5615 = t5614 * t209;
    let t5616 = t221 * t5615;
    let t5619 = t1494 * t1184;
    let t5620 = t5619 * t476;
    let t5621 = t221 * t5620;
    let t5624 = t1468 * t1212;
    let t5625 = t221 * t5624;
    let t5630 = t1515 * t1516 * t998;
    (t5607, t5611, t5615, t5616, t5620, t5621, t5624, t5625, t5630)
}
