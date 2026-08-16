//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1997/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1997(t102951: f64, t25411: f64, t102928: f64, t25387: f64, t28404: f64, t689: f64, t25431: f64, t28384: f64, t1558: f64, t25391: f64, t28425: f64, t95551: f64, t95553: f64, t95556: f64, t95562: f64, t95567: f64, t95569: f64, t95572: f64, t95576: f64, t99155: f64) -> (f64, f64) {
    let t102956 = 0.25702851531048074406e-1_f64 * t25411 * t102951;
    let t102964 = 0.51405703062096148812e-1_f64 * t25387 * t102928;
    let t102967 = t28404 * t689;
    let t102969 = 0.14456046980341999104e-1_f64 * t25431 * t102967;
    let t102971 = 0.25702851531048074406e-1_f64 * t25411 * t102967;
    let t102972 = t28384 * t689;
    let t102974 = 0.14456046980341999104e-1_f64 * t25431 * t102972;
    let t102977 = t102956 - 0.19274729307122665471e-1_f64 * t95551 + 0.34694512752820797848e1_f64 * t25391 * t28425 * t1558 * t99155 - 0.28912093960683998208e-1_f64 * t95553 + t102964 + 0.54878743191129263322e-2_f64 * t95556 - 0.13009920719177044025e-2_f64 * t95562 - t102969 + t102971 - t102974 + t95567 + t95569 - 0.14456046980341999104e-1_f64 * t95572 - 0.19274729307122665471e-1_f64 * t95576;
    (t102972, t102977)
}
